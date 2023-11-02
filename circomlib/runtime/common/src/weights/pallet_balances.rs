// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_balances
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn transfer_allow_death() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `72`
		//  Estimated: `3581`
		// Minimum execution time: 20_916_000 picoseconds.
		Weight::from_parts(21_458_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `72`
		//  Estimated: `3581`
		// Minimum execution time: 20_547_000 picoseconds.
		Weight::from_parts(21_033_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn force_set_balance_creating() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3581`
		// Minimum execution time: 9_298_000 picoseconds.
		Weight::from_parts(9_738_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn force_set_balance_killing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3581`
		// Minimum execution time: 10_243_000 picoseconds.
		Weight::from_parts(10_523_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `267`
		//  Estimated: `6172`
		// Minimum execution time: 23_572_000 picoseconds.
		Weight::from_parts(24_097_000, 0)
			.saturating_add(Weight::from_parts(0, 6172))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `72`
		//  Estimated: `3581`
		// Minimum execution time: 22_177_000 picoseconds.
		Weight::from_parts(22_556_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn force_unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3581`
		// Minimum execution time: 10_680_000 picoseconds.
		Weight::from_parts(10_979_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:999 w:999)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// The range of component `u` is `[1, 1000]`.
	fn upgrade_accounts(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `585 + u * (123 ±0)`
		//  Estimated: `990 + u * (2591 ±0)`
		// Minimum execution time: 9_772_000 picoseconds.
		Weight::from_parts(9_938_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 9_375
			.saturating_add(Weight::from_parts(8_373_767, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 2591).saturating_mul(u.into()))
	}
}