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

//! Autogenerated weights for `pallet_asset_manager`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-05-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_asset_manager
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./benchmarking/frame-weight-template.hbs
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_asset_manager`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_manager::WeightInfo for WeightInfo<T> {
	/// Storage: `AssetManager::AssetIdType` (r:1 w:1)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(152), added: 2627, mode: `MaxEncodedLen`)
	/// Storage: `AssetManager::AssetTypeId` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `82`
		//  Estimated: `3639`
		// Minimum execution time: 29_567_000 picoseconds.
		Weight::from_parts(30_462_000, 3639)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `AssetManager::AssetTypeId` (r:1 w:0)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn set_asset_units_per_second(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `611 + x * (9 ±0)`
		//  Estimated: `4000 + x * (10 ±0)`
		// Minimum execution time: 20_117_000 picoseconds.
		Weight::from_parts(19_957_388, 4000)
			// Standard Error: 2_959
			.saturating_add(Weight::from_parts(790_570, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(x.into()))
	}
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetIdType` (r:1 w:1)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:1 w:2)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeId` (r:0 w:2)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn change_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `926 + x * (13 ±0)`
		//  Estimated: `4309 + x * (15 ±0)`
		// Minimum execution time: 30_384_000 picoseconds.
		Weight::from_parts(31_517_144, 4309)
			// Standard Error: 4_220
			.saturating_add(Weight::from_parts(862_478, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(Weight::from_parts(0, 15).saturating_mul(x.into()))
	}
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn remove_supported_asset(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + x * (5 ±0)`
		//  Estimated: `1678 + x * (5 ±0)`
		// Minimum execution time: 15_973_000 picoseconds.
		Weight::from_parts(15_091_577, 1678)
			// Standard Error: 2_841
			.saturating_add(Weight::from_parts(716_846, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 5).saturating_mul(x.into()))
	}
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetIdType` (r:1 w:1)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeId` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn remove_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482 + x * (10 ±0)`
		//  Estimated: `3955 + x * (10 ±0)`
		// Minimum execution time: 22_408_000 picoseconds.
		Weight::from_parts(21_901_691, 3955)
			// Standard Error: 3_158
			.saturating_add(Weight::from_parts(740_471, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(x.into()))
	}
}
