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

use crate::{RatMat, Rational, ValOrRef};

impl<'a, T> From<T> for ValOrRef<'a, RatMat>
where
    T: Into<RatMat>,
{
    fn from(x: T) -> ValOrRef<'a, RatMat> {
        ValOrRef::Val(x.into())
    }
}

/*
impl_from! {
    RatMat, IntModMat
    {
        fn from(x: &IntModMat) -> RatMat {
            RatMat { data: x.data.mat[0].clone() }
        }
    }
}
*/

impl_from! {
    String, RatMat
    {
        fn from(x: &RatMat) -> String {
            let r = x.nrows();
            let c = x.ncols();
            let mut out = Vec::with_capacity(usize::try_from(r).ok().unwrap());

            for i in 0..r {
                let mut row = Vec::with_capacity(usize::try_from(c).ok().unwrap() + 2);
                row.push("[".to_string());
                for j in 0..c {
                    row.push(format!(" {} ", x.get_entry(i, j)));
                }
                if i == r - 1 {
                    row.push("]".to_string());
                } else {
                    row.push("]\n".to_string());
                }
                out.push(row.join(""));
            }
            out.join("")
        }
    }
}

impl<'a, T: 'a> From<&[&'a [T]]> for RatMat
where
    &'a T: Into<ValOrRef<'a, Rational>>,
{
    fn from(mat: &[&'a [T]]) -> RatMat {
        let m = mat.len() as i64;
        let n = mat.first().unwrap_or(&vec![].as_slice()).len() as i64;

        let mut res = RatMat::default(m, n);
        if m == 0 || n == 0 {
            res
        } else {
            for (i, &row) in mat.iter().enumerate() {
                assert_eq!(n, row.len() as i64);
                for (j, x) in row.iter().enumerate() {
                    res.set_entry(i as i64, j as i64, &*x.into());
                }
            }
            res
        }
    }
}

impl<'a, T: 'a> From<Vec<&'a [T]>> for RatMat
where
    &'a T: Into<ValOrRef<'a, Rational>>,
{
    fn from(mat: Vec<&'a [T]>) -> RatMat {
        RatMat::from(mat.as_slice())
    }
}

impl<'a, T> From<Vec<Vec<T>>> for RatMat
where
    T: Into<ValOrRef<'a, Rational>>,
{
    fn from(mat: Vec<Vec<T>>) -> RatMat {
        let m = mat.len() as i64;
        let n = mat.first().unwrap_or(&vec![]).len() as i64;

        let mut res = RatMat::default(m, n);
        if m == 0 || n == 0 {
            res
        } else {
            for (i, row) in mat.into_iter().enumerate() {
                assert_eq!(n, row.len() as i64);
                for (j, x) in row.into_iter().enumerate() {
                    res.set_entry(i as i64, j as i64, x);
                }
            }
            res
        }
    }
}
