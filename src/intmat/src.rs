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

use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;

use flint_sys::{fmpz, fmpz_mat};
use crate::{
    ops::Assign,
    Integer,
    ValOrRef, 
    IntoValOrRef
};

#[derive(Clone, Copy, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct IntMatSpace {
    nrows: i64,
    ncols: i64
}

impl IntMatSpace {

    /// Initialize the space of matrices with the given number of rows and columns.
    #[inline]
    pub fn init(nrows: i64, ncols: i64) -> Self {
        IntMatSpace { nrows, ncols }
    }

    #[inline]
    pub fn default(&self) -> IntMat {
        IntMat::new(self.nrows, self.ncols)
    }
    
    #[inline]
    pub fn new<T>(&self, x: T) -> IntMat where
        T: Into<IntMat>
    {
        x.into()
    }
}

#[derive(Debug)]
pub struct IntMat {
    inner: fmpz_mat::fmpz_mat_struct,
}

impl<'a, T> Assign<T> for IntMat where
    T: IntoValOrRef<'a, IntMat>
{
    fn assign(&mut self, other: T) {
        let x = other.val_or_ref();
        assert_eq!(self.nrows(), x.nrows());
        assert_eq!(self.ncols(), x.ncols());
        unsafe {
            fmpz_mat::fmpz_mat_set(self.as_mut_ptr(), x.as_ptr());
        }
    }
}

impl Clone for IntMat {
    #[inline]
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            fmpz_mat::fmpz_mat_init_set(z.as_mut_ptr(), self.as_ptr()); 
            IntMat { inner: z.assume_init() }
        }
    }
}

impl Drop for IntMat {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz_mat::fmpz_mat_clear(self.as_mut_ptr())}
    }
}

impl fmt::Display for IntMat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Hash for IntMat {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entries().hash(state);
    }
}

impl<'a, T> IntoValOrRef<'a, IntMat> for T where
    T: Into<IntMat>
{
    #[inline]
    fn val_or_ref(self) -> ValOrRef<'a, IntMat> {
        ValOrRef::Val(self.into())
    }
}

impl IntMat {
    
    /// Returns a pointer to the inner [FLINT integer matrix][fmpz_mat::fmpz_mat].
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpz_mat::fmpz_mat_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [FLINT integer matrix][fmpz_mat::fmpz_mat].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpz_mat::fmpz_mat_struct {
        &mut self.inner
    }
 
    #[inline]
    pub fn new(nrows: i64, ncols: i64) -> IntMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mat::fmpz_mat_init(z.as_mut_ptr(), nrows, ncols);
            IntMat { inner: z.assume_init() }
        }
    }

    #[inline]
    pub fn parent(&self) -> IntMatSpace {
        IntMatSpace { nrows: self.nrows(), ncols: self.ncols() }
    }

    /// Return the number of rows of the integer matrix.
    #[inline]
    pub fn nrows(&self) -> i64 {
        unsafe { fmpz_mat::fmpz_mat_nrows(self.as_ptr()) }
    }

    /// Return the number of columns of the integer matrix.
    #[inline]
    pub fn ncols(&self) -> i64 {
        unsafe { fmpz_mat::fmpz_mat_ncols(self.as_ptr()) }
    }
    
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nrows() == 0 || self.ncols() == 0
    }

    #[inline]
    pub fn is_square(&self) -> bool {
        self.nrows() == self.ncols()
    }
    
    /// Get the `(i, j)`-th entry of an integer matrix.
    #[inline]
    pub fn get_entry(&self, i: i64, j: i64) -> Integer {
        let mut res = Integer::default();
        unsafe {
            let x = fmpz_mat::fmpz_mat_entry(self.as_ptr(), i, j);
            fmpz::fmpz_set(res.as_mut_ptr(), x);
        }
        res
    }
    
    /// Set the `(i, j)`-th entry of an integer matrix.
    #[inline]
    pub fn set_entry<'a, T>(&mut self, i: i64, j: i64, e: T) where
        T: IntoValOrRef<'a, Integer>
    {
        unsafe {
            let x = fmpz_mat::fmpz_mat_entry(self.as_ptr(), i, j);
            fmpz::fmpz_set(x, e.val_or_ref().as_ptr());
        }
    }
    
    pub fn get_str(&self) -> String {
        let r = self.nrows();
        let c = self.ncols();
        let mut out = Vec::with_capacity(usize::try_from(r).ok().unwrap());

        for i in 0..r {
            let mut row = Vec::with_capacity(usize::try_from(c).ok().unwrap() + 2);
            row.push("[".to_string());
            for j in 0..c {
                row.push(format!(" {} ", self.get_entry(i, j)));
            }
            if i == r-1 {
                row.push("]".to_string());
            } else {
                row.push("]\n".to_string());
            }
            out.push(row.join(""));
        }
        out.join("")
    }

    pub fn entries(&self) -> Vec<Integer> {
        let r = self.nrows();
        let c = self.ncols();
        let mut out = Vec::with_capacity(usize::try_from(r*c).ok().unwrap());

        for i in 0..r {
            for j in 0..c {
                out.push(self.get_entry(i, j));
            }
        }
        out
    }
}
