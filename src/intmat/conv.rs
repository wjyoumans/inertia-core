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

use flint_sys::fmpz_mat;
use crate::{IntMat, Integer, IntModMat, ValOrRef};
use std::mem::MaybeUninit;

impl<'a, T> From<T> for ValOrRef<'a, IntMat>
where
    T: Into<IntMat>,
{
    fn from(x: T) -> ValOrRef<'a, IntMat> {
        ValOrRef::Val(x.into())
    }
}

impl_from! {
    IntMat, IntModMat
    {
        fn from(x: &IntModMat) -> IntMat {
            unsafe {
                let mut z = MaybeUninit::uninit();
                fmpz_mat::fmpz_mat_init_set(z.as_mut_ptr(), &(*x.as_ptr()).mat[0]);
                IntMat::from_raw(z.assume_init())
            }
        }
    }
}

impl_from! {
    String, IntMat
    {
        fn from(x: &IntMat) -> String {
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

impl<'a, T: 'a> From<&[&'a [T]]> for IntMat
where
    &'a T: Into<ValOrRef<'a, Integer>>,
{
    fn from(mat: &[&'a [T]]) -> IntMat {
        let m = mat.len() as i64;
        let n = mat.first().unwrap_or(&vec![].as_slice()).len() as i64;

        let mut res = IntMat::default(m, n);
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

impl<'a, T: 'a> From<Vec<&'a [T]>> for IntMat
where
    &'a T: Into<ValOrRef<'a, Integer>>,
{
    fn from(mat: Vec<&'a [T]>) -> IntMat {
        IntMat::from(mat.as_slice())
    }
}

impl<'a, T> From<Vec<Vec<T>>> for IntMat
where
    T: Into<ValOrRef<'a, Integer>>,
{
    fn from(mat: Vec<Vec<T>>) -> IntMat {
        let m = mat.len() as i64;
        let n = mat.first().unwrap_or(&vec![]).len() as i64;

        let mut res = IntMat::default(m, n);
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
