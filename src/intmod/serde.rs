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

use crate::{Integer, IntMod, IntModCtx};
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeTuple, Serializer};
use std::fmt;

impl Serialize for IntMod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_tuple(2)?;
        state.serialize_element(&Integer::from(self))?;
        state.serialize_element(&self.modulus())?;
        state.end()
    }
}

struct IntModVisitor {}

impl IntModVisitor {
    fn new() -> Self {
        IntModVisitor {}
    }
}

impl<'de> Visitor<'de> for IntModVisitor {
    type Value = IntMod;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IntMod")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let val: Integer = access
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let modulus: Integer = access
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;


        let ctx = IntModCtx::new(modulus);
        Ok(IntMod::new(val, &ctx))
    }
}

impl<'de> Deserialize<'de> for IntMod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, IntModVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn serde() {
        let ctx = IntModCtx::new(12);
        let x = IntMod::new("18446744073709551616".parse::<Integer>().unwrap(), &ctx);
        let ser = bincode::serialize(&x).unwrap();
        let y: Integer = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
