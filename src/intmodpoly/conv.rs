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
use flint_sys::{
    fmpz_mod_poly,
    fq_default as fq
};

use std::rc::Rc;

/* TODO: maybe fix macros so this work
impl_from_unsafe! {
    ctx
    IntModPoly, IntMod
    fmpz_mod_poly::fmpz_mod_poly_set_fmpz
}
*/

impl_from! {
    IntModPoly, IntMod
    {
        fn from(x: &IntMod) -> IntModPoly {
            let temp = &x.parent.inner;
            let parent = IntModPolyRing { inner: Rc::clone(temp) };
            let mut res = parent.zero();
            unsafe {
                fmpz_mod_poly::fmpz_mod_poly_set_fmpz(
                    res.as_mut_ptr(),
                    x.as_ptr(),
                    x.ctx_as_ptr(),
                );
            }
            res
        }
    }
}

/*
impl_from! {
    IntModPoly, FinFldElem
    {
        fn from(x: &FinFldElem) -> IntModPoly {
            let parent = IntModPolyRing::init(x.prime());
            let mut res = parent.zero();
            unsafe {
                fq::fq_default_get_fmpz_mod_poly(
                    res.as_mut_ptr(),
                    x.as_ptr(),
                    x.ctx_as_ptr(),
                );
            }
            res
        }
    }
}
*/
