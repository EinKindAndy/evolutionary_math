# evolutionary_math
It's a tiny project to practice my rust programming skills. I hope to learn something about Rust. More importantly, I'm going to implement most numerical algorithms by myself to enhence my experiences.

## evolutionary_algebra
### linear_algebra
#### matrix
##### dense
```
//baisc matrix operations
//operations of dense matrices at the current stage
extern crate num;

use super::super::common::MatIndex;
use num::{Num, Float};
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

    pub fn mul(&self, mat: &DenseMatrix<T>) -> DenseMatrix<T> {
        let calc_elm = |i, j| {
            self.get_v(i, j) * mat.get_v(i, j)
        };
        build_dense_matrix_f!(self.row_num , self.col_num , calc_elm)
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

    pub fn eye(num: usize) -> DenseMatrix<T> {
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

    pub fn diag(&self) -> DenseMatrix<T> {
        let calc_elm = |i, j| {
            if i == j {
                self.get_v(i, j)
            }
            else {
                T::zero()
            }
        };
        build_dense_matrix_f!(self.row_num , self.col_num , calc_elm)
    }

    pub fn tri_u(&self) -> DenseMatrix<T> {
        let calc_elm = |i, j| {
            if i <= j {
                self.get_v(i, j)
            }
            else {
                T::zero()
            }
        };
        build_dense_matrix_f!(self.row_num , self.col_num , calc_elm)
    }

    pub fn tri_l(&self) -> DenseMatrix<T> {
        let calc_elm = |i, j| {
            if i >= j {
                self.get_v(i, j)
            }
            else {
                T::zero()
            }
        };
        build_dense_matrix_f!(self.row_num , self.col_num , calc_elm)
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

impl<T> DenseMatrix<T> 
where T: Float + Clone + Copy + Display
{
    pub fn cofactors(&self, row: usize, col: usize) -> DenseMatrix<T>
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
        build_dense_matrix_f!(row_num , col_num , calc_elm)
    }

    pub fn adjoint(&self) -> DenseMatrix<T>
    {
        let scalar = -T::one();
        let calc_elm = |i, j| {
            scalar.powi((i + j) as i32) * self.cofactors(i, j).det_adj()
        };
        build_dense_matrix_f!(self.row_num , self.col_num , calc_elm).t()
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

    pub fn inv_adj(&self) -> Option<DenseMatrix<T>>
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

}
```
## evolutionary_geometry
### euclidean
#### point, vector, hexahedron, tetragon
```
//basic geometry structures
use evolutionary_geometry::euclidean::Tetragon;
use evolutionary_geometry::euclidean::Hexahedron;
use evolutionary_geometry::euclidean::Point2D;
use evolutionary_geometry::euclidean::Point3D;

fn main() {
    let p1 = Point2D::new(0.0, 0.0);
    let p2 = Point2D::new(2.0, 0.0);
    let p3 = Point2D::new(2.0, 2.0);
    let p4 = Point2D::new(0.0, 2.0);
    let tetragon = Tetragon::new(p1, p2, p3, p4);
    tetragon.show();

    let p_1 = Point3D::new(2.0, 0.0, 0.0);
    let p_2 = Point3D::new(2.0, 2.0, 0.0);
    let p_3 = Point3D::new(0.0, 2.0, 0.0);
    let p_4 = Point3D::new(0.0, 0.0, 0.0);
    let p_5 = Point3D::new(2.0, 0.0, 2.0);
    let p_6 = Point3D::new(2.0, 2.0, 2.0);
    let p_7 = Point3D::new(0.0, 2.0, 2.0);
    let p_8 = Point3D::new(0.0, 0.0, 2.0);

    let hexahedron = Hexahedron::new(p_1, p_2, p_3, p_4,
                                     p_5, p_6, p_7, p_8);
    hexahedron.show();
}
```

## evolutionary_pde
### mesh
#### structured
##### HexahedronMesh, TetragonMesh
```
#[allow(dead_code)]
pub struct TetragonMesh {
    size: usize,
    cells: HashMap<usize, TetragonCell>,
}

#[allow(dead_code)]
pub struct HexahedronMesh {
    size: usize,
    cells: HashMap<usize, HexahedronCell>,
}

impl TetragonMesh {
    pub fn new() -> TetragonMesh {
        TetragonMesh {
            size: 0,
            cells: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: usize, cell: TetragonCell) {
        self.cells.insert(id, cell);
        self.size += 1;
    }
}

impl HexahedronMesh {
    pub fn new() -> HexahedronMesh {
        HexahedronMesh {
            size: 0,
            cells: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: usize, cell: HexahedronCell) {
        self.cells.insert(id, cell);
        self.size += 1;
    }
}
```
