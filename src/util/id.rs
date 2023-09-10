use std::marker::PhantomData;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd, Debug)]
pub struct Id<Tag> {
    id: usize,
    _phantom: PhantomData<fn() -> Tag>,
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
