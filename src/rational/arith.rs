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

use std::mem::MaybeUninit;
use std::ops::*;
use flint_sys::{fmpz, fmpq};
use libc::{c_long, c_ulong};
use crate::{Integer, Rational};
use crate::ops::*;


impl_unop_unsafe! {
    None
    Rational
    Neg {neg}
    NegAssign {neg_assign}
    fmpq::fmpq_neg
}

impl_unop_unsafe! {
    None
    Rational
    Inv {inv}
    InvAssign {inv_assign}
    fmpq::fmpq_inv
}

impl_binop_unsafe! {
    None
    Rational, Rational, Rational
    
    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq::fmpq_add;
    
    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq::fmpq_sub;
    
    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq::fmpq_mul;
    
    Div {div}
    DivAssign {div_assign}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpq::fmpq_div;
}

impl_binop_unsafe! {
    None
    op_assign
    Rational, Integer, Rational
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpq::fmpq_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpq::fmpq_sub_fmpz;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpq::fmpq_mul_fmpz;
    
    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpq::fmpq_div_fmpz;
    
    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpq::fmpq_pow_fmpz;
}

impl_binop_unsafe! {
    None
    Rational, Integer, Integer

    Rem {rem}
    AssignRem {assign_rem}
    fmpq::fmpq_mod_fmpz;
}

impl_binop_unsafe! {
    None
    op_from
    Integer, Rational, Rational
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_fmpz_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_fmpz_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_fmpz_mul;
    
    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpq_fmpz_div;
}

impl_binop_unsafe! {
    None
    op_assign
    Rational, u64 {u64 u32 u16 u8}, Rational
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpq::fmpq_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpq::fmpq_sub_ui;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpq::fmpq_mul_ui;
    
    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpq_div_ui;
    
    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpq_pow_ui;
}

impl_binop_unsafe! {
    None
    Rational, u64 {u64 u32 u16 u8}, Integer

    Rem {rem}
    AssignRem {assign_rem}
    fmpq_mod_ui;
}

impl_binop_unsafe! {
    None
    op_assign
    Rational, i64 {i64 i32 i16 i8}, Rational
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpq::fmpq_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpq::fmpq_sub_si;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpq::fmpq_mul_si;
    
    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpq_div_si;
    
    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpq::fmpq_pow_si;
}

impl_binop_unsafe! {
    None
    Rational, i64 {i64 i32 i16 i8}, Integer

    Rem {rem}
    AssignRem {assign_rem}
    fmpq_mod_si;
}

impl_binop_unsafe! {
    None
    op_from
    u64 {u64 u32 u16 u8}, Rational, Rational
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_ui_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_ui_mul;
    
    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpq_ui_div;
}

impl_binop_unsafe! {
    None
    op_from
    i64 {i64 i32 i16 i8}, Rational, Rational
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_si_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_si_mul;
    
    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpq_si_div;
}


#[inline]
unsafe fn fmpq_fmpz_add(
    res: *mut fmpq::fmpq,
    x: *const fmpz::fmpz, 
    f: *const fmpq::fmpq)
{
    fmpq::fmpq_add_fmpz(res, f, x);
}

#[inline]
unsafe fn fmpq_fmpz_sub(
    res: *mut fmpq::fmpq,
    x: *const fmpz::fmpz, 
    f: *const fmpq::fmpq)
{
    fmpq::fmpq_sub_fmpz(res, f, x);
    fmpq::fmpq_neg(res, res);
}

#[inline]
unsafe fn fmpq_fmpz_mul(
    res: *mut fmpq::fmpq,
    x: *const fmpz::fmpz, 
    f: *const fmpq::fmpq)
{
    fmpq::fmpq_mul_fmpz(res, f, x);
}


#[inline]
unsafe fn fmpq_fmpz_div(
    res: *mut fmpq::fmpq,
    x: *const fmpz::fmpz, 
    f: *const fmpq::fmpq)
{
    fmpq::fmpq_div_fmpz(res, f, x);
    fmpq::fmpq_inv(res, res);
}

#[inline]
unsafe fn fmpq_ui_add(
    res: *mut fmpq::fmpq,
    x: c_ulong,
    f: *const fmpq::fmpq) 
{
    fmpq::fmpq_add_ui(res, f, x);
}

#[inline]
unsafe fn fmpq_si_add(
    res: *mut fmpq::fmpq,
    x: c_long,
    f: *const fmpq::fmpq) 
{
    fmpq::fmpq_add_si(res, f, x);
}

#[inline]
unsafe fn fmpq_ui_sub(
    res: *mut fmpq::fmpq,
    x: c_ulong,
    f: *const fmpq::fmpq) 
{
    fmpq::fmpq_sub_ui(res, f, x);
    fmpq::fmpq_neg(res, res);
}

#[inline]
unsafe fn fmpq_si_sub(
    res: *mut fmpq::fmpq,
    x: c_long,
    f: *const fmpq::fmpq) 
{
    fmpq::fmpq_sub_si(res, f, x);
    fmpq::fmpq_neg(res, res);
}

#[inline]
unsafe fn fmpq_ui_mul(
    res: *mut fmpq::fmpq,
    f: c_ulong,
    g: *const fmpq::fmpq,
    )
{
    fmpq::fmpq_mul_ui(res, g, f);
}

#[inline]
unsafe fn fmpq_si_mul(
    res: *mut fmpq::fmpq,
    f: c_long,
    g: *const fmpq::fmpq,
    )
{
    fmpq::fmpq_mul_si(res, g, f);
}

#[inline]
unsafe fn fmpq_div_ui(
    res: *mut fmpq::fmpq,
    f: *const fmpq::fmpq,
    g: c_ulong,
    )
{
    fmpq::fmpq_set_ui_den1(res, g);
    fmpq::fmpq_div(res, f, res); 
}

#[inline]
unsafe fn fmpq_div_si(
    res: *mut fmpq::fmpq,
    f: *const fmpq::fmpq,
    g: c_long,
    )
{
    fmpq::fmpq_set_si_den1(res, g);
    fmpq::fmpq_div(res, f, res); 
}

#[inline]
unsafe fn fmpq_ui_div(
    res: *mut fmpq::fmpq,
    f: c_ulong,
    g: *const fmpq::fmpq,
    )
{
    fmpq::fmpq_set_ui_den1(res, f);
    fmpq::fmpq_div(res, res, g); 
}

#[inline]
unsafe fn fmpq_si_div(
    res: *mut fmpq::fmpq,
    f: c_long,
    g: *const fmpq::fmpq,
    )
{
    fmpq::fmpq_set_si_den1(res, f);
    fmpq::fmpq_div(res, res, g);
}

#[inline]
unsafe fn fmpq_pow_ui(
    res: *mut fmpq::fmpq,
    f: *const fmpq::fmpq,
    g: c_ulong,
    )
{
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_ui(z.as_mut_ptr(), g);
    fmpq::fmpq_pow_fmpz(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
unsafe fn fmpq_mod_ui(
    res: *mut fmpz::fmpz,
    f: *const fmpq::fmpq,
    g: c_ulong,
    )
{
    fmpz::fmpz_set_ui(res, g);
    fmpq::fmpq_mod_fmpz(res, f, res);
}

#[inline]
unsafe fn fmpq_mod_si(
    res: *mut fmpz::fmpz,
    f: *const fmpq::fmpq,
    g: c_long,
    )
{
    fmpz::fmpz_set_si(res, g);
    fmpq::fmpq_mod_fmpz(res, f, res);
}
