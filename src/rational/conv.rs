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

use crate::{Integer, Rational, IntMod};
use flint_sys::fmpq;
use std::str::FromStr;

impl FromStr for Rational {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s.split("/").collect::<Vec<_>>();
        match r.len() {
            1 => Ok(Rational::from(Integer::from_str(r[0])?)),
            2 => Ok(Rational::from([
                Integer::from_str(r[0])?,
                Integer::from_str(r[1])?,
            ])),
            _ => Err("Input must be of the form \"x\" or \"x/y\" where x and y are 
                     integers."),
        }
    }
}

///////////////////////////////////////////////////////////////////
// From
///////////////////////////////////////////////////////////////////

impl_from_unsafe! {
    None
    Rational, u64 {usize u64 u32 u16 u8}
    fmpq::fmpq_set_ui_den1
}

impl_from_unsafe! {
    None
    Rational, i64 {isize i64 i32 i16 i8}
    fmpq::fmpq_set_si_den1
}

impl_from_unsafe! {
    None
    Rational, Integer
    fmpq::fmpq_set_fmpz_den1
}

impl_from_unsafe! {
    None
    Rational, IntMod
    fmpq::fmpq_set_fmpz_den1
}

impl<T: Into<Integer>> From<[T; 2]> for Rational {
    fn from(src: [T; 2]) -> Rational {
        match src {
            [num, den] => {
                let d = den.into();
                assert!(!d.is_zero());
                let mut res = Rational::default();
                unsafe {
                    fmpq::fmpq_set_fmpz_frac(
                        res.as_mut_ptr(), 
                        num.into().as_ptr(), 
                        d.as_ptr()
                    );
                }
                res
            }
        }
    }
}

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
