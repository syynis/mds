use std::marker::PhantomData;

pub struct FastSet<T: Into<usize> + Copy> {
    set: Vec<usize>,
    generation: usize,
    phantom: PhantomData<T>,
}

impl<T> FastSet<T>
where
    T: Into<usize> + Copy,
{
    pub fn new(size: usize) -> Self {
        Self {
            set: vec![0; size],
            generation: 1,
            phantom: PhantomData::default(),
        }
    }

    pub fn insert(&mut self, key: T) {
        self.set[key.into()] = self.generation;
    }

    pub fn has_insert(&mut self, key: T) -> bool {
        let res = !self.contains(key);
        self.insert(key);
        res
    }

    pub fn contains(&self, key: T) -> bool {
        self.set[key.into()] == self.generation
    }

    pub fn clear(&mut self) {
        if self.generation == usize::MAX {
            self.set.fill(0);
            self.generation = 0;
        }
        self.generation += 1;
    }
}

pub struct DenseFastSet<T: Into<usize> + Copy> {
    dense: Vec<T>,
    sparse: Vec<Option<usize>>,
}

impl<T> DenseFastSet<T>
where
    T: Into<usize> + Copy,
{
    pub fn new(size: usize) -> Self {
        Self {
            dense: Vec::default(),
            sparse: vec![None; size],
        }
    }

    pub fn insert_unchecked(&mut self, key: T) {
        self.dense.push(key);
        self.sparse[key.into()] = Some(self.dense.len() - 1);
    }

    pub fn insert(&mut self, key: T) -> bool {
        if !self.contains(key) {
            self.insert_unchecked(key);
            return true;
        }
        false
    }

    pub fn remove(&mut self, key: T) {
        if let Some(dense_idx) = self.dense_idx(key) {
            let back = (*self.dense.last().unwrap()).into();
            let removed = self.dense.swap_remove(dense_idx);
            self.sparse[back] = Some(dense_idx);
            self.sparse[removed.into()] = None;
        }
    }

    pub fn contains(&self, key: T) -> bool {
        self.sparse[key.into()].is_some()
    }

    pub fn dense_idx(&self, key: T) -> Option<usize> {
        self.sparse.get(key.into()).cloned().flatten()
    }

    pub fn clear(&mut self) {
        self.dense.clear();
        self.sparse.fill(None);
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.dense.iter().copied()
    }
}
