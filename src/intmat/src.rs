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
//use serde::ser::{Serialize, Serializer, SerializeSeq};
//use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess};
use crate::{
    ops::Assign,
    Integer,
    IntegerRing,
    ValOrRef, 
};

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct IntMatSpace {
    nrows: i64,
    ncols: i64
}

impl fmt::Display for IntMatSpace {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Space of {} by {} matrices over Integer Ring", self.nrows, self.ncols)
    }
}

impl Hash for IntMatSpace {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nrows().hash(state);
        self.ncols().hash(state);
    }
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

    #[inline]
    pub fn nrows(&self) -> i64 {
        self.nrows
    }
    
    #[inline]
    pub fn ncols(&self) -> i64 {
        self.ncols
    }

    #[inline]
    pub fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
    }
}

#[derive(Debug)]
pub struct IntMat {
    inner: fmpz_mat::fmpz_mat_struct,
}

impl<'a, T> Assign<T> for IntMat where
    T: Into<ValOrRef<'a, IntMat>>
{
    fn assign(&mut self, other: T) {
        let x = other.into();
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

impl fmt::Display for IntMat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for IntMat {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz_mat::fmpz_mat_clear(self.as_mut_ptr())}
    }
}

impl Hash for IntMat {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entries().hash(state);
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
        unsafe { fmpz_mat::fmpz_mat_is_empty(self.as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_square(&self) -> bool {
        unsafe { fmpz_mat::fmpz_mat_is_square(self.as_ptr()) != 0 }
    }
    
    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { fmpz_mat::fmpz_mat_is_zero(self.as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fmpz_mat::fmpz_mat_is_one(self.as_ptr()) != 0 }
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
        T: Into<ValOrRef<'a, Integer>>
    {
        unsafe {
            let x = fmpz_mat::fmpz_mat_entry(self.as_ptr(), i, j);
            fmpz::fmpz_set(x, e.into().as_ptr());
        }
    }
    
    pub fn get_str_pretty(&self) -> String {
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

/*
impl Serialize for IntMat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries = self.entries();
        let mut seq = serializer.serialize_seq(Some(entries.len()))?;
        for e in entries.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct IntMatVisitor {}

impl IntMatVisitor {
    fn new() -> Self {
        IntMatVisitor {}
    }
}

impl<'de> Visitor<'de> for IntMatVisitor {
    type Value = IntMat;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IntMat")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut entries: Vec<Integer> = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            entries.push(x);
        }

        Ok(IntMat::from(entries))
    }
}

impl<'de> Deserialize<'de> for IntMat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(IntMatVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::IntMat;

    #[test]
    fn serde() {
        let x = IntMat::from(vec![1, 0, 0, 2, 1]);
        let ser = bincode::serialize(&x).unwrap();
        let y: IntMat = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}*/
