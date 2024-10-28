//! # Market Pallet.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::{DispatchErrorWithPostInfo, PostDispatchInfo},
	ensure, fail,
	pallet_prelude::*,
	traits::{
		tokens::{currency::MultiTokenVestingLocks, Balance, CurrencyId},
		Contains, Currency, ExistenceRequirement, MultiTokenCurrency, WithdrawReasons,
	},
	transactional, PalletId,
};
use frame_system::pallet_prelude::*;
use mangata_support::{
	pools::{Inspect, Mutate},
	traits::{
		ActivationReservesProviderTrait, AssetRegistryProviderTrait, GetMaintenanceStatusTrait,
		ProofOfStakeRewardsApi, XykFunctionsTrait,
	},
};
use mangata_types::multipurpose_liquidity::ActivateKind;

use sp_arithmetic::traits::Unsigned;
use sp_runtime::{
	traits::{
		checked_pow, AccountIdConversion, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Ensure,
		One, Saturating, TrailingZeroInput, Zero,
	},
	ModuleError,
};
use sp_std::{convert::TryInto, fmt::Debug, vec, vec::Vec};

use orml_tokens::MultiTokenCurrencyExtended;
use orml_traits::asset_registry::Inspect as AssetRegistryInspect;

mod weights;
use crate::weights::WeightInfo;

pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Eq, PartialEq, Debug, Clone, TypeInfo)]
pub enum PoolKind {
	/// Classic XYK invariant
	Xyk,
	/// StableSwap
	StableSwap,
}

#[derive(Clone)]
pub struct PoolInfo<CurrencyId> {
	pool_id: CurrencyId,
	kind: PoolKind,
	pool: mangata_support::pools::PoolInfo<CurrencyId>,
}

// use LP token as pool id, extra type for readability
pub type PoolIdOf<T> = <T as Config>::CurrencyId;
// pools are composed of a pair of assets
pub type PoolInfoOf<T> = PoolInfo<<T as Config>::CurrencyId>;
pub type AssetPairOf<T> = (<T as Config>::CurrencyId, <T as Config>::CurrencyId);

#[frame_support::pallet]
pub mod pallet {
	use sp_runtime::ModuleError;

	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Currency type that this works on.
		type Currency: MultiTokenCurrencyExtended<
			Self::AccountId,
			Balance = Self::Balance,
			CurrencyId = Self::CurrencyId,
		>;

		/// The `Currency::Balance` type of the currency.
		type Balance: Balance;

		/// Identifier for the assets.
		type CurrencyId: CurrencyId;

		/// Native currency
		type NativeCurrencyId: Get<Self::CurrencyId>;

		/// Xyk pools
		type Xyk: XykFunctionsTrait<Self::AccountId, Self::Balance, Self::CurrencyId>
			+ Inspect<Self::AccountId, CurrencyId = Self::CurrencyId>;

		/// StableSwap pools
		type StableSwap: Mutate<
			Self::AccountId,
			CurrencyId = Self::CurrencyId,
			Balance = Self::Balance,
		>;

		/// Reward apis for native asset LP tokens activation
		type Rewards: ProofOfStakeRewardsApi<Self::AccountId, Self::Balance, Self::CurrencyId>;

		// type LiquidityReservations: ActivationReservesProviderTrait<
		// 	Self::AccountId,
		// 	Self::Balance,
		// 	Self::CurrencyId,
		// >;

		/// Vesting apis for providing native vested liquidity
		type Vesting: MultiTokenVestingLocks<
			Self::AccountId,
			Moment = BlockNumberFor<Self>,
			Currency = Self::Currency,
		>;

		/// Apis for LP asset creation in asset registry
		type AssetRegistry: AssetRegistryProviderTrait<Self::CurrencyId>
			+ AssetRegistryInspect<AssetId = Self::CurrencyId>;

		/// List of tokens ids that are not allowed to be used at all
		type DisabledTokens: Contains<Self::CurrencyId>;

		/// List of assets that are not allowed to form a pool
		type DisallowedPools: Contains<AssetPairOf<Self>>;

