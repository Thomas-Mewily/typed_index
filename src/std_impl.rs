use crate::*;


impl<T> Index<IndexTo<T>> for Vec<T>
{
    type Output=T;
    #[inline]
    fn index(&self, index: IndexTo<T>) -> &Self::Output { self.index(index.index()) }
}

impl<T> IndexMut<IndexTo<T>> for Vec<T>
{
    #[inline]
    fn index_mut(&mut self, index: IndexTo<T>) -> &mut Self::Output { self.index_mut(index.index()) }
}

impl<T> Index<IndexTo<T>> for [T]
{
    type Output=T;
    #[inline]
    fn index(&self, index: IndexTo<T>) -> &Self::Output { self.index(index.index()) }
}

impl<T> IndexMut<IndexTo<T>> for [T]
{
    #[inline]
    fn index_mut(&mut self, index: IndexTo<T>) -> &mut Self::Output { self.index_mut(index.index()) }
}

impl Index<IndexTo<u8>> for str
{
    type Output=u8;
    #[inline]
    fn index(&self, index: IndexTo<u8>) -> &Self::Output { self.as_bytes().index(index.index) }
}