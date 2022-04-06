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

use crate::{FinFldElem, IntMod, IntModPoly, IntPoly, Integer, ValOrRef};
use flint_sys::fmpz_poly;

impl<'a, T> From<T> for ValOrRef<'a, IntPoly>
where
    T: Into<IntPoly>,
{
    fn from(x: T) -> ValOrRef<'a, IntPoly> {
        ValOrRef::Val(x.into())
    }
}

impl_from_unsafe! {
    None
    IntPoly, u64 {usize u64 u32 u16 u8}
    fmpz_poly::fmpz_poly_set_ui
}

impl_from_unsafe! {
    None
    IntPoly, i64 {isize i64 i32 i16 i8}
    fmpz_poly::fmpz_poly_set_si
}

impl_from_unsafe! {
    None
    IntPoly, Integer
    fmpz_poly::fmpz_poly_set_fmpz
}

impl_from! {
    IntPoly, IntMod
    {
        fn from(x: &IntMod) -> IntPoly {
            let mut res = IntPoly::default();
            unsafe {
                fmpz_poly::fmpz_poly_set_fmpz(res.as_mut_ptr(), x.as_ptr());
            }
            res
        }
    }
}

impl_from! {
    IntPoly, IntModPoly
    {
        fn from(x: &IntModPoly) -> IntPoly {
            let mut res = IntPoly::default();
            unsafe {
                flint_sys::fmpz_mod_poly::fmpz_mod_poly_get_fmpz_poly(
                    res.as_mut_ptr(),
                    x.as_ptr(),
                    x.ctx_as_ptr(),
                );
            }
            res
        }
    }
}

impl_from! {
    IntPoly, FinFldElem
    {
        fn from(x: &FinFldElem) -> IntPoly {
            let mut res = IntPoly::default();
            unsafe {
                flint_sys::fq_default::fq_default_get_fmpz_poly(
                    res.as_mut_ptr(),
                    x.as_ptr(),
                    x.ctx_as_ptr()
                );
            }
            res
        }
    }
}
/*
impl_from! {
    IntPoly, PadicElem
    {
        fn from(x: &PadicElem) -> IntPoly {
            let mut res = IntPoly::default();
            let tmp = Integer::from(x);
            res.set_coeff(0, &tmp);
            res
        }
    }
}*/

impl_from! {
    String, IntPoly
    {
        fn from(x: &IntPoly) -> String {
            x.get_str_pretty()
        }
    }
}

/* TODO: improve conversions.
impl<'a, T> From<T> for IntPoly where
    T: Into<ValOrRef<'a, [Integer]>>
{
    fn from(src: T) -> IntPoly {
        let mut res = IntPoly::default();
        for (i, x) in src.into().iter().enumerate() {
            res.set_coeff(i as i64, x);
        }
        res
    }
}
*/
impl<'a, T: 'a> From<&'a [T]> for IntPoly
where
    &'a T: Into<ValOrRef<'a, Integer>>,
{
    fn from(src: &'a [T]) -> IntPoly {
        let mut res = IntPoly::default();
        for (i, x) in src.iter().enumerate() {
            res.set_coeff(i as i64, x);
        }
        res
    }
}

impl<'a, T: 'a> From<Vec<T>> for IntPoly
where
    T: Into<ValOrRef<'a, Integer>>,
{
    #[inline]
    fn from(src: Vec<T>) -> IntPoly {
        let mut res = IntPoly::default();
        for (i, x) in src.into_iter().enumerate() {
            res.set_coeff(i as i64, x);
        }
        res
    }
}
