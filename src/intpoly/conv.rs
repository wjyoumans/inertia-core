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
    fmpz_poly, 
    fmpz_mod_poly, 
    fq_default as fq
};


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
