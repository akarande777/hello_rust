use crate::common::*;
use std::fmt::Debug;

pub fn sort<L: List + Debug>(list: &mut L) {
    for _i in 1..list.len() {
        let mut swapped = false;
        for j in 1..list.len() {
            if let Compare::Greater = compare(list, j - 1, j) {
                swap(list, j - 1, j);
                swapped = true;
            }
        }
        if !swapped {break}
    }
}
