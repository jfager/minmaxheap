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

    pub fn peek_min(&self) -> Option<&T> {
        if self.is_empty() { None } else { Some(&self.dat[0]) }
    }

    pub fn peek_max(&self) -> Option<&T> {
        match self.max_idx() {
            None => None,
            Some(i) => Some(&self.dat[i])
        }
    }

    fn max_idx(&self) -> Option<uint> {
        match self.len() {
            0 => None,
            1 => Some(0),
            2 => Some(1),
            _ => Some(if self.dat[1] >= self.dat[2] { 1 } else { 2 })
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
                if self.dat[1] >= self.dat[2] {
                    self.dat.swap_remove(1)
                } else {
                    self.dat.pop()
                }
            },
            _ => {
                let idx = if self.dat[1] >= self.dat[2] { 1 } else { 2 };
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
            let out = mem::replace(&mut self.dat[0], item);
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
            let out = mem::replace(&mut self.dat[idx], item);
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
        let o = match i.level_type() {
            Min => std::cmp::Greater,
            Max => std::cmp::Less
        };
        if self.dat[i].cmp(&self.dat[i.parent()]) == o {
            self.dat.as_mut_slice().swap(i, i.parent());
            self._bubble_up(i.parent(), o);
        } else {
            self._bubble_up(i, rev_order(o));
        }
    }

    fn _bubble_up(&mut self, i: uint, o: std::cmp::Ordering) {
        if i < 3 { // no grandparent
            return;
        }
        if self.dat[i].cmp(&self.dat[i.grandparent()]) == o {
            self.dat.as_mut_slice().swap(i, i.grandparent());
            self._bubble_up(i.grandparent(), o);
        }
    }

    fn trickle_down(&mut self, i: uint) {
        match i.level_type() {
            Min => self._trickle_down(i, std::cmp::Less),
            Max => self._trickle_down(i, std::cmp::Greater)
        }
    }

    fn _trickle_down(&mut self, i: uint, o: std::cmp::Ordering) {
        let m = self.child_or_grandchild(i, o);
        if m == 0 {
            return;
        }
        if self.dat[m].cmp(&self.dat[i]) == o {
            self.dat.as_mut_slice().swap(m, i);
        }
        if !m.is_child_of(i) { //m is a grandchild
            if self.dat[m.parent()].cmp(&self.dat[m]) == o {
                self.dat.as_mut_slice().swap(m, m.parent());
            }
            self._trickle_down(m, o);
        }
    }

    fn child_or_grandchild(&self, i: uint, o: std::cmp::Ordering) -> uint {
        let l = i.left();
        if l < self.dat.len() {
            let mut out = l;
            let r = i.right();
            for idx in [r, l.left(), l.right(), r.left(), r.right()].iter() {
                if *idx >= self.dat.len() {
                    break;
                }
                if self.dat[*idx].cmp(&self.dat[out]) == o {
                    out = *idx;
                }
            }
            out
        } else {
            0
        }
    }

}

trait HeapIdx {
    fn left(self) -> Self;
    fn right(self) -> Self;
    fn parent(self) -> Self;
    fn grandparent(self) -> Self;
    fn is_child_of(self, i: Self) -> bool;
    fn level(self) -> uint;
    fn level_type(self) -> LevelType;
}

impl HeapIdx for uint {
    fn left(self) -> uint {
        (self * 2) + 1
    }

    fn right(self) -> uint {
        (self * 2) + 2
    }

    fn parent(self) -> uint {
        if self == 0 { 0 } else { (self - 1) / 2 }
    }

    fn grandparent(self) -> uint {
        self.parent().parent()
    }

    fn is_child_of(self, parent: uint) -> bool {
        self == parent.left() || self == parent.right()
    }

    fn level(self) -> uint {
        let mut c = self;
        let mut out = 0;
        while c != 0 {
            c = c.parent();
            out += 1;
        }
        out
    }

    fn level_type(self) -> LevelType {
        if self.level() % 2 == 0 { Min } else { Max }
    }

}


enum LevelType {
    Min,
    Max
}

fn rev_order(o: std::cmp::Ordering) -> std::cmp::Ordering {
    match o {
        Less    => Greater,
        Equal   => Equal,
        Greater => Less
    }
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
