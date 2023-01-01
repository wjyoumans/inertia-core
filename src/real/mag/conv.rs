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

use crate::{*, mag::Mag};
use arb_sys::mag::*;

impl_assign_unsafe! {
    None
    Mag, Mag
    mag_set
}

// sets to upper bound for absolute value, can be inexact
impl_assign_unsafe! {
    None
    Mag, u64 {usize u64 u32 u16 u8}
    mag_set_ui
}

// sets to upper bound for absolute value, can be inexact
impl_assign_unsafe! {
    None
    Mag, f64 {f64 f32}
    mag_set_d
}

// sets to upper bound for absolute value, can be inexact
impl_assign_unsafe! {
    None
    Mag, Integer
    mag_set_fmpz
}

impl_from_unsafe! {
    None
    Mag, u64 {usize u64 u32 u16 u8}
    mag_set_ui
}

impl_from_unsafe! {
    None
    Mag, f64 {f64 f32}
    mag_set_d
}

impl_from_unsafe! {
    None
    Mag, Integer
    mag_set_fmpz
}
