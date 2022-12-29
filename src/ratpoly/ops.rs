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

use flint_sys::{fmpz, fmpq, fmpz_poly, fmpq_poly};
use inertia_algebra::ops::*;
use libc::{c_int, c_long, c_ulong};

use std::mem::MaybeUninit;

// TODO: Rem, Pow

impl_assign_unsafe! {
    None
    RatPoly, RatPoly
    fmpq_poly::fmpq_poly_set
}

impl_assign_unsafe! {
    None
    RatPoly, IntPoly
    fmpq_poly::fmpq_poly_set_fmpz_poly
}

impl_assign_unsafe! {
    None
    RatPoly, IntMod
    fmpq_poly::fmpq_poly_set_fmpz
}

impl_assign_unsafe! {
    None
    RatPoly, Integer
    fmpq_poly::fmpq_poly_set_fmpz
}

impl_assign_unsafe! {
    None
    RatPoly, u64 {u64 u32 u16 u8}
    fmpq_poly::fmpq_poly_set_ui
}

impl_assign_unsafe! {
    None
    RatPoly, i64 {i64 i32 i16 i8}
    fmpq_poly::fmpq_poly_set_si
}

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
    op_assign
    RatPoly, u64 {u64 u32 u16 u8}, RatPoly
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpq_poly_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpq_poly_sub_ui;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpq_poly::fmpq_poly_scalar_mul_ui;
    
    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpq_poly::fmpq_poly_scalar_div_ui;
    
    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpq_poly::fmpq_poly_pow;
    
    /*
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpq_poly_scalar_mod_ui;
    */
}

impl_binop_unsafe! {
    None
    op_assign
    RatPoly, i64 {i64 i32 i16 i8}, RatPoly
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpq_poly::fmpq_poly_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpq_poly::fmpq_poly_sub_si;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpq_poly::fmpq_poly_scalar_mul_si;
    
    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpq_poly::fmpq_poly_scalar_div_si;

    /*
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpq_poly_scalar_mod_si;
    */
}


impl_binop_unsafe! {
    None
    op_assign
    RatPoly, Integer, RatPoly
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpq_poly::fmpq_poly_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpq_poly::fmpq_poly_sub_fmpz;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpq_poly::fmpq_poly_scalar_mul_fmpz;
    
    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpq_poly::fmpq_poly_scalar_div_fmpz;
    
    /*
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpq_poly_scalar_mod_fmpz;
    */
}

impl_binop_unsafe! {
    None
    op_assign
    RatPoly, Rational, RatPoly
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpq_poly::fmpq_poly_add_fmpq;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpq_poly::fmpq_poly_sub_fmpq;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpq_poly::fmpq_poly_scalar_mul_fmpq;
    
    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpq_poly::fmpq_poly_scalar_div_fmpq;
    
    /*
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpq_poly_scalar_mod_fmpq;
    */
}

impl_binop_unsafe! {
    None
    op_from
    u64 {u64 u32 u16 u8}, RatPoly, RatPoly
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_poly_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_poly_ui_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_poly_ui_scalar_mul;
}

impl_binop_unsafe! {
    None
    op_from
    i64 {i64 i32 i16 i8}, RatPoly, RatPoly
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_poly_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_poly_si_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_poly_si_scalar_mul;
}

