use crate::algorithms::*;

pub enum Algo {
    BubbleSort,
}

pub enum Compare {
    Smaller,
    Greater,
    Equal,
}

pub trait List {
    type Item: PartialOrd;
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> Self::Item;
    fn set(&mut self, i: usize, value: Self::Item);
}

pub struct Sorter<L: List> {
    pub list: L,
}

impl<L> Sorter<L> where L: List {
    pub fn compare(&self, i: usize, j: usize) -> Compare {
        let Sorter { list } = self;
        if list.get(i) < list.get(j) {Compare::Smaller}
        else if list.get(i) > list.get(j) {Compare::Greater}
        else {Compare::Equal}
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        let Sorter { list } = self;
        let temp = list.get(i);
        list.set(i, list.get(j));
        list.set(j, temp);
    }

    pub fn sort(&mut self, algo: Algo) {
        match algo {
            Algo::BubbleSort => {
                bubble_sort::sort::<L>(self);
            },
        }
    }
}
