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

use crate::{IntPoly, IntModPoly, IntModPolyRing, FinFldElem, ValOrRef};


impl<'a, T> From<T> for ValOrRef<'a, IntModPoly> where
    T: Into<IntModPoly>
{
    fn from(x: T) -> ValOrRef<'a, IntModPoly> {
        ValOrRef::Val(x.into())
    }
}

impl_from! {
    String, IntModPoly
    {
        fn from(x: &IntModPoly) -> String {
            String::from(IntPoly::from(x))
        }
    }
}

impl_from! {
    IntModPoly, FinFldElem
    {
        fn from(x: &FinFldElem) -> IntModPoly {
            let zp = IntModPolyRing::init(x.prime(), "x");
            let mut res = zp.default();
            unsafe { 
                flint_sys::fq_default::fq_default_get_fmpz_mod_poly(
                    res.as_mut_ptr(),
                    x.as_ptr(),
                    x.ctx_as_ptr(),
                );
            }
            res
        }
    }
}
