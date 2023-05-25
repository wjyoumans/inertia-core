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

use flint_sys::{fmpq, fmpz};
use inertia_algebra::ops::*;
use libc::{c_long, c_ulong};

use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::mem::MaybeUninit;


impl_assign_unsafe! {
    None
    Integer, Integer
    fmpz::fmpz_set
}

impl_assign_unsafe! {
    None
    Integer, u64 {u64 u32 u16 u8 usize}
    fmpz::fmpz_set_ui
}

impl_assign_unsafe! {
    None
    Integer, i64 {i64 i32 i16 i8 isize}
    fmpz::fmpz_set_si
}

impl_cmp_unsafe! {
    eq
    Integer
    fmpz::fmpz_equal
}

impl_cmp_unsafe! {
    ord
    Integer
    fmpz::fmpz_cmp
}

impl_cmp_unsafe! {
    partial_eq
    Integer, u64 {u64 u32 u16 u8 usize}
    fmpz::fmpz_equal_ui
}

impl_cmp_unsafe! {
    partial_ord
    Integer, u64 {u64 u32 u16 u8 usize}
    fmpz::fmpz_cmp_ui
}

impl_cmp_unsafe! {
    partial_eq
    Integer, i64 {i64 i32 i16 i8 isize}
    fmpz::fmpz_equal_si
}

impl_cmp_unsafe! {
    partial_ord
    Integer, i64 {i64 i32 i16 i8 isize}
    fmpz::fmpz_cmp_si
}

impl_unop_unsafe! {
    None
    Integer
    Neg {neg}
    NegAssign {neg_assign}
    fmpz::fmpz_neg
}

impl_unop_unsafe! {
    None
    Integer
    Not {not}
    NotAssign {not_assign}
    fmpz::fmpz_complement
}

impl_unop_unsafe! {
    None
    Integer, Rational
    Inv {inv}
    fmpq_inv_fmpz
}

impl_binop_unsafe! {
    None
    Integer, Integer, Integer

    BitAnd {bitand}
    BitAndAssign {bitand_assign}
    BitAndFrom {bitand_from}
    AssignBitAnd {assign_bitand}
    fmpz::fmpz_and;

    BitOr {bitor}
    BitOrAssign {bitor_assign}
    BitOrFrom {bitor_from}
    AssignBitOr {assign_bitor}
    fmpz::fmpz_or;

    BitXor {bitxor}
    BitXorAssign {bitxor_assign}
    BitXorFrom {bitxor_from}
    AssignBitXor {assign_bitxor}
    fmpz::fmpz_xor;

    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz::fmpz_add;

    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz::fmpz_sub;

    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz::fmpz_mul;

    Rem {rem}
    RemAssign {rem_assign}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz::fmpz_tdiv_r;

}

impl_binop_unsafe! {
    None
    Integer, Integer, Rational
    
    Div {div}
    AssignDiv {assign_div}
    fmpq::fmpq_set_fmpz_frac;

    Pow {pow}
    AssignPow {assign_pow}
    fmpz_pow_fmpz;
}

impl_binop_unsafe! {
    None
    op_assign
    Integer, u64 {u64 u32 u16 u8}, Integer

    BitAnd {bitand}
    BitAndAssign {bitand_assign}
    AssignBitAnd {assign_bitand}
    fmpz_and_ui;

    BitOr {bitor}
    BitOrAssign {bitor_assign}
    AssignBitOr {assign_bitor}
    fmpz_or_ui;

    BitXor {bitxor}
    BitXorAssign {bitxor_assign}
    AssignBitXor {assign_bitxor}
    fmpz_xor_ui;

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz::fmpz_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz::fmpz_sub_ui;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz::fmpz_mul_ui;

    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_tdiv_r_ui;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpz::fmpz_pow_ui;
}

impl_binop_unsafe! {
    None
    Integer, u64 {u64 u32 u16 u8}, Rational

    Div {div}
    AssignDiv {assign_div}
    fmpz_div_ui;
}

