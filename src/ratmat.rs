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

mod ops;
//mod conv;

//#[cfg(feature = "serde")]
//mod serde;

use crate::*;
use flint_sys::{fmpq, fmpq_mat};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;


#[derive(Debug)]
pub struct RatMat {
    inner: fmpq_mat::fmpq_mat_struct,
}

impl AsRef<RatMat> for RatMat {
    fn as_ref(&self) -> &RatMat {
        self
    }
}

impl Clone for RatMat {
    #[inline]
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpq_mat::fmpq_mat_init_set(z.as_mut_ptr(), self.as_ptr());
            RatMat::from_raw(z.assume_init())
        }
    }
}

impl fmt::Display for RatMat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = self.nrows().try_into().expect(
            "Cannot convert signed long to usize.");
        let c = self.ncols().try_into().expect(
            "Cannot convert signed long to usize.");
        let mut out = Vec::with_capacity(r);

        for i in 0..r {
            let mut row = Vec::with_capacity(c + 2);
            row.push("[".to_string());
            for j in 0..c {
                row.push(format!(" {} ", self.get_entry(i, j)));
            }
            if i == r - 1 {
                row.push("]".to_string());
            } else {
                row.push("]\n".to_string());
            }
            out.push(row.join(""));
        }
        write!(f, "{}", out.join(""))
    }
}

impl Drop for RatMat {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpq_mat::fmpq_mat_clear(self.as_mut_ptr()) }
    }
}

// TODO: make entries method that borrows so we dont need to copy entries
impl Hash for RatMat {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_entries().hash(state);
    }
}

impl<const CAP: usize> NewMatrix<[&Rational; CAP]> for RatMat {
    fn new(src: [&Rational; CAP], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = RatMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.into_iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x);
        }
        res
    }
}

impl<T, const CAP: usize> NewMatrix<[T; CAP]> for RatMat 
where
    T: Into<Rational>
{
    fn new(src: [T; CAP], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = RatMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.into_iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x.into());
        }
        res
    }
}

impl NewMatrix<&[Rational]> for RatMat {
    fn new(src: &[Rational], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = RatMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x);
        }
        res
    }
}

impl<'a, T> NewMatrix<&'a [T]> for RatMat
where
    &'a T: Into<Rational>
{
    fn new(src: &'a [T], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = RatMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x.into());
        }
        res
    }
}

impl RatMat {
    // private helper methods to convert usize indices to i64, emit consistent
    // messages on panic, and bounds check
    fn check_indices(&self, i: usize, j: usize) -> (i64, i64) {
        (self.check_row_index(i), self.check_col_index(j))
    }

    fn check_row_index(&self, i: usize) -> i64 {
        let i = i.try_into().expect("Cannot convert index to a signed long.");
        assert!(i < self.nrows_si());
        i
    }
    
    fn check_col_index(&self, j: usize) -> i64 {
        let j = j.try_into().expect("Cannot convert index to a signed long.");
        assert!(j < self.ncols_si());
        j
    }

    #[inline]
    pub fn new<S>(src: S, nrows: i64, ncols: i64) -> RatMat 
    where
        Self: NewMatrix<S>
    {
        <RatMat as NewMatrix<S>>::new(src, nrows, ncols)
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const fmpq_mat::fmpq_mat_struct {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpq_mat::fmpq_mat_struct {
        &mut self.inner
    }

    #[inline]
    pub fn from_raw(raw: fmpq_mat::fmpq_mat_struct) -> RatMat {
        RatMat { inner: raw }
    }

    #[inline]
    pub fn zero(nrows: i64, ncols: i64) -> RatMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpq_mat::fmpq_mat_init(z.as_mut_ptr(), nrows, ncols);
            RatMat::from_raw(z.assume_init())
        }
    }
    
    #[inline]
    pub fn one(dim: i64) -> RatMat {
        let mut res = RatMat::zero(dim, dim);
        unsafe {
            fmpq_mat::fmpq_mat_one(res.as_mut_ptr());
        }
        res
    }

