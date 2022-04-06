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

use crate::{IntMod, Integer, Rational, ValOrRef};
use flint_sys::fmpq;
use std::str::FromStr;

impl<'a, T> From<T> for ValOrRef<'a, Rational>
where
    T: Into<Rational>,
{
    fn from(x: T) -> ValOrRef<'a, Rational> {
        ValOrRef::Val(x.into())
    }
}

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
            _ => Err("Input is not a rational."),
        }
    }
}

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

impl_from! {
    Rational, IntMod
    {
        fn from(x: &IntMod) -> Rational {
            let mut res = Rational::default();
            unsafe { fmpq::fmpq_set_fmpz_den1(res.as_mut_ptr(), x.as_ptr()); }
            res
        }
    }
}

/*
impl_from! {
    Rational, PadicElem
    {
        fn from(x: &PadicElem) -> Rational {
            let mut res = Rational::default();
            unsafe {
                padic::padic_get_fmpq(res.as_mut_ptr(), x.as_ptr(), x.ctx_as_ptr());
            }
            res
        }
    }
}
*/

impl_from! {
    String, Rational
    {
        fn from(x: &Rational) -> String {
            if x.denominator() == 1 {
                x.numerator().to_str_radix(10)
            } else {
                format!("{}/{}", &x.numerator().to_str_radix(10), &x.denominator().to_str_radix(10))
            }
        }
    }
}

impl<'a, T: 'a> From<[T; 2]> for Rational
where
    T: Into<ValOrRef<'a, Integer>> + Clone,
{
    fn from(src: [T; 2]) -> Rational {
        let n = src[0].clone().into();
        let d = src[1].clone().into();
        assert!(!d.is_zero());

        let mut res = Rational::default();
        unsafe {
            fmpq::fmpq_set_fmpz_frac(res.as_mut_ptr(), n.as_ptr(), d.as_ptr());
        }
        res
    }
}

impl<'a, T: 'a> From<&[T]> for Rational
where
    T: Into<ValOrRef<'a, Integer>> + Clone,
{
    fn from(src: &[T]) -> Rational {
        assert_eq!(src.len(), 2);

        let n = src[0].clone().into();
        let d = src[1].clone().into();
        assert!(!d.is_zero());

        let mut res = Rational::default();
        unsafe {
            fmpq::fmpq_set_fmpz_frac(res.as_mut_ptr(), n.as_ptr(), d.as_ptr());
        }
        res
    }
}

impl<'a, T: 'a> From<Vec<T>> for Rational
where
    T: Into<ValOrRef<'a, Integer>> + Clone,
{
    #[inline]
    fn from(src: Vec<T>) -> Rational {
        Rational::from(src.as_slice())
    }
}

/*
impl<'a, T: 'a> From<Vec<T>> for IntPoly where
    T: IntoValOrRef<'a, Integer> + Clone
{
    #[inline]
    fn from(src: Vec<T>) -> IntPoly {
        IntPoly::from(src.as_slice())
    }

}

impl<T> From<[T; 2]> for Rational where
    T: Copy,
    T: Into<Integer>,
{
    #[inline]
    fn from(src: [T; 2]) -> Rational {
        Rational::from([&src[0].into(), &src[1].into()])
    }
}

impl From<[&Integer; 2]> for Rational {
    #[inline]
    fn from(src: [&Integer; 2]) -> Rational {
        assert!(!src[1].is_zero());
        let mut res = Rational::default();
        unsafe {
            fmpq::fmpq_set_fmpz_frac(
                res.as_mut_ptr(),
                src[0].as_ptr(),
                src[1].as_ptr()
            );
        }
        res
    }
}
*/
#[cfg(test)]
mod tests {
    //use crate::{Rational, IntModRing};
    use crate::Rational;

    #[test]
    fn conv() {
        assert_eq!(Rational::from(1u8), 1);
        assert_eq!(Rational::from(1u16), 1);
        assert_eq!(Rational::from(1u32), 1);
        assert_eq!(Rational::from(1u64), 1);
        assert_eq!(Rational::from(1usize), 1);

        assert_eq!(Rational::from(-1i8), -1);
        assert_eq!(Rational::from(-1i16), -1);
        assert_eq!(Rational::from(-1i32), -1);
        assert_eq!(Rational::from(-1i64), -1);
        assert_eq!(Rational::from(-1isize), -1);
        /*
            let zn = IntModRing::init(10);
            let z = zn.new(11);
            assert_eq!(Rational::from(z), 1);
        */
    }
}
