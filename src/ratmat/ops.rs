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

use flint_sys::{
    fmpz, 
    //fmpq, 
    fmpz_mat, 
    fmpq_mat
};
use inertia_algebra::ops::*;
use libc::{c_long, c_ulong};

use std::mem::MaybeUninit;

// TODO: Pow

impl_cmp_unsafe! {
    eq
    RatMat
    fmpq_mat::fmpq_mat_equal
}

impl_unop_unsafe! {
    matrix
    RatMat
    Neg {neg}
    NegAssign {neg_assign}
    fmpq_mat::fmpq_mat_neg
}

impl_binop_unsafe! {
    matrix
    RatMat, RatMat, RatMat

    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_mat::fmpq_mat_add;

    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_mat::fmpq_mat_sub;

    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_mat::fmpq_mat_mul;
}

impl_binop_unsafe! {
    scalar_rhs
    op_assign
    RatMat, Integer, RatMat

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpq_mat::fmpq_mat_scalar_mul_fmpz;
    
    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpq_mat::fmpq_mat_scalar_div_fmpz;
    
    /*
    Pow {pow}
    AssignPow {assign_pow}
    fmpq_mat::fmpq_mat_set_fmpq_mat_pow_fmpq;
    */
}

impl_binop_unsafe! {
    scalar_rhs
    op_assign
    RatMat, u64 {u64 u32 u16 u8}, RatMat

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpq_mat_scalar_mul_ui;

    /*
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpq_mat_scalar_mod_ui;
    */

    /*
    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpq_mat::fmpq_mat_pow;
    */
}

impl_binop_unsafe! {
    scalar_rhs
    op_assign
    RatMat, i64 {i64 i32 i16 i8}, RatMat

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpq_mat_scalar_mul_si;

    /*
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpq_mat_scalar_mod_si;
    */
    
    /*
    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpq_mat::fmpq_mat_pow;
    */
}

impl_binop_unsafe! {
    scalar_rhs
    RatMat, Integer, IntMat

    Rem {rem}
    AssignRem {assign_rem}
    fmpq_mat::fmpq_mat_get_fmpz_mat_mod_fmpz;
}

impl_binop_unsafe! {
    scalar_rhs
    RatMat, u64 {u64 u32 u16 u8}, IntMat

    Rem {rem}
    AssignRem {assign_rem}
    fmpq_mat_get_fmpz_mat_mod_ui;
}

impl_binop_unsafe! {
    scalar_rhs
    RatMat, i64 {i64 i32 i16 i8}, IntMat

    Rem {rem}
    AssignRem {assign_rem}
    fmpq_mat_get_fmpz_mat_mod_si;
}

impl_binop_unsafe! {
    scalar_lhs
    op_from
    Integer, RatMat, RatMat

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_mat_fmpz_scalar_mul;
}

impl_binop_unsafe! {
    scalar_lhs
    op_from
    u64 {u64 u32 u16 u8}, RatMat, RatMat

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_mat_ui_scalar_mul;
}

impl_binop_unsafe! {
    scalar_lhs
    op_from
    i64 {i64 i32 i16 i8}, RatMat, RatMat

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_mat_si_scalar_mul;
}

#[inline]
unsafe fn fmpq_mat_scalar_mul_ui(
    res: *mut fmpq_mat::fmpq_mat_struct,
    f: *const fmpq_mat::fmpq_mat_struct,
    g: c_ulong,
) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_ui(z.as_mut_ptr(), g);
    fmpq_mat::fmpq_mat_scalar_mul_fmpz(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
unsafe fn fmpq_mat_scalar_mul_si(
    res: *mut fmpq_mat::fmpq_mat_struct,
    f: *const fmpq_mat::fmpq_mat_struct,
    g: c_long,
) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_si(z.as_mut_ptr(), g);
    fmpq_mat::fmpq_mat_scalar_mul_fmpz(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
unsafe fn fmpq_mat_fmpz_scalar_mul(
    res: *mut fmpq_mat::fmpq_mat_struct,
    f: *const fmpz::fmpz,
    g: *const fmpq_mat::fmpq_mat_struct,
) {
    fmpq_mat::fmpq_mat_scalar_mul_fmpz(res, g, f);
}

/*
#[inline]
unsafe fn fmpq_mat_fmpq_scalar_mul(
    res: *mut fmpq_mat::fmpq_mat_struct,
    f: *const fmpq::fmpq,
    g: *const fmpq_mat::fmpq_mat_struct,
) {
    fmpq_mat::fmpq_mat_scalar_mul_fmpq(res, g, f);
}*/

#[inline]
unsafe fn fmpq_mat_ui_scalar_mul(
    res: *mut fmpq_mat::fmpq_mat_struct,
    f: c_ulong,
    g: *const fmpq_mat::fmpq_mat_struct,
) {
    fmpq_mat_scalar_mul_ui(res, g, f);
}

#[inline]
unsafe fn fmpq_mat_si_scalar_mul(
    res: *mut fmpq_mat::fmpq_mat_struct,
    f: c_long,
    g: *const fmpq_mat::fmpq_mat_struct,
) {
    fmpq_mat_scalar_mul_si(res, g, f);
}

#[inline]
unsafe fn fmpq_mat_get_fmpz_mat_mod_ui(
    res: *mut fmpz_mat::fmpz_mat_struct,
    f: *const fmpq_mat::fmpq_mat_struct,
    g: c_ulong,
) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_ui(z.as_mut_ptr(), g);
    fmpq_mat::fmpq_mat_get_fmpz_mat_mod_fmpz(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
unsafe fn fmpq_mat_get_fmpz_mat_mod_si(
    res: *mut fmpz_mat::fmpz_mat_struct,
    f: *const fmpq_mat::fmpq_mat_struct,
    g: c_long,
) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_si(z.as_mut_ptr(), g);
    fmpq_mat::fmpq_mat_get_fmpz_mat_mod_fmpz(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}
