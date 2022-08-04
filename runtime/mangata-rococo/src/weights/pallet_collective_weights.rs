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

//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_collective
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./benchmarks/pallet_collective_weights.rs
// --template
// ./templates/module-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_collective.
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

/// Weights for pallet_collective using the Mangata node and recommended hardware.
pub struct ModuleWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for ModuleWeight<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Voting (r:100 w:100)
	// Storage: Council Prime (r:0 w:1)
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 14_000
			.saturating_add((12_680_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 14_000
			.saturating_add((17_039_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Council Members (r:1 w:0)
	fn execute(b: u32, m: u32, ) -> Weight {
		(18_450_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((38_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		(21_949_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((65_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		(29_149_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((54_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((253_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	fn vote(m: u32, ) -> Weight {
		(36_863_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((85_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		(38_649_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((61_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((194_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(40_947_000 as Weight)
			// Standard Error: 0
			.saturating_add((7_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((96_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((253_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		(42_214_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((70_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((196_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(47_554_000 as Weight)
			// Standard Error: 0
			.saturating_add((6_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((77_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((246_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32, ) -> Weight {
		(22_670_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((257_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 14_000
			.saturating_add((12_680_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 14_000
			.saturating_add((17_039_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn execute(b: u32, m: u32, ) -> Weight {
		(18_450_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((38_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		(21_949_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((65_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	}
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		(29_149_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((54_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((253_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn vote(m: u32, ) -> Weight {
		(36_863_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((85_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		(38_649_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((61_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((194_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(40_947_000 as Weight)
			// Standard Error: 0
			.saturating_add((7_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((96_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((253_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		(42_214_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((70_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((196_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(47_554_000 as Weight)
			// Standard Error: 0
			.saturating_add((6_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((77_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((246_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn disapprove_proposal(p: u32, ) -> Weight {
		(22_670_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((257_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}