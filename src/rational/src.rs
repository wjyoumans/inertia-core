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

use crate::Integer;
use flint_sys::fmpq;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeTuple, Serializer};

#[derive(Clone, Copy, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct RationalField {}
pub type Rationals = RationalField;

impl Eq for RationalField {}

impl PartialEq for RationalField {
    fn eq(&self, _rhs: &RationalField) -> bool {
        true
    }
}

impl fmt::Display for RationalField {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rational field")
    }
}

impl RationalField {
    #[inline]
    pub fn init() -> Self {
        RationalField {}
    }

    #[inline]
    pub fn default(&self) -> Rational {
        Rational::default()
    }

    #[inline]
    pub fn new<T: Into<Rational>>(&self, x: T) -> Rational {
        x.into()
    }
}

#[derive(Debug)]
pub struct Rational {
    inner: fmpq::fmpq,
}

impl Clone for Rational {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = Rational::default();
        unsafe {
            fmpq::fmpq_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for Rational {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpq::fmpq_init(z.as_mut_ptr());
            Rational {
                inner: z.assume_init(),
            }
        }
    }
}

impl fmt::Display for Rational {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for Rational {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpq::fmpq_clear(self.as_mut_ptr()) }
    }
}

impl Hash for Rational {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numerator().hash(state);
        self.denominator().hash(state);
    }
}

impl Rational {
    /// Returns a pointer to the inner [FLINT rational][fmpq::fmpq].
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpq::fmpq {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [FLINT rational][fmpq::fmpq].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpq::fmpq {
        &mut self.inner
    }
    
    /// Instantiate an `Rational` from a [FLINT rational][fmpq::fmpq].
    #[inline]
    pub fn from_raw(raw: fmpq::fmpq) -> Rational {
        Rational { inner: raw }
    }

    /// Returns the numerator of a rational number as an [Integer].
    ///
    /// ```
    /// use inertia_core::Rational;
    ///
    /// let q = Rational::from([3, 4]);
    /// assert_eq!(q.numerator(), 3);
    /// ```
    #[inline]
    pub fn numerator(&self) -> Integer {
        Integer::from_raw(self.inner.num)
    }

    /// Returns the denominator of a rational number as an [Integer].
    ///
    /// ```
    /// use inertia_core::Rational;
    ///
    /// let q = Rational::from([3, 4]);
    /// assert_eq!(q.denominator(), 4);
    /// ```
    #[inline]
    pub fn denominator(&self) -> Integer {
        Integer::from_raw(self.inner.den)
    }
}

impl Serialize for Rational {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_tuple(2)?;
        state.serialize_element(&self.numerator())?;
        state.serialize_element(&self.denominator())?;
        state.end()
    }
}

struct RationalVisitor {}

impl RationalVisitor {
    fn new() -> Self {
        RationalVisitor {}
    }
}

impl<'de> Visitor<'de> for RationalVisitor {
    type Value = Rational;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Rational")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let num: Integer = access
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let den: Integer = access
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;

        Ok(Rational::from([num, den]))
    }
}

impl<'de> Deserialize<'de> for Rational {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, RationalVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::Rational;

    #[test]
    fn serde() {
        let x = Rational::from([1, 2]);
        let ser = bincode::serialize(&x).unwrap();
        let y: Rational = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
