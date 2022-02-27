use crate::common::*;

pub struct Array<T> {
    arr: Vec<T>,
}

impl <T> List<T> for Array<T> {
    fn len(&self) -> usize {self.arr.len()}

    fn get(&self, i: usize) -> T {self.arr[i]}
    
    fn set(&self, i: usize, value: T) {
        self.arr[i] = value;
    }
}
