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

use crate::{IntMod, Integer, ValOrRef};

impl<'a, T> From<T> for ValOrRef<'a, IntMod>
where
    T: Into<IntMod>,
{
    fn from(x: T) -> ValOrRef<'a, IntMod> {
        ValOrRef::Val(x.into())
    }
}

impl_from! {
    String, IntMod
    {
        fn from(x: &IntMod) -> String {
            String::from(Integer::from(x))
        }
    }
}
