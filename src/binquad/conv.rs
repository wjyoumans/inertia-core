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

use crate::{Integer, BinQuadForm};
use flint_sys::fmpz::fmpz_set;


impl<T: Into<Integer>> From<[T; 3]> for BinQuadForm {
    fn from(src: [T; 3]) -> BinQuadForm {
        match src {
            [a, b, c] => {
                let mut res = BinQuadForm::default();
                unsafe {
                    fmpz_set(&mut res.inner.a[0], a.into().as_ptr());
                    fmpz_set(&mut res.inner.b[0], b.into().as_ptr());
                    fmpz_set(&mut res.inner.c[0], c.into().as_ptr());
                }
                res
            }
        }
    }
}

impl From<[&Integer; 3]> for BinQuadForm {
    fn from(src: [&Integer; 3]) -> BinQuadForm {
        match src {
            [a, b, c] => {
                let mut res = BinQuadForm::default();
                unsafe {
                    fmpz_set(&mut res.inner.a[0], a.as_ptr());
                    fmpz_set(&mut res.inner.b[0], b.as_ptr());
                    fmpz_set(&mut res.inner.c[0], c.as_ptr());
                }
                res
            }
        }
    }
}
