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

use crate::{IntMod, Integer, Rational};

use flint_sys::{fmpq, fmpz, fmpz_mod};
use inertia_algebra::ops::*;
use libc::{c_int, c_long, c_ulong};

impl_assign_unsafe! {
    ctx
    IntMod, IntMod
    fmpz_mod::fmpz_mod_set_fmpz
}

impl_assign_unsafe! {
    ctx
    IntMod, Integer
    fmpz_mod::fmpz_mod_set_fmpz
}

impl_assign_unsafe! {
    ctx
    IntMod, u64 {u64 u32 u16 u8}
    fmpz_mod::fmpz_mod_set_ui
}

impl_assign_unsafe! {
    ctx
    IntMod, i64 {i64 i32 i16 i8}
    fmpz_mod::fmpz_mod_set_si
}

impl_assign! {
    IntMod, Rational
    {
        fn assign(&mut self, src: &Rational) {
            if let Some(den_inv) = src.denominator().invmod(self.modulus()) {
                let temp = src.numerator() * den_inv;
                unsafe {
                    fmpz_mod::fmpz_mod_set_fmpz(
                        self.as_mut_ptr(), 
                        temp.as_ptr(),
                        self.ctx_as_ptr()
                    );
                }
            } else {
                panic!("Denominator not invertible!");
            }
        }
    }
}

impl_cmp_unsafe! {
    eq
    IntMod
    fmpz::fmpz_equal
}

impl_cmp_unsafe! {
    eq
    IntMod, Integer
    fmpz::fmpz_equal
}

impl_cmp_unsafe! {
    eq
    IntMod, Rational
    fmpz_equal_fmpq
}

impl_cmp_unsafe! {
    eq
    IntMod, u64 {u64 u32 u16 u8}
    fmpz::fmpz_equal_ui
}

impl_cmp_unsafe! {
    eq
    IntMod, i64 {i64 i32 i16 i8}
    fmpz::fmpz_equal_si
}

#[inline]
unsafe fn fmpz_equal_fmpq(f: *const fmpz::fmpz, g: *const fmpq::fmpq) -> c_int {
    if fmpq::fmpq_cmp_fmpz(g, f) == 0 {
        1
    } else {
        0
    }
}

impl_unop_unsafe! {
    ctx
    IntMod
    Neg {neg}
    NegAssign {neg_assign}
    fmpz_mod::fmpz_mod_neg
}

impl_unop_unsafe! {
    ctx
    IntMod
    Inv {inv}
    InvAssign {inv_assign}
    fmpz_mod::fmpz_mod_inv
}

impl_binop_unsafe! {
    ctx
    IntMod, IntMod, IntMod

    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add;

    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub;

    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul;

    Div {div}
    DivAssign {div_assign}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpz_mod_div;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntMod, u64 {u64 u32 u16 u8}, IntMod

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_ui;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_ui;

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpz_mod_div_ui;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpz_mod::fmpz_mod_pow_ui;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntMod, i64 {i64 i32 i16 i8}, IntMod

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_si;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_si;

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpz_mod_div_si;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpz_mod_pow_si;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntMod, Integer, IntMod

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_fmpz;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_fmpz;

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpz_mod_div;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpz_mod::fmpz_mod_pow_fmpz;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntMod, Rational, IntMod

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod_add_fmpq;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod_sub_fmpq;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod_mul_fmpq;

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpz_mod_div_fmpq;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    u64 {u64 u32 u16 u8}, IntMod, IntMod

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_ui_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_ui_mul;

    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpz_mod_ui_div;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    i64 {i64 i32 i16 i8}, IntMod, IntMod

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_si_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_si_mul;

    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpz_mod_si_div;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    Integer, IntMod, IntMod

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_fmpz;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_fmpz;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_fmpz;

    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpz_mod_div;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    Rational, IntMod, IntMod

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_fmpq_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod_fmpq_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_fmpq_mul;

    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpz_mod_fmpq_div;
}

#[inline]
unsafe fn fmpz_mod_ui_add(
    res: *mut fmpz::fmpz,
    f: c_ulong,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_add_ui(res, g, f, ctx);
}

#[inline]
unsafe fn fmpz_mod_si_add(
    res: *mut fmpz::fmpz,
    f: c_long,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_add_si(res, g, f, ctx);
}

