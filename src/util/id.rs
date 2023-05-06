use std::marker::PhantomData;

pub struct Id<T> {
    _phantom: PhantomData<fn() -> T>,
    id: usize,
}

impl<T> Id<T> {
    fn new(id: usize) -> Self {
        Self {
            _phantom: Default::default(),
            id,
        }
    }
}

pub struct IdGenerator<T> {
    _phantom: PhantomData<fn() -> T>,
    next_id: usize,
}


impl<T> IdGenerator<T> {
    pub fn new() -> Self {
        Self {
            _phantom: Default::default(),
            next_id: 0,
        }
    }

    pub fn gen(&mut self) -> Id<T> {
        let id = Id::new(self.next_id);
        self.next_id += 1;
        id
    }
}
