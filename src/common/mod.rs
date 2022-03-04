use crate::algorithms::*;
use std::fmt::Debug;

pub enum Algo {
    BubbleSort,
}

pub enum Compare {
    Smaller,
    Greater,
    Equal,
}

pub trait List {
    type Item: PartialOrd + Clone;
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> Self::Item;
    fn set(&mut self, i: usize, value: Self::Item);
}

pub fn compare<L: List>(list: &L, i: usize, j: usize) -> Compare {
    if list.get(i) < list.get(j) {Compare::Smaller}
    else if list.get(i) > list.get(j) {Compare::Greater}
    else {Compare::Equal}
}

pub fn swap<L: List>(list: &mut L, i: usize, j: usize) {
    let temp = list.get(i);
    list.set(i, list.get(j));
    list.set(j, temp);
}

pub fn sort<L: List + Debug>(list: &mut L, algo: Algo) {
    match algo {
        Algo::BubbleSort => {
            bubble_sort::sort::<L>(list);
        },
    }
}
