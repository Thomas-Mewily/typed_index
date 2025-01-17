//! A strongly typed index that know what it is indexing : `IndexTo<Data, Idx=usize>`
//! 
//! Also define a `10usize.get(&myVec)` and `IndexLike.getMut(Inside)` to access value from an index using the [IndexLike] trait.
//! 
//! Provides optional support for [Serde](https://docs.rs/serde/latest/serde/) (serialization / deserialization) when the "serde" feature is enabled.
//! 
//! ```rust
//! use typed_index::*;
//! use std::ops::Index;
//! 
//! struct IntAndBool
//! {
//!     integers : Vec<i32>,
//!     booleans : Vec<bool>,
//! }
//! 
//! type IntegerIdx = IndexTo<i32>;
//! type BooleanIdx = IndexTo<bool>;
//! 
//! impl Index<IntegerIdx> for IntAndBool
//! {
//!     type Output=i32;
//!     fn index(&self, index: IntegerIdx) -> &Self::Output { &self.integers[index.index()] }
//! }
//! 
//! impl Index<BooleanIdx> for IntAndBool
//! {
//!     type Output=bool;
//!     fn index(&self, index: BooleanIdx) -> &Self::Output { &self.booleans[index.index()] }
//! }
//! 
//! let mut int_and_bool = IntAndBool { integers : vec![10, 20, 30], booleans : vec![true, false] };
//! 
//! let int_idx  = int_and_bool.integers.index_to(1); // 20
//! let bool_idx = int_and_bool.booleans.index_to(0); // true
//! 
//! // the magic in strongly typed index is here :
//! assert_eq!(int_and_bool[int_idx ], 20);
//! assert_eq!(int_and_bool[bool_idx], true);
//! 
//! // compile time error :
//! // let b = int_and_bool.booleans[int_idx ];
//! // let i = int_and_bool.integers[bool_idx];
//! 
//! let int_idx_2  = IntegerIdx::from_index(1); // 20
//! let bool_idx_2 = BooleanIdx::from_index(0); // true
//! 
//! assert_eq!(int_and_bool[int_idx_2 ], 20);
//! assert_eq!(int_and_bool[bool_idx_2], true);
//! 
//! // Also defined the `.get()` and `.get_mut()` method on index
//! assert_eq!(int_idx_2.get(&int_and_bool), &20);
//! assert_eq!(bool_idx_2.get(&int_and_bool), &true);
//! 
//! // Also define the `typed_index()` method similar to `index()`
//! assert_eq!(int_and_bool.integers.typed_index(IntegerIdx::from_index(1)), &20);
//! assert_eq!(int_and_bool.booleans.typed_index(BooleanIdx::from_index(0)), &true);
//! 
//! // Also define the `typed_index_mut()` method similar to `index_mut()`
//! assert_eq!(int_and_bool.integers.typed_index_mut(IntegerIdx::from_index(1)), &mut 20);
//! assert_eq!(int_and_bool.booleans.typed_index_mut(BooleanIdx::from_index(0)), &mut true);
//! 
//! 
//! // `index.get_mut(&mut collection)` example
//! impl IndexMut<IntegerIdx> for IntAndBool
//! {
//!     fn index_mut(&mut self, index: IntegerIdx) -> &mut Self::Output { &mut self.integers[index.index()] }
//! }
//! 
//! assert_eq!(int_idx_2.get(&int_and_bool), &20);
//! *int_idx_2.get_mut(&mut int_and_bool) = 50;
//! assert_eq!(int_idx_2.get(&int_and_bool), &50);
//! ```

use std::fmt::{Debug, Formatter, Result as DResult};
use std::marker::PhantomData;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

#[cfg(feature = "serde")]
pub(crate) mod serde_support;
#[cfg(feature = "serde")]
pub(crate) use serde_support::*;

mod std_impl;

mod index_extension;
pub use index_extension::*;

/// A strongly typed index that know what it is indexing 
pub struct IndexTo<Data, Idx=usize> 
    where
    Data : ?Sized, 
{
    index : Idx,
    index_data  : PhantomData<Data>,
}

impl<Data, Idx> IndexTo<Data, Idx>
    where
    Data : ?Sized, 
{
    #[inline]
    pub const fn from_index(index : Idx) -> Self { Self { index, index_data: PhantomData }}
    #[inline]
    pub const fn index(self) -> Idx where Idx : Copy { self.index }
    #[inline]
    pub fn set_index(&mut self, index : Idx) -> &mut Self { self.index = index; self }
    #[inline]
    pub fn with_index(mut self, index : Idx) -> Self { self.set_index(index); self }
}

