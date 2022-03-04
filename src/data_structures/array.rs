use crate::common::*;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Array<T>(Vec<T>);

impl<T> Array<T> {
    pub fn new(list: Vec<T>) -> Self {Array(list)}
}

impl<T> List for Array<T> where T: PartialOrd + Clone {
    type Item = T;

    fn len(&self) -> usize {self.0.len()}

    fn get(&self, i: usize) -> T {self.0[i].clone()}
    
    fn set(&mut self, i: usize, value: T) {
        self.0[i] = value;
    }
}
