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

use crate::{FinFldElem, IntMod, IntModPoly, IntPoly, Integer, RatPoly, Rational, ValOrRef};
use flint_sys::fmpq_poly;
use std::ffi::{CStr, CString};

impl<'a, T> From<T> for ValOrRef<'a, RatPoly>
where
    T: Into<RatPoly>,
{
    fn from(x: T) -> ValOrRef<'a, RatPoly> {
        ValOrRef::Val(x.into())
    }
}

impl_from_unsafe! {
    None
    RatPoly, u64 {usize u64 u32 u16 u8}
    fmpq_poly::fmpq_poly_set_ui
}

impl_from_unsafe! {
    None
    RatPoly, i64 {isize i64 i32 i16 i8}
    fmpq_poly::fmpq_poly_set_si
}

impl_from_unsafe! {
    None
    RatPoly, Integer
    fmpq_poly::fmpq_poly_set_fmpz
}

impl_from_unsafe! {
    None
    RatPoly, Rational
    fmpq_poly::fmpq_poly_set_fmpq
}

impl_from_unsafe! {
    None
    RatPoly, IntPoly
    fmpq_poly::fmpq_poly_set_fmpz_poly
}

impl_from! {
    RatPoly, IntMod
    {
        fn from(x: &IntMod) -> RatPoly {
            let mut res = RatPoly::default();
            unsafe {
                fmpq_poly::fmpq_poly_set_fmpz(res.as_mut_ptr(), x.as_ptr());
            }
            res
        }
    }
}

impl_from! {
    RatPoly, IntModPoly
    {
        fn from(x: &IntModPoly) -> RatPoly {
            RatPoly::from(IntPoly::from(x))
        }
    }
}

impl_from! {
    RatPoly, FinFldElem
    {
        fn from(x: &FinFldElem) -> RatPoly {
            RatPoly::from(IntPoly::from(x))
        }
    }
}

/*
impl_from! {
    RatPoly, PadicElem
    {
        fn from(x: &PadicElem) -> RatPoly {
            let mut res = RatPoly::default();
            let tmp = Integer::from(x);
            res.set_coeff(0, &tmp);
            res
        }
    }
}*/

impl_from! {
    String, RatPoly
    {
        fn from(x: &RatPoly) -> String {
            let v = CString::new(x.var()).unwrap();
            unsafe {
                let ptr = fmpq_poly::fmpq_poly_get_str_pretty(x.as_ptr(), v.as_ptr());
                let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                flint_sys::flint::flint_free(ptr as _);
                s
            }
        }
    }
}

impl<'a, T: 'a> From<&'a [T]> for RatPoly
where
    &'a T: Into<ValOrRef<'a, Rational>>,
{
    fn from(src: &'a [T]) -> RatPoly {
        let mut res = RatPoly::default();
        for (i, x) in src.iter().enumerate() {
            res.set_coeff(i as i64, x);
        }
        res
    }
}

impl<'a, T: 'a> From<Vec<T>> for RatPoly
where
    T: Into<ValOrRef<'a, Rational>>
{
    #[inline]
    fn from(src: Vec<T>) -> RatPoly {
        let mut res = RatPoly::default();
        for (i, x) in src.into_iter().enumerate() {
            res.set_coeff(i as i64, x);
        }
        res
    }
}
