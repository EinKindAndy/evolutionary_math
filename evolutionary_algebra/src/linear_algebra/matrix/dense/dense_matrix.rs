extern crate num;

use super::super::common::MatIndex;
use num::Num;
use std::collections::HashMap;
use std::fmt::Display;

#[allow(dead_code)]
pub struct DenseMatrix<T> 
where T: Num + Clone + Copy + Display
{
    row_num: usize,
    col_num: usize,
    elements: HashMap<MatIndex, T>,
}

macro_rules! build_dense_matrix_v {
    ($row_num:expr , $col_num:expr , $insert_item:expr) => {
        {
            let mut elements = HashMap::<MatIndex, T>::with_capacity($row_num * $col_num);
            for i in 0 .. $row_num {
                for j in 0 .. $col_num {
                    elements.insert((i,j), $insert_item);
                }
            }
            DenseMatrix {
                row_num: $row_num,
                col_num: $col_num,
                elements: elements,
            }
        } 
    };
}

macro_rules! build_dense_matrix_f {
    ($row_num:expr , $col_num:expr , $insert_fun:expr) => {
        {
            let mut elements = HashMap::<MatIndex, T>::with_capacity($row_num * $col_num);
            for i in 0 .. $row_num {
                for j in 0 .. $col_num {
                    elements.insert((i,j), $insert_fun(i, j));
                }
            }
            DenseMatrix {
                row_num: $row_num,
                col_num: $col_num,
                elements: elements,
            }
        } 
    };
}

impl<T> DenseMatrix<T> 
where T: Num + Clone + Copy + Display
{
    pub fn new(row_num: usize, col_num: usize) -> DenseMatrix<T>
    {
        DenseMatrix {
            row_num: row_num,
            col_num: col_num,
            elements: HashMap::with_capacity(row_num * col_num),
        }
    }

    pub fn from(row_num: usize, col_num: usize, slice: &[T]) -> DenseMatrix<T>
    {
        build_dense_matrix_f!(row_num , col_num , |i, j|{slice[i * col_num + j]})
    }

    pub fn t(&self) -> DenseMatrix<T>
    {
        let row_num = self.row();
        let col_num = self.col();
        let mut elements = HashMap::with_capacity(row_num * col_num);
        for i in 0 .. row_num {
            for j in 0 .. col_num {
                elements.insert((j, i), self.get_v(i, j));
            }
        }
        DenseMatrix {
            row_num: col_num,
            col_num: row_num,
            elements: elements,
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

    pub fn slice(&self, row_from: usize, row_to: usize, col_from: usize, col_to: usize) -> DenseMatrix<T>
    {
        let row_num = row_to - row_from + 1;
        let col_num = col_to - col_from + 1;
        let mut elements = HashMap::with_capacity(row_num * col_num);
        for i in row_from .. row_to + 1 {
            for j in col_from .. col_to + 1 {
                elements.insert((i - row_from, j - col_from), self.get_v(i, j));
            }
        } 
        DenseMatrix {
            row_num: row_num,
            col_num: col_num,
            elements: elements,
        }
    }

    pub fn safe_slice(&self, row_from: usize, row_to: usize, col_from: usize, col_to: usize) -> Option<DenseMatrix<T>>
    {
        if row_to < self.row_num && col_to < self.col_num {
            Some(self.slice(row_from, row_to, col_from, col_to))
        }
        else {
            None
        }
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

    pub fn scalar_mul(&self, k: T) -> DenseMatrix<T> {
        build_dense_matrix_f!(self.row_num , self.col_num , |i, j|{ k * self.get_v(i, j) })
    }

    pub fn dot_mul(&self, mat: &DenseMatrix<T>) -> DenseMatrix<T> {
        let row_num = self.row();
        let col_num = mat.col();
        let calc_elm = |i, j| {
            let mut sum = T::zero();
            for k in 0 .. self.col() {
                sum = sum + self.get_v(i, k) * mat.get_v(k, j);
            }
            sum
        };
        build_dense_matrix_f!(row_num , col_num , calc_elm)
    }

    pub fn safe_dot_mul(&self, mat: &DenseMatrix<T>) -> Option<DenseMatrix<T>>
    {
        if self.col_num == mat.row_num {
            Some(self.dot_mul(mat))
        }
        else {
            None
        }
    }

    pub fn neg(&self) -> DenseMatrix<T> {
        let zero = T::zero();
        build_dense_matrix_f!(self.row_num , self.col_num , |i, j|{ zero - self.get_v(i, j) })
    }

    pub fn add(&self, mat: &DenseMatrix<T>) -> DenseMatrix<T> {
        build_dense_matrix_f!(self.row_num , self.col_num , |i, j|{ self.get_v(i, j) + mat.get_v(i, j) })
    }

    pub fn safe_add(&self, mat: &DenseMatrix<T>) -> Option<DenseMatrix<T>>
    {
        if self.row_num == mat.row_num && self.col_num == mat.col_num {
            Some(self.add(mat))
        }
        else {
            None
        }
    }

    pub fn sub(&self, mat: &DenseMatrix<T>) -> DenseMatrix<T> {
        build_dense_matrix_f!(self.row_num , self.col_num , |i, j|{ self.get_v(i, j) - mat.get_v(i, j) })
    }

    pub fn safe_sub(&self, mat: &DenseMatrix<T>) -> Option<DenseMatrix<T>>
    {
        if self.row_num == mat.row_num && self.col_num == mat.col_num {
            Some(self.sub(mat))
        }
        else {
            None
        }
    }

    pub fn show(&self) {
        if self.row() * self.col() == self.elements.len() {
            for i in 0 .. self.row_num {
                for j in 0 .. self.col_num {
                    print!("{} ", self.get_v(i, j));
                }
                println!("");
            }
        }
    }

}

impl<T> DenseMatrix<T> 
where T: Num + Clone + Copy + Display
{
    pub fn ones(num: usize) -> DenseMatrix<T> {
        build_dense_matrix_v!(num , num , T::one())
    }

    pub fn zeros(num: usize) -> DenseMatrix<T> {
        build_dense_matrix_v!(num , num , T::zero())
    }

    pub fn eyes(num: usize) -> DenseMatrix<T> {
        let calc_elm = |i, j| {
            if i == j {
                T::one()
            }
            else {
                T::zero()
            }
        };
        build_dense_matrix_f!(num , num , calc_elm)
    }
}
