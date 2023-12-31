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
//! Autogenerated weights for `pallet_identity`
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
// --pallet=pallet_identity
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

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 7_157_000 picoseconds.
		Weight::from_parts(7_518_611, 0)
			.saturating_add(Weight::from_parts(0, 2386))
			// Standard Error: 991
			.saturating_add(Weight::from_parts(100_030, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430 + r * (5 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 18_174_000 picoseconds.
		Weight::from_parts(15_674_240, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 2_987
			.saturating_add(Weight::from_parts(138_510, 0).saturating_mul(r.into()))
			// Standard Error: 582
			.saturating_add(Weight::from_parts(130_131, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:100 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89`
		//  Estimated: `10991 + s * (2565 ±0)`
		// Minimum execution time: 6_059_000 picoseconds.
		Weight::from_parts(12_820_827, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 1_998
			.saturating_add(Weight::from_parts(1_521_486, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_parts(0, 2565).saturating_mul(s.into()))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169 + p * (20 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 6_030_000 picoseconds.
		Weight::from_parts(13_394_171, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 1_864
			.saturating_add(Weight::from_parts(718_549, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445 + r * (5 ±0) + s * (20 ±0) + x * (66 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 30_009_000 picoseconds.
		Weight::from_parts(19_908_009, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 9_160
			.saturating_add(Weight::from_parts(116_985, 0).saturating_mul(r.into()))
			// Standard Error: 1_788
			.saturating_add(Weight::from_parts(728_921, 0).saturating_mul(s.into()))
			// Standard Error: 1_788
			.saturating_add(Weight::from_parts(122_488, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `355 + r * (45 ±0) + x * (66 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 19_215_000 picoseconds.
		Weight::from_parts(17_566_122, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 3_387
			.saturating_add(Weight::from_parts(103_336, 0).saturating_mul(r.into()))
			// Standard Error: 660
			.saturating_add(Weight::from_parts(148_440, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `386 + x * (66 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 16_100_000 picoseconds.
		Weight::from_parts(15_271_614, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 2_865
			.saturating_add(Weight::from_parts(64_053, 0).saturating_mul(r.into()))
			// Standard Error: 559
			.saturating_add(Weight::from_parts(144_144, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 4_963_000 picoseconds.
		Weight::from_parts(5_207_820, 0)
			.saturating_add(Weight::from_parts(0, 2386))
			// Standard Error: 817
			.saturating_add(Weight::from_parts(82_469, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 4_912_000 picoseconds.
		Weight::from_parts(5_187_188, 0)
			.saturating_add(Weight::from_parts(0, 2386))
			// Standard Error: 807
			.saturating_add(Weight::from_parts(81_365, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 4_889_000 picoseconds.
		Weight::from_parts(5_229_571, 0)
			.saturating_add(Weight::from_parts(0, 2386))
			// Standard Error: 758
			.saturating_add(Weight::from_parts(76_901, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `421 + r * (45 ±0) + x * (66 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 14_325_000 picoseconds.
		Weight::from_parts(11_759_619, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 3_780
			.saturating_add(Weight::from_parts(131_885, 0).saturating_mul(r.into()))
			// Standard Error: 699
			.saturating_add(Weight::from_parts(231_319, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `680 + r * (5 ±0) + s * (20 ±0) + x * (66 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 45_060_000 picoseconds.
		Weight::from_parts(30_044_300, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 8_943
			.saturating_add(Weight::from_parts(270_510, 0).saturating_mul(r.into()))
			// Standard Error: 1_746
			.saturating_add(Weight::from_parts(747_775, 0).saturating_mul(s.into()))
			// Standard Error: 1_746
			.saturating_add(Weight::from_parts(150_732, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `536 + s * (23 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 15_619_000 picoseconds.
		Weight::from_parts(19_839_769, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 1_102
			.saturating_add(Weight::from_parts(73_705, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `615 + s * (4 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 8_768_000 picoseconds.
		Weight::from_parts(10_076_423, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 1_363
			.saturating_add(Weight::from_parts(68_020, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `647 + s * (24 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 19_137_000 picoseconds.
		Weight::from_parts(23_700_395, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 1_375
			.saturating_add(Weight::from_parts(94_036, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `620 + s * (24 ±0)`
		//  Estimated: `5511`
		// Minimum execution time: 15_642_000 picoseconds.
		Weight::from_parts(17_017_310, 0)
			.saturating_add(Weight::from_parts(0, 5511))
			// Standard Error: 571
			.saturating_add(Weight::from_parts(71_443, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