    /// Set `self` to the zero matrix.
    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe {
            fmpq_mat::fmpq_mat_zero(self.as_mut_ptr());
        }
    }
    
    /// Set `self` to the identity matrix. Panics if the matrix is not square.
    #[inline]
    pub fn one_assign(&mut self) {
        assert!(self.is_square());
        unsafe {
            fmpq_mat::fmpq_mat_one(self.as_mut_ptr());
        }
    }

    /// Return the number of rows.
    #[inline]
    pub fn nrows(&self) -> usize {
        self.nrows_si().try_into().expect("Cannot convert signed long to usize.")
    }
    
    /// Return the number of rows.
    #[inline]
    pub fn nrows_si(&self) -> i64 {
        unsafe { fmpq_mat::fmpq_mat_nrows(self.as_ptr())}
    }

    /// Return the number of columns.
    #[inline]
    pub fn ncols(&self) -> usize {
        self.ncols_si().try_into().expect("Cannot convert signed long to usize.")
    }
    
    /// Return the number of columns.
    #[inline]
    pub fn ncols_si(&self) -> i64 {
        unsafe { fmpq_mat::fmpq_mat_ncols(self.as_ptr())}
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

    /// Get the `(i, j)`-th entry of the matrix.
    #[inline]
    pub fn get_entry(&self, i: usize, j: usize) -> Rational {
        let mut res = Rational::zero();
        self.assign_entry(i, j, &mut res);
        res
    }
    
    // TODO: need consistent naming convention
    // even better: remove, replace with 'entry' returning a borrow which can 
    // be assigned.
    /// Get the `(i, j)`-th entry of an integer matrix and assign it to `out`. 
    /// Avoids unnecessary allocation.
    #[inline]
    pub fn assign_entry(&self, i: usize, j: usize, out: &mut Rational) {
        let (i, j) = self.check_indices(i, j);
        unsafe {
            let x = fmpq_mat::fmpq_mat_entry(self.as_ptr(), i, j);
            fmpq::fmpq_set(out.as_mut_ptr(), x);
        }
    }

    /// Set the `(i, j)`-th entry of the matrix.
    #[inline]
    pub fn set_entry<T: AsRef<Rational>>(&mut self, i: usize, j: usize, e: T) {
        let (i, j) = self.check_indices(i, j);
        unsafe {
            let x = fmpq_mat::fmpq_mat_entry(self.as_ptr(), i, j);
            fmpq::fmpq_set(x, e.as_ref().as_ptr());
        }
    }

    /// Get a vector with all of the entries of the matrix.
    pub fn get_entries(&self) -> Vec<Rational> {
        let r = self.nrows();
        let c = self.ncols();
        let mut out = Vec::with_capacity(r * c);

        for i in 0..r {
            for j in 0..c {
                out.push(self.get_entry(i, j));
            }
        }
        out
    }

    /*
    /// Swap two integer matrices. The dimensions are allowed to be different.
    #[inline]
    pub fn swap(&mut self, other: &mut RatMat) {
        unsafe { 
            fmpq_mat::fmpq_mat_swap(self.as_mut_ptr(), other.as_mut_ptr()); 
        }
    }

    /// Swap the rows `r1` and `r2` of an integer matrix. 
    pub fn swap_rows(&mut self, r1: usize, r2: usize) {
        let r1 = self.check_row_index(r1);
        let r2 = self.check_row_index(r2);
        unsafe { 
            fmpq_mat::fmpq_mat_swap_rows(
                self.as_mut_ptr(), 
                std::ptr::null(),
                r1,
                r2
            ); 
        }
    }
    
    /// Swap the columns `r` and `s` of an integer matrix. 
    pub fn swap_cols(&mut self, c1: usize, c2: usize) {
        let c1 = self.check_col_index(c1);
        let c2 = self.check_col_index(c2);
        unsafe { 
            fmpq_mat::fmpq_mat_swap_rows(
                self.as_mut_ptr(), 
                std::ptr::null(),
                c1,
                c2
            ); 
        }
    }
    
    /// Swap row `i` and `r - i` for `0 <= i < r/2` where `r` is the number 
    /// of rows of the input matrix.
    #[inline]
    pub fn invert_rows(&mut self) {
        unsafe { 
            fmpq_mat::fmpq_mat_invert_rows(
                self.as_mut_ptr(), 
                std::ptr::null()
            ); 
        }
    }
    
    /// Swap columns `i` and `c - i` for `0 <= i < c/2` where `c` is the number
    /// of columns of the input matrix.
    #[inline]
    pub fn invert_columns(&mut self) {
        unsafe { 
            fmpq_mat::fmpq_mat_invert_cols(
                self.as_mut_ptr(), 
                std::ptr::null()
            ); 
        }
    }
   
    /* TODO: function missing from bindings
    /// Swap two integer matrices by swapping the individual entries rather 
    /// than swapping the contents of their structs.
    #[inline]
    pub fn swap_entrywise(&mut self, other: &mut RatMat) {
        unsafe { 
            fmpq_mat::fmpq_mat_swap_entrywise(
                self.as_mut_ptr(), 
                other.as_mut_ptr()
            ); 
        }
    }
    */

    /*
    /// Return true if the matrix is invertible.
    #[inline]
    pub fn is_invertible(&self) -> bool {
        self.is_square() && !self.det().is_zero()
    }*/

    /// Return true if row `i` is all zeros.
    pub fn is_zero_row(&self, i: usize) -> bool { 
        let i = self.check_row_index(i);
        unsafe {
            fmpq_mat::fmpq_mat_is_zero_row(self.as_ptr(), i) != 0
        }
    }

    /// Return true if column `i` is all zeros.
    // TODO: Does an additional allocation compared to `is_zero_row`.
    #[inline]
    pub fn is_zero_col(&self, i: usize) -> bool {
        self.column(i).is_zero()
    }

    /// Return the transpose.
    #[inline]
    pub fn transpose(&self) -> RatMat {
        let mut res = RatMat::zero(self.nrows_si(), self.ncols_si());
        unsafe {
            fmpq_mat::fmpq_mat_transpose(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }

    /// Transpose the matrix in place. Panics if the matrix is not square.
    #[inline]
    pub fn transpose_assign(&mut self) {
        assert!(self.is_square());
        unsafe { fmpq_mat::fmpq_mat_transpose(self.as_mut_ptr(), self.as_ptr()); }
    }
    
    /// Horizontally concatenate two matrices. Panics if the number of rows of 
    /// both matrices do not agree.
    pub fn hcat<T>(&self, other: T) -> RatMat where
        T: AsRef<RatMat>
    {
        let other = other.as_ref();
        let nrows = self.nrows_si();
        assert_eq!(nrows, other.nrows_si());

        let mut res = RatMat::zero(nrows, self.ncols_si() + other.ncols_si());
        unsafe {
            fmpq_mat::fmpq_mat_concat_horizontal(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
        }
        res
    }
    
    /// Vertically concatenate two matrices. Panics if the number of columns of 
    /// both matrices do not agree.
    pub fn vcat<T>(&self, other: T) -> RatMat where
        T: AsRef<RatMat>
    {
        let other = other.as_ref();
        let ncols = self.ncols_si();
        assert_eq!(ncols, other.ncols_si());

        let mut res = RatMat::zero(self.nrows_si() + other.nrows_si(), ncols);
        unsafe {
            fmpq_mat::fmpq_mat_concat_horizontal(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
        }
        res
    }
   
    // TODO: 'window' version to avoid allocation
    /// Return a new matrix containing the `r2 - r1` by `c2 - c1` submatrix of 
    /// an integer matrix whose `(0, 0)` entry is the `(r1, c1)` entry of the input.
    pub fn submatrix(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> RatMat {
        if r1 == r2 || c1 == c2 {
            return RatMat::zero(0, 0)
        }
        
        assert!(r1 <= r2);
        assert!(c1 <= c2);
        let (r1, c1) = self.check_indices(r1, c1);
        let (r2, c2) = self.check_indices(r2, c2);

        let mut res = RatMat::zero(r2 - r1, c2 - c1);
        let mut win = MaybeUninit::uninit();
        unsafe {
            fmpq_mat::fmpq_mat_window_init(
                win.as_mut_ptr(), 
                self.as_ptr(),
                r1,
                c1,
                r2,
                c2
            );
            fmpq_mat::fmpq_mat_set(res.as_mut_ptr(), win.as_ptr());
            fmpq_mat::fmpq_mat_window_clear(win.as_mut_ptr());
        }
        res

    }
    
    /// Return row `i` as an integer matrix.
    #[inline]
    pub fn row(&self, i: usize) -> RatMat {
        self.submatrix(i, 0, i + 1, self.ncols())
    }
   
    /// Return column `j` as an integer matrix.
    #[inline]
    pub fn column(&self, j: usize) -> RatMat {
        self.submatrix(0, j, self.nrows(), j + 1)
    }

    /// Square an integer matrix. The matrix must be square.
    #[inline]
    pub fn square(&self) -> Self {
        assert!(self.is_square());
        let mut res = RatMat::zero(self.nrows_si(), self.ncols_si());
        unsafe {
            fmpq_mat::fmpq_mat_sqr(res.as_mut_ptr(), self.as_ptr()) 
        }
        res
    }
    
    /// Square an integer matrix in place. The matrix must be square.
    #[inline]
    pub fn square_assign(&mut self) {
        assert!(self.is_square());
        unsafe { 
            fmpq_mat::fmpq_mat_sqr(self.as_mut_ptr(), self.as_ptr());
        }
    }
    
    /// Return the kronecker product of two integer matrices.
    pub fn kronecker_product<T>(&self, other: T) -> RatMat where 
        T: AsRef<RatMat>
    {
        let other = other.as_ref();
        let mut res = RatMat::zero(
            self.nrows_si() * other.nrows_si(),
            self.ncols_si() * other.ncols_si()
        );
        unsafe { 
            fmpq_mat::fmpq_mat_kronecker_product(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            ); 
        }
        res
    }
    
    /// Compute the trace of a square integer matrix.
    #[inline]
    pub fn trace(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::zero();
        unsafe { 
            fmpq_mat::fmpq_mat_trace(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }

    /// Return the content of an integer matrix, that is, the gcd of all its 
    /// entries. Returns zero if the matrix is empty.
    #[inline]
    pub fn content(&self) -> Integer {
        let mut res = Integer::zero();
        unsafe { 
            fmpq_mat::fmpq_mat_content(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Compute the determinant of the matrix.
    #[inline]
    pub fn det(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::zero();
        unsafe { 
            fmpq_mat::fmpq_mat_det(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Return an absolute upper bound on the determinant of a square integer 
    /// matrix computed from the Hadamard inequality.
    #[inline]
    pub fn det_bound(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::zero();
        unsafe { 
            fmpq_mat::fmpq_mat_det_bound(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Return a positive divisor of the determinant of a square integer matrix. 
    /// If the determinant is zero this will always return zero.
    #[inline]
    pub fn det_divisor(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::zero();
        unsafe { 
            fmpq_mat::fmpq_mat_det_divisor(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Applies a similarity transform to an `n` by `n` integer matrix. If `P` 
    /// is the identity matrix whose zero entries in row `r` have been replaced 
    /// by `d`, this transform is equivalent to `P^-1 * M * P`. 
    #[inline]
    pub fn similarity<T>(&self, r: usize, d: T) -> RatMat where 
        T: AsRef<Integer>
    {
        let mut res = self.clone();
        res.similarity_assign(r, d);
        res
    }
    
    /// Applies a similarity transform to an `n` by `n` integer matrix in place.
    pub fn similarity_assign<T>(&mut self, r: usize, d: T) where 
        T: AsRef<Integer>
    {
        let r = self.check_row_index(r);
        assert!(self.is_square());
        unsafe { 
            fmpq_mat::fmpq_mat_similarity(
                self.as_mut_ptr(), 
                r.into(),
                d.as_ref().as_ptr()
            ); 
        }
    }
  
    /// Return the characteristic polynomial of a square integer matrix.
    #[inline]
    pub fn charpoly(&self) -> IntPoly {
        assert!(self.is_square());
        let mut res = IntPoly::zero();
        unsafe { 
            fmpq_mat::fmpq_mat_charpoly(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Return the minimal polynomial of a square integer matrix.
    #[inline]
    pub fn minpoly(&self) -> IntPoly {
        assert!(self.is_square());
        let mut res = IntPoly::zero();
        unsafe { 
            fmpq_mat::fmpq_mat_minpoly(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }

    /// Return the rank of a matrix, that is, the number of linearly independent 
    /// columns (equivalently, rows) of an integer matrix. The rank is computed by 
    /// row reducing a copy of the input matrix.
    #[inline]
    pub fn rank(&self) -> i64 {
        unsafe { fmpq_mat::fmpq_mat_rank(self.as_ptr()) }
    }
    */
}
