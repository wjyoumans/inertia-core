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
use flint_sys::fmpz;
use crate::{Integer, IntegerRing};

impl Eq for IntegerRing {}

impl PartialEq for IntegerRing {
    fn eq(&self, _rhs: &IntegerRing) -> bool {
        true
    }
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
    eq
    Integer, u64 {u64 u32 u16 u8}
    fmpz::fmpz_equal_ui
}

impl_cmp_unsafe! {
    ord
    Integer, u64 {u64 u32 u16 u8}
    fmpz::fmpz_cmp_ui
}

impl_cmp_unsafe! {
    eq
    Integer, i64 {i64 i32 i16 i8}
    fmpz::fmpz_equal_si
}

impl_cmp_unsafe! {
    ord
    Integer, i64 {i64 i32 i16 i8}
    fmpz::fmpz_cmp_si
}