		/// Disable trading with maintenance mode
		type MaintenanceStatusProvider: GetMaintenanceStatusTrait;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// No such pool exists
		NoSuchPool,
		/// Asset id is not allowed
		FunctionNotAvailableForThisToken,
		/// Asset ids are not allowed to create a pool
		DisallowedPool,
		/// Insufficient output amount does not meet min requirements
		InsufficientOutputAmount,
		/// Pool is not paired with native currency id
		NotPairedWithNativeAsset,
		/// Not a promoted pool
		NotAPromotedPool,
		/// Asset does not exists
		AssetDoesNotExists,
		/// Operation not available for such pool type
		FunctionNotAvailableForThisPoolKind,
		/// Trading blocked by maintenance mode
		TradingBlockedByMaintenanceMode,
		/// Multi swap path contains repetive pools
		MultiSwapSamePool,
		/// Input asset id is not connected with output asset id for given pools
		MultiSwapPathInvalid,
		/// Unexpected failure
		UnexpectedFailure,
	}

	// Pallet's events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Atomic swap failed
		MultiSwapAssetFailedOnAtomicSwap {
			who: T::AccountId,
			swap_pool_list: Vec<PoolIdOf<T>>,
			swap_assets_list: Vec<AssetPairOf<T>>,
			module_err: ModuleError,
		},

		/// Assets where swapped successfully
		AssetsSwapped {
			who: T::AccountId,
			swap_pool_list: Vec<PoolIdOf<T>>,
			swap_assets_list: Vec<AssetPairOf<T>>,
			amount_in: T::Balance,
			amount_out: T::Balance,
		},
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn integrity_test() {
			assert!(true, "template",);
		}
	}

	/// Pallet's callable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates a liquidity pool and an associated new `lp_token` asset
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_pool())]
		pub fn create_pool(
			origin: OriginFor<T>,
			kind: PoolKind,
			first_asset_id: T::CurrencyId,
			first_asset_amount: T::Balance,
			second_asset_id: T::CurrencyId,
			second_asset_amount: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::check_assets_allowed((first_asset_id, second_asset_id))?;

			ensure!(
				!T::DisallowedPools::contains(&(first_asset_id, second_asset_id)),
				Error::<T>::DisallowedPool,
			);

			match kind {
				PoolKind::Xyk => T::Xyk::create_pool(
					sender,
					first_asset_id,
					first_asset_amount,
					second_asset_id,
					second_asset_amount,
				)?,
				PoolKind::StableSwap => {
					let first_decimal = T::AssetRegistry::metadata(&first_asset_id)
						.map(|meta| meta.decimals)
						.ok_or(Error::<T>::AssetDoesNotExists)?;
					let second_decimal = T::AssetRegistry::metadata(&second_asset_id)
						.map(|meta| meta.decimals)
						.ok_or(Error::<T>::AssetDoesNotExists)?;

					let lp_token = T::StableSwap::create_pool(
						&sender,
						first_asset_id,
						first_decimal,
						second_asset_id,
						second_decimal,
					)?;

					T::StableSwap::add_liquidity(
						&sender,
						lp_token,
						(first_asset_amount, second_asset_amount),
						Zero::zero(),
					)?;

					T::AssetRegistry::create_pool_asset(lp_token, first_asset_id, second_asset_id)?;
				},
			}

			Ok(())
		}

		/// Provide liquidity into the pool of `pool_id`, suitable for Xyk pools.
		/// An optimal amount of the other asset will be calculated on current rate,
		/// a maximum amount should be provided to limit possible rate slippage.
		/// For a StableSwap pool a rate of 1:1 is used.
		/// Liquidity tokens that represent this share of the pool will be sent to origin.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::mint_liquidity())]
		pub fn mint_liquidity(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			asset_id: T::CurrencyId,
			asset_amount: T::Balance,
			max_other_asset_amount: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let pool_info = Self::get_pool_info(pool_id)?;
			Self::check_assets_allowed(pool_info.pool)?;

			Self::do_mint_liquidity(
				&sender,
				pool_info,
				asset_id,
				asset_amount,
				max_other_asset_amount,
				true,
			)?;
			Ok(())
		}

		/// Provide fixed liquidity into the pool of `pool_id`, suitable for StableSwap pools.
		/// For Xyk pools, if a single amount is defined, it will swap internally to to match current rate,
		/// setting both values results in error.
		/// Liquidity tokens that represent this share of the pool will be sent to origin.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::mint_liquidity())]
		pub fn mint_liquidity_fixed_amounts(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			amounts: (T::Balance, T::Balance),
			min_amount_lp_tokens: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let pool_info = Self::get_pool_info(pool_id)?;
			Self::check_assets_allowed(pool_info.pool)?;

			match pool_info.kind {
				PoolKind::Xyk => {
					ensure!(
						amounts.0 == Zero::zero() || amounts.1 == Zero::zero(),
						Error::<T>::FunctionNotAvailableForThisPoolKind
					);

					let (id, amount) = if amounts.1 == Zero::zero() {
						(pool_info.pool.0, amounts.0)
					} else {
						(pool_info.pool.1, amounts.1)
					};

					let (_, lp_amout) = T::Xyk::provide_liquidity_with_conversion(
						sender,
						pool_info.pool.0,
						pool_info.pool.1,
						id,
						amount,
						true,
					)?;
					ensure!(lp_amout > min_amount_lp_tokens, Error::<T>::InsufficientOutputAmount);
				},
				PoolKind::StableSwap => {
					let amount = T::StableSwap::add_liquidity(
						&sender,
						pool_id,
						amounts,
						min_amount_lp_tokens,
					)?;
					T::Rewards::activate_liquidity(
						sender.clone(),
						pool_id,
						amount,
						Some(ActivateKind::AvailableBalance),
					)?;
				},
			}

			Ok(())
		}

		/// Provides liquidity from vested native asset. Tokens are added to pool and
		/// minted LP tokens are then vested instead.
		/// Only pools paired with native asset are allowed.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::mint_liquidity())]
		pub fn mint_liquidity_using_vesting_native_tokens_by_vesting_index(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			native_asset_vesting_index: u32,
			vesting_native_asset_unlock_some_amount_or_all: Option<T::Balance>,
			max_other_asset_amount: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let pool_info = Self::get_pool_info(pool_id)?;
			Self::check_assets_allowed(pool_info.pool)?;

			let native_id = T::NativeCurrencyId::get();
			ensure!(
				native_id == pool_info.pool.0 || native_id == pool_info.pool.1,
				Error::<T>::NotPairedWithNativeAsset
			);

			ensure!(T::Rewards::native_rewards_enabled(pool_id), Error::<T>::NotAPromotedPool);

			let (unlocked_amount, vesting_starting_block, vesting_ending_block_as_balance) =
				T::Vesting::unlock_tokens_by_vesting_index(
					&sender,
					native_id,
					native_asset_vesting_index,
					vesting_native_asset_unlock_some_amount_or_all,
				)?;

			let lp_amount = Self::do_mint_liquidity(
				&sender,
				pool_info,
				native_id,
				unlocked_amount,
				max_other_asset_amount,
				false,
			)?;

			T::Vesting::lock_tokens(
				&sender,
				pool_id,
				lp_amount,
				Some(vesting_starting_block),
				vesting_ending_block_as_balance,
			)?;

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::mint_liquidity())]
		pub fn mint_liquidity_using_vesting_native_tokens(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			native_asset_vesting_amount: T::Balance,
			max_other_asset_amount: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let pool_info = Self::get_pool_info(pool_id)?;
			Self::check_assets_allowed(pool_info.pool)?;

			let native_id = T::NativeCurrencyId::get();
			ensure!(
				native_id == pool_info.pool.0 || native_id == pool_info.pool.1,
				Error::<T>::NotPairedWithNativeAsset
			);

			ensure!(T::Rewards::native_rewards_enabled(pool_id), Error::<T>::NotAPromotedPool);

			let (vesting_starting_block, vesting_ending_block_as_balance) =
				T::Vesting::unlock_tokens(&sender, native_id, native_asset_vesting_amount)?;

			let lp_amount = Self::do_mint_liquidity(
				&sender,
				pool_info,
				native_id,
				native_asset_vesting_amount,
				max_other_asset_amount,
				false,
			)?;

			T::Vesting::lock_tokens(
				&sender,
				pool_id,
				lp_amount,
				Some(vesting_starting_block),
				vesting_ending_block_as_balance,
			)?;

			Ok(())
		}

		/// Allows you to remove liquidity by providing the `lp_burn_amount` tokens that will be
		/// burned in the process. The usage of `min_first_asset_amount`/`min_second_asset_amount`
		/// controls the min amount of returned tokens.
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::burn_liquidity())]
		pub fn burn_liquidity(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			liquidity_burn_amount: T::Balance,
			min_first_asset_amount: T::Balance,
			min_second_asset_amount: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let pool_info = Self::get_pool_info(pool_id)?;
			Self::check_assets_allowed(pool_info.pool)?;

			match pool_info.kind {
				PoolKind::Xyk => {
					// todo min amouonts
					T::Xyk::burn_liquidity(
						sender,
						pool_info.pool.0,
						pool_info.pool.1,
						liquidity_burn_amount,
					)?;
				},
				PoolKind::StableSwap => {
					// deactivate liquidity if low balance
					let balance = T::Currency::available_balance(pool_id, &sender);
					let deactivate = liquidity_burn_amount.saturating_sub(balance);
					// noop on zero amount
					T::Rewards::deactivate_liquidity(sender.clone(), pool_id, deactivate)?;

					T::StableSwap::remove_liquidity(
						&sender,
						pool_id,
						liquidity_burn_amount,
						(min_first_asset_amount, min_second_asset_amount),
					)?;
				},
			}

			Ok(())
		}

		/// Executes a multiswap asset in a series of swap asset atomic swaps.
		///
		/// Multiswaps must fee lock instead of paying transaction fees.
		///
		/// First the multiswap is prevalidated, if it is successful then the extrinsic is accepted
		/// and the exchange commission will be charged upon execution on the **first** swap using **sold_asset_amount**.
		///
		/// Upon failure of an atomic swap or bad slippage, all the atomic swaps are reverted and the exchange commission is charged.
		/// Upon such a failure, the extrinsic is marked "successful", but an event for the failure is emitted
		///
		/// # Args:
		/// - `swap_token_list` - This list of tokens is the route of the atomic swaps, starting with the asset sold and ends with the asset finally bought
		/// - `asset_id_in`: The id of the asset sold
		/// - `asset_amount_in`: The amount of the asset sold
		/// - `asset_id_out`: The id of the asset received
		/// - `min_amount_out` - The minimum amount of requested asset that must be bought in order to not fail on slippage. Slippage failures still charge exchange commission.
		// This call is part of the fee lock mechanism, which allows free execution in some cases
		// in case of an error a 'trade fee' is subtracted from input swap asset to avoid DOS attacks
		// `OnChargeTransaction` impl should check whether the sender has funds to cover such fee
		// or consider transaction invalid
		#[pallet::call_index(6)]
		#[pallet::weight((T::WeightInfo::multiswap_asset(swap_pool_list.len() as u32), DispatchClass::Operational, Pays::No))]
		pub fn multiswap_asset(
			origin: OriginFor<T>,
			swap_pool_list: Vec<PoolIdOf<T>>,
			asset_id_in: T::CurrencyId,
			asset_amount_in: T::Balance,
			asset_id_out: T::CurrencyId,
			min_amount_out: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			// ensure maintenance mode
			ensure!(
				!T::MaintenanceStatusProvider::is_maintenance(),
				Error::<T>::TradingBlockedByMaintenanceMode
			);

			// let path = Self::get_path_for_in(&swap_pool_list)?;
			// Self::validate_path()?;

			// at least one swap
			ensure!(swap_pool_list.len() > 0, Error::<T>::NoSuchPool);

			// check pools repetition
			let mut dedup = swap_pool_list.clone();
			dedup.sort();
			dedup.dedup();
			ensure!(dedup.len() == swap_pool_list.len(), Error::<T>::MultiSwapSamePool);

			let mut path: Vec<AssetPairOf<T>> = vec![];
			let mut pools: Vec<PoolInfoOf<T>> = vec![];
			for &pool_id in swap_pool_list.iter() {
				let pool_info = Self::get_pool_info(pool_id)?;
				pools.push(pool_info.clone());
				// function not available for tokens
				Self::check_assets_allowed(pool_info.pool)?;

				// check pools' asset connection
				// first is asset_id_in, last is asset_id_out
				let prev_asset_id =
					if let Some(&last) = path.last() { last.1 } else { asset_id_in };

				if pool_info.pool.0 == prev_asset_id {
					path.push(pool_info.pool);
				} else if pool_info.pool.1 == prev_asset_id {
					path.push((pool_info.pool.1, pool_info.pool.0));
				} else {
					fail!(Error::<T>::MultiSwapPathInvalid)
				}
			}

			ensure!(
				path.last().is_some_and(|&l| l.1 == asset_id_out),
				Error::<T>::MultiSwapPathInvalid
			);

			// due to fee lock
			// check sender's balance to pay for the trade fee not needed
			// such check should be in `OnChargeTransaction` for runtime to allow fee lock

			match frame_support::storage::with_storage_layer(|| -> Result<T::Balance, DispatchError> {
				// atomic swaps, reverts on error
				Self::do_swaps(&sender, pools, path.clone(), asset_amount_in, min_amount_out)
			}) {
				Ok(amount_out) => {
					// deposit event swapped ok
					Self::deposit_event(Event::AssetsSwapped {
						who: sender.clone(),
						swap_pool_list: swap_pool_list.clone(),
						swap_assets_list: path,
						amount_in: asset_amount_in,
						amount_out,
					});

					Ok(())
				},
				Err(e) => {
					// charge fee
					// deposit failed event
					if let DispatchError::Module(module_err) = e {
						Self::deposit_event(Event::MultiSwapAssetFailedOnAtomicSwap {
							who: sender.clone(),
							swap_pool_list: swap_pool_list.clone(),
							swap_assets_list: path,
							module_err,
						});
						Err(e)
					} else {
						Err(Error::<T>::UnexpectedFailure.into())
					}
				},
			}
			// unexpected error within above
			.map_err(|err| DispatchErrorWithPostInfo {
				post_info: PostDispatchInfo {
					actual_weight: Some(
						T::WeightInfo::multiswap_asset(swap_pool_list.len() as u32),
					),
					pays_fee: Pays::Yes,
				},
				error: err,
			})?;

			// total swaps inc

			Ok(Pays::No.into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn get_pool_info(pool_id: PoolIdOf<T>) -> Result<PoolInfoOf<T>, Error<T>> {
			if let Some(pool) = T::Xyk::get_pool_info(pool_id) {
				return Ok(PoolInfo { pool_id, kind: PoolKind::Xyk, pool })
			}
			if let Some(pool) = T::StableSwap::get_pool_info(pool_id) {
				return Ok(PoolInfo { pool_id, kind: PoolKind::StableSwap, pool })
			}
			
			return Err(Error::<T>::NoSuchPool);
		}

		fn check_assets_allowed(assets: AssetPairOf<T>) -> DispatchResult {
			ensure!(
				!T::DisabledTokens::contains(&assets.0) && !T::DisabledTokens::contains(&assets.1),
				Error::<T>::FunctionNotAvailableForThisToken
			);
			Ok(())
		}

		fn do_mint_liquidity(
			sender: &T::AccountId,
			pool_info: PoolInfoOf<T>,
			asset_id: T::CurrencyId,
			amount: T::Balance,
			max_amount: T::Balance,
			activate: bool,
		) -> Result<T::Balance, DispatchError> {
			let (asset_with_amount, asset_other) = if asset_id == pool_info.pool.0 {
				pool_info.pool
			} else {
				(pool_info.pool.1, pool_info.pool.0)
			};

			let lp_amount = match pool_info.kind {
				PoolKind::Xyk => {
					let (_, amount) = T::Xyk::mint_liquidity(
						sender.clone(),
						asset_with_amount,
						asset_other,
						amount,
						max_amount,
						activate,
					)?;
					amount
				},
				PoolKind::StableSwap => {
					// use 1:1 rate for amounts
					let amount = T::StableSwap::add_liquidity(
						&sender,
						pool_info.pool_id,
						(amount, amount),
						Zero::zero(),
					)?;
					if activate && T::Rewards::native_rewards_enabled(pool_info.pool_id) {
						T::Rewards::activate_liquidity(
							sender.clone(),
							pool_info.pool_id,
							amount,
							Some(ActivateKind::AvailableBalance),
						)?;
					}
					amount
				},
			};

			Ok(lp_amount)
		}

		fn do_swaps(
			sender: &T::AccountId,
			pools: Vec<PoolInfoOf<T>>,
			swaps: Vec<AssetPairOf<T>>,
			amount_in: T::Balance,
			min_amount_out: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let mut amount_out = amount_in;
			for (pool, swap) in pools.iter().zip(swaps.into_iter()) {
				amount_out = match pool.kind {
					PoolKind::StableSwap => T::StableSwap::swap(
						sender,
						pool.pool_id,
						swap.0,
						swap.1,
						amount_out,
						Zero::zero(),
					)?,
					PoolKind::Xyk => T::Xyk::sell_asset(
						sender.clone(),
						swap.0,
						swap.1,
						amount_out,
						Zero::zero(),
						true,
					)?,
				}
			}

			ensure!(amount_out >= min_amount_out, Error::<T>::InsufficientOutputAmount);

			Ok(amount_out)
		}
	}
}

