/*
 *  Copyright (C) 2021 William Youmans
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use flint_sys::fmpq_mat;
use crate::*;

impl_from! {
    RatMat, IntMat
    {
        fn from(x: &IntMat) -> RatMat {
            let rm = RatMatSpace::init(x.nrows(), x.ncols());
            let mut res = rm.default();
            unsafe {
                fmpq_mat::fmpq_mat_set_fmpz_mat(res.as_mut_ptr(), x.as_ptr());
            }
            res
        }
    }
}

impl_from! {
    RatMat, IntModMat
    {
        fn from(x: &IntModMat) -> RatMat {
            RatMat::from(IntMat::from(x))
        }
    }
}
