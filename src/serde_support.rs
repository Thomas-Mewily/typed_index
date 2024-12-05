#[cfg(feature = "serde")]
use crate::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
impl<Data, In, Idx> Serialize for TypedIndex<Data, In, Idx>
where
    Data: ?Sized,
    In: ?Sized,
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
impl<'de, Data, In, Idx> Deserialize<'de> for TypedIndex<Data, In, Idx>
where
    Data: ?Sized,
    In: ?Sized,
    Idx: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let index = Idx::deserialize(deserializer)?;
        Ok(TypedIndex {
            index,
            index_data: PhantomData,
            index_owner: PhantomData,
        })
    }
}
