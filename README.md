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
    
    DenseMatrix::<f32>::eye(5).show();

    DenseMatrix::<f32>::ones(3).show();

    DenseMatrix::<f32>::zeros(1).show();

    //Not recommend: cramer's rule or calculated by the adjoint matrix
    let cnr = 7;
    let mut cmat = DenseMatrix::<f32>::new(cnr, cnr);
    for i in 0 .. cnr {
        for j in 0 .. cnr {
            cmat.set_v(i, j, rand::random::<f32>());
        }
    }
    cmat.diag().show();
    cmat.tri_u().show();
    cmat.tri_l().show();
    println!("the cofactor when the indices are (0, 0)");
    cmat.cofactors(0, 0).show();
    println!("the cofactor when the indices are (5, 5)");
    cmat.cofactors(5, 5).show();
    println!("the cofactor when the indices are (2, 2)");
    cmat.cofactors(2, 2).show();
    println!("adjoint(A) is ");
    cmat.adjoint().show();
    println!("det(A) is {} by cramer's rule", cmat.det_adj());
    println!("det(A) is {} by GE", cmat.det_ge());
    println!("inverse(A) by adjoint matrix");
    cmat.inv_adj().unwrap().show();
    println!("inverse(A) by GE");
    cmat.inv_ge().unwrap().show();

    let nr = 10; // = 100;
    let mut matx = DenseMatrix::<f32>::new(nr, 1);
    for i in 0 .. nr {
        matx.set_v(i, 0, (i + 1) as f32)
    }

    let mut rmat = DenseMatrix::<f32>::new(nr, nr);
    for i in 0 .. nr {
        for j in 0 .. nr {
            rmat.set_v(i, j, rand::random::<f32>());
        }
    }
    println!("A =");
    rmat.show();
    let bmat = rmat.dot_mul(&matx);
    println!("b =");
    bmat.show();
    println!("real x is");
    matx.show();
    let xmat = DenseMatrix::solve_ge(&rmat, &bmat).unwrap();
    println!("x_ calculated by GE is");
    xmat.show();
    println!("||A * x_ - x|| is");
    println!("{}", xmat.sub(&matx).norm2());

    println!("trace(inv(A) * A)  by GE is {}", rmat.inv_ge().unwrap().dot_mul(&rmat).trace());
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
