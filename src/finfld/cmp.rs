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

use flint_sys::fq_default as fq;
use crate::{FiniteField, FinFldElem};


impl Eq for FiniteField {}
impl PartialEq for FiniteField {
    fn eq(&self, other: &FiniteField) -> bool {
        self.modulus() == other.modulus()
    }
}

impl_cmp! {
    eq
    FinFldElem
    {
        fn eq(&self, rhs: &FinFldElem) -> bool {
            assert_eq!(self.parent(), rhs.parent());
            unsafe { fq::fq_default_equal(self.as_ptr(), rhs.as_ptr(), self.ctx_as_ptr()) != 0 }
        }
    }
}


/*
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
}*/
