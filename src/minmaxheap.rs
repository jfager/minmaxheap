extern crate std;

use std::mem;

#[deriving(Clone)]
pub struct MinMaxHeap<T> {
    dat: Vec<T>,
    cap: uint
}

impl<T:Ord+Clone> MinMaxHeap<T> {

    pub fn new(cap: uint) -> MinMaxHeap<T> {
        MinMaxHeap { dat: vec![], cap: cap }
    }

    pub fn with_capacity(cap: uint) -> MinMaxHeap<T> {
        MinMaxHeap { dat: Vec::with_capacity(cap), cap: cap }
    }

    pub fn from_vec(v: Vec<T>) -> MinMaxHeap<T> {
        let len = v.len();
        let mut out = MinMaxHeap { dat: v, cap: len };
        for i in range(0, out.len()).rev() {
            out.trickle_down(i);
        }
        out
    }

    pub fn from_vec_growable(v: Vec<T>) -> MinMaxHeap<T> {
        let mut out = MinMaxHeap { dat: v, cap: 0 };
        for i in range(0, out.len()).rev() {
            out.trickle_down(i);
        }
        out
    }

    pub fn len(&self) -> uint {
        self.dat.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_capped(&self) -> bool {
        self.cap != 0
    }

    pub fn peek_min<'a>(&'a self) -> Option<&'a T> {
        if self.is_empty() { None } else { Some(self.dat.get(0)) }
    }

    pub fn peek_max<'a>(&'a self) -> Option<&'a T> {
        match self.max_idx() {
            None => None,
            Some(i) => Some(self.dat.get(i))
        }
    }

    fn max_idx(&self) -> Option<uint> {
        match self.len() {
            0 => None,
            1 => Some(0),
            2 => Some(1),
            _ => Some(if self.dat.get(1) >= self.dat.get(2) { 1 } else { 2 })
        }
    }

    pub fn pop_min(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            1 => self.dat.pop(),
            _ => {
                let out = self.dat.swap_remove(0);
                self.trickle_down(0);
                out
            }
        }
    }

    pub fn pop_max(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            1|2 => self.dat.pop(),
            3 => {
                if self.dat.get(1) >= self.dat.get(2) {
                    self.dat.swap_remove(1)
                } else {
                    self.dat.pop()
                }
            },
            _ => {
                let idx = if self.dat.get(1) >= self.dat.get(2) { 1 } else { 2 };
                let out = self.dat.swap_remove(idx);
                self.trickle_down(idx);
                out
            }
        }
    }

    pub fn push(&mut self, item: T) -> Option<T> {
        self.push_min(item)
    }

    pub fn push_all(&mut self, items: &[T]) {
        self.push_all_min(items)
    }

    pub fn push_min(&mut self, item: T) -> Option<T> {
        if self.cap == 0 || self.len() < self.cap {
            self.push_grow(item);
            None
        } else if *self.peek_min().unwrap() < item {
            let out = mem::replace(self.dat.get_mut(0), item);
            self.trickle_down(0);
            Some(out)
        } else {
            Some(item)
        }
    }

    pub fn push_all_min(&mut self, items: &[T]) {
        for i in items.iter() {
            self.push_min((*i).clone());
        }
    }

    pub fn push_max(&mut self, item: T) -> Option<T> {
        if self.cap == 0 || self.len() < self.cap {
            self.push_grow(item);
            None
        } else if *self.peek_max().unwrap() > item {
            let idx = self.max_idx().unwrap();
            let out = mem::replace(self.dat.get_mut(idx), item);
            self.trickle_down(idx);
            Some(out)
        } else {
            Some(item)
        }
    }

    pub fn push_all_max(&mut self, items: &[T]) {
        for i in items.iter() {
            self.push_max((*i).clone());
        }
    }

    fn push_grow(&mut self, item: T) {
        self.dat.push(item);
        let last = self.len() - 1;
        self.bubble_up(last);
    }

    fn bubble_up(&mut self, i: uint) {
        if i == 0 {
            return;
        }
        match level_type(i) {
            Min => {
                if self.dat.get(i) > self.dat.get(parent(i)) {
                    self.dat.as_mut_slice().swap(i, parent(i));
                    self.bubble_up_max(parent(i));
                } else {
                    self.bubble_up_min(i);
                }
            },
            Max => {
                if self.dat.get(i) < self.dat.get(parent(i)) {
                    self.dat.as_mut_slice().swap(i, parent(i));
                    self.bubble_up_min(parent(i));
                } else {
                    self.bubble_up_max(i);
                }
            }
        }
    }

    fn bubble_up_min(&mut self, i: uint) {
        if i < 3 { // no grandparent
            return;
        }
        if self.dat.get(i) < self.dat.get(grandparent(i)) {
            self.dat.as_mut_slice().swap(i, grandparent(i));
            self.bubble_up_min(grandparent(i));
        }
    }

    fn bubble_up_max(&mut self, i: uint) {
        if i < 3 { // no grandparent
            return;
        }
        if self.dat.get(i) > self.dat.get(grandparent(i)) {
            self.dat.as_mut_slice().swap(i, grandparent(i));
            self.bubble_up_max(grandparent(i));
        }
    }

    fn trickle_down(&mut self, i: uint) {
        match level_type(i) {
            Min => self.trickle_down_min(i),
            Max => self.trickle_down_max(i)
        }
    }

    fn trickle_down_min(&mut self, i: uint) {
        let m = self.smallest_child_or_grandchild(i);
        if m == 0 {
            return;
        }
        if self.dat.get(m) < self.dat.get(i) {
            self.dat.as_mut_slice().swap(m, i);
        }
        if !(m == left(i) || m == right(i)) { // m is a grandchild
            if self.dat.get(m) > self.dat.get(parent(m)) {
                self.dat.as_mut_slice().swap(m, parent(m));
            }
            self.trickle_down_min(m);
        }
    }

    fn trickle_down_max(&mut self, i: uint) {
        let m = self.largest_child_or_grandchild(i);
        if self.dat.get(m) > self.dat.get(i) {
            self.dat.as_mut_slice().swap(m, i);
        }
        if m == 0 {
            return;
        }
        if !(m == left(i) || m == right(i)) { // m is a grandchild
            if self.dat.get(m) < self.dat.get(parent(m)) {
                self.dat.as_mut_slice().swap(m, parent(m));
            }
            self.trickle_down_max(m);
        }
    }

    fn smallest_child_or_grandchild(&self, i: uint) -> uint {
        let l = left(i);
        if l < self.dat.len() {
            let mut min_idx = l;
            let r = right(i);
            let idxs = [r, left(l), right(l), left(r), right(r)];
            for idx in idxs.iter() {
                if *idx >= self.dat.len() {
                    break;
                }
                if self.dat.get(*idx) < self.dat.get(min_idx) {
                    min_idx = *idx;
                }
            }
            min_idx
        } else {
            0
        }
    }

    fn largest_child_or_grandchild(&self, i: uint) -> uint {
        let l = left(i);
        if l < self.dat.len() {
            let mut max_idx = l;
            let r = right(i);
            let idxs = [r, left(l), right(l), left(r), right(r)];
            for idx in idxs.iter() {
                if *idx >= self.dat.len() {
                    break;
                }
                if self.dat.get(*idx) > self.dat.get(max_idx) {
                    max_idx = *idx;
                }
            }
            max_idx
        } else {
            0
        }
    }
}

