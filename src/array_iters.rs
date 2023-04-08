//! Provides different iterators for [`Array`](crate::Array)

pub use iter::Iter;
pub use itermut::IterMut;
pub use into_iter::IntoIter;


mod iter {

    use std::marker::PhantomData;
    use crate::array::Array;

    /// Immutable array iterator.
    ///
    /// This `struct` is created by the [`iter`] method on [`Array`](Array).
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let array : Array<i32> = vec![1,2,3].into();
    /// for item in array.iter() {
    ///     println!("{item}");
    /// }
    /// ```
    ///
    /// [`iter`]: Array::iter
    #[must_use = "iterators are lazy and do nothing unless consumed"]
    pub struct Iter<'a, T> {
        // this make borrow checker not let you to drop array before iterator
        // otherwise iterator will have access to freed memory
        marker: PhantomData<&'a T>,
        ptr: *const T,
        end: *const T,
    }


    impl<'a, T> Iter<'a, T> {

        #[inline]
        pub(crate) fn new(array: &'a Array<T>) -> Self {
            let ptr = array.pointer;
            Self {
                marker: PhantomData,
                ptr,
                end: unsafe { ptr.add(array.size()) }
            }
        }
    }


    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            if self.ptr == self.end {
                None
            } else {
                unsafe {
                    let p = self.ptr;
                    self.ptr = self.ptr.add(1);
                    Some(&*p)
                }
            }
        }
    }
}


mod itermut {

    use std::marker::PhantomData;
    use crate::array::Array;

    /// Mutable array iterator.
    ///
    /// This `struct` is created by the [`iter_mut`] method on [`Array`](Array).
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use runtime_sized_array::Array;
    ///
    /// let mut array : Array<i32> = vec![1,2,3].into();
    /// for item in array.iter_mut() {
    ///     *item += 1;
    /// }
    /// assert_eq!(array.try_get(1), Some(&3));
    /// ```
    ///
    /// [`iter_mut`]: Array::iter_mut
    #[must_use = "iterators are lazy and do nothing unless consumed"]
    pub struct IterMut<'a, T> {
        // this make borrow checker not let you to drop array before iterator
        // otherwise iterator will have access to freed memory
        marker: PhantomData<&'a T>,
        ptr: *mut T,
        end: *mut T,
    }


    impl<'a, T> IterMut<'a, T> {

        #[inline]
        pub(crate) fn new(array: &'a mut Array<T>) -> Self {
            let ptr = array.pointer;
            let size = array.size();
            Self {
                marker: PhantomData,
                ptr,
                end: unsafe { ptr.add(size) }
            }
        }
    }


    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            if self.ptr == self.end {
                None
            } else {
                unsafe {
                    let p = self.ptr;
                    self.ptr = self.ptr.add(1);
                    Some(&mut *p)
                }
            }
        }
    }
}


mod into_iter {

    use std::marker::PhantomData;
    use crate::array::Array;


    /// An iterator that moves out of an array.
    ///
    /// This `struct` is created by the `into_iter` method on [`Array`](Array)
    /// (provided by the [`IntoIterator`] trait).
    ///
    /// # Example
    ///
    /// ```
    /// use runtime_sized_array::{Array, IntoIter};
    ///
    /// let mut array : Array<i32> = vec![1,2,3].into();
    /// let iter: IntoIter<_>  = array.into_iter();
    /// ```
    pub struct IntoIter<T> {
        // do not let array be dropped before one's time
        array: Array<T>,
        ptr: *const T,
        end: *const T,
    }


    impl<T> IntoIter<T> {

        #[inline]
        pub(crate) fn new(array: Array<T>) -> Self {
            let end = unsafe { array.pointer.add(array.size()) };
            let ptr = array.pointer.as_const();
            Self { array, ptr, end }
        }
    }


    impl<T> Iterator for IntoIter<T> {
        type Item = T;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            if self.ptr == self.end {
                None
            } else {
                unsafe {
                    let p = self.ptr;
                    self.ptr = self.ptr.add(1);
                    Some(std::ptr::read(p))
                }
            }
        }
    }
}