impl_binop_unsafe! {
    None
    op_assign
    Integer, i64 {i64 i32 i16 i8}, Integer

    BitAnd {bitand}
    BitAndAssign {bitand_assign}
    AssignBitAnd {assign_bitand}
    fmpz_and_si;

    BitOr {bitor}
    BitOrAssign {bitor_assign}
    AssignBitOr {assign_bitor}
    fmpz_or_si;

    BitXor {bitxor}
    BitXorAssign {bitxor_assign}
    AssignBitXor {assign_bitxor}
    fmpz_xor_si;

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz::fmpz_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz::fmpz_sub_si;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz::fmpz_mul_si;

    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_tdiv_r_si;
}

impl_binop_unsafe! {
    None
    Integer, i64 {i64 i32 i16 i8}, Rational

    Pow {pow}
    AssignPow {assign_pow}
    fmpz_pow_si;

    Div {div}
    AssignDiv {assign_div}
    fmpz_div_si;
}

impl_binop_unsafe! {
    None
    op_from
    u64 {u64 u32 u16 u8}, Integer, Integer

    BitAnd {bitand}
    BitAndFrom {bitand_from}
    AssignBitAnd {assign_bitand}
    fmpz_ui_and;

    BitOr {bitor}
    BitOrFrom {bitor_from}
    AssignBitOr {assign_bitor}
    fmpz_ui_or;

    BitXor {bitxor}
    BitXorFrom {bitxor_from}
    AssignBitXor {assign_bitxor}
    fmpz_ui_xor;

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_ui_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_ui_mul;

    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_ui_tdiv_r;
}

impl_binop_unsafe! {
    None
    u64 {u64 u32 u16 u8}, Integer, Rational

    Pow {pow}
    AssignPow {assign_pow}
    fmpz_ui_pow;

    Div {div}
    AssignDiv {assign_div}
    fmpz_ui_div;
}

impl_binop_unsafe! {
    None
    op_from
    i64 {i64 i32 i16 i8}, Integer, Integer

    BitAnd {bitand}
    BitAndFrom {bitand_from}
    AssignBitAnd {assign_bitand}
    fmpz_si_and;

    BitOr {bitor}
    BitOrFrom {bitor_from}
    AssignBitOr {assign_bitor}
    fmpz_si_or;

    BitXor {bitxor}
    BitXorFrom {bitxor_from}
    AssignBitXor {assign_bitxor}
    fmpz_si_xor;

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_si_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_si_mul;

    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_si_tdiv_r;
}

impl_binop_unsafe! {
    None
    i64 {i64 i32 i16 i8}, Integer, Rational

    Pow {pow}
    AssignPow {assign_pow}
    fmpz_si_pow;

    Div {div}
    AssignDiv {assign_div}
    fmpz_si_div;
}

#[inline]
unsafe fn fmpz_and_ui(res: *mut fmpz::fmpz, f: *const fmpz::fmpz, x: c_ulong) {
    fmpz::fmpz_init_set_ui(res, x);
    fmpz::fmpz_and(res, f, res);
}

#[inline]
unsafe fn fmpz_and_si(res: *mut fmpz::fmpz, f: *const fmpz::fmpz, x: c_long) {
    fmpz::fmpz_init_set_si(res, x);
    fmpz::fmpz_and(res, f, res);
}

#[inline]
unsafe fn fmpz_ui_and(res: *mut fmpz::fmpz, x: c_ulong, f: *const fmpz::fmpz) {
    fmpz_and_ui(res, f, x);
}

#[inline]
unsafe fn fmpz_si_and(res: *mut fmpz::fmpz, x: c_long, f: *const fmpz::fmpz) {
    fmpz_and_si(res, f, x);
}

#[inline]
unsafe fn fmpz_or_ui(res: *mut fmpz::fmpz, f: *const fmpz::fmpz, x: c_ulong) {
    fmpz::fmpz_init_set_ui(res, x);
    fmpz::fmpz_or(res, f, res);
}

#[inline]
unsafe fn fmpz_or_si(res: *mut fmpz::fmpz, f: *const fmpz::fmpz, x: c_long) {
    fmpz::fmpz_init_set_si(res, x);
    fmpz::fmpz_or(res, f, res);
}

