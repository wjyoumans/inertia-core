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

use crate::{IntModMat, ValOrRef};

impl<'a, T> From<T> for ValOrRef<'a, IntModMat>
where
    T: Into<IntModMat>,
{
    fn from(x: T) -> ValOrRef<'a, IntModMat> {
        ValOrRef::Val(x.into())
    }
}

impl_from! {
    String, IntModMat
    {
        fn from(x: &IntModMat) -> String {
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
