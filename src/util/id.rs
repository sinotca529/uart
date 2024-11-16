use std::marker::PhantomData;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd, Debug)]
pub struct Id<Tag> {
    id: usize,
    _phantom: PhantomData<fn() -> Tag>,
}

impl<Tag> Id<Tag> {
    fn new(id: usize) -> Self {
        Self {
            id,
            _phantom: Default::default(),
        }
    }
}

pub struct IdGenerator<Tag> {
    next_id: usize,
    _phantom: PhantomData<fn() -> Tag>,
}

impl<Tag> Default for IdGenerator<Tag> {
    fn default() -> Self {
        Self {
            next_id: 0,
            _phantom: Default::default(),
        }
    }
}

impl<Tag> IdGenerator<Tag> {
    pub fn gen(&mut self) -> Id<Tag> {
        let id = Id {
            id: self.next_id,
            _phantom: Default::default(),
        };
        self.next_id += 1;
        id
    }
}

/// Bitset of Id
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IdSet<Tag> {
    blocks: Vec<u32>,
    is_empty: bool,
    _phantom: PhantomData<fn() -> Tag>,
}

impl<Tag> Default for IdSet<Tag> {
    fn default() -> Self {
        Self {
            blocks: Default::default(),
            is_empty: true,
            _phantom: Default::default(),
        }
    }
}

impl<Tag> IdSet<Tag> {
    #[inline(always)]
    fn extend(&mut self, num_blocks: usize) {
        if num_blocks > self.blocks.len() {
            self.blocks.resize(num_blocks, 0);
        }
    }

    /// Return true if the `id` is newly inserted.
    pub fn insert(&mut self, id: &Id<Tag>) -> bool {
        self.is_empty = false;
        let block_idx = id.id >> 5;
        let inner_idx = id.id & 31;
        self.extend(block_idx + 1);
        let old = self.blocks[block_idx];
        self.blocks[block_idx] |= 1 << inner_idx;
        old != self.blocks[block_idx]
    }

    /// Return true if the `id` is removed.
    pub fn remove(&mut self, id: &Id<Tag>) -> bool {
        if !self.contains(id) {
            return false;
        }
        let block_idx = id.id >> 5;
        let inner_idx = id.id & 31;
        self.blocks[block_idx] ^= 1 << inner_idx;
        if self.num_elems() == 0 {
            self.is_empty = true;
        }
        true
    }

    pub fn toggle(&mut self, id: &Id<Tag>) {
        if self.contains(id) {
            self.remove(id);
        } else {
            self.insert(id);
        }
    }

    #[allow(unused)]
    pub fn contains(&self, id: &Id<Tag>) -> bool {
        let block_idx = id.id >> 5;
        let inner_idx = id.id & 31;
        if block_idx >= self.blocks.len() {
            return false;
        }
        (self.blocks[block_idx] & (1 << inner_idx)) != 0
    }

    pub fn union_with(&mut self, other: &Self) -> bool {
        self.is_empty &= other.is_empty;
        self.extend(other.blocks.len());
        let mut updated = false;
        for (s, o) in self.blocks.iter_mut().zip(other.blocks.iter()) {
            let before = *s;
            *s |= o;
            updated |= before != *s;
        }

        updated
    }

    pub fn num_elems(&self) -> u32 {
        self.blocks.iter().map(|b| b.count_ones()).sum()
    }

    #[inline(always)]
    pub fn iter(&self) -> IdSetIterator<Tag> {
        IdSetIterator::new(self)
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }
}

pub struct IdSetIterator<'a, Tag> {
    bitset: &'a IdSet<Tag>,
    block: u32,
    next_block_idx: u32,
}

impl<'a, Tag> IdSetIterator<'a, Tag> {
    fn new(bitset: &'a IdSet<Tag>) -> Self {
        Self {
            bitset,
            block: 0,
            next_block_idx: 0,
        }
    }
}

impl<'a, Tag> Iterator for IdSetIterator<'a, Tag> {
    type Item = Id<Tag>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.block == 0 {
            if self.next_block_idx < self.bitset.blocks.len() as u32 {
                self.block = self.bitset.blocks[self.next_block_idx as usize];
                self.next_block_idx += 1;
            } else {
                return None;
            }
        }

        let tz = self.block.trailing_zeros() as usize;
        let id = tz + ((self.next_block_idx as usize - 1) << 5);
        self.block &= self.block - 1;
        Some(Id::<Tag>::new(id))
    }
}
