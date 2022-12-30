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

use crate::*;
use flint_sys::fq_default_mat::*;
use inertia_algebra::ops::*;
//use libc::{c_long, c_ulong};
//use std::mem::MaybeUninit;

/*
impl_cmp! {
    eq
    IntModMat
    {
        fn eq(&self, rhs: &IntModMat) -> bool {
            unsafe {
                self.parent() == rhs.parent() && fmpz_mod_mat::fmpz_mod_mat_equal(
                    self.as_ptr(),
                    rhs.as_ptr()
                ) != 0
            }
        }
    }
}*/

impl_unop_unsafe! {
    matrix_ctx
    FinFldMat
    Neg {neg}
    NegAssign {neg_assign}
    fq_default_mat_neg
}

impl_binop_unsafe! {
    matrix_ctx
    FinFldMat, FinFldMat, FinFldMat

    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fq_default_mat_add;

    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fq_default_mat_sub;

    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fq_default_mat_mul;
}
