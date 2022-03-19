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

use crate::{Integer, IntMat, IntoValOrRef};

/*
impl_from! {
    IntMat, IntModMat
    {
        fn from(x: &IntModMat) -> IntMat {
            IntMat { data: x.data.mat[0].clone() }
        }
    }
}
*/

impl_from! {
    String, IntMat
    {
        fn from(x: &IntMat) -> String {
            x.get_str()
        }
    }
}

// TODO: why can't we pass x.val_or_ref() and avoid clone?
impl<'a, T> From<&[&[T]]> for IntMat where
    T: IntoValOrRef<'a, Integer> + Clone
{
    fn from(mat: &[&[T]]) -> IntMat {
        let m = mat.len() as i64;
        let n = mat.first().unwrap_or(&vec![].as_slice()).len() as i64;

        let mut res = IntMat::new(m, n);
        if m == 0 || n == 0 {
            res
        } else {
            for (i, &row) in mat.iter().enumerate() {
                assert_eq!(n, row.len() as i64);
                for (j, x) in row.iter().enumerate() {
                    res.set_entry(i as i64, j as i64, &*x.clone().val_or_ref());
                }
            }
            res
        }
    }
}

impl<'a, T> From<Vec<Vec<T>>> for IntMat where
    T: IntoValOrRef<'a, Integer>
{
    fn from(mat: Vec<Vec<T>>) -> IntMat {
        let m = mat.len() as i64;
        let n = mat.first().unwrap_or(&vec![]).len() as i64;

        let mut res = IntMat::new(m, n);
        if m == 0 || n == 0 {
            res
        } else {
            for (i, row) in mat.into_iter().enumerate() {
                assert_eq!(n, row.len() as i64);
                for (j, x) in row.into_iter().enumerate() {
                    res.set_entry(i as i64, j as i64, &*x.val_or_ref());
                }
            }
            res
        }
    }
}
