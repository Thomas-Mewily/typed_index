use crate::*;

/// Trait for marking index.
/// 
/// Allow to do `index.get(&collection)` or `index.get_mut(&mut collection)`.
pub trait IndexLike : Copy
{
    fn get<T>(self, inside : &T) -> &T::Output where T : Index<Self> { inside.index(self) }
    fn get_mut<T>(self, inside : &mut T) -> &mut T::Output where T : IndexMut<Self> { inside.index_mut(self) }
}

impl IndexLike for usize where {}
impl<Data, Idx>  IndexLike for IndexTo<Data, Idx> where Self : Copy, Data : ?Sized {}