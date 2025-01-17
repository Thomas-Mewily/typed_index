#[cfg(feature = "serde")]
use crate::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
impl<Data, Idx> Serialize for IndexTo<Data, Idx>
where
    Data: ?Sized,
    Idx: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.index.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, Data, Idx> Deserialize<'de> for IndexTo<Data, Idx>
where
    Data: ?Sized,
    Idx: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let index = Idx::deserialize(deserializer)?;
        Ok(IndexTo::from_index(index))
    }
}
