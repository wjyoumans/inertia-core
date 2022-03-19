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

use flint_sys::{fmpz, fmpq};
use libc::c_int;
use crate::{Integer, Rational, IntMod, IntModRing};

impl Eq for IntModRing {}

impl PartialEq for IntModRing {
    fn eq(&self, rhs: &IntModRing) -> bool {
        self.modulus() == rhs.modulus()
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
unsafe fn fmpz_equal_fmpq(
    f: *const fmpz::fmpz,
    g: *const fmpq::fmpq) -> c_int
{
    if fmpq::fmpq_cmp_fmpz(g, f) == 0 {
        1
    } else {
        0
    }
}