impl<Data : ?Sized, Idx> Hash       for IndexTo<Data, Idx> where Idx : Hash       { #[inline] fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.index.hash(state); } }
impl<Data : ?Sized, Idx> Clone      for IndexTo<Data, Idx> where Idx : Clone      { #[inline] fn clone(&self) -> Self { Self::from_index(self.index.clone()) } }
impl<Data : ?Sized, Idx> Copy       for IndexTo<Data, Idx> where Idx : Copy       {}
impl<Data : ?Sized, Idx> Debug      for IndexTo<Data, Idx> where Idx : Debug      { fn fmt(&self, f: &mut Formatter<'_>) -> DResult { write!(f, "{}#{:?}", std::any::type_name::<Data>(), self.index) } }
impl<Data : ?Sized, Idx> Eq         for IndexTo<Data, Idx> where Idx : Eq         {}
impl<Data : ?Sized, Idx> PartialEq  for IndexTo<Data, Idx> where Idx : PartialEq  { #[inline] fn eq(&self, other: &Self) -> bool { self.index == other.index } }
impl<Data : ?Sized, Idx> Ord        for IndexTo<Data, Idx> where Idx : Ord        { #[inline] fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.index.cmp(&other.index) } }
impl<Data : ?Sized, Idx> PartialOrd for IndexTo<Data, Idx> where Idx : PartialOrd { #[inline] fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { self.index.partial_cmp(&other.index) } }

pub trait HaveTypedIndex<Idx=usize> : Index<Idx>
{
    type IndexTo;
    type Output : ?Sized;
    /// return a strongly typed index for vector, slice, and str type.
    fn index_to(&self, index : Idx) -> Self::IndexTo;
}
impl<Idx, T> HaveTypedIndex<Idx> for T where T : Index<Idx> + ?Sized
{
    type IndexTo = IndexTo<<T as Index<Idx>>::Output, Idx>;
    /// The returned type after indexing.
    type Output = T::Output;

    fn index_to(&self, index : Idx) -> Self::IndexTo {
        IndexTo::from_index(index)
    }
}

pub trait TypedIndex<Idx=usize> : Index<Idx>
{
    // The documentation comment is copied from the standard library

    /// Performs the indexing (`container[index]`) operation.
    ///
    /// # Panics
    ///
    /// May panic if the index is out of bounds.
    fn typed_index(&self, index : IndexTo<Self::Output, Idx>) -> &<Self as Index<Idx>>::Output { self.index(index.index) }
}
impl<Idx, T> TypedIndex<Idx> for T where T : Index<Idx> { }

pub trait TypedIndexMut<Idx=usize> : IndexMut<Idx>
{
    // The documentation comment is copied from the standard library

    /// Performs the mutable indexing (`container[index]`) operation.
    ///
    /// # Panics
    ///
    /// May panic if the index is out of bounds.
    fn typed_index_mut(&mut self, index : IndexTo<Self::Output, Idx>) -> &mut <Self as Index<Idx>>::Output { self.index_mut(index.index) }
}
impl<Idx, T> TypedIndexMut<Idx> for T where T : IndexMut<Idx> { }


#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_int_and_bool() 
    {
        use crate::*;
        use std::ops::Index;

        struct IntAndBool
        {
            integers : Vec<i32>,
            booleans : Vec<bool>,
        }
                
        type IntegerIdx = IndexTo<i32>;
        type BooleanIdx = IndexTo<bool>;
        
        impl Index<IntegerIdx> for IntAndBool
        {
            type Output=i32;
            fn index(&self, index: IntegerIdx) -> &Self::Output { &self.integers[index.index()] }
        }
        
        impl Index<BooleanIdx> for IntAndBool
        {
            type Output=bool;
            fn index(&self, index: BooleanIdx) -> &Self::Output { &self.booleans[index.index()] }
        }
        
        let mut int_and_bool = IntAndBool { integers : vec![10, 20, 30], booleans : vec![true, false] };
        
        let int_idx  = int_and_bool.integers.index_to(1); // 20
        let bool_idx = int_and_bool.booleans.index_to(0); // true
        
        // the magic in strongly typed index is here :
        assert_eq!(int_and_bool[int_idx ], 20);
        assert_eq!(int_and_bool[bool_idx], true);
        
        // compile time error :
        // let b = int_and_bool.booleans[int_idx ];
        // let i = int_and_bool.integers[bool_idx];
        
        let int_idx_2  = IntegerIdx::from_index(1); // 20
        let bool_idx_2 = BooleanIdx::from_index(0); // true
        
        assert_eq!(int_and_bool[int_idx_2 ], 20);
        assert_eq!(int_and_bool[bool_idx_2], true);

        // Also defined the `.get()` and `.get_mut()` method on index
        assert_eq!(int_idx_2.get(&int_and_bool), &20);
        assert_eq!(bool_idx_2.get(&int_and_bool), &true);
        
        // Also define the `typed_index()` method similar to `index()`
        assert_eq!(int_and_bool.integers.typed_index(IntegerIdx::from_index(1)), &20);
        assert_eq!(int_and_bool.booleans.typed_index(BooleanIdx::from_index(0)), &true);
        
        // Also define the `typed_index_mut()` method similar to `index_mut()`
        assert_eq!(int_and_bool.integers.typed_index_mut(IntegerIdx::from_index(1)), &mut 20);
        assert_eq!(int_and_bool.booleans.typed_index_mut(BooleanIdx::from_index(0)), &mut true);


        // `index.get_mut(&mut collection)` example
        impl IndexMut<IntegerIdx> for IntAndBool
        {
            fn index_mut(&mut self, index: IntegerIdx) -> &mut Self::Output { &mut self.integers[index.index()] }
        }
        
        assert_eq!(int_idx_2.get(&int_and_bool), &20);
        *int_idx_2.get_mut(&mut int_and_bool) = 50;
        assert_eq!(int_idx_2.get(&int_and_bool), &50);
    }
}