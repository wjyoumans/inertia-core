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
use flint_sys::fmpq_poly;


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
    RatPoly, IntMod
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
    RatPoly, IntModPoly
    {
        fn from(x: &IntModPoly) -> RatPoly {
            RatPoly::from(IntPoly::from(x))
        }
    }
}

/*
impl_from! {
    RatPoly, FinFldElem
    {
        fn from(x: &FinFldElem) -> RatPoly {
            RatPoly::from(IntPoly::from(x))
        }
    }
}*/

impl<T, const CAP: usize> From<[T; CAP]> for RatPoly
where
    T: Into<Rational>
{
    fn from(coeffs: [T; CAP]) -> RatPoly {
        let mut res = RatPoly::with_capacity(coeffs.len());
        for (i, x) in coeffs.into_iter().enumerate() {
            res.set_coeff(i, x.into());
        }
        res
    }
}

impl<const CAP: usize> From<[&Rational; CAP]> for RatPoly {
    fn from(coeffs: [&Rational; CAP]) -> RatPoly {
        let mut res = RatPoly::with_capacity(coeffs.len());
        for (i, x) in coeffs.into_iter().enumerate() {
            res.set_coeff(i, x);
        }
        res
    }
}

impl<'a, T> From<&'a [T]> for RatPoly 
where
    &'a T: Into<Rational>
{
    fn from(coeffs: &'a [T]) -> RatPoly {
        let mut res = RatPoly::with_capacity(coeffs.len());
        for (i, x) in coeffs.iter().enumerate() {
            res.set_coeff(i, x.into());
        }
        res
    }
}

impl From<&[Rational]> for RatPoly {
    fn from(coeffs: &[Rational]) -> RatPoly {
        let mut res = RatPoly::with_capacity(coeffs.len());
        for (i, x) in coeffs.iter().enumerate() {
            res.set_coeff(i, x);
        }
        res
    }
}
