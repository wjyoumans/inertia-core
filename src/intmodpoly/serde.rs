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

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};

impl Serialize for IntModPoly {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let coeffs = self
            .coefficients()
            .iter()
            .map(|x| Integer::from(x))
            .collect::<Vec<_>>();
        let mut seq = serializer.serialize_seq(Some(coeffs.len() + 1))?;
        seq.serialize_element(&self.modulus())?;
        for e in coeffs.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct IntModPolyVisitor {}

impl IntModPolyVisitor {
    fn new() -> Self {
        IntModPolyVisitor {}
    }
}

impl<'de> Visitor<'de> for IntModPolyVisitor {
    type Value = IntModPoly;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IntModPoly")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut coeffs: Vec<Integer> = Vec::with_capacity(
            access.size_hint().unwrap_or(0));
        let m: Integer = access
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        while let Some(x) = access.next_element()? {
            coeffs.push(x);
        }
        let zn = IntModPolyRing::init(m, "x");
        Ok(zn.new(&coeffs[..]))
    }
}

impl<'de> Deserialize<'de> for IntModPoly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(IntModPolyVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn serde() {
        let zn = IntModPolyRing::init(72u32, "x");
        let x = zn.new([1, 0, 0, 2, -19]);
        let ser = bincode::serialize(&x).unwrap();
        let y: IntModPoly = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
