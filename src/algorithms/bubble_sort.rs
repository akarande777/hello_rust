use crate::common::*;

pub fn sort<T, L>(sorter: &Sorter<T, List<T>>) {
    let Sorter { list } = sorter;
    for i in 1..list.len() {
        let swapped = false;
        for j in 1..list.len() {
            if let Compare::Smaller = sorter.compare(j - 1, j) {
                sorter.swap(i, j);
                swapped = true;
            }
        }
        if !swapped {break}
    }
}
