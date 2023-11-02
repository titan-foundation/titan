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


//! Autogenerated weights for pallet_asset_manager
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmarker`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_asset_manager.
pub trait WeightInfo {
	fn register_foreign_asset() -> Weight;
	fn set_asset_units_per_second(x: u32, ) -> Weight;
	fn change_existing_asset_type(x: u32, ) -> Weight;
	fn remove_supported_asset(x: u32, ) -> Weight;
	fn register_local_asset() -> Weight;
	fn remove_existing_asset_type(x: u32, ) -> Weight;
}

/// Weights for pallet_asset_manager using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: AssetManager AssetIdType (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(174), added: 2649, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: AssetManager AssetTypeId (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	fn register_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `82`
		//  Estimated: `10885`
		// Minimum execution time: 51_631_000 picoseconds.
		Weight::from_parts(52_681_000, 10885)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: AssetManager AssetTypeId (r:1 w:0)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn set_asset_units_per_second(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `611 + x * (9 ±0)`
		//  Estimated: `6555 + x * (30 ±0)`
		// Minimum execution time: 30_927_000 picoseconds.
		Weight::from_parts(30_990_835, 6555)
			// Standard Error: 2_254
			.saturating_add(Weight::from_parts(494_375, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 30).saturating_mul(x.into()))
	}
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdType (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:1 w:2)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeId (r:0 w:2)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn change_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `926 + x * (13 ±0)`
		//  Estimated: `11791 + x * (60 ±0)`
		// Minimum execution time: 42_959_000 picoseconds.
		Weight::from_parts(43_255_055, 11791)
			// Standard Error: 3_394
			.saturating_add(Weight::from_parts(543_897, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(Weight::from_parts(0, 60).saturating_mul(x.into()))
	}
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn remove_supported_asset(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + x * (5 ±0)`
		//  Estimated: `1871 + x * (10 ±0)`
		// Minimum execution time: 25_453_000 picoseconds.
		Weight::from_parts(24_977_319, 1871)
			// Standard Error: 2_109
			.saturating_add(Weight::from_parts(407_717, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(x.into()))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: AssetManager LocalAssetCounter (r:1 w:1)
	/// Proof Skipped: AssetManager LocalAssetCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: LocalAssets Asset (r:1 w:1)
	/// Proof: LocalAssets Asset (max_values: None, max_size: Some(174), added: 2649, mode: MaxEncodedLen)
	/// Storage: EVM AccountCodes (r:0 w:1)
	/// Proof Skipped: EVM AccountCodes (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager LocalAssetDeposit (r:0 w:1)
	/// Proof Skipped: AssetManager LocalAssetDeposit (max_values: None, max_size: None, mode: Measured)
	fn register_local_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `310`
		//  Estimated: `9635`
		// Minimum execution time: 59_851_000 picoseconds.
		Weight::from_parts(60_452_000, 9635)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdType (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeId (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn remove_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482 + x * (10 ±0)`
		//  Estimated: `6910 + x * (40 ±0)`
		// Minimum execution time: 32_960_000 picoseconds.
		Weight::from_parts(33_257_599, 6910)
			// Standard Error: 2_430
			.saturating_add(Weight::from_parts(421_651, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(x.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: AssetManager AssetIdType (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(174), added: 2649, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: AssetManager AssetTypeId (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	fn register_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `82`
		//  Estimated: `10885`
		// Minimum execution time: 51_631_000 picoseconds.
		Weight::from_parts(52_681_000, 10885)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: AssetManager AssetTypeId (r:1 w:0)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn set_asset_units_per_second(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `611 + x * (9 ±0)`
		//  Estimated: `6555 + x * (30 ±0)`
		// Minimum execution time: 30_927_000 picoseconds.
		Weight::from_parts(30_990_835, 6555)
			// Standard Error: 2_254
			.saturating_add(Weight::from_parts(494_375, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 30).saturating_mul(x.into()))
	}
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdType (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:1 w:2)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeId (r:0 w:2)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn change_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `926 + x * (13 ±0)`
		//  Estimated: `11791 + x * (60 ±0)`
		// Minimum execution time: 42_959_000 picoseconds.
		Weight::from_parts(43_255_055, 11791)
			// Standard Error: 3_394
			.saturating_add(Weight::from_parts(543_897, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
			.saturating_add(Weight::from_parts(0, 60).saturating_mul(x.into()))
	}
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn remove_supported_asset(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + x * (5 ±0)`
		//  Estimated: `1871 + x * (10 ±0)`
		// Minimum execution time: 25_453_000 picoseconds.
		Weight::from_parts(24_977_319, 1871)
			// Standard Error: 2_109
			.saturating_add(Weight::from_parts(407_717, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(x.into()))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: AssetManager LocalAssetCounter (r:1 w:1)
	/// Proof Skipped: AssetManager LocalAssetCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: LocalAssets Asset (r:1 w:1)
	/// Proof: LocalAssets Asset (max_values: None, max_size: Some(174), added: 2649, mode: MaxEncodedLen)
	/// Storage: EVM AccountCodes (r:0 w:1)
	/// Proof Skipped: EVM AccountCodes (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager LocalAssetDeposit (r:0 w:1)
	/// Proof Skipped: AssetManager LocalAssetDeposit (max_values: None, max_size: None, mode: Measured)
	fn register_local_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `310`
		//  Estimated: `9635`
		// Minimum execution time: 59_851_000 picoseconds.
		Weight::from_parts(60_452_000, 9635)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdType (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeId (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn remove_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482 + x * (10 ±0)`
		//  Estimated: `6910 + x * (40 ±0)`
		// Minimum execution time: 32_960_000 picoseconds.
		Weight::from_parts(33_257_599, 6910)
			// Standard Error: 2_430
			.saturating_add(Weight::from_parts(421_651, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(x.into()))
	}
}