# evolutionary_math
It's a tiny project to practice my rust programming skills. I hope to learn something about Rust. More importantly, I'm going to implement most numerical algorithms by myself to enhence my experiences.

## evolutionary_algebra
### linear_algebra
#### matrix
##### dense
```
//baisc matrix operations
//operations of dense matrices at the current stage
use rand::prelude::*;
use evolutionary_algebra::linear_algebra::matrix::dense::DenseMatrix;

fn main() {
    
    let mut ones = DenseMatrix::<f32>::new(5, 6);
    for i in 0 .. 5 {
        for j in 0 .. 6 {
            ones.set_v(i, j, 1.0);
        }
    }

    ones.show();

    let mut twos = DenseMatrix::<f32>::new(5, 6);
    for i in 0 .. 5 {
        for j in 0 .. 6 {
            twos.set_v(i, j, 2.0);
        }
    }

    twos.show();

    for i in 0 .. twos.row() {
        for j in 0 .. twos.col() {
            print!("{} ", twos.get_v(i, j));
        }
        println!("");
    }

    println!("{}", twos.get_v(0, 0));

    let threes = ones.add(&twos);

    twos.add(&threes).slice(1, 4, 2, 3).show();

    threes.show();
    for i in 0 .. threes.row() {
        for j in 0 .. threes.col() {
            print!("{} ", threes.get_v(i, j));
        }
        println!("");
    }

    let negones = ones.sub(&twos);
    negones.show();
    //negones.neg().show();
    threes.show();
    twos.show();
    ones.show();

    let mut mat = DenseMatrix::<f32>::new(3, 4);
    for i in 0 .. 3 {
        for j in 0 .. 4 {
            mat.set_v(i, j, (i * 4 + j) as f32);
        }
    }

    mat.show();

    let matt = mat.t();
    matt.show();

    //mat.t_mut();
    //mat.show();

    let sevens = ones.scalar_mul(7.0);
    sevens.show();
    sevens.diag().show();
    sevens.tri_down().show();
    sevens.tri_up().show();

    let mut matx = DenseMatrix::<f32>::new(6, 1);
    for i in 0 .. 6 {
        matx.set_v(i, 0, i as f32)
    }
    matx.show();

    //println!("{}", f32::default());

    let matb = ones.dot_mul(&matx);
    matb.show();
    //let mat = matt.dot_multiply(&matx).unwrap();
    //mat.show();

    let eye5 = DenseMatrix::<f32>::eye(5);
    eye5.show();

    DenseMatrix::<f32>::ones(3).show();

    DenseMatrix::<f32>::zeros(1).show();

    let mut rmat = DenseMatrix::<f32>::new(6, 6);
    for i in 0 .. 6 {
        for j in 0 .. 6 {
            rmat.set_v(i, j, rand::random::<f32>());
        }
    }
    rmat.diag().show();
    rmat.tri_up().show();
    rmat.tri_down().show();
    println!("");
    rmat.cofactors(0, 0).show();
    println!("");
    rmat.cofactors(5, 5).show();
    println!("");
    rmat.cofactors(2, 2).show();
    println!("{}", rmat.det_adj());
    rmat.adjoint().show();
    println!("");
    rmat.show();
    println!("");
    let inv = rmat.inv_adj().unwrap();
    inv.show();
    println!("");
    inv.dot_mul(&rmat).show();
    println!("Hello, world!");

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
