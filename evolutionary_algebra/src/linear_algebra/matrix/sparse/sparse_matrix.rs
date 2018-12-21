extern crate num;

use super::super::common::MatIndex;
use num::Num;
use std::collections::HashMap;
use std::fmt::Display;

#[allow(dead_code)]
pub struct SparseMatrix<T> 
where T: Num + Clone + Copy + Display
{
    row_num: usize,
    col_num: usize,
    elements: HashMap<MatIndex, T>,
}


impl<T> SparseMatrix<T> 
where T: Num + Clone + Copy + Display
{
    pub fn new(row_num: usize, col_num: usize) -> SparseMatrix<T>
    {
        SparseMatrix {
            row_num: row_num,
            col_num: col_num,
            elements: HashMap::new(),
        }
    }
    
    #[inline]
    pub fn set_v(&mut self, row: usize, col: usize, v: T)
    {
        if row < self.row_num && col < self.col_num {
            let elm = self.elements.entry((row, col)).or_insert(v);
            *elm = v;
        }
    }

    #[inline]
    pub fn get_v(&self, row: usize, col: usize) -> T
    {
        T::clone(self.elements.get(&(row, col)).expect("not find key!"))
    }

    #[inline]
    pub fn row(&self) -> usize
    {
        self.row_num
    }

    #[inline]
    pub fn col(&self) -> usize
    {
        self.col_num
    }

}