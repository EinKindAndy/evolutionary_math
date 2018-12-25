extern crate num;

use super::super::common::MatIndex;
use num::{Num, Float};
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
        if row < self.row_num && col < self.col_num && v != T::zero() {
            let elm = self.elements.entry((row, col)).or_insert(v);
            *elm = v;
        }
    }

    #[inline]
    pub fn get_v(&self, row: usize, col: usize) -> T
    {
        match self.elements.get(&(row, col)) {
            Some(v) => T::clone(v),
            None => T::zero(),
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

}

macro_rules! build_sparse_matrix_f {
    ($row_num:expr , $col_num:expr , $insert_fun:expr) => {
        {
            let mut matrix = SparseMatrix::<T>::new($row_num, $col_num);
            for i in 0 .. $row_num {
                for j in 0 .. $col_num {
                    matrix.set_v(i, j, $insert_fun(i, j));
                }
            }
            matrix
        } 
    };
}


impl<T> SparseMatrix<T> 
where T: Num + Clone + Copy + Display
{
    pub fn t(&self) -> SparseMatrix<T>
    {
        let row_num = self.row();
        let col_num = self.col();
        let mut matrix = SparseMatrix::<T>::new(row_num, col_num);
        for i in 0 .. row_num {
                for j in 0 .. col_num {
                    matrix.set_v(j, i, self.get_v(i, j));
                }
        }
        matrix
    }


    pub fn slice(&self, row_from: usize, row_to: usize, col_from: usize, col_to: usize) -> SparseMatrix<T>
    {
        let row_num = row_to - row_from + 1;
        let col_num = col_to - col_from + 1;
        let mut matrix = SparseMatrix::<T>::new(row_num, col_num);
        for i in row_from .. row_to + 1 {
            for j in col_from .. col_to + 1 {
                matrix.set_v( i - row_from, j - col_from, self.get_v(i, j) );
            }
        } 
        matrix
    }

    pub fn scalar_mul(&self, k: T) -> SparseMatrix<T> {
        build_sparse_matrix_f!(self.row_num , self.col_num , |i, j|{ k * self.get_v(i, j) })
    }

    pub fn concat(&self, mat: &SparseMatrix<T>) -> SparseMatrix<T> {
        let row_num = self.row_num;
        let col_num = self.col_num + mat.col_num;
        let calc_elm = |i, j| {
            if j < self.col_num {
                self.get_v(i, j)
            }
            else {
                mat.get_v(i, j - self.col_num)
            }
        };
        build_sparse_matrix_f!(row_num , col_num , calc_elm)
    }

    pub fn dot_mul(&self, mat: &SparseMatrix<T>) -> SparseMatrix<T> {
        let row_num = self.row();
        let col_num = mat.col();
        let calc_elm = |i, j| {
            let mut sum = T::zero();
            for k in 0 .. self.col() {
                sum = sum + self.get_v(i, k) * mat.get_v(k, j);
            }
            sum
        };
        build_sparse_matrix_f!(row_num , col_num , calc_elm)
    }

    pub fn mul(&self, mat: &SparseMatrix<T>) -> SparseMatrix<T> {
        let calc_elm = |i, j| {
            self.get_v(i, j) * mat.get_v(i, j)
        };
        build_sparse_matrix_f!(self.row_num , self.col_num , calc_elm)
    }

    pub fn neg(&self) -> SparseMatrix<T> {
        let zero = T::zero();
        build_sparse_matrix_f!(self.row_num , self.col_num , |i, j|{ zero - self.get_v(i, j) })
    }

    pub fn add(&self, mat: &SparseMatrix<T>) -> SparseMatrix<T> {
        build_sparse_matrix_f!(self.row_num , self.col_num , |i, j|{ self.get_v(i, j) + mat.get_v(i, j) })
    }

    pub fn sub(&self, mat: &SparseMatrix<T>) -> SparseMatrix<T> {
        build_sparse_matrix_f!(self.row_num , self.col_num , |i, j|{ self.get_v(i, j) - mat.get_v(i, j) })
    }

    pub fn show(&self) {
        for i in 0 .. self.row_num {
            for j in 0 .. self.col_num {
                print!("{} ", self.get_v(i, j));
            }
            println!("");
        }
    }

}

impl<T> SparseMatrix<T> 
where T: Num + Clone + Copy + Display
{
    pub fn ones(num: usize) -> SparseMatrix<T> {
        build_sparse_matrix_f!(num , num , |_i, _j|{ T::one() })
    }

    pub fn zeros(num: usize) -> SparseMatrix<T> {
        SparseMatrix {
            row_num: num,
            col_num: num,
            elements: HashMap::new(),
        }
    }

    pub fn eye(num: usize) -> SparseMatrix<T> {
        let calc_elm = |i, j| {
            if i == j {
                T::one()
            }
            else {
                T::zero()
            }
        };
        build_sparse_matrix_f!(num , num , calc_elm)
    }

    pub fn diag(&self) -> SparseMatrix<T> {
        let calc_elm = |i, j| {
            if i == j {
                self.get_v(i, j)
            }
            else {
                T::zero()
            }
        };
        build_sparse_matrix_f!(self.row_num , self.col_num , calc_elm)
    }

    pub fn tri_u(&self) -> SparseMatrix<T> {
        let calc_elm = |i, j| {
            if i <= j {
                self.get_v(i, j)
            }
            else {
                T::zero()
            }
        };
        build_sparse_matrix_f!(self.row_num , self.col_num , calc_elm)
    }

    pub fn tri_l(&self) -> SparseMatrix<T> {
        let calc_elm = |i, j| {
            if i >= j {
                self.get_v(i, j)
            }
            else {
                T::zero()
            }
        };
        build_sparse_matrix_f!(self.row_num , self.col_num , calc_elm)
    }

    pub fn tri_strict_u(&self) -> SparseMatrix<T> {
        let calc_elm = |i, j| {
            if i < j {
                self.get_v(i, j)
            }
            else {
                T::zero()
            }
        };
        build_sparse_matrix_f!(self.row_num , self.col_num , calc_elm)
    }

    pub fn tri_strict_l(&self) -> SparseMatrix<T> {
        let calc_elm = |i, j| {
            if i > j {
                self.get_v(i, j)
            }
            else {
                T::zero()
            }
        };
        build_sparse_matrix_f!(self.row_num , self.col_num , calc_elm)
    }

    pub fn trace(&self) -> T {
        let mut sum = T::zero();
        let num = self.row_num.min(self.col_num);
        for i in 0 .. num {
            sum = sum + self.get_v(i, i);
        }
        sum
    }

    
}

impl<T> SparseMatrix<T> 
where T: Float + Clone + Copy + Display
{
    pub fn cofactors(&self, row: usize, col: usize) -> SparseMatrix<T>
    {
        let row_num = self.row_num - 1;
        let col_num = self.col_num - 1;
        let calc_elm = |i, j| {
            let ri = if i < row {
                i
            } else {
                i + 1
            };
            let cj = if j < col {
                j
            } else {
                j + 1
            };
            self.get_v(ri, cj)
        };
        build_sparse_matrix_f!(row_num , col_num , calc_elm)
    }

    pub fn adjoint(&self) -> SparseMatrix<T>
    {
        let scalar = -T::one();
        let calc_elm = |i, j| {
            scalar.powi((i + j) as i32) * self.cofactors(i, j).det_adj()
        };
        build_sparse_matrix_f!(self.row_num , self.col_num , calc_elm).t()
    }

    pub fn det_adj(&self) -> T
    {
        if self.row_num == self.col_num {
            match self.row_num {
                1 => self.get_v(0, 0),
                2 => self.get_v(0, 0) * self.get_v(1, 1) - self.get_v(0, 1) * self.get_v(1, 0),
                _ => {
                    let mut sum = T::zero();
                    let scalar = -T::one();
                    for i in 0 .. self.row_num {
                        sum = sum + scalar.powi(i as i32) * self.get_v(i, 0) * self.cofactors(i, 0).det_adj();
                    }
                    sum
                }
            }
        }
        else {
            T::zero()
        }
    }

    pub fn inv_adj(&self) -> Option<SparseMatrix<T>>
    {
        let det = self.det_adj();
        if det.abs() == T::zero() {
            None
        }
        else {
            Some(
                self.adjoint().scalar_mul(T::one() / det)
            )
        }
    }

    pub fn norm2(&self) -> T {
        self.t().dot_mul(&self).trace().sqrt()
    }

    #[allow(dead_code)]
    fn permute_rows(&mut self, row_i: usize, row_j: usize) {
        for col in 0 .. self.col_num {
            let tmp = self.get_v(row_i, col);
            self.set_v(row_i, col, self.get_v(row_j, col));
            self.set_v(row_j, col, tmp);
        }
    }

    #[allow(dead_code)]
    fn pivoting(&mut self, site: usize) -> i32 {
        let mut tmp = self.get_v(site, site).abs();
        let mut index = site;
        for row in (site + 1) .. self.row_num {
            if self.get_v(row, site).abs() > tmp {
                tmp = self.get_v(row, site).abs();
                index = row;
            }
        }
        if index != site {
            self.permute_rows(index, site);
            return 1;
        }
        return 0;
    }

    pub fn solve_ge(mat_a: &SparseMatrix<T>, mat_b: &SparseMatrix<T>) -> Option<SparseMatrix<T>>
    {
        let mut mat = mat_a.concat(&mat_b);
        for r in 0 .. mat.row_num {
            mat.pivoting(r);
            let head = mat.get_v(r, r);
            if head == T::zero() {
                return None;
            }    
            for c in r .. mat.col_num {
                mat.set_v(r, c, mat.get_v(r, c) / head);
            }
            for nr in r + 1 .. mat.row_num {
                let row_head = mat.get_v(nr, r);
                for c in r .. mat.col_num {
                    mat.set_v(nr, c, mat.get_v(nr, c) - row_head * mat.get_v(r, c));
                }
            }  
        }

        for r in (0 .. mat.row_num).rev() {
            for nr in (0 .. r).rev() {
                let row_head = mat.get_v(nr, r);
                for c in r .. mat.col_num {
                    mat.set_v(nr, c, mat.get_v(nr, c) - row_head * mat.get_v(r, c));
                }
            }
        }

        Some(mat.slice(0, mat.row_num - 1, mat_a.col_num, mat.col_num - 1))
    }

    fn mul_scalar_f(&self, k: T) -> SparseMatrix<T> {
        build_sparse_matrix_f!(self.row_num , self.col_num , |i, j|{ k * self.get_v(i, j) })
    }

    pub fn solve_sor(mat_a: &SparseMatrix<T>, mat_b: &SparseMatrix<T>, w_param: T, max_it: usize) -> SparseMatrix<T>
    {
        // Note that 0 < w < 2 !!!
        let mut xmat = Self::new(mat_a.row_num, mat_b.col_num);
        let mat_a_ = mat_a.t().dot_mul(&mat_a);
        let mat_b_ = mat_a.t().dot_mul(&mat_b);
        let mat_d = mat_a_.diag();
        let mat_e = mat_a_.tri_strict_l().scalar_mul(-T::one());
        let mat_f = mat_a_.tri_strict_u().scalar_mul(-T::one());
        let mat_m = mat_d.sub(&mat_e.mul_scalar_f(w_param)).inv_ge().unwrap();
        let b = mat_m.dot_mul(&mat_d.mul_scalar_f(T::one() - w_param).add(&mat_f.mul_scalar_f(w_param)));
        let c = mat_m.dot_mul(&mat_b_.mul_scalar_f(w_param));
        for _ in 0 .. max_it {
            xmat = b.dot_mul(&xmat).add(&c);
        }
        xmat
    }

    pub fn solve_cg(mat_a: &SparseMatrix<T>, mat_b: &SparseMatrix<T>, max_it: usize) -> SparseMatrix<T>
    {
        // Note that 0 < w < 2 !!!
        let mut xmat = Self::new(mat_a.row_num, mat_b.col_num);
        let mut r = mat_b.sub(&mat_a.dot_mul(&xmat));
        let mut s = mat_a.t().dot_mul(&r);
        let mut p = s.mul_scalar_f(T::one());
        let mut sts = s.t().dot_mul(&s).get_v(0, 0);
        for _ in 0 .. max_it {
            let q = mat_a.dot_mul(&p);
            let alpha = sts / q.t().dot_mul(&q).get_v(0, 0);
            xmat = xmat.add(&p.mul_scalar_f(alpha));
            r = r.sub(&q.mul_scalar_f(alpha));
            s = mat_a.t().dot_mul(&r);
            let sts_ = sts;
            if sts_ == T::zero() {
                break;
            }
            sts = s.t().dot_mul(&s).get_v(0, 0);
            let beta = sts / sts_;
            p = s.add(&p.mul_scalar_f(beta));
        }
        xmat
    }

    pub fn inv_ge(&self) -> Option<SparseMatrix<T>>
    {
        Self::solve_ge(&self, &Self::eye(self.row_num))
    }

    pub fn det_ge(&self) -> T {
        let mut mat = self.scalar_mul(T::one());
        let mut count = 0;
        for r in 0 .. mat.row_num {
            count += mat.pivoting(r);
            let head = mat.get_v(r, r);
            if head == T::zero() {
                return T::zero();
            }    
            for nr in r + 1 .. mat.row_num {
                let row_head = mat.get_v(nr, r) / head;
                for c in r .. mat.col_num {
                    mat.set_v(nr, c, mat.get_v(nr, c) - row_head * mat.get_v(r, c));
                }
            }  
        }
        let mut product = T::one();
        for i in 0 .. self.row_num {
            product = product * mat.get_v(i, i);
        }
        (-T::one()).powi(count) * product
    }
}
