//! A strongly typed index for vector, slice, and str type : `IndexTo<Data, Owner, Idx=usize>`
//! 
//! Provides optional support for [Serde](https://docs.rs/serde/latest/serde/) (serialization / deserialization) when the "serde" feature is enabled.
//!
//! ```rust
//! use index_to::*;
//! use std::ops::Index;
//! 
//! struct IntAndBool
//! {
//!     integers : Vec<i32>,
//!     booleans : Vec<bool>,
//! }
//!         
//! type IntegerIdx = <Vec<i32>  as HaveTypedIndex>::IndexTo;
//! type BooleanIdx = <Vec<bool> as HaveTypedIndex>::IndexTo;
//! 
//! // can also be written as
//! // type IntegerIdx = IndexTo<i32 , Vec<i32 >>;
//! // type BooleanIdx = IndexTo<bool, Vec<bool>>;
//! 
//! impl Index<IntegerIdx> for IntAndBool
//! {
//!     type Output=i32;
//!     fn index(&self, index: IntegerIdx) -> &Self::Output { &self.integers[index] }
//! }
//! 
//! impl Index<BooleanIdx> for IntAndBool
//! {
//!     type Output=bool;
//!     fn index(&self, index: BooleanIdx) -> &Self::Output { &self.booleans[index] }
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
//! // Also define the `typed_index()` method similar to `index()`
//! assert_eq!(int_and_bool.integers.typed_index(IntegerIdx::from_index(1)), &20);
//! assert_eq!(int_and_bool.booleans.typed_index(BooleanIdx::from_index(0)), &true);
//! 
//! // Also define the `typed_index_mut()` method similar to `index_mut()`
//! assert_eq!(int_and_bool.integers.typed_index_mut(IntegerIdx::from_index(1)), &mut 20);
//! assert_eq!(int_and_bool.booleans.typed_index_mut(BooleanIdx::from_index(0)), &mut true);
//! ```

use std::fmt::{Display, Debug, Formatter, Result as DResult};
use std::marker::PhantomData;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

#[cfg(feature = "serde")]
pub(crate) mod serde_support;
#[cfg(feature = "serde")]
pub(crate) use serde_support::*;

/// A strongly typed index for vector, slice, and str type.
/// 
/// ```rust
/// use index_to::*;
/// use std::ops::Index;
/// 
/// struct IntAndBool
/// {
///     integers : Vec<i32>,
///     booleans : Vec<bool>,
/// }
///         
/// type IntegerIdx = <Vec<i32>  as HaveTypedIndex>::IndexTo;
/// type BooleanIdx = <Vec<bool> as HaveTypedIndex>::IndexTo;
/// 
/// // can also be written as
/// // type IntegerIdx = IndexTo<i32 , Vec<i32 >>;
/// // type BooleanIdx = IndexTo<bool, Vec<bool>>;
/// 
/// impl Index<IntegerIdx> for IntAndBool
/// {
///     type Output=i32;
///     fn index(&self, index: IntegerIdx) -> &Self::Output { &self.integers[index] }
/// }
/// 
/// impl Index<BooleanIdx> for IntAndBool
/// {
///     type Output=bool;
///     fn index(&self, index: BooleanIdx) -> &Self::Output { &self.booleans[index] }
/// }
/// 
/// let mut int_and_bool = IntAndBool { integers : vec![10, 20, 30], booleans : vec![true, false] };
/// 
/// let int_idx  = int_and_bool.integers.index_to(1); // 20
/// let bool_idx = int_and_bool.booleans.index_to(0); // true
/// 
/// // the magic in strongly typed index is here :
/// assert_eq!(int_and_bool[int_idx ], 20);
/// assert_eq!(int_and_bool[bool_idx], true);
/// 
/// // compile time error :
/// // let b = int_and_bool.booleans[int_idx ];
/// // let i = int_and_bool.integers[bool_idx];
/// 
/// let int_idx_2  = IntegerIdx::from_index(1); // 20
/// let bool_idx_2 = BooleanIdx::from_index(0); // true
/// 
/// assert_eq!(int_and_bool[int_idx_2 ], 20);
/// assert_eq!(int_and_bool[bool_idx_2], true);
/// 
/// // Also define the `typed_index()` method similar to `index()`
/// assert_eq!(int_and_bool.integers.typed_index(IntegerIdx::from_index(1)), &20);
/// assert_eq!(int_and_bool.booleans.typed_index(BooleanIdx::from_index(0)), &true);
/// 
/// // Also define the `typed_index_mut()` method similar to `index_mut()`
/// assert_eq!(int_and_bool.integers.typed_index_mut(IntegerIdx::from_index(1)), &mut 20);
/// assert_eq!(int_and_bool.booleans.typed_index_mut(BooleanIdx::from_index(0)), &mut true);
/// ```
pub struct IndexTo<Data, In, Idx=usize> 
    where
    Data : ?Sized, 
    In : ?Sized
{
    index : Idx,
    index_data  : PhantomData<Data>,
    index_owner : PhantomData<In>,
}

