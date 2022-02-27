use crate::algorithms::*;

pub enum Algo {
    BubbleSort,
}

pub enum Compare {
    Smaller,
    Greater,
    Equal,
}

pub trait List<T> {
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn set(&self, i: usize, value: T);
}

pub struct Sorter<T, L: List<T>> {
    list: L,
}

impl <T, L> Sorter<T, L> where T: PartialOrd, L: List<T> {
    pub fn compare(&self, i: usize, j: usize) -> Compare {
        let Sorter { list } = self;
        if list.get(i) < list.get(j) {Compare::Smaller}
        else if list.get(i) > list.get(j) {Compare::Greater}
        else {Compare::Equal}
    }

    pub fn swap(&self, i: usize, j: usize) {
        let Sorter { list } = self;
        let temp = list.get(i);
        list.set(i, list.get(j));
        list.set(j, temp);
    }

    pub fn apply(&self, algo: Algo) {
        match algo {
            Algo::BubbleSort => {
                bubble_sort::sort::<T, L>(self);
            },
        }
    }
}
