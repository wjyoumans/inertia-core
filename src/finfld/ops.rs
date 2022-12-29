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

use flint_sys::fmpz;
use flint_sys::fq_default as fq;
use inertia_algebra::ops::*;
use libc::{c_long, c_ulong};

impl_cmp! {
    eq
    FinFldElem
    {
        fn eq(&self, rhs: &FinFldElem) -> bool {
            unsafe {
                self.context() == rhs.context() && fq::fq_default_equal(
                    self.as_ptr(),
                    rhs.as_ptr(),
                    self.ctx_as_ptr()
                ) != 0
            }
        }
    }
}

impl_unop_unsafe! {
    ctx
    FinFldElem
    Neg {neg}
    NegAssign {neg_assign}
    fq::fq_default_neg
}

impl_unop_unsafe! {
    ctx
    FinFldElem
    Inv {inv}
    InvAssign {inv_assign}
    fq::fq_default_inv
}

impl_binop_unsafe! {
    ctx
    FinFldElem, FinFldElem, FinFldElem

    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fq::fq_default_add;

    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fq::fq_default_sub;

    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fq::fq_default_mul;

    Div {div}
    DivAssign {div_assign}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fq::fq_default_div;
}

impl_binop_unsafe! {
    ctx
    op_assign
    FinFldElem, Integer, FinFldElem

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fq_default_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fq_default_sub_fmpz;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fq::fq_default_mul_fmpz;

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fq_default_div_fmpz;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fq::fq_default_pow;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    FinFldElem, u64 {u64 u32 u16 u8}, FinFldElem

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fq_default_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fq_default_sub_ui;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fq::fq_default_mul_ui;

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fq_default_div_ui;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fq::fq_default_pow_ui;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    FinFldElem, i64 {i64 i32 i16 i8}, FinFldElem

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fq_default_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fq_default_sub_si;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fq::fq_default_mul_si;

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fq_default_div_si;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fq_default_pow_si;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    Integer, FinFldElem, FinFldElem

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fq_default_fmpz_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fq_default_fmpz_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fq_default_fmpz_mul;

    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fq_default_fmpz_div;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    u64 {u64 u32 u16 u8}, FinFldElem, FinFldElem

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fq_default_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fq_default_ui_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fq_default_ui_mul;

    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fq_default_ui_div;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    i64 {i64 i32 i16 i8}, FinFldElem, FinFldElem

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fq_default_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fq_default_si_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fq_default_si_mul;

    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fq_default_si_div;
}

#[inline]
unsafe fn fq_default_add_fmpz(
    res: *mut fq::fq_default_struct,
    f: *const fq::fq_default_struct,
    g: *const fmpz::fmpz,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_fmpz(res, g, ctx);
    fq::fq_default_add(res, f, res, ctx);
}

#[inline]
unsafe fn fq_default_sub_fmpz(
    res: *mut fq::fq_default_struct,
    f: *const fq::fq_default_struct,
    g: *const fmpz::fmpz,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_fmpz(res, g, ctx);
    fq::fq_default_sub(res, f, res, ctx);
}

#[inline]
unsafe fn fq_default_div_fmpz(
    res: *mut fq::fq_default_struct,
    f: *const fq::fq_default_struct,
    g: *const fmpz::fmpz,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_fmpz(res, g, ctx);
    fq::fq_default_div(res, f, res, ctx);
}

#[inline]
unsafe fn fq_default_add_ui(
    res: *mut fq::fq_default_struct,
    f: *const fq::fq_default_struct,
    g: c_ulong,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_ui(res, g, ctx);
    fq::fq_default_add(res, f, res, ctx);
}

#[inline]
unsafe fn fq_default_sub_ui(
    res: *mut fq::fq_default_struct,
    f: *const fq::fq_default_struct,
    g: c_ulong,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_ui(res, g, ctx);
    fq::fq_default_sub(res, f, res, ctx);
}

#[inline]
unsafe fn fq_default_div_ui(
    res: *mut fq::fq_default_struct,
    f: *const fq::fq_default_struct,
    g: c_ulong,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_ui(res, g, ctx);
    fq::fq_default_div(res, f, res, ctx);
}