impl_binop_unsafe! {
    None
    op_from
    Integer, RatPoly, RatPoly
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_poly_fmpz_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_poly::fmpq_poly_fmpz_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_poly_fmpz_scalar_mul;
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

impl_binop_unsafe! {
    None
    op_assign
    RatPoly, IntPoly, RatPoly
    
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpq_poly_add_fmpz_poly;
    
    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpq_poly_sub_fmpz_poly;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpq_poly_mul_fmpz_poly;
    
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpq_poly_rem_fmpz_poly;
}

impl_binop_unsafe! {
    None
    op_from
    IntPoly, RatPoly, RatPoly
    
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_poly_fmpz_poly_add;
    
    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_poly_fmpz_poly_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_poly_fmpz_poly_mul;
    
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpq_poly_fmpz_poly_rem;
}

/*
impl_binop_unsafe! {
    None
    RatPoly, IntModPoly, RatPoly
    
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
}*/

#[inline]
unsafe fn fmpq_poly_add_ui(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: *const fmpq_poly::fmpq_poly_struct,
    x: c_ulong,
    )
{
    fmpq_poly::fmpq_poly_set_ui(res, x);
    fmpq_poly::fmpq_poly_add(res, f, res);
}

#[inline]
unsafe fn fmpq_poly_sub_ui(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: *const fmpq_poly::fmpq_poly_struct,
    x: c_ulong,
    )
{
    fmpq_poly::fmpq_poly_set_ui(res, x);
    fmpq_poly::fmpq_poly_sub(res, f, res);
}

#[inline]
unsafe fn fmpq_poly_add_fmpz_poly(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: *const fmpq_poly::fmpq_poly_struct,
    x: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_add(res, f, res);
}

#[inline]
unsafe fn fmpq_poly_sub_fmpz_poly(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: *const fmpq_poly::fmpq_poly_struct,
    x: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_sub(res, f, res);
}

#[inline]
unsafe fn fmpq_poly_mul_fmpz_poly(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: *const fmpq_poly::fmpq_poly_struct,
    x: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_mul(res, f, res);
}

#[inline]
unsafe fn fmpq_poly_rem_fmpz_poly(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: *const fmpq_poly::fmpq_poly_struct,
    x: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_rem(res, f, res);
}

#[inline]
unsafe fn fmpq_poly_fmpz_poly_add(
    res: *mut fmpq_poly::fmpq_poly_struct,
    x: *const fmpz_poly::fmpz_poly_struct,
    f: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_add(res, res, f);
}

#[inline]
unsafe fn fmpq_poly_fmpz_poly_sub(
    res: *mut fmpq_poly::fmpq_poly_struct,
    x: *const fmpz_poly::fmpz_poly_struct,
    f: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_sub(res, res, f);
}

#[inline]
unsafe fn fmpq_poly_fmpz_poly_mul(
    res: *mut fmpq_poly::fmpq_poly_struct,
    x: *const fmpz_poly::fmpz_poly_struct,
    f: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_mul(res, res, f);
}

#[inline]
unsafe fn fmpq_poly_fmpz_poly_rem(
    res: *mut fmpq_poly::fmpq_poly_struct,
    x: *const fmpz_poly::fmpz_poly_struct,
    f: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_rem(res, res, f);
}

#[inline]
unsafe fn fmpq_poly_ui_add(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: c_ulong,
    g: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_ui(res, f);
    fmpq_poly::fmpq_poly_add(res, res, g);
}

#[inline]
unsafe fn fmpq_poly_ui_sub(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: c_ulong,
    g: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_ui(res, f);
    fmpq_poly::fmpq_poly_sub(res, res, g);
}

#[inline]
unsafe fn fmpq_poly_ui_scalar_mul(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: c_ulong,
    g: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_scalar_mul_ui(res, g, f);
}

#[inline]
unsafe fn fmpq_poly_si_add(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: c_long,
    g: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_add_si(res, g, f);
}

#[inline]
unsafe fn fmpq_poly_si_sub(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: c_long,
    g: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_si(res, f);
    fmpq_poly::fmpq_poly_sub(res, res, g);
}

unsafe fn fmpq_poly_si_scalar_mul(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: c_long,
    g: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_scalar_mul_si(res, g, f);
}

#[inline]
unsafe fn fmpq_poly_fmpz_add(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: *const fmpz::fmpz,
    g: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_add_fmpz(res, g, f);
}

#[inline]
unsafe fn fmpq_poly_fmpz_scalar_mul(
    res: *mut fmpq_poly::fmpq_poly_struct,
    f: *const fmpz::fmpz,
    g: *const fmpq_poly::fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_scalar_mul_fmpz(res, g, f);
}


