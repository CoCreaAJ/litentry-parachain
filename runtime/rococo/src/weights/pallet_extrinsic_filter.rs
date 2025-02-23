// Copyright 2020-2022 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_extrinsic_filter`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-14, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_extrinsic_filter
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/rococo/src/weights/pallet_extrinsic_filter.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_extrinsic_filter`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_extrinsic_filter::WeightInfo for WeightInfo<T> {
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:1 w:1)
	fn block_extrinsics(p: u32, f: u32, ) -> Weight {
		(19_522_000 as Weight)
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(p as Weight))
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(f as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:1 w:1)
	fn unblock_extrinsics(p: u32, f: u32, ) -> Weight {
		(20_624_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(p as Weight))
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(f as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
