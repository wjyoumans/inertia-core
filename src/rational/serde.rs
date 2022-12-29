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

use crate::{Integer, Rational};
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeTuple, Serializer};
use std::fmt;

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
    use crate::*;

    #[test]
    fn serde() {
        let x = Rational::from([1, 2]);
        let ser = bincode::serialize(&x).unwrap();
        let y: Rational = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
