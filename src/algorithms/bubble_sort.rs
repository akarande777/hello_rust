use crate::common::*;

pub fn sort<T>(sorter: &mut Sorter<T>) where T: List {
    let list = &mut sorter.list;
    for i in 1..list.len() {
        let mut swapped = false;
        for j in 1..list.len() {
            if let Compare::Smaller = sorter.compare(j - 1, j) {
                sorter.swap(i, j);
                swapped = true;
            }
        }
        if !swapped {break}
    }
}
