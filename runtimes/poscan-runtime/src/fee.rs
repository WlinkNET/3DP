// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of 3DPass.
//
// Copyright (c) 2019-2020 Wei Tang
// Copyright (c) 2022 3DPass
//
// 3DPass is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// 3DPass is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Kulupu. If not, see <http://www.gnu.org/licenses/>.

use crate::{Balance, ExtrinsicBaseWeight};
use frame_support::weights::{
	WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
};
use sp_consensus_poscan::CENTS;
use smallvec::smallvec;
use sp_runtime::Perbill;

pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		// in 3DPass, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
		let p = CENTS;
		let q = 10 * Balance::from(ExtrinsicBaseWeight::get());
		smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational_approximation(p, q), // TODO: from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

#[cfg(test)]
mod tests {
	use super::WeightToFee;
	use crate::{ExtrinsicBaseWeight, MAXIMUM_BLOCK_WEIGHT};
	use frame_support::weights::WeightToFeePolynomial;
	use sp_consensus_poscan::{CENTS, DOLLARS};

	#[test]
	// This function tests that the fee for `MaximumBlockWeight` of weight is correct
	fn full_block_fee_is_correct() {
		// A full block should cost 16 DOLLARS
		assert_eq!(WeightToFee::calc(&MAXIMUM_BLOCK_WEIGHT), 16 * DOLLARS)
	}

	#[test]
	// This function tests that the fee for `ExtrinsicBaseWeight` of weight is correct
	fn extrinsic_base_fee_is_correct() {
		// `ExtrinsicBaseWeight` should cost 1/10 of a CENT
		assert_eq!(WeightToFee::calc(&ExtrinsicBaseWeight::get()), CENTS / 10)
	}
}
