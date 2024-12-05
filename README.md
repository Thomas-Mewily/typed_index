Define `IndexTo<Data, Owner, Idx=usize>`, a strongly typed index for vector, slice, and str type.

Provides optional support for [Serde](https://docs.rs/serde/latest/serde/) (serialization / deserialization) when the "serde" feature is enabled.

```rust
use typed_index::*;
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
```