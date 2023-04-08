//! Provides a variable-length array (VLA), also called variable-sized or runtime-sized.
//!
//! It is an array data structure whose length is determined at run time (instead of at compile time).
//!
//! The main purpose of VLAs is to simplify programming of numerical algorithms.
//!
//! # Note
//!
//! Unlike a dynamic array, a VLA cannot change its size, which is determined once,
//! at the time of creation.
//! But they may be more comfortable in use than static arrays, whose size must be known at compile time.
//!
//! What is more, the VLA, provided by this crate in some cases is more
//! efficient than [`std::vec::Vec`](std::vec::Vec). That's because of some optimizations and closeness to
//! c++ arrays, allocated by `malloc`. That's why some methods are unsafe.
//!
//! # Examples
//!
//! ## Creating arrays
//!
//! ```
//! use runtime_sized_array::Array;
//!
//! let arr1: Array<i32> = Array::new(10).expect("cant' create new array");
//!
//! let vec = vec![1,2,3];
//! let ptr = vec.as_ptr_mut();
//! let size = vec.len();
//! let arr2: Array<i32> = unsafe { Array::from_pointer(ptr, size) };
//!
//! let arr3: Array<i32> = vec.into();
//!
//! ```
//!
//! ## Iterating
//!
//! ```
//! use runtime_sized_array::Array;
//!
//! let mut array : Array<i32> = vec![1,2,3].into();
//!
//! // mutable iterator
//! for item in array.iter_mut() {
//!     *item += 1;
//! }
//!
//! // immutable iterator
//! for item in array.iter() {
//!     println!("{item}");
//! }
//!
//! // immutable iterator
//! for item in &array {
//!     println!("{item}");
//! }
//!
//! // again mutable iterator
//! for item in &mut array {
//!     *item *= 0;
//! }
//! ```
//!
//! ## Access to elements
//!
//! Safe access:
//!
//! ```
//! use runtime_sized_array::Array;
//!
//! let mut arr: Array<i32> = vec![1,2,4].into();
//!
//! // immutable access
//! assert_eq!(arr.try_get(1), Some(&2));
//! assert_eq!(arr.try_get(10), None);
//!
//! // mutable access
//! *arr.try_get_mut(1).unwrap() = 5;
//! assert_eq!(arr.try_get_mut(10), None);
//!
//! // alternative mutable access
//! assert_eq!(try_set(1, 5), Some(()));
//! assert_eq!(arr.try_set(10, 4), None);
//!
//! // by brackets
//! *arr[0] = 17;
//! assert_eq!(arr[0], &17)
//!
//! ```
//!
//! Unsafe access:
//!
//! ```
//! use runtime_sized_array::Array;
//!
//! let mut arr: Array<i32> = vec![1,2,4].into();
//!
//! // immutable access
//! unsafe { assert_eq!(arr.get(1), &2) }
//! // arr.get(10) - undefined behaviour
//!
//! unsafe { *arr.get_mut(1) = 2; }
//! // *arr.get_mut(10) == 4; - undefined behaviour
//!
//! // alternative mutable access
//! unsafe { arr.set(1, 5); }
//! // arr.set(10, 4); - undefined behaviour
//!
//! unsafe {
//!     *arr.get_ptr_mut(0) = 10;
//!     assert_eq!(*arr.get_ptr(0), 10)
//! }
//!
//! ```
//!
//!




#![feature(ptr_const_cast)]
#![feature(rustc_attrs)]

mod array;
mod array_iters;
mod error;

pub use array::Array;
pub use array_iters::{Iter, IterMut, IntoIter};
pub use error::ArrayError;