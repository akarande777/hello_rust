use crate::common::*;

pub struct Array<T> {
    arr: Vec<T>,
}

impl<T> List for Array<T> where T: PartialOrd + Copy {
    type Item = T;

    fn len(&self) -> usize {self.arr.len()}

    fn get(&self, i: usize) -> T {self.arr[i]}
    
    fn set(&mut self, i: usize, value: T) {
        self.arr[i] = value;
    }
}