impl<Data, Inside, Idx> IndexTo<Data, Inside, Idx>
    where
    Data : ?Sized, 
    Inside : ?Sized
{
    #[inline]
    pub const fn from_index(index : Idx) -> Self { Self { index, index_data: PhantomData, index_owner : PhantomData }}
    #[inline]
    pub const fn index(self) -> Idx where Idx : Copy { self.index }
    #[inline]
    pub fn set_index(&mut self, index : Idx) -> &mut Self { self.index = index; self }
    #[inline]
    pub fn with_index(mut self, index : Idx) -> Self { self.set_index(index); self }
}

impl<Data : ?Sized, In : ?Sized, Idx> Hash       for IndexTo<Data, In, Idx> where Idx : Hash       { #[inline] fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.index.hash(state); } }
impl<Data : ?Sized, In : ?Sized, Idx> Clone      for IndexTo<Data, In, Idx> where Idx : Clone      { #[inline] fn clone(&self) -> Self { Self::from_index(self.index.clone()) } }
impl<Data : ?Sized, In : ?Sized, Idx> Copy       for IndexTo<Data, In, Idx> where Idx : Copy       {}
impl<Data : ?Sized, In : ?Sized, Idx> Debug      for IndexTo<Data, In, Idx> where Idx : Debug      { fn fmt(&self, f: &mut Formatter<'_>) -> DResult { write!(f, "IndexTo<{}>({:?})", std::any::type_name::<In>(), self.index) } }
impl<Data : ?Sized, In : ?Sized, Idx> Display    for IndexTo<Data, In, Idx> where Idx : Display    { fn fmt(&self, f: &mut Formatter<'_>) -> DResult { write!(f, "IndexTo<{}>({})", std::any::type_name::<In>(), self.index) } }
impl<Data : ?Sized, In : ?Sized, Idx> Eq         for IndexTo<Data, In, Idx> where Idx : Eq         {}
impl<Data : ?Sized, In : ?Sized, Idx> PartialEq  for IndexTo<Data, In, Idx> where Idx : PartialEq  { #[inline] fn eq(&self, other: &Self) -> bool { self.index == other.index } }
impl<Data : ?Sized, In : ?Sized, Idx> Ord        for IndexTo<Data, In, Idx> where Idx : Ord        { #[inline] fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.index.cmp(&other.index) } }
impl<Data : ?Sized, In : ?Sized, Idx> PartialOrd for IndexTo<Data, In, Idx> where Idx : PartialOrd { #[inline] fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { self.index.partial_cmp(&other.index) } }

impl<D> Index<IndexTo<D, Vec<D>>> for Vec<D>
{
    type Output=D;
    #[inline]
    fn index(&self, index: IndexTo<D, Vec<D>>) -> &Self::Output { self.index(index.index) }
}

impl<T> IndexMut<IndexTo<T, Vec<T>>> for Vec<T>
{
    #[inline]
    fn index_mut(&mut self, index: IndexTo<T, Vec<T>>) -> &mut Self::Output { self.index_mut(index.index) }
}

impl<T> Index<IndexTo<T, [T]>> for [T]
{
    type Output=T;
    #[inline]
    fn index(&self, index: IndexTo<T, [T]>) -> &Self::Output { self.get(index.index).unwrap() }
}

impl<T> IndexMut<IndexTo<T, [T]>> for [T]
{
    #[inline]
    fn index_mut(&mut self, index: IndexTo<T, [T]>) -> &mut Self::Output { self.get_mut(index.index).unwrap() }
}

impl Index<IndexTo<u8, str>> for str
{
    type Output=u8;
    #[inline]
    fn index(&self, index: IndexTo<u8, str>) -> &Self::Output { self.as_bytes().index(index.index) }
}

pub trait HaveTypedIndex<Idx=usize> where Idx : ?Sized
{
    type IndexTo;
    type Output : ?Sized;
    /// return a strongly typed index for vector, slice, and str type.
    fn index_to(&self, index : Idx) -> Self::IndexTo;
}
impl<Idx, T> HaveTypedIndex<Idx> for T where T : Index<Idx> + ?Sized
{
    type IndexTo = IndexTo<Self::Output, Self, Idx>;
    /// The returned type after indexing.
    type Output = T::Output;

    fn index_to(&self, index : Idx) -> Self::IndexTo {
        IndexTo::from_index(index)
    }
}

pub trait TypedIndex<Idx:?Sized> : HaveTypedIndex<Idx> + Index<Self::IndexTo>
{
    // The documentation comment is copied from the standard library

    /// Performs the indexing (`container[index]`) operation.
    ///
    /// # Panics
    ///
    /// May panic if the index is out of bounds.
    fn typed_index(&self, index : Self::IndexTo) -> &<Self as Index<Self::IndexTo>>::Output { self.index(index) }
}
impl<T, Idx:?Sized> TypedIndex<Idx> for T where T : HaveTypedIndex<Idx> + Index<T::IndexTo> { }

pub trait TypedIndexMut<Idx:?Sized> : HaveTypedIndex<Idx> + IndexMut<Self::IndexTo>
{
    // The documentation comment is copied from the standard library

    /// Performs the mutable indexing (`container[index]`) operation.
    ///
    /// # Panics
    ///
    /// May panic if the index is out of bounds.
    fn typed_index_mut(&mut self, index : Self::IndexTo) -> &mut <Self as Index<Self::IndexTo>>::Output { self.index_mut(index) }
}
impl<T, Idx:?Sized> TypedIndexMut<Idx> for T where T : HaveTypedIndex<Idx> + IndexMut<T::IndexTo> { }


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
                
        type IntegerIdx = <Vec<i32>  as HaveTypedIndex>::IndexTo;
        type BooleanIdx = <Vec<bool> as HaveTypedIndex>::IndexTo;

        // can also be written as
        // type IntegerIdx = IndexTo<i32 , Vec<i32 >>;
        // type BooleanIdx = IndexTo<bool, Vec<bool>>;
        
        impl Index<IntegerIdx> for IntAndBool
        {
            type Output=i32;
            fn index(&self, index: IntegerIdx) -> &Self::Output { &self.integers[index] }
        }
        
        impl Index<BooleanIdx> for IntAndBool
        {
            type Output=bool;
            fn index(&self, index: BooleanIdx) -> &Self::Output { &self.booleans[index] }
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
        
        // Also define the `typed_index()` method similar to `index()`
        assert_eq!(int_and_bool.integers.typed_index(IntegerIdx::from_index(1)), &20);
        assert_eq!(int_and_bool.booleans.typed_index(BooleanIdx::from_index(0)), &true);
        
        // Also define the `typed_index_mut()` method similar to `index_mut()`
        assert_eq!(int_and_bool.integers.typed_index_mut(IntegerIdx::from_index(1)), &mut 20);
        assert_eq!(int_and_bool.booleans.typed_index_mut(BooleanIdx::from_index(0)), &mut true);
    }
}