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

use std::cmp::Ordering::{self, Less, Greater, Equal};
use flint_sys::{fmpz, fmpq};
use libc::c_int;
use crate::{Integer, Rational};

impl_cmp_unsafe! {
    eq
    Rational
    fmpq::fmpq_equal
}

impl_cmp_unsafe! {
    ord
    Rational
    fmpq::fmpq_cmp
}

impl_cmp_unsafe! {
    eq
    Rational, Integer
    fmpq_equal_fmpz
}

impl_cmp_unsafe! {
    ord
    Rational, Integer
    fmpq::fmpq_cmp_fmpz
}

impl_cmp_unsafe! {
    eq
    Rational, u64 {u64 u32 u16 u8}
    fmpq::fmpq_equal_ui
}

impl_cmp_unsafe! {
    ord
    Rational, u64 {u64 u32 u16 u8}
    fmpq::fmpq_cmp_ui
}

impl_cmp_unsafe! {
    eq
    Rational, i64 {i64 i32 i16 i8}
    fmpq::fmpq_equal_si
}

impl_cmp_unsafe! {
    ord
    Rational, i64 {i64 i32 i16 i8}
    fmpq::fmpq_cmp_si
}

#[inline]
unsafe fn fmpq_equal_fmpz(
    f: *const fmpq::fmpq,
    g: *const fmpz::fmpz) -> c_int
{
    if fmpq::fmpq_cmp_fmpz(f, g) == 0 {
        1
    } else {
        0
    }
}
