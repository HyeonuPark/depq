
use std::cmp::Ord;

#[derive(Debug, Clone)]
pub struct Depq<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> Depq<T> {
    pub fn new() -> Self {
        Depq {
            data: vec![],
        }
    }

    pub fn min(&self) -> Option<&T> {
        self.data.get(Idx::root().min())
    }

    pub fn max(&self) -> Option<&T> {
        self.data.get(Idx::root().max())
    }

    pub fn min_mut(&mut self) -> Option<&mut T> {
        self.data.get_mut(Idx::root().max())
    }

    pub fn max_mut(&mut self) -> Option<&mut T> {
        self.data.get_mut(Idx::root().max())
    }

    pub fn push(&mut self, value: T) {
        let mut idx = Idx::from_pos(self.data.len());
        let data = &mut self.data;
        data.push(value);

        while !idx.is_root() {
            let parent = idx.parent();
            let mut modified = false;

            let (idx_min, parent_min, parent_max) = (idx.min(), parent.min(), parent.max());

            let idx_max = if idx.max() >= data.len() {
                idx.min()
            } else {
                idx.max()
            };

            if data[idx_min] > data[idx_max] {

            }

            if data[idx_min] < data[parent_min] {
                modified = true;
                data.swap(idx_min, parent_min);
            }

            if data[idx_max] > data[parent_max] {
                modified = true;
                data.swap(idx_max, parent_max);
            }

            if !modified {
                break
            }

            idx = parent;
        }
    }

    fn pop(&mut self, kind: Kind) -> T {
        let mut idx = Idx::root();
        let data = &mut self.data;
        let res = data.swap_remove(idx.leaf(kind));

        loop {
            let (left, right) = (idx.left(), idx.right());

            let next = if kind.cmp(&data, left.leaf(kind), right.leaf(kind)) {
                left
            } else {
                right
            };

            if kind.cmp(&data, idx.leaf(kind), next.leaf(kind)) {
                break
            }

            data.swap(idx.leaf(kind), next.leaf(kind));
            idx = next;
        }

        res
    }

    pub fn pop_min(&mut self) -> Option<T> {
        if self.data.len() <= Idx::root().min() {
            self.data.pop()
        } else {
            Some(self.pop(Kind::Min))
        }
    }

    pub fn pop_max(&mut self) -> Option<T> {
        if self.data.len() <= Idx::root().max() {
            self.data.pop()
        } else {
            Some(self.pop(Kind::Max))
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Min,
    Max,
}

impl Kind {
    fn cmp<T: Ord>(self, buf: &[T], left: usize, right: usize) -> bool {
        match self {
            Kind::Min => &buf[left] < &buf[right],
            Kind::Max => &buf[left] > &buf[right],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Idx(usize);

impl Idx {
    fn root() -> Self {
        Idx(0)
    }

    fn from_pos(pos: usize) -> Self {
        Idx(pos >> 1)
    }

    fn is_root(self) -> bool {
        self.0 == 0
    }

    fn min(self) -> usize {
        self.0 << 1
    }

    fn max(self) -> usize {
        (self.0 << 1) | 1
    }

    fn leaf(self, kind: Kind) -> usize {
        match kind {
            Kind::Min => self.min(),
            Kind::Max => self.max(),
        }
    }

    fn parent(self) -> Self {
        Idx(self.0 >> 1)
    }

    fn left(self) -> Self {
        Idx(self.0 << 1)
    }

    fn right(self) -> Self {
        Idx((self.0 << 1) | 1)
    }
}
