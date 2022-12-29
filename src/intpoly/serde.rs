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


use crate::{Integer, IntPoly};
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::fmt;

impl Serialize for IntPoly {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let coeffs = self.get_coeffs();
        let mut seq = serializer.serialize_seq(Some(coeffs.len()))?;
        for e in coeffs.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct IntPolyVisitor {}

impl IntPolyVisitor {
    fn new() -> Self {
        IntPolyVisitor {}
    }
}

impl<'de> Visitor<'de> for IntPolyVisitor {
    type Value = IntPoly;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IntPoly")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut coeffs: Vec<Integer> = Vec::with_capacity(
            access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            coeffs.push(x);
        }
        Ok(IntPoly::new(&coeffs[..]))
    }
}

impl<'de> Deserialize<'de> for IntPoly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(IntPolyVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn serde() {
        let x = IntPoly::new([1, 0, 0, 2, 1]);
        let ser = bincode::serialize(&x).unwrap();
        let y: IntPoly = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
