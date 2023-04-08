use std::alloc::{Layout, LayoutError};

use super::ArrayError;
use super::{Iter, IterMut, IntoIter};


/// Base `struct` of the crate.
///
/// A variable-length array - data structure whose length is determined at run time
/// (instead of at compile time).
///
/// # Example
///
/// Basic usage:
///
/// ```
/// use runtime_sized_array::Array;
/// let arr: Array<i32> = Array::new(10).unwrap();
/// *arr[2] == 3;
/// ```
///
pub struct Array<T> {
    pub(in super) pointer : *mut T,
    size : usize
}

impl<T> Array<T> {

    /// Creates an `Array` with the given size or returns `ArrayError`
    /// if any of the following cases happened:
    /// * failed creating a [`layout`] with the following size,
    /// * failed [allocating] memory for the array.
    ///
    /// [allocating]: std::alloc
    /// [`layout`]: std::alloc::Layout
    #[inline]
    pub fn new(size: usize) -> Result<Array<T>, ArrayError> {
        unsafe {
            let layout = std::alloc::Layout::array::<T>(size)?;
            let ptr = std::alloc::alloc(layout) as *mut T;
            if ptr.is_null() {
                Err(ArrayError("allocation returned null pointer".to_string()))
            } else {
                Ok(Self { pointer: ptr, size })
            }
        }
    }


    /// Creates an `Array` from the given raw pointer with the given size
    ///
    /// # Safety
    ///
    /// The caller must ensure that the memory the `ptr` refers can be deallocated
    /// by another structure. Also dropping the array, returned by this function
    /// will immediately cause deallocating of the memory. All this may cause undefined
    /// behaviour.
    ///
    /// What's more, the function does not check is the pointer is null.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::Array;
    /// let vec = vec![1,2,3];
    /// let ptr = vec.as_ptr_mut();
    /// let size = vec.len();
    /// unsafe {
    ///     let arr: Array<i32> = Array::from_pointer(ptr, size);
    /// }
    /// ```
    #[inline]
    pub unsafe fn from_pointer(ptr: *mut T, size: usize) -> Self {
        Self { pointer : ptr, size }
    }