#[inline]
unsafe fn fmpz_ui_or(res: *mut fmpz::fmpz, x: c_ulong, f: *const fmpz::fmpz) {
    fmpz_or_ui(res, f, x);
}

#[inline]
unsafe fn fmpz_si_or(res: *mut fmpz::fmpz, x: c_long, f: *const fmpz::fmpz) {
    fmpz_or_si(res, f, x);
}

#[inline]
unsafe fn fmpz_xor_ui(res: *mut fmpz::fmpz, f: *const fmpz::fmpz, x: c_ulong) {
    fmpz::fmpz_init_set_ui(res, x);
    fmpz::fmpz_xor(res, f, res);
}

#[inline]
unsafe fn fmpz_xor_si(res: *mut fmpz::fmpz, f: *const fmpz::fmpz, x: c_long) {
    fmpz::fmpz_init_set_si(res, x);
    fmpz::fmpz_xor(res, f, res);
}

#[inline]
unsafe fn fmpz_ui_xor(res: *mut fmpz::fmpz, x: c_ulong, f: *const fmpz::fmpz) {
    fmpz_xor_ui(res, f, x);
}

#[inline]
unsafe fn fmpz_si_xor(res: *mut fmpz::fmpz, x: c_long, f: *const fmpz::fmpz) {
    fmpz_xor_si(res, f, x);
}

#[inline]
unsafe fn fmpz_ui_add(res: *mut fmpz::fmpz, x: c_ulong, f: *const fmpz::fmpz) {
    fmpz::fmpz_add_ui(res, f, x);
}

#[inline]
unsafe fn fmpz_si_add(res: *mut fmpz::fmpz, x: c_long, f: *const fmpz::fmpz) {
    fmpz::fmpz_add_si(res, f, x);
}

#[inline]
unsafe fn fmpz_ui_sub(res: *mut fmpz::fmpz, x: c_ulong, f: *const fmpz::fmpz) {
    fmpz::fmpz_sub_ui(res, f, x);
    fmpz::fmpz_neg(res, res);
}

#[inline]
unsafe fn fmpz_si_sub(res: *mut fmpz::fmpz, x: c_long, f: *const fmpz::fmpz) {
    fmpz::fmpz_sub_si(res, f, x);
    fmpz::fmpz_neg(res, res);
}

#[inline]
unsafe fn fmpz_ui_mul(res: *mut fmpz::fmpz, f: c_ulong, g: *const fmpz::fmpz) {
    fmpz::fmpz_mul_ui(res, g, f);
}

#[inline]
unsafe fn fmpz_si_mul(res: *mut fmpz::fmpz, f: c_long, g: *const fmpz::fmpz) {
    fmpz::fmpz_mul_si(res, g, f);
}

#[inline]
unsafe fn fmpz_tdiv_r_ui(f: *mut fmpz::fmpz, g: *const fmpz::fmpz, h: c_ulong) {
    let r = fmpz::fmpz_tdiv_ui(g, h);
    fmpz::fmpz_set_ui(f, r);
}

#[inline]
unsafe fn fmpz_tdiv_r_si(f: *mut fmpz::fmpz, g: *const fmpz::fmpz, h: c_long) {
    let r = fmpz::fmpz_tdiv_ui(g, h as u64);
    fmpz::fmpz_set_ui(f, r);
}

#[inline]
unsafe fn fmpz_ui_tdiv_r(res: *mut fmpz::fmpz, x: c_ulong, h: *const fmpz::fmpz) {
    fmpz::fmpz_init_set_ui(res, x);
    fmpz::fmpz_tdiv_r(res, res, h);
}

#[inline]
unsafe fn fmpz_si_tdiv_r(res: *mut fmpz::fmpz, x: c_long, h: *const fmpz::fmpz) {
    fmpz::fmpz_init_set_si(res, x);
    fmpz::fmpz_tdiv_r(res, res, h);
}

#[inline]
unsafe fn fmpq_inv_fmpz(res: *mut fmpq::fmpq, f: *const fmpz::fmpz) {
    fmpq::fmpq_set_fmpz_den1(res, f);
    fmpq::fmpq_inv(res, res);
}

