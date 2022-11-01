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

//! Autogenerated weights for pallet_elections_phragmen
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// /home/ubuntu/mangata-node/scripts/..//target/release/mangata-node
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_elections_phragmen
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./benchmarks/pallet_elections_phragmen_weights.rs
// --template
// ./templates/module-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_elections_phragmen.
pub trait WeightInfo {
	fn vote_equal(v: u32, ) -> Weight;
	fn vote_more(v: u32, ) -> Weight;
	fn vote_less(v: u32, ) -> Weight;
	fn remove_voter() -> Weight;
	fn submit_candidacy(c: u32, ) -> Weight;
	fn renounce_candidacy_candidate(c: u32, ) -> Weight;
	fn renounce_candidacy_members() -> Weight;
	fn renounce_candidacy_runners_up() -> Weight;
	fn remove_member_without_replacement() -> Weight;
	fn remove_member_with_replacement() -> Weight;
	fn remove_member_wrong_refund() -> Weight;
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight;
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight;
}

/// Weights for pallet_elections_phragmen using the Mangata node and recommended hardware.
pub struct ModuleWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for ModuleWeight<T> {
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	fn vote_equal(v: u32, ) -> Weight {
		(Weight::from_ref_time(36_052_000))
			// Standard Error: 4_000
			.saturating_add((Weight::from_ref_time(357_000)).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	fn vote_more(v: u32, ) -> Weight {
		(Weight::from_ref_time(55_043_000))
			// Standard Error: 6_000
			.saturating_add((Weight::from_ref_time(351_000)).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	fn vote_less(v: u32, ) -> Weight {
		(Weight::from_ref_time(50_626_000))
			// Standard Error: 5_000
			.saturating_add((Weight::from_ref_time(399_000)).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn remove_voter() -> Weight {
		(Weight::from_ref_time(50_758_000))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	fn submit_candidacy(c: u32, ) -> Weight {
		(Weight::from_ref_time(52_760_000))
			// Standard Error: 2_000
			.saturating_add((Weight::from_ref_time(164_000)).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		(Weight::from_ref_time(44_733_000))
			// Standard Error: 3_000
			.saturating_add((Weight::from_ref_time(75_000)).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn renounce_candidacy_members() -> Weight {
		(Weight::from_ref_time(57_321_000))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn renounce_candidacy_runners_up() -> Weight {
		(Weight::from_ref_time(41_693_000))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn remove_member_without_replacement() -> Weight {
		(Weight::from_ref_time(250_000_000_000))
	}
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Elections Members (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn remove_member_with_replacement() -> Weight {
		(Weight::from_ref_time(66_871_000))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Elections Voting (r:251 w:250)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Tokens Locks (r:250 w:250)
	// Storage: Tokens Accounts (r:250 w:250)
	// Storage: System Account (r:250 w:250)
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight {
		(Weight::from_ref_time(0))
			// Standard Error: 54_000
			.saturating_add((Weight::from_ref_time(80_487_000)).saturating_mul(v as u64))
			// Standard Error: 52_000
			.saturating_add((Weight::from_ref_time(89_000)).saturating_mul(d as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((4 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes((4 as u64).saturating_mul(v as u64)))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Elections Voting (r:502 w:0)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Elections ElectionRounds (r:1 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		(Weight::from_ref_time(0))
			// Standard Error: 1_757_000
			.saturating_add((Weight::from_ref_time(18_761_000)).saturating_mul(c as u64))
			// Standard Error: 730_000
			.saturating_add((Weight::from_ref_time(50_599_000)).saturating_mul(v as u64))
			// Standard Error: 49_000
			.saturating_add((Weight::from_ref_time(3_256_000)).saturating_mul(e as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn vote_equal(v: u32, ) -> Weight {
		(Weight::from_ref_time(36_052_000))
			// Standard Error: 4_000
			.saturating_add((Weight::from_ref_time(357_000)).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn vote_more(v: u32, ) -> Weight {
		(Weight::from_ref_time(55_043_000))
			// Standard Error: 6_000
			.saturating_add((Weight::from_ref_time(351_000)).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn vote_less(v: u32, ) -> Weight {
		(Weight::from_ref_time(50_626_000))
			// Standard Error: 5_000
			.saturating_add((Weight::from_ref_time(399_000)).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn remove_voter() -> Weight {
		(Weight::from_ref_time(50_758_000))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn submit_candidacy(c: u32, ) -> Weight {
		(Weight::from_ref_time(52_760_000))
			// Standard Error: 2_000
			.saturating_add((Weight::from_ref_time(164_000)).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		(Weight::from_ref_time(44_733_000))
			// Standard Error: 3_000
			.saturating_add((Weight::from_ref_time(75_000)).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	fn renounce_candidacy_members() -> Weight {
		(Weight::from_ref_time(57_321_000))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn renounce_candidacy_runners_up() -> Weight {
		(Weight::from_ref_time(41_693_000))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	fn remove_member_without_replacement() -> Weight {
		(Weight::from_ref_time(250_000_000_000))
	}
	fn remove_member_with_replacement() -> Weight {
		(Weight::from_ref_time(66_871_000))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	fn remove_member_wrong_refund() -> Weight {
		(Weight::from_ref_time(7_501_000))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight {
		(Weight::from_ref_time(0))
			// Standard Error: 54_000
			.saturating_add((Weight::from_ref_time(80_487_000)).saturating_mul(v as u64))
			// Standard Error: 52_000
			.saturating_add((Weight::from_ref_time(89_000)).saturating_mul(d as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().reads((4 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes((4 as u64).saturating_mul(v as u64)))
	}
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		(Weight::from_ref_time(0))
			// Standard Error: 1_757_000
			.saturating_add((Weight::from_ref_time(18_761_000)).saturating_mul(c as u64))
			// Standard Error: 730_000
			.saturating_add((Weight::from_ref_time(50_599_000)).saturating_mul(v as u64))
			// Standard Error: 49_000
			.saturating_add((Weight::from_ref_time(3_256_000)).saturating_mul(e as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(c as u64)))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
}