fn left(i: uint) -> uint {
    (i * 2) + 1
}

fn right(i: uint) -> uint {
    (i * 2) + 2
}

fn parent(i: uint) -> uint {
    if i == 0 { 0 } else { (i - 1) / 2 }
}

fn grandparent(i: uint) -> uint {
    parent(parent(i))
}

fn level(i: uint) -> uint {
    let mut c = i;
    let mut out = 0;
    while c != 0 {
        c = parent(c);
        out += 1;
    }
    out
}

enum LevelType {
    Min,
    Max
}

fn level_type(i: uint) -> LevelType {
    if level(i) % 2 == 0 { Min } else { Max }
}



#[cfg(test)]
fn chk_order<T: Ord+Eq+Clone>(heap: &MinMaxHeap<T>) {
    let mut desc = heap.clone();
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
    heap.push_all([3i,5,9]);
    assert_eq!(heap.len(), 2);
    assert_eq!(*heap.peek_max().unwrap(), 9);
    assert_eq!(*heap.peek_min().unwrap(), 5);
    chk_order(&heap);
}

#[test]
fn test_med() {
    let mut heap = MinMaxHeap::new(7);
    heap.push_all([3i,5,9,7,6,1,0,8,4,2]);
    assert_eq!(heap.len(), 7);
    assert_eq!(*heap.peek_max().unwrap(), 9);
    assert_eq!(*heap.peek_min().unwrap(), 3);
    chk_order(&heap);
}