#[inline]
unsafe fn fmpz_pow_fmpz(res: *mut fmpq::fmpq, f: *const fmpz::fmpz, g: *const fmpz::fmpz) {
    fmpq::fmpq_set_fmpz_den1(res, f);
    fmpq::fmpq_pow_fmpz(res, res, g);
}

#[inline]
unsafe fn fmpz_pow_si(res: *mut fmpq::fmpq, f: *const fmpz::fmpz, g: c_long) {
    fmpq::fmpq_set_fmpz_den1(res, f);
    fmpq::fmpq_pow_si(res, res, g);
}

#[inline]
unsafe fn fmpz_ui_pow(res: *mut fmpq::fmpq, f: c_ulong, g: *const fmpz::fmpz) {
    fmpq::fmpq_set_ui_den1(res, f);
    fmpq::fmpq_pow_fmpz(res, res, g);
}

#[inline]
unsafe fn fmpz_si_pow(res: *mut fmpq::fmpq, f: c_long, g: *const fmpz::fmpz) {
    fmpq::fmpq_set_si_den1(res, f);
    fmpq::fmpq_pow_fmpz(res, res, g);
}

#[inline]
unsafe fn fmpz_div_ui(res: *mut fmpq::fmpq, f: *const fmpz::fmpz, g: c_ulong) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_ui(z.as_mut_ptr(), g);
    fmpq::fmpq_set_fmpz_frac(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
unsafe fn fmpz_div_si(res: *mut fmpq::fmpq, f: *const fmpz::fmpz, g: c_long) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_si(z.as_mut_ptr(), g);
    fmpq::fmpq_set_fmpz_frac(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
unsafe fn fmpz_ui_div(res: *mut fmpq::fmpq, f: c_ulong, g: *const fmpz::fmpz) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_ui(z.as_mut_ptr(), f);
    fmpq::fmpq_set_fmpz_frac(res, z.as_ptr(), g);
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
unsafe fn fmpz_si_div(res: *mut fmpq::fmpq, f: c_long, g: *const fmpz::fmpz) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_si(z.as_mut_ptr(), f);
    fmpq::fmpq_set_fmpz_frac(res, z.as_ptr(), g);
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[cfg(test)]
mod tests {
    use crate::{Integer, Rational, int};
    use inertia_algebra::ops::*;

    #[test]
    fn assign() {
        let mut res = Integer::zero();

        // assign an Integer
        res.assign(Integer::one());
        assert_eq!(res, 1);

        res.assign(&Integer::new(-2));
        assert_eq!(res, -2);
        
        // assign a primitive integer
        macro_rules! check {
            ($($ty:ident)*) => ($(
                res.assign(1 as $ty);
                assert_eq!(res, 1 as $ty);

                res.assign(&(2 as $ty));
                assert_eq!(res, 2 as $ty);
            )*)
        }

        check!{usize u8 u16 u32 u64}
        check!{isize i8 i16 i32 i64}
    }

    #[test]
    fn cmp() {
        let a = Integer::one();
        
        assert_eq!(a, Integer::one());
        assert!(a < Integer::new(2));
        assert!(a > Integer::new(0));

        macro_rules! check {
            ($($id:ident)*) => ($(
                // ==
                assert_eq!(a, 1 as $id);
                assert_eq!(1 as $id, a);

                // <
                assert!(a < (2 as $id));
                assert!((0 as $id) < a);
                
                // >
                assert!(a > (0 as $id));
                assert!((2 as $id) > a);
            )*)
        }
        
        check!{usize u8 u16 u32 u64}
        check!{isize i8 i16 i32 i64}
    }

    #[test]
    fn unops() {
        let a = Integer::new(3);
        let b = Integer::new(-3);
        
        assert_eq!(-&a, b);
        assert_eq!(-a.clone(), b);
    
        assert_eq!(!&a, Integer::new(-4));
        assert_eq!(!a.clone(), Integer::new(-4));

        assert_eq!((&a).inv(), Rational::new([1, 3]));
        assert_eq!(a.inv(), Rational::new([1, 3]));
        
    }
}
