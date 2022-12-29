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

impl<'de> Deserialize<'de> for RatMat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RatMatVisitor::new())
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
        let mut entries: Vec<Rational> = Vec::with_capacity(
            access.size_hint().unwrap_or(0));
        let nrows: i64 = access.next_element()?.unwrap();
        let ncols: i64 = access.next_element()?.unwrap();

        while let Some(x) = access.next_element()? {
            entries.push(x);
        }

        let zm = RatMatSpace::init(nrows, ncols);
        Ok(zm.new(&entries[..]))
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn serde() {
        let mr = RatMatSpace::init(2, 2);
        let x = mr.new([1, 0, 0, 2]);
        let ser = bincode::serialize(&x).unwrap();
        let y: RatMat = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
