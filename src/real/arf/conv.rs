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

use crate::{*, arf::Arf, mag::Mag};
use arb_sys::arf::*;

impl_assign_unsafe! {
    None
    Arf, Arf
    arf_set
}

impl_assign_unsafe! {
    None
    Arf, u64 {usize u64 u32 u16 u8}
    arf_set_ui
}

impl_assign_unsafe! {
    None
    Arf, i64 {isize i64 i32 i16 i8}
    arf_set_si
}

impl_assign_unsafe! {
    None
    Arf, f64 {f64 f32}
    arf_set_d
}

impl_assign_unsafe! {
    None
    Arf, Integer
    arf_set_fmpz
}

impl_assign_unsafe! {
    None
    Arf, Mag
    arf_set_mag
}

impl_from_unsafe! {
    None
    Arf, u64 {usize u64 u32 u16 u8}
    arf_set_ui
}

impl_from_unsafe! {
    None
    Arf, i64 {isize i64 i32 i16 i8}
    arf_set_si
}

impl_from_unsafe! {
    None
    Arf, f64 {f64 f32}
    arf_set_d
}

impl_from_unsafe! {
    None
    Arf, Integer
    arf_set_fmpz
}

impl_from_unsafe! {
    None
    Arf, Mag
    arf_set_mag
}
