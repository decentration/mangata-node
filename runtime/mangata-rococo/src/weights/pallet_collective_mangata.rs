// This file is part of Mangata.

// Copyright (C) 2020-2022 Mangata Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_collective_mangata
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama"), DB CACHE: 1024

// Executed Command:
// target/release/mangata-node
// benchmark
// pallet
// -l=info,xyk=error,collective-mangata=warn,bootstrap=warn
// --chain
// kusama
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template
// ./templates/module-weight-template.hbs
// --output
// ./benchmarks/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_collective_mangata.
pub trait WeightInfo {
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight;
	fn execute(b: u32, m: u32, ) -> Weight;
	fn propose_execute(b: u32, m: u32, ) -> Weight;
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight;
	fn vote(m: u32, ) -> Weight;
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight;
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight;
	fn close_disapproved(m: u32, p: u32, ) -> Weight;
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight;
	fn disapprove_proposal(p: u32, ) -> Weight;
}

/// Weights for pallet_collective_mangata using the Mangata node and recommended hardware.
pub struct ModuleWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective_mangata::WeightInfo for ModuleWeight<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Voting (r:100 w:100)
	// Storage: Council Prime (r:0 w:1)
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(88_200_000))
			// Standard Error: 54_474
			.saturating_add((Weight::from_ref_time(4_335_303)).saturating_mul(m as u64))
			// Standard Error: 54_474
			.saturating_add((Weight::from_ref_time(8_584_322)).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: Council Members (r:1 w:0)
	fn execute(b: u32, m: u32, ) -> Weight {
		(Weight::from_ref_time(34_718_802))
			// Standard Error: 78
			.saturating_add((Weight::from_ref_time(1_902)).saturating_mul(b as u64))
			// Standard Error: 808
			.saturating_add((Weight::from_ref_time(23_278)).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		(Weight::from_ref_time(37_653_564))
			// Standard Error: 91
			.saturating_add((Weight::from_ref_time(2_506)).saturating_mul(b as u64))
			// Standard Error: 942
			.saturating_add((Weight::from_ref_time(34_864)).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:0 w:1)
	// Storage: Council Voting (r:0 w:1)
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(48_763_101))
			// Standard Error: 167
			.saturating_add((Weight::from_ref_time(5_627)).saturating_mul(b as u64))
			// Standard Error: 1_745
			.saturating_add((Weight::from_ref_time(38_909)).saturating_mul(m as u64))
			// Standard Error: 1_723
			.saturating_add((Weight::from_ref_time(276_072)).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	fn vote(m: u32, ) -> Weight {
		(Weight::from_ref_time(53_098_075))
			// Standard Error: 1_452
			.saturating_add((Weight::from_ref_time(55_177)).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(66_640_835))
			// Standard Error: 1_816
			.saturating_add((Weight::from_ref_time(49_001)).saturating_mul(m as u64))
			// Standard Error: 1_770
			.saturating_add((Weight::from_ref_time(240_158)).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(86_880_904))
			// Standard Error: 243
			.saturating_add((Weight::from_ref_time(3_349)).saturating_mul(b as u64))
			// Standard Error: 2_577
			.saturating_add((Weight::from_ref_time(56_626)).saturating_mul(m as u64))
			// Standard Error: 2_512
			.saturating_add((Weight::from_ref_time(265_201)).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(70_370_119))
			// Standard Error: 1_899
			.saturating_add((Weight::from_ref_time(50_837)).saturating_mul(m as u64))
			// Standard Error: 1_851
			.saturating_add((Weight::from_ref_time(241_458)).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(90_986_041))
			// Standard Error: 229
			.saturating_add((Weight::from_ref_time(3_603)).saturating_mul(b as u64))
			// Standard Error: 2_429
			.saturating_add((Weight::from_ref_time(54_870)).saturating_mul(m as u64))
			// Standard Error: 2_368
			.saturating_add((Weight::from_ref_time(262_444)).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:0 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32, ) -> Weight {
		(Weight::from_ref_time(41_286_428))
			// Standard Error: 1_851
			.saturating_add((Weight::from_ref_time(245_206)).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Voting (r:100 w:100)
	// Storage: Council Prime (r:0 w:1)
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(88_200_000))
			// Standard Error: 54_474
			.saturating_add((Weight::from_ref_time(4_335_303)).saturating_mul(m as u64))
			// Standard Error: 54_474
			.saturating_add((Weight::from_ref_time(8_584_322)).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: Council Members (r:1 w:0)
	fn execute(b: u32, m: u32, ) -> Weight {
		(Weight::from_ref_time(34_718_802))
			// Standard Error: 78
			.saturating_add((Weight::from_ref_time(1_902)).saturating_mul(b as u64))
			// Standard Error: 808
			.saturating_add((Weight::from_ref_time(23_278)).saturating_mul(m as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		(Weight::from_ref_time(37_653_564))
			// Standard Error: 91
			.saturating_add((Weight::from_ref_time(2_506)).saturating_mul(b as u64))
			// Standard Error: 942
			.saturating_add((Weight::from_ref_time(34_864)).saturating_mul(m as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:0 w:1)
	// Storage: Council Voting (r:0 w:1)
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(48_763_101))
			// Standard Error: 167
			.saturating_add((Weight::from_ref_time(5_627)).saturating_mul(b as u64))
			// Standard Error: 1_745
			.saturating_add((Weight::from_ref_time(38_909)).saturating_mul(m as u64))
			// Standard Error: 1_723
			.saturating_add((Weight::from_ref_time(276_072)).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	fn vote(m: u32, ) -> Weight {
		(Weight::from_ref_time(53_098_075))
			// Standard Error: 1_452
			.saturating_add((Weight::from_ref_time(55_177)).saturating_mul(m as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(66_640_835))
			// Standard Error: 1_816
			.saturating_add((Weight::from_ref_time(49_001)).saturating_mul(m as u64))
			// Standard Error: 1_770
			.saturating_add((Weight::from_ref_time(240_158)).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(86_880_904))
			// Standard Error: 243
			.saturating_add((Weight::from_ref_time(3_349)).saturating_mul(b as u64))
			// Standard Error: 2_577
			.saturating_add((Weight::from_ref_time(56_626)).saturating_mul(m as u64))
			// Standard Error: 2_512
			.saturating_add((Weight::from_ref_time(265_201)).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(70_370_119))
			// Standard Error: 1_899
			.saturating_add((Weight::from_ref_time(50_837)).saturating_mul(m as u64))
			// Standard Error: 1_851
			.saturating_add((Weight::from_ref_time(241_458)).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(Weight::from_ref_time(90_986_041))
			// Standard Error: 229
			.saturating_add((Weight::from_ref_time(3_603)).saturating_mul(b as u64))
			// Standard Error: 2_429
			.saturating_add((Weight::from_ref_time(54_870)).saturating_mul(m as u64))
			// Standard Error: 2_368
			.saturating_add((Weight::from_ref_time(262_444)).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalProposedTime (r:0 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32, ) -> Weight {
		(Weight::from_ref_time(41_286_428))
			// Standard Error: 1_851
			.saturating_add((Weight::from_ref_time(245_206)).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
}
