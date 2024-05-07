// Copyright (C) 2021 Mangata team
#![cfg_attr(not(feature = "std"), no_std)]

use sp_core::{Decode, Encode, H256};
use sp_std::vec::Vec;
sp_api::decl_runtime_apis! {
	pub trait RolldownRuntimeApi<L1Update, L1> where
	L1Update: Decode, L1: Encode {
		fn get_pending_updates_hash() -> H256;
		fn get_pending_updates() -> Vec<u8>;
		fn get_native_l1_update(hex_payload: Vec<u8>) -> Option<L1Update>;
		fn verify_pending_requests(hash: H256, request_id: u128) -> Option<bool>;
		fn get_last_processed_request_on_l2(l1: L1) -> Option<u128>;
		fn get_number_of_pending_requests(l1: L1) -> Option<u128>;
		fn get_total_number_of_deposits() -> u32;
		fn get_total_number_of_withdrawals() -> u32;
	}
}