#[inline]
unsafe fn fmpz_mod_ui_mul(
    res: *mut fmpz::fmpz,
    f: c_ulong,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_mul_ui(res, g, f, ctx);
}

#[inline]
unsafe fn fmpz_mod_si_mul(
    res: *mut fmpz::fmpz,
    f: c_long,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_mul_si(res, g, f, ctx);
}

#[inline]
unsafe fn fmpz_mod_div(
    res: *mut fmpz::fmpz,
    f: *const fmpz::fmpz,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, g);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, f, res, ctx);
}

#[inline]
unsafe fn fmpz_mod_add_fmpq(
    res: *mut fmpz::fmpz,
    f: *const fmpz::fmpz,
    g: *const fmpq::fmpq,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*g).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*g).num, ctx);
    fmpz_mod::fmpz_mod_add(res, f, res, ctx);
}

#[inline]
unsafe fn fmpz_mod_sub_fmpq(
    res: *mut fmpz::fmpz,
    f: *const fmpz::fmpz,
    g: *const fmpq::fmpq,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*g).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*g).num, ctx);
    fmpz_mod::fmpz_mod_sub(res, f, res, ctx);
}

#[inline]
unsafe fn fmpz_mod_mul_fmpq(
    res: *mut fmpz::fmpz,
    f: *const fmpz::fmpz,
    g: *const fmpq::fmpq,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*g).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*g).num, ctx);
    fmpz_mod::fmpz_mod_mul(res, f, res, ctx);
}

#[inline]
unsafe fn fmpz_mod_div_fmpq(
    res: *mut fmpz::fmpz,
    f: *const fmpz::fmpz,
    g: *const fmpq::fmpq,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*g).num);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*g).den, ctx);
    fmpz_mod::fmpz_mod_mul(res, f, res, ctx);
}

#[inline]
unsafe fn fmpz_mod_fmpq_add(
    res: *mut fmpz::fmpz,
    f: *const fmpq::fmpq,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*f).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*f).num, ctx);
    fmpz_mod::fmpz_mod_add(res, res, g, ctx);
}

#[inline]
unsafe fn fmpz_mod_fmpq_sub(
    res: *mut fmpz::fmpz,
    f: *const fmpq::fmpq,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*f).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*f).num, ctx);
    fmpz_mod::fmpz_mod_sub(res, res, g, ctx);
}

#[inline]
unsafe fn fmpz_mod_fmpq_mul(
    res: *mut fmpz::fmpz,
    f: *const fmpq::fmpq,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*f).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*f).num, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, g, ctx);
}

#[inline]
unsafe fn fmpz_mod_fmpq_div(
    res: *mut fmpz::fmpz,
    f: *const fmpq::fmpq,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*f).num);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*f).den, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, g, ctx);
}

#[inline]
unsafe fn fmpz_mod_div_ui(
    res: *mut fmpz::fmpz,
    f: *const fmpz::fmpz,
    g: c_ulong,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz::fmpz_set_ui(res, g);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, f, res, ctx);
}

#[inline]
unsafe fn fmpz_mod_ui_div(
    res: *mut fmpz::fmpz,
    f: c_ulong,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_inv(res, g, ctx);
    fmpz_mod_ui_mul(res, f, res, ctx);
}

#[inline]
unsafe fn fmpz_mod_div_si(
    res: *mut fmpz::fmpz,
    f: *const fmpz::fmpz,
    g: c_long,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz::fmpz_set_si(res, g);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, f, res, ctx);
}

#[inline]
unsafe fn fmpz_mod_si_div(
    res: *mut fmpz::fmpz,
    f: c_long,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_inv(res, g, ctx);
    fmpz_mod_si_mul(res, f, res, ctx);
}

#[inline]
unsafe fn fmpz_mod_pow_si(
    res: *mut fmpz::fmpz,
    f: *const fmpz::fmpz,
    g: c_long,
    ctx: *const fmpz_mod::fmpz_mod_ctx,
) {
    if g < 0 {
        fmpz_mod::fmpz_mod_inv(res, f, ctx);
    } else {
        fmpz::fmpz_set(res, f);
    }

    fmpz_mod::fmpz_mod_pow_ui(res, res, g.abs() as u64, ctx);
}
