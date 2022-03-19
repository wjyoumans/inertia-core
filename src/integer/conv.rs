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

use std::convert::TryFrom;
use std::ffi::CString;
use flint_sys::fmpz;
use crate::{Integer, Rational, IntMod};

impl From<&str> for Integer {
    fn from(s: &str) -> Integer {
        let c_str = CString::new(s).expect("String contains 0 byte.");

        let mut z = Integer::default();
        unsafe {
            let res = flint_sys::fmpz::fmpz_set_str(z.as_mut_ptr(), c_str.as_ptr(), 10);
            assert_eq!(res, 0);
        }
        z
    }
}

impl_from_unsafe! {
    None
    Integer, u64 {usize u64 u32 u16 u8}
    fmpz::fmpz_set_ui
}

impl_from_unsafe! {
    None
    Integer, i64 {isize i64 i32 i16 i8}
    fmpz::fmpz_set_si
}

impl_from! {
    Integer, IntMod
    {
        fn from(x: &IntMod) -> Integer {
            unsafe { Integer::from_raw(*x.as_ptr()) }
        }
    }
}

/*
impl_from! {
    Integer, PadicElem
    {
        fn from(x: &PadicElem) -> Integer {
            let mut res = Integer::default();
            unsafe {
                flint_sys::padic::padic_get_fmpz(res.as_mut_ptr(), x.as_ptr(), x.ctx_as_ptr());
            }
            res
        }
    }
}
*/

impl_tryfrom! {
    Integer, Rational
    {
        fn try_from(x: &Rational) -> Result<Self,Self::Error> {
            if x.denominator().is_one() {
                Ok(x.numerator())
            } else {
                Err("Rational cannot be coerced to an Integer.")
            }
        }
    }
}

impl_from! {
    String, Integer
    {
        fn from(x: &Integer) -> String {
           format!("{}", &x.to_str_radix(10))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Integer;

    #[test]
    fn integer_from_ui() {
        assert_eq!(Integer::from(1u8), 1);
        assert_eq!(Integer::from(1u16), 1);
        assert_eq!(Integer::from(1u32), 1);
        assert_eq!(Integer::from(1u64), 1);
        assert_eq!(Integer::from(1usize), 1);
    }
    
    #[test]
    fn integer_from_si() {
        assert_eq!(Integer::from(-1i8), -1);
        assert_eq!(Integer::from(-1i16), -1);
        assert_eq!(Integer::from(-1i32), -1);
        assert_eq!(Integer::from(-1i64), -1);
        assert_eq!(Integer::from(-1isize), -1);
    }
 
    /*
    #[test]
    fn integer_from_intmod() {
        let zn = IntModRing::init(10);
        let z = zn.new(11);
        assert_eq!(Integer::from(z), 1);
    }*/
    
    #[test]
    fn integer_from_str() {
    }
    
    #[test]
    fn string_from_integer() {
    }
}
