#[feature(globs)];
#[cfg(test)];

use minmaxheap::MinMaxHeap;

mod minmaxheap;

fn chk_order<T: Ord+Eq+Clone>(desc: &mut MinMaxHeap<T>) {
    let len = desc.len();
    let mut asc = MinMaxHeap::new(len);

    let mut counter = 1;
    let mut last = desc.pop_max().unwrap();
    while !desc.is_empty() {
        let next = desc.pop_max().unwrap();
        assert!(next <= last);
        asc.push(last);
        last = next;
        counter += 1;
    }
    asc.push(last);
    assert_eq!(counter, len);

    counter = 1;
    last = asc.pop_min().unwrap();
    while !asc.is_empty() {
        let next = asc.pop_min().unwrap();
        assert!(next >= last);
        last = next;
        counter += 1;
    }
    assert_eq!(counter, len);
}

#[test]
fn test_small() {
    let mut heap = MinMaxHeap::new(2);
    heap.push_all([3,5,9]);
    assert_eq!(heap.len(), 2);
    assert_eq!(*heap.peek_max().unwrap(), 9);
    assert_eq!(*heap.peek_min().unwrap(), 5);
    chk_order(&mut heap);
}

#[test]
fn test_med() {
    let mut heap = MinMaxHeap::new(7);
    heap.push_all([3,5,9,7,6,1,0,8,4,2]);
    assert_eq!(heap.len(), 7);
    assert_eq!(*heap.peek_max().unwrap(), 9);
    assert_eq!(*heap.peek_min().unwrap(), 3);
    chk_order(&mut heap);
}

#[test]
fn test_large() {
    let mut heap = MinMaxHeap::new(24);
    heap.push_all([3,5,9,7,6,1,0,8,4,2]);
    assert_eq!(heap.len(), 10);
    assert_eq!(*heap.peek_max().unwrap(), 9);
    assert_eq!(*heap.peek_min().unwrap(), 0);
    chk_order(&mut heap);
}

#[test]
fn test_dupes() {
    let mut heap = MinMaxHeap::new(16);
    heap.push_all([3,5,9,7,6,1,0,8,4,2,2,4,8,0,1,6,7,9,5,3]);
    assert_eq!(heap.len(), 16);
    assert_eq!(*heap.peek_max().unwrap(), 9);
    assert_eq!(*heap.peek_min().unwrap(), 2);
    chk_order(&mut heap);
}

#[test]
fn test_push_max() {
    let mut heap = MinMaxHeap::new(16);
    heap.push_all_max([3,5,9,7,6,1,0,8,4,2,2,4,8,0,1,6,7,9,5,3]);
    assert_eq!(heap.len(), 16);
    assert_eq!(*heap.peek_max().unwrap(), 7);
    assert_eq!(*heap.peek_min().unwrap(), 0);
    chk_order(&mut heap);
}

#[test]
fn test_from_vec_growable() {
    let vec = ~[3,5,9,7,6,1,0,8,4,2,2,4,8,0,1,6,7,9,5,3];
    let len = vec.len();
    let mut heap = MinMaxHeap::from_vec_growable(vec);
    assert_eq!(heap.len(), len);
    assert_eq!(*heap.peek_max().unwrap(), 9);
    assert_eq!(*heap.peek_min().unwrap(), 0);

    heap.push(10);

    assert_eq!(heap.len(), len+1);
    assert_eq!(*heap.peek_max().unwrap(), 10);
    assert_eq!(*heap.peek_min().unwrap(), 0);
    chk_order(&mut heap);
}