#[test]
fn test_large() {
    let mut heap = MinMaxHeap::new(24);
    heap.push_all([3i,5,9,7,6,1,0,8,4,2]);
    assert_eq!(heap.len(), 10);
    assert_eq!(*heap.peek_max().unwrap(), 9);
    assert_eq!(*heap.peek_min().unwrap(), 0);
    chk_order(&heap);
}

#[test]
fn test_dupes() {
    let mut heap = MinMaxHeap::new(16);
    heap.push_all([3i,5,9,7,6,1,0,8,4,2,2,4,8,0,1,6,7,9,5,3]);
    assert_eq!(heap.len(), 16);
    assert_eq!(*heap.peek_max().unwrap(), 9);
    assert_eq!(*heap.peek_min().unwrap(), 2);
    chk_order(&heap);
}

#[test]
fn test_push_max() {
    let mut heap = MinMaxHeap::new(16);
    heap.push_all_max([3i,5,9,7,6,1,0,8,4,2,2,4,8,0,1,6,7,9,5,3]);
    assert_eq!(heap.len(), 16);
    assert_eq!(*heap.peek_max().unwrap(), 7);
    assert_eq!(*heap.peek_min().unwrap(), 0);
    chk_order(&heap);
}

#[test]
fn test_from_vec() {
    let vec = vec![3i,5,9,7,6,1,8,4,2];
    let len = vec.len();
    let mut heap = MinMaxHeap::from_vec(vec);
    assert_eq!(heap.len(), len);
    assert_eq!(*heap.peek_max().unwrap(), 9);
    assert_eq!(*heap.peek_min().unwrap(), 1);

    heap.push(10);

    assert_eq!(heap.len(), len);
    assert_eq!(*heap.peek_max().unwrap(), 10);
    assert_eq!(*heap.peek_min().unwrap(), 2);
    chk_order(&heap);

    heap.push(0);

    assert_eq!(heap.len(), len);
    assert_eq!(*heap.peek_max().unwrap(), 10);
    assert_eq!(*heap.peek_min().unwrap(), 2);
    chk_order(&heap);
}

#[test]
fn test_from_vec_growable() {
    let vec = vec![3i,5,9,7,6,1,8,4,2];
    let len = vec.len();
    let mut heap = MinMaxHeap::from_vec_growable(vec);
    assert_eq!(heap.is_capped(), false);
    assert_eq!(heap.len(), len);
    assert_eq!(*heap.peek_max().unwrap(), 9);
    assert_eq!(*heap.peek_min().unwrap(), 1);

    heap.push(10);

    assert_eq!(heap.len(), len+1);
    assert_eq!(*heap.peek_max().unwrap(), 10);
    assert_eq!(*heap.peek_min().unwrap(), 1);
    chk_order(&heap);

    heap.push(0);
    assert_eq!(heap.len(), len+2);
    assert_eq!(*heap.peek_max().unwrap(), 10);
    assert_eq!(*heap.peek_min().unwrap(), 0);
    chk_order(&heap);
}