// sp_api::decl_runtime_apis! {
// 	/// This runtime api allows people to query the size of the liquidity pools
// 	/// and quote prices for swaps.
// 	pub trait MarketApi<Balance, AssetId>
// 	where
// 		Balance: frame_support::traits::tokens::Balance + MaybeDisplay,
// 		AssetId: Codec,
// 	{
// 		fn calculate_sell_price(
// 			pool_id: AssetId,
// 			sell_asset_id: AssetId,
// 			sell_amount: Balance
// 		) -> Balance;

// 		fn calculate_buy_price(
// 			pool_id: AssetId,
// 			buy_asset_id: AssetId,
// 			buy_amount: Balance
// 		) -> Balance;

// 		fn get_burn_amount(
// 			pool_id: AssetId,
// 			lp_burn_amount: Balance,
// 		) -> (Balance, Balance);

// 		fn get_max_instant_burn_amount(
// 			user: AccountId,
// 			liquidity_asset_id: AssetId,
// 		) -> Balance;

// 		fn get_max_instant_unreserve_amount(
// 			user: AccountId,
// 			liquidity_asset_id: AssetId,
// 		) -> Balance;

// 		fn calculate_rewards_amount(
// 			user: AccountId,
// 			liquidity_asset_id: AssetId,
// 		) -> Balance;

// 		fn calculate_balanced_sell_amount(
// 			total_amount: Balance,
// 			reserve_amount: Balance,
// 		) -> Balance;

// 		fn is_buy_asset_lock_free(
// 			path: sp_std::vec::Vec<AssetId>,
// 			input_amount: Balance,
// 		) -> Option<bool>;

// 		fn is_sell_asset_lock_free(
// 			path: sp_std::vec::Vec<AssetId>,
// 			input_amount: Balance,
// 		) -> Option<bool>;

// 		fn get_tradeable_tokens() -> Vec<RpcAssetMetadata<AssetId>>;

// 		fn get_pools_for_trading(
// 		) -> Vec<AssetId>;

// 		fn get_total_number_of_swaps() -> u128;
// 	}
// }
