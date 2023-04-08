use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use runtime_sized_array::Array;

#[test]
fn clone() {
    let old_arr: Array<i32> = vec![5, 1, 0, 3].into();
    let new_arr : Array<i32> = old_arr.clone();

    for i in 0..4 {
        assert_eq!(old_arr[i], new_arr[i]);
    }
}


#[test]
fn deref() {
    let arr : Array<i32 >= vec![1,2,3].into();
    assert_eq!(arr.deref(), &[1,2,3]);
}


#[test]
fn deref_mut() {
    let mut arr : Array<i32 >= vec![1,2,3].into();
    assert_eq!(arr.deref_mut(), &mut [1,2,3]);
}


#[test]
fn from() {
    let source: Vec<i32> = vec![0, 1, 2, 3];
    let arr : Array<i32> = source.into();

    for i in 0..4 {
        assert_eq!(arr[i], i as i32)
    }
}


#[test]
fn index() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    assert_eq!(arr[1], 2);
}


#[test]
fn index_mut() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    arr[0] = 17;
    assert_eq!(arr[0], 17);
}


#[test]
fn into_iterator_ref() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    for item in &arr {
        println!("{item}");
    }
}


#[test]
fn into_iterator_ref_mut() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    for elem in &mut arr {
        *elem += 2;
    }
    assert_eq!(arr[0], 3);
}


#[test]
fn into_iterator() {
    let mut arr: Array<i32> = vec![0,1,2].into();
    let mut i = 0;
    for x in arr {
        assert_eq!(x, i);
        i += 1;
    }
}