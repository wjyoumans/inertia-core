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
use flint_sys::{fmpq, fmpq_mat};
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use crate::{ops::Assign, Integer, IntegerRing, Rational, RationalField, ValOrRef};

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct RatMatSpace {
    nrows: i64,
    ncols: i64,
}

impl Eq for RatMatSpace {}

impl PartialEq for RatMatSpace {
    fn eq(&self, other: &RatMatSpace) -> bool {
        if self.nrows() == other.nrows() && self.ncols() == other.ncols() {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for RatMatSpace {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Space of {} by {} matrices over {}",
            self.nrows, 
            self.ncols, 
            self.base_ring()
        )
    }
}

impl Hash for RatMatSpace {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nrows().hash(state);
        self.ncols().hash(state);
    }
}

impl RatMatSpace {
    /// Initialize the space of matrices with the given number of rows and columns.
    #[inline]
    pub fn init(nrows: i64, ncols: i64) -> Self {
        RatMatSpace { nrows, ncols }
    }

    #[inline]
    pub fn default(&self) -> RatMat {
        RatMat::new(self.nrows, self.ncols)
    }

    #[inline]
    pub fn new<'a, T: 'a>(&self, entries: &'a [T]) -> RatMat
    where
        &'a T: Into<ValOrRef<'a, Rational>>,
    {
        let nrows = self.nrows() as usize;
        let ncols = self.ncols() as usize;
        assert_eq!(entries.len(), nrows * ncols);

        let mut row = 0;
        let mut col;
        let mut res = self.default();
        for (i, x) in entries.iter().enumerate() {
            col = (i % ncols) as i64;
            if col == 0 && i != 0 {
                row += 1;
            }

            res.set_entry(row, col, x);
        }
        res
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
    pub fn base_ring(&self) -> RationalField {
        RationalField {}
    }
}

#[derive(Debug)]
pub struct RatMat {
    inner: fmpq_mat::fmpq_mat_struct,
}

impl<'a, T> Assign<T> for RatMat
where
    T: Into<ValOrRef<'a, RatMat>>,
{
    fn assign(&mut self, other: T) {
        let x = other.into();
        assert_eq!(self.nrows(), x.nrows());
        assert_eq!(self.ncols(), x.ncols());
        unsafe {
            fmpq_mat::fmpq_mat_set(self.as_mut_ptr(), x.as_ptr());
        }
    }
}

impl Clone for RatMat {
    #[inline]
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpq_mat::fmpq_mat_init_set(z.as_mut_ptr(), self.as_ptr());
            RatMat {
                inner: z.assume_init(),
            }
        }
    }
}

impl fmt::Display for RatMat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for RatMat {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpq_mat::fmpq_mat_clear(self.as_mut_ptr()) }
    }
}

impl Hash for RatMat {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entries().hash(state);
    }
}

impl RatMat {
    /// Returns a pointer to the inner [FLINT rational matrix][fmpq_mat::fmpq_mat].
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpq_mat::fmpq_mat_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [FLINT rational matrix][fmpq_mat::fmpq_mat].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpq_mat::fmpq_mat_struct {
        &mut self.inner
    }

    #[inline]
    pub fn new(nrows: i64, ncols: i64) -> RatMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpq_mat::fmpq_mat_init(z.as_mut_ptr(), nrows, ncols);
            RatMat {
                inner: z.assume_init(),
            }
        }
    }

    #[inline]
    pub fn parent(&self) -> RatMatSpace {
        RatMatSpace {
            nrows: self.nrows(),
            ncols: self.ncols(),
        }
    }

    /// Return the number of rows of the matrix.
    #[inline]
    pub fn nrows(&self) -> i64 {
        unsafe { fmpq_mat::fmpq_mat_nrows(self.as_ptr()) }
    }

    /// Return the number of columns of the matrix.
    #[inline]
    pub fn ncols(&self) -> i64 {
        unsafe { fmpq_mat::fmpq_mat_ncols(self.as_ptr()) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { fmpq_mat::fmpq_mat_is_empty(self.as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_square(&self) -> bool {
        unsafe { fmpq_mat::fmpq_mat_is_square(self.as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { fmpq_mat::fmpq_mat_is_zero(self.as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fmpq_mat::fmpq_mat_is_one(self.as_ptr()) != 0 }
    }

    /// Get the `(i, j)`-th entry of a rational matrix.
    #[inline]
    pub fn get_entry(&self, i: i64, j: i64) -> Rational {
        let mut res = Rational::default();
        unsafe {
            let x = fmpq_mat::fmpq_mat_entry(self.as_ptr(), i, j);
            fmpq::fmpq_set(res.as_mut_ptr(), x);
        }
        res
    }

    /// Set the `(i, j)`-th entry of a rational matrix.
    #[inline]
    pub fn set_entry<'a, T>(&mut self, i: i64, j: i64, e: T)
    where
        T: Into<ValOrRef<'a, Rational>>,
    {
        unsafe {
            let x = fmpq_mat::fmpq_mat_entry(self.as_ptr(), i, j);
            fmpq::fmpq_set(x, e.into().as_ptr());
        }
    }

    pub fn entries(&self) -> Vec<Rational> {
        let r = self.nrows();
        let c = self.ncols();
        let mut out = Vec::with_capacity(usize::try_from(r * c).ok().unwrap());

        for i in 0..r {
            for j in 0..c {
                out.push(self.get_entry(i, j));
            }
        }
        out
    }
}

impl Serialize for RatMat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries = self.entries();
        let mut seq = serializer.serialize_seq(Some(entries.len() + 2))?;

        seq.serialize_element(&self.nrows())?;
        seq.serialize_element(&self.ncols())?;
        for e in entries.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct RatMatVisitor {}

impl RatMatVisitor {
    fn new() -> Self {
        RatMatVisitor {}
    }
}

impl<'de> Visitor<'de> for RatMatVisitor {
    type Value = RatMat;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a RatMat")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut entries: Vec<Integer> = Vec::with_capacity(access.size_hint().unwrap_or(0));
        let nrows = access.next_element()?.unwrap();
        let ncols = access.next_element()?.unwrap();

        while let Some(x) = access.next_element()? {
            entries.push(x);
        }

        let zm = RatMatSpace::init(nrows, ncols);
        Ok(zm.new(&entries))
    }
}

impl<'de> Deserialize<'de> for RatMat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RatMatVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::RatMat;

    #[test]
    fn serde() {
        let x = RatMat::from(vec![vec![1, 0], vec![0, 2]]);
        let ser = bincode::serialize(&x).unwrap();
        let y: RatMat = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
