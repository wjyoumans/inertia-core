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
    fmpz_poly_q::*, 
    //fmpz_poly::fmpz_poly_struct, 
};

impl From<IntPoly> for RatFunc {
    fn from(x: IntPoly) -> RatFunc {
        let res = RatFunc::default();
        unsafe {
            *res.inner.num = x.into_raw()
        }
        res
    }
}

impl From<&IntPoly> for RatFunc {
    #[inline]
    fn from(x: &IntPoly) -> RatFunc {
        RatFunc::from(x.clone())
    }
}

macro_rules! derive_from_intpoly {
    ($($ident:ident)*) => ($(
        impl_from! {
            RatFunc, $ident
            {
                fn from(x: &$ident) -> RatFunc {
                    RatFunc::from(IntPoly::from(x))
                }
            }
        }
    )*);
}

derive_from_intpoly! { 
    usize u64 u32 u16 u8 
    isize i64 i32 i16 i8 
    Integer IntMod IntModPoly FinFldElem 
}


impl<T: Into<IntPoly>> From<[T; 2]> for RatFunc {
    fn from(src: [T; 2]) -> RatFunc {
        match src {
            [num, den] => {
                let d = den.into();
                assert!(!d.is_zero());
                let mut res = RatFunc::default();
                unsafe {
                    *res.inner.num = num.into().into_raw();
                    *res.inner.den = d.into_raw();
                    fmpz_poly_q_canonicalise(res.as_mut_ptr());
                }
                res
            }
        }
    }
}

impl From<[&IntPoly; 2]> for RatFunc {
    fn from(src: [&IntPoly; 2]) -> RatFunc {
        match src {
            [num, den] => {
                assert!(!den.is_zero());
                let mut res = RatFunc::default();
                unsafe {
                    *res.inner.num = num.clone().into_raw();
                    *res.inner.den = den.clone().into_raw();
                    fmpz_poly_q_canonicalise(res.as_mut_ptr());
                }
                res
            }
        }
    }
}

/*
impl From<[&Integer; 2]> for Rational {
    fn from(src: [&Integer; 2]) -> Rational {
        match src {
            [num, den] => {
                assert!(!den.is_zero());
                let mut res = Rational::default();
                unsafe {
                    fmpq::fmpq_set_fmpz_frac(
                        res.as_mut_ptr(), 
                        num.as_ptr(), 
                        den.as_ptr()
                    );
                }
                res
            }
        }
    }
}
*/
/*
impl_from_unsafe! {
    None
    IntPoly, u64 {usize u64 u32 u16 u8}
    fmpz_poly::fmpz_poly_set_ui
}
*/

/*
impl_from_unsafe! {
    None
    RatFunc, i64 {isize i64 i32 i16 i8}
    fmpz_poly_q_set_si
}

macro_rules! derive_from_intpoly {
    ($($ident:ident)*) => ($(
        impl_from! {
            RatFunc, $ident
            {
                fn from(x: &$ident) -> RatFunc {
                    RatFunc::from(IntPoly::from(x))
                }
            }
        }
    )*);
}

derive_from_intpoly! { usize u64 u32 u16 u8 Integer IntMod IntModPoly FinFldElem }


impl_from_unsafe! {
    None
    IntPoly, IntMod
    fmpz_poly::fmpz_poly_set_fmpz
}

impl_from_unsafe! {
    ctx_in
    IntPoly, IntModPoly
    fmpz_mod_poly::fmpz_mod_poly_get_fmpz_poly
}

impl_from_unsafe! {
    ctx_in
    IntPoly, FinFldElem
    fq::fq_default_get_fmpz_poly
}

impl<T, const CAP: usize> From<[T; CAP]> for IntPoly
where
    T: Into<Integer>
{
    fn from(coeffs: [T; CAP]) -> IntPoly {
        let mut res = IntPoly::with_capacity(coeffs.len());
        for (i, x) in coeffs.into_iter().enumerate() {
            res.set_coeff(i, x.into());
        }
        res
    }
}

impl<const CAP: usize> From<[&Integer; CAP]> for IntPoly {
    fn from(coeffs: [&Integer; CAP]) -> IntPoly {
        let mut res = IntPoly::with_capacity(coeffs.len());
        for (i, x) in coeffs.into_iter().enumerate() {
            res.set_coeff(i, x);
        }
        res
    }
}

impl<'a, T> From<&'a [T]> for IntPoly 
where
    &'a T: Into<Integer>
{
    fn from(coeffs: &'a [T]) -> IntPoly {
        let mut res = IntPoly::with_capacity(coeffs.len());
        for (i, x) in coeffs.iter().enumerate() {
            res.set_coeff(i, x.into());
        }
        res
    }
}

impl From<&[Integer]> for IntPoly {
    fn from(coeffs: &[Integer]) -> IntPoly {
        let mut res = IntPoly::with_capacity(coeffs.len());
        for (i, x) in coeffs.iter().enumerate() {
            res.set_coeff(i, x);
        }
        res
    }
}
*/