    /// size of the array
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }


    /// Returns an immutable raw pointer at an element by the given index
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let arr: Array<i32> = vec![1,2,4].into();
    /// unsafe {
    ///     assert_eq!(*arr.get_ptr(1), 2)
    /// }
    ///
    /// // arr.get_ptr(10) - undefined behaviour
    /// ```
    ///
    /// # Safety
    ///
    /// This method does not check the index bounds, so it's more efficient,
    /// but can produce undefined behaviour
    ///
    #[inline]
    pub unsafe fn get_ptr(&self, index: usize) -> *const T {
        self.pointer.add(index)
    }

    /// Returns a mutable raw pointer at an element by the given index
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let arr: Array<i32> = vec![1,2,4].into();
    /// unsafe {
    ///     assert_eq!(*arr.get_ptr(1), 2)
    /// }
    ///
    /// // arr.get_ptr(10) - undefined behaviour
    /// ```
    ///
    /// # Safety
    ///
    /// This method does not check the index bounds, so it's more efficient,
    /// but can produce undefined behaviour
    ///
    #[inline]
    pub unsafe fn get_mut_ptr(&self, index: usize) -> *mut T {
        self.pointer.add(index)
    }

    /// Returns immutable reference at an element
    /// or None if the given index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let arr: Array<i32> = vec![1,2,4].into();
    /// assert_eq!(arr.try_get(1), Some(&2));
    /// assert_eq!(arr.try_get(10), None);
    /// ```
    #[inline]
    pub fn try_get(&self, index: usize) -> Option<&T> {
        if self.size <= index {
            None
        } else {
            unsafe { Some(self.get(index)) }
        }
    }

    /// Returns immutable reference at an element by the given index
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let arr: Array<i32> = vec![1,2,4].into();
    /// unsafe {
    ///     assert_eq!(arr.get(1), &2)
    /// }
    ///
    /// // arr.get(10) - undefined behaviour
    /// ```
    ///
    /// # Safety
    ///
    /// This method does not check the index bounds, so it's more efficient,
    /// but can produce undefined behaviour
    ///
    /// If you want safe immutable access, use [`try_get`](Array::try_get).
    #[inline]
    pub unsafe fn get(&self, index: usize) -> &T {
        &(*(self.pointer.add(index)))
    }

    /// Returns mutable reference at an element
    /// or None if the given index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let mut arr: Array<i32> = vec![1,2,4].into();
    /// *arr.try_get_mut(1).unwrap() = 5;
    /// assert_eq!(arr.try_get_mut(10), None);
    /// ```
    #[inline]
    pub fn try_get_mut(&mut self, index: usize) -> Option<&mut T> {
        if self.size <= index {
            None
        } else {
            unsafe { Some(self.get_mut(index)) }
        }
    }

    /// Returns mutable reference at an element by the given index
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let mut arr: Array<i32> = vec![1,2,4].into();
    /// unsafe {
    ///     *arr.get_mut(1) = 2;
    /// }
    ///
    /// // *arr.get_mut(10) == 4; - undefined behaviour
    /// ```
    ///
    /// # Safety
    ///
    /// This method does not check the index bounds, so it's more efficient,
    /// but can produce undefined behaviour
    ///
    /// If you want safe mutable access, use [`try_get_mut`](Array::try_get_mut).
    #[inline]
    pub unsafe fn get_mut(&mut self, index: usize) -> &mut T {
        &mut (*(self.pointer.add(index)))
    }

    /// Alternative way to safely change elements of the array.
    ///
    /// Returns () or None if the given index is out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let mut arr: Array<i32> = vec![1,2,4].into();
    /// assert_eq!(try_set(1, 5), Some(()));
    /// assert_eq!(arr.try_set(10, 4), None);
    /// ```
    #[inline]
    pub fn try_set(&mut self, index: usize, value: T) -> Option<()> {
        if self.size <= index {
            None
        } else {
            unsafe { Some(self.set(index, value)) }
        }
    }

    /// Alternative way to unsafely change elements of the array.
    ///
    /// Returns () or None if the given index is out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let mut arr: Array<i32> = vec![1,2,4].into();
    /// unsafe {
    ///     arr.set(1, 5);
    /// }
    ///
    /// // arr.set(10, 4); - undefined behaviour
    /// ```
    ///
    /// # Safety
    ///
    /// This method does not check the index bounds, so it's more efficient,
    /// but can produce undefined behaviour
    ///
    /// If you look for safe version, use [`try_set`](Array::try_set).
    #[inline]
    pub unsafe fn set(&mut self, index: usize, value: T) {
        *(self.pointer.add(index)) = value
    }

    /// Returns an iterator over the array.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let mut arr: Array<i32> = vec![1,2,4].into();
    /// let mut iterator = arr.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&2));
    /// assert_eq!(iterator.next(), Some(&4));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    /// Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let mut arr: Array<i32> = vec![1,2,4].into();
    /// for elem in arr.iter_mut() {
    ///     *elem += 2;
    /// }
    /// assert_eq!(arr[0], Some(&3));
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }

    /// Converts the array into a [`Vec`](std::vec::Vec)
    ///
    /// The array cannot be used after calling this.
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        unsafe{
            Vec::from_raw_parts(self.pointer, self.size, self.size)
        }
    }

    /// Returns immutable raw pointer to the memory, allocated by the array.
    ///
    /// The caller must ensure that the array outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    ///
    /// The caller must also ensure that the memory the pointer (non-transitively) points to
    /// is never written to (except inside an `UnsafeCell`) using this pointer or any pointer
    /// derived from it. If you need to mutate the contents of the array, use [`as_mut_ptr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let mut arr: Array<i32> = vec![1,2,4].into();
    /// let ptr = arr.as_ptr();
    /// unsafe {
    ///     for i in 0..arr.size() {
    ///         assert_eq!(arr[i], &*ptr.add(i));
    ///     }
    /// }
    /// ```
    ///
    /// [`as_mut_ptr`]: Array::as_mut_ptr
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.pointer.as_const()
    }

    /// Returns mutable raw pointer to the memory, allocated by the array.
    ///
    /// The caller must ensure that the array outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    ///
    /// # Examples
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let mut arr: Array<i32> = vec![1,2,4].into();
    /// let ptr = arr.as_mut_ptr();
    /// unsafe {
    ///     for i in 0..arr.size() {
    ///         *ptr.add(i) += 2;
    ///     }
    /// }
    /// ```
    ///
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.pointer
    }

}


impl<'a, T> IntoIterator for &'a Array<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    /// Returns an iterator over the array.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let mut arr: Array<i32> = vec![1,2,4].into();
    /// for item in &arr {
    ///     println!("{item}");
    /// }
    /// ```
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Array<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    /// Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let mut arr: Array<i32> = vec![1,2,4].into();
    /// for elem in &mut arr {
    ///     *elem += 2;
    /// }
    /// assert_eq!(arr[0], Some(&3));
    /// ```
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


