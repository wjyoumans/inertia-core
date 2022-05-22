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
use flint_sys::{fmpz, fmpq, fmpz_poly};
use libc::{c_int, c_long, c_ulong};
use crate::{Integer, Rational, IntPoly, IntPolyRing};

impl Eq for IntPolyRing {}

impl PartialEq for IntPolyRing {
    fn eq(&self, rhs: &IntPolyRing) -> bool {
        self.var() == rhs.var()
    }
}

impl_cmp_unsafe! {
    eq
    IntPoly
    fmpz_poly::fmpz_poly_equal
}

impl_cmp_unsafe! {
    eq
    IntPoly, Integer
    fmpz_poly::fmpz_poly_equal_fmpz
}

impl_cmp_unsafe! {
    eq
    IntPoly, Rational
    fmpz_poly_equal_fmpq
}

impl_cmp_unsafe! {
    eq
    IntPoly, u64 {u64 u32 u16 u8}
    fmpz_poly_equal_ui
}

impl_cmp_unsafe! {
    eq
    IntPoly, i64 {i64 i32 i16 i8}
    fmpz_poly_equal_si
}

#[inline]
unsafe fn fmpz_poly_equal_fmpq(
    f: *const fmpz_poly::fmpz_poly_struct,
    x: *const fmpq::fmpq,
    ) -> c_int
{
    if fmpz::fmpz_is_one(&(*x).den) == 1 {
        fmpz_poly::fmpz_poly_equal_fmpz(f, &(*x).num)
    } else {
        0
    }
}

#[inline]
unsafe fn fmpz_poly_equal_ui(
    f: *const fmpz_poly::fmpz_poly_struct,
    x: c_ulong,
    ) -> c_int
{
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_ui(z.as_mut_ptr(), x);
    let b = fmpz_poly::fmpz_poly_equal_fmpz(f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
    b
}

#[inline]
unsafe fn fmpz_poly_equal_si(
    f: *const fmpz_poly::fmpz_poly_struct,
    x: c_long,
    ) -> c_int
{
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_si(z.as_mut_ptr(), x);
    let b = fmpz_poly::fmpz_poly_equal_fmpz(f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
    b
}
