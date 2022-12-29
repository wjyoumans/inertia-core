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


use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};

impl Serialize for RatPoly {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let coeffs = self.coefficients();
        let mut seq = serializer.serialize_seq(Some(coeffs.len()))?;
        for e in coeffs.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct RatPolyVisitor {}

impl RatPolyVisitor {
    fn new() -> Self {
        RatPolyVisitor {}
    }
}

impl<'de> Visitor<'de> for RatPolyVisitor {
    type Value = RatPoly;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a RatPoly")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut coeffs: Vec<Integer> = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            coeffs.push(x);
        }

        let rx = RatPolyRing::init("x");
        Ok(rx.new(&coeffs[..]))
        //Ok(RatPoly::from(&coeffs[..]))
    }
}

impl<'de> Deserialize<'de> for RatPoly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RatPolyVisitor::new())
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::RatPoly;

    #[test]
    fn serde() {
        let x = RatPoly::from(vec![1, 0, 0, 2, 1]);
        let ser = bincode::serialize(&x).unwrap();
        let y: RatPoly = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}*/
