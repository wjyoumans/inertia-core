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

use std::ops::*;
use flint_sys::{fmpz, fmpq, fmpq_poly};
use libc::{c_int, c_long, c_ulong};
use crate::{Integer, Rational, RatPoly};
use crate::ops::*;
use std::mem::MaybeUninit;

impl_cmp_unsafe! {
    eq
    RatPoly
    fmpq_poly::fmpq_poly_equal
}

impl_cmp_unsafe! {
    eq
    RatPoly, Rational
    fmpq_poly_equal_fmpq
}

impl_cmp_unsafe! {
    eq
    RatPoly, Integer
    fmpq_poly_equal_fmpz
}

impl_cmp_unsafe! {
    eq
    RatPoly, u64 {u64 u32 u16 u8}
    fmpq_poly_equal_ui
}

impl_cmp_unsafe! {
    eq
    RatPoly, i64 {i64 i32 i16 i8}
    fmpq_poly_equal_si
}

#[inline]
unsafe fn fmpq_poly_equal_fmpq(
    f: *const fmpq_poly::fmpq_poly_struct,
    x: *const fmpq::fmpq,
) -> c_int {
    if fmpq_poly::fmpq_poly_length(f) == 1 {
        let mut z = MaybeUninit::uninit();
        fmpq::fmpq_init(z.as_mut_ptr());
        fmpq_poly::fmpq_poly_get_coeff_fmpq(z.as_mut_ptr(), f, 0);
        let b = fmpq::fmpq_equal(z.as_ptr(), x);
        fmpq::fmpq_clear(z.as_mut_ptr());
        b
    } else {
        0
    }
}

#[inline]
unsafe fn fmpq_poly_equal_fmpz(
    f: *const fmpq_poly::fmpq_poly_struct,
    x: *const fmpz::fmpz,
) -> c_int {
    if fmpq_poly::fmpq_poly_length(f) == 1 {
        let mut z = MaybeUninit::uninit();
        fmpq::fmpq_init(z.as_mut_ptr());
        fmpq_poly::fmpq_poly_get_coeff_fmpq(z.as_mut_ptr(), f, 0);
        let b = fmpq::fmpq_cmp_fmpz(z.as_ptr(), x);
        fmpq::fmpq_clear(z.as_mut_ptr());
        
        if b == 0 {
            1
        } else {
            0
        }
    } else {
        0
    }
}

#[inline]
unsafe fn fmpq_poly_equal_ui(
    f: *const fmpq_poly::fmpq_poly_struct,
    x: c_ulong,
) -> c_int {
    if fmpq_poly::fmpq_poly_length(f) == 1 {
        let mut z = MaybeUninit::uninit();
        fmpq::fmpq_init(z.as_mut_ptr());
        fmpq_poly::fmpq_poly_get_coeff_fmpq(z.as_mut_ptr(), f, 0);
        let b = fmpq::fmpq_cmp_ui(z.as_ptr(), x);
        fmpq::fmpq_clear(z.as_mut_ptr());
        
        if b == 0 {
            1
        } else {
            0
        }
    } else {
        0
    }
}

#[inline]
unsafe fn fmpq_poly_equal_si(
    f: *const fmpq_poly::fmpq_poly_struct,
    x: c_long,
) -> c_int {
    if fmpq_poly::fmpq_poly_length(f) == 1 {
        let mut z = MaybeUninit::uninit();
        fmpq::fmpq_init(z.as_mut_ptr());
        fmpq_poly::fmpq_poly_get_coeff_fmpq(z.as_mut_ptr(), f, 0);
        let b = fmpq::fmpq_cmp_si(z.as_ptr(), x);
        fmpq::fmpq_clear(z.as_mut_ptr());
        
        if b == 0 {
            1
        } else {
            0
        }
    } else {
        0
    }
}

impl_unop_unsafe! {
    None
    RatPoly
    Neg {neg}
    NegAssign {neg_assign}
    fmpq_poly::fmpq_poly_neg
}

impl_binop_unsafe! {
    None
    RatPoly, RatPoly, RatPoly
    
    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_poly::fmpq_poly_add;
    
    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_poly::fmpq_poly_sub;
    
    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_poly::fmpq_poly_mul;
    
    Rem {rem}
    RemAssign {rem_assign}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpq_poly::fmpq_poly_rem;
}