impl<T> IntoIterator for Array<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the array (from start to end).
    ///
    /// The array cannot be used after calling this.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}


impl<T> std::ops::Index<usize> for Array<T> {
    type Output = T;

    #[inline]
    #[rustc_on_unimplemented(
        message = "array indices are of type `usize` or `Index`",
        label = "array indices are of type `usize` or `Index`"
    )]
    fn index(&self, index: usize) -> &Self::Output {
        self.try_get(index).expect("index out of bounds")
    }
}


impl<T> std::ops::IndexMut<usize> for Array<T> {

    #[inline]
    #[rustc_on_unimplemented(
        message = "array indices are of type `usize` or `Index`",
        label = "array indices are of type `usize` or `Index`"
    )]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.try_get_mut(index).expect("index out of bounds")
    }
}


impl<T> Drop for Array<T> {

    fn drop(&mut self) {
        println!("array dropped");
        unsafe { self.pointer.drop_in_place() };
    }
}


impl<T> From<Vec<T>> for Array<T> {

    /// Converts a `Vec<T>` to `Array<T>`.
    ///
    /// # Panics
    ///
    /// if any of the following cases happened:
    /// * failed creating a [`layout`] with the following size,
    /// * failed allocating memory for the array.
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::Array;
    /// let source: Vec<i32> = vec![0, 1, 2, 3];
    /// let arr : Array<i32> = source.into();
    ///
    /// for i in 0..4 {
    ///     assert_eq!(arr[i], i as i32)
    /// }
    /// ```
    ///
    /// [allocating]: std::alloc
    /// [`layout`]: std::alloc::Layout
    fn from(vec: Vec<T>) -> Self {
        let size = vec.len();
        let mut array = Array::new(size)
            .expect("failed to create new Array");
        let mut i = 0_usize;
        unsafe {
            for item in vec {
                *array.get_mut(i) = item;
                i += 1
            }
        }
        array
    }
}


impl<T: Clone> Clone for Array<T> {

    /// Copies all elements of one array to another.
    ///
    /// # Note
    ///
    /// The elements are copied by value, not by reference.
    /// So changing elements of the new array will not cause
    /// changing elements of the old one.
    ///
    /// # Advanced
    ///
    /// That's because multi-handling one pointer will lead to
    /// undefined behaviour, when one of them is dropped and deallocates
    /// the memory of it's pointer
    /// and another one is still trying to use the same pointer.
    ///
    /// # Panics
    ///
    /// if any of the following cases happened:
    /// * failed creating a [`layout`] with the following size,
    /// * failed allocating memory for the array.
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::Array;
    /// let old_arr: Array<i32> = vec![5, 1, 0, 3].into();
    /// let new_arr : Array<i32> = old_arr.clone();
    ///
    /// for i in 0..4 {
    ///     assert_eq!(old_arr[i], new_arr[i]);
    /// }
    /// ```
    ///
    /// [allocating]: std::alloc
    /// [`layout`]: std::alloc::Layout
    fn clone(&self) -> Self {
        let arr = Array::new(self.size)
            .expect("failed to crate new Array");
        unsafe {
            for i in 0..self.size {
                *arr.get_mut_ptr(i) = std::ptr::read(self.get_ptr(i));
            }
        }
        arr
    }
}


impl<T> std::ops::Deref for Array<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.pointer, self.size) }
    }
}


impl<T> std::ops::DerefMut for Array<T> {

    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.pointer, self.size) }
    }
}


// additional functionality
impl<T> Array<T> {

    /// Tries to take `n` items from the given `iterator` and
    /// to put them into array of size `n`
    ///
    /// If the `iterator` ends, before the array is filled, then
    /// some elements will be uninitialized
    ///
    /// # Panics
    ///
    /// if any of the following cases happened:
    /// * failed creating a [`layout`] with the following size,
    /// * failed allocating memory for the array.
    ///
    /// [allocating]: std::alloc
    /// [`layout`]: std::alloc::Layout
    pub fn take_from_iter<I: Iterator>(iterator: &mut I, n: usize) -> Self
        where
            I : Iterator,
            T : From<I::Item>
    {
        let mut arr = Array::new(n)
            .expect("failed to create new Array");
        unsafe {
            for i in 0..n {
                match iterator.next() {
                    None => break,
                    Some(val) => *arr.get_mut_ptr(i) = val.into()
                }
            }
        }
        arr
    }
}