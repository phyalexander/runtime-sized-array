use runtime_sized_array::{Array, ArrayError};

#[deprecated]
#[inline]
fn testable() -> Array<i32> {
    let mut vec = vec![1,2,3];
    vec.into()
}

#[test]
fn as_mut_ptr() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    let ptr = arr.as_mut_ptr();
    unsafe {
        for i in 0..arr.size() {
            *ptr.add(i) += 2;
        }
    }
}


#[test]
fn as_ptr() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    let ptr = arr.as_ptr();
    unsafe {
        for i in 0..arr.size() {
            assert_eq!(arr[i], *ptr.add(i));
        }
    }
}


#[test]
fn from_pointer() {
    let mut vec = vec![1,2,3];
    let ptr = vec.as_mut_ptr();
    let size = vec.len();
    unsafe {
        let arr: Array<i32> = Array::from_pointer(ptr, size);
    }
}


#[test]
fn get() {
    let arr: Array<i32> = vec![1, 2, 4].into();
    unsafe {
        assert_eq!(arr.get(1), &2)
    }
}


#[test]
fn get_mut() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    unsafe {
        *arr.get_mut(1) = 2;
    }
}


#[test]
fn get_mut_ptr() {
    let arr: Array<i32> = vec![1,2,4].into();
    unsafe {
        assert_eq!(*arr.get_ptr(1), 2)
    }
}


#[test]
fn get_ptr() {
    let arr: Array<i32> = vec![1,2,4].into();
    unsafe {
        assert_eq!(*arr.get_ptr(1), 2)
    }

}


#[test]
fn into_vec() {
    let vec = vec![1,2,4];
    let arr: Array<i32> = vec.clone().into();
    assert_eq!(arr.into_vec(), vec);
}


#[test]
fn iter() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    let mut iterator = arr.iter();

    assert_eq!(iterator.next(), Some(&1));
    assert_eq!(iterator.next(), Some(&2));
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), None);
}


#[test]
fn iter_mut() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    for elem in arr.iter_mut() {
        *elem += 2;
    }
    assert_eq!(arr[0], 3);
}


#[test]
fn new() {
    let arr: Result<Array<i32>, ArrayError> = Array::new(5);
    assert!(arr.is_ok());
    let arr = arr.unwrap();
    assert_eq!(arr.size(), 5);
}


#[test]
fn set() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    unsafe {
        arr.set(1, 5);
    }
}


#[test]
fn size() {
    let arr: Array<i32> = vec![1,2,4].into();
    unsafe {
        assert_eq!(*arr.get_ptr(1), 2)
    }
}


#[test]
fn take_from_iter() {
    let mut iter = vec![0,1,2,3,4,5].into_iter();
    let arr : Array<i32> = Array::take_from_iter(&mut iter, 3);

    for i in 0..3 {
        assert_eq!(arr[i], i as i32)
    }
}


#[test]
fn try_get() {
    let arr: Array<i32> = vec![1,2,4].into();
    assert_eq!(arr.try_get(1), Some(&2));
    assert_eq!(arr.try_get(10), None);
}


#[test]
fn try_get_mut() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    *arr.try_get_mut(1).unwrap() = 5;
    assert_eq!(arr.try_get_mut(10), None);
}


#[test]
fn try_set() {
    let mut arr: Array<i32> = vec![1,2,4].into();
    assert_eq!(arr.try_set(1, 5), Some(()));
    assert_eq!(arr.try_set(10, 4), None);
}




