extern mod std;

use std::{util, vec};

#[deriving(Clone)]
pub struct MinMaxHeap<T> {
    dat: ~[T],
    cap: uint
}

impl<T:Ord+Clone> MinMaxHeap<T> {

    pub fn new(cap: uint) -> MinMaxHeap<T> {
        MinMaxHeap { dat: ~[], cap: cap }
    }

    pub fn with_capacity(cap: uint) -> MinMaxHeap<T> {
        MinMaxHeap { dat: vec::with_capacity(cap), cap: cap }
    }

    pub fn from_vec(v: ~[T]) -> MinMaxHeap<T> {
        let len = v.len();
        let mut out = MinMaxHeap { dat: v, cap: len };
        for i in range(0, out.len()).invert() {
            out.trickle_down(i);
        }
        out
    }

    pub fn from_vec_growable(v: ~[T]) -> MinMaxHeap<T> {
        let mut out = MinMaxHeap { dat: v, cap: 0 };
        for i in range(0, out.len()).invert() {
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
        if self.is_empty() { None } else { Some(&self.dat[0]) }
    }

    pub fn peek_max<'a>(&'a self) -> Option<&'a T> {
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
            1 => self.dat.pop_opt(),
            _ => {
                let out = self.dat.swap_remove(0);
                self.trickle_down(0);
                Some(out)
            }
        }
    }

    pub fn pop_max(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            1|2 => self.dat.pop_opt(),
            3 => {
                if self.dat[1] >= self.dat[2] {
                    Some(self.dat.swap_remove(1))
                } else {
                    self.dat.pop_opt()
                }
            },
            _ => {
                let idx = if self.dat[1] >= self.dat[2] { 1 } else { 2 };
                let out = self.dat.swap_remove(idx);
                self.trickle_down(idx);
                Some(out)
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
            let out = util::replace(&mut self.dat[0], item);
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
            let out = util::replace(&mut self.dat[idx], item);
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
                if self.dat[i] > self.dat[parent(i)] {
                    self.dat.swap(i, parent(i));
                    self.bubble_up_max(parent(i));
                } else {
                    self.bubble_up_min(i);
                }
            },
            Max => {
                if self.dat[i] < self.dat[parent(i)] {
                    self.dat.swap(i, parent(i));
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
        if self.dat[i] < self.dat[grandparent(i)] {
            self.dat.swap(i, grandparent(i));
            self.bubble_up_min(grandparent(i));
        }
    }

    fn bubble_up_max(&mut self, i: uint) {
        if i < 3 { // no grandparent
            return;
        }
        if self.dat[i] > self.dat[grandparent(i)] {
            self.dat.swap(i, grandparent(i));
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
        if self.dat[m] < self.dat[i] {
            self.dat.swap(m, i);
        }
        if !(m == left(i) || m == right(i)) { // m is a grandchild
            if self.dat[m] > self.dat[parent(m)] {
                self.dat.swap(m, parent(m));
            }
            self.trickle_down_min(m);
        }
    }

    fn trickle_down_max(&mut self, i: uint) {
        let m = self.largest_child_or_grandchild(i);
        if self.dat[m] > self.dat[i] {
            self.dat.swap(m, i);
        }
        if m == 0 {
            return;
        }
        if !(m == left(i) || m == right(i)) { // m is a grandchild
            if self.dat[m] < self.dat[parent(m)] {
                self.dat.swap(m, parent(m));
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
                if self.dat[*idx] < self.dat[min_idx] {
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
                if self.dat[*idx] > self.dat[max_idx] {
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