#[inline]
unsafe fn fq_default_add_si(
    res: *mut fq::fq_default_struct,
    f: *const fq::fq_default_struct,
    g: c_long,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_si(res, g, ctx);
    fq::fq_default_add(res, f, res, ctx);
}

#[inline]
unsafe fn fq_default_sub_si(
    res: *mut fq::fq_default_struct,
    f: *const fq::fq_default_struct,
    g: c_long,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_si(res, g, ctx);
    fq::fq_default_sub(res, f, res, ctx);
}

#[inline]
unsafe fn fq_default_div_si(
    res: *mut fq::fq_default_struct,
    f: *const fq::fq_default_struct,
    g: c_long,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_si(res, g, ctx);
    fq::fq_default_div(res, f, res, ctx);
}

#[inline]
unsafe fn fq_default_pow_si(
    res: *mut fq::fq_default_struct,
    f: *const fq::fq_default_struct,
    g: c_long,
    ctx: *const fq::fq_default_ctx_struct,
) {
    if g < 0 {
        fq::fq_default_inv(res, f, ctx);
    }
    fq::fq_default_pow_ui(res, f, g.abs() as u64, ctx);
}

#[inline]
unsafe fn fq_default_ui_add(
    res: *mut fq::fq_default_struct,
    f: c_ulong,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_ui(res, f, ctx);
    fq::fq_default_add(res, res, g, ctx);
}

#[inline]
unsafe fn fq_default_ui_sub(
    res: *mut fq::fq_default_struct,
    f: c_ulong,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_ui(res, f, ctx);
    fq::fq_default_sub(res, res, g, ctx);
}

#[inline]
unsafe fn fq_default_ui_mul(
    res: *mut fq::fq_default_struct,
    f: c_ulong,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_ui(res, f, ctx);
    fq::fq_default_mul(res, res, g, ctx);
}

#[inline]
unsafe fn fq_default_ui_div(
    res: *mut fq::fq_default_struct,
    f: c_ulong,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_ui(res, f, ctx);
    fq::fq_default_div(res, res, g, ctx);
}

#[inline]
unsafe fn fq_default_si_add(
    res: *mut fq::fq_default_struct,
    f: c_long,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_si(res, f, ctx);
    fq::fq_default_add(res, res, g, ctx);
}

#[inline]
unsafe fn fq_default_si_sub(
    res: *mut fq::fq_default_struct,
    f: c_long,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_si(res, f, ctx);
    fq::fq_default_sub(res, res, g, ctx);
}

#[inline]
unsafe fn fq_default_si_mul(
    res: *mut fq::fq_default_struct,
    f: c_long,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_si(res, f, ctx);
    fq::fq_default_mul(res, res, g, ctx);
}

#[inline]
unsafe fn fq_default_si_div(
    res: *mut fq::fq_default_struct,
    f: c_long,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_si(res, f, ctx);
    fq::fq_default_div(res, res, g, ctx);
}

#[inline]
unsafe fn fq_default_fmpz_add(
    res: *mut fq::fq_default_struct,
    f: *const fmpz::fmpz,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_fmpz(res, f, ctx);
    fq::fq_default_add(res, res, g, ctx);
}

#[inline]
unsafe fn fq_default_fmpz_sub(
    res: *mut fq::fq_default_struct,
    f: *const fmpz::fmpz,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_fmpz(res, f, ctx);
    fq::fq_default_sub(res, res, g, ctx);
}

#[inline]
unsafe fn fq_default_fmpz_mul(
    res: *mut fq::fq_default_struct,
    f: *const fmpz::fmpz,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_mul_fmpz(res, g, f, ctx);
}

#[inline]
unsafe fn fq_default_fmpz_div(
    res: *mut fq::fq_default_struct,
    f: *const fmpz::fmpz,
    g: *const fq::fq_default_struct,
    ctx: *const fq::fq_default_ctx_struct,
) {
    fq::fq_default_set_fmpz(res, f, ctx);
    fq::fq_default_div(res, res, g, ctx);
}
