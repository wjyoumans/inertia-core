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

use crate::Integer;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::fmt;

impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ui_vec = self.get_ui_vector();
        let mut seq = serializer.serialize_seq(Some(ui_vec.len()))?;
        for e in ui_vec.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct IntegerVisitor {}

impl IntegerVisitor {
    fn new() -> Self {
        IntegerVisitor {}
    }
}

impl<'de> Visitor<'de> for IntegerVisitor {
    type Value = Integer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an Integer")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec_ui = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            vec_ui.push(x);
        }

        let mut out = Integer::default();
        out.set_ui_vector(vec_ui);
        Ok(out)
    }
}

impl<'de> Deserialize<'de> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(IntegerVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::Integer;

    #[test]
    fn serde() {
        let x: Integer = "18446744073709551616".parse().unwrap();
        let ser = bincode::serialize(&x).unwrap();
        let y: Integer = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
