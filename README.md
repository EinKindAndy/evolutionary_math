# evolutionary_math
It's a tiny project to practice my rust programming skills. I hope to learn something about Rust. More importantly, I'm going to implement most numerical algorithms by myself to enhence my experiences.

## evolutionary_algebra
### linear_algebra
#### the dense matrix and the sparse matrix and solvers for linear systems
It's well known that the matrix computation to solve a linear system plays an important role in almost all numerical regions, as at the final stage, the normal form of Ax=b represents the initial mathematical model. Sometimes, it's approximating the nonlinear linearly. Or it's a real relation between variables.
In practical problems, the matrix is large but sparse, so we usually store it in a sparse way. And different problems have different suitable solvers respectively.
Direct methods are more stable and accurate in terms of the smaller matrix. The iterative method is usually the only choice for large sparse matrix solving. Sometimes, it's good that the matrix is hermitian and defined. However, sometimes, it's not! There is no special method that is well performed in all situations. Hence, beyond the direct method, we have some stationary iterative methods (to recommend SOR) when matrices are diagonally dominant, and some Krylov-based methods (GMRES or GCR and their variants) for more general cases.
```
//baisc matrix operations
//operations of dense matrices at the current stage
use rand::prelude::*;
use evolutionary_algebra::linear_algebra::matrix::dense::DenseMatrix;
use evolutionary_algebra::linear_algebra::matrix::sparse::SparseMatrix;

fn main() {
    
    DenseMatrix::<f32>::eye(5).show();

    DenseMatrix::<f32>::ones(3).show();

    DenseMatrix::<f32>::zeros(1).show();

    //Not recommand: cramer's rule or calculated by adjoint matrix
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

    SparseMatrix::<f32>::eye(5).show();

    SparseMatrix::<f32>::ones(3).show();

    SparseMatrix::<f32>::zeros(1).show();

    //Not recommand: cramer's rule or calculated by adjoint matrix
    let snr = 7;
    let mut smat = SparseMatrix::<f32>::new(snr, snr);
    for i in 0 .. snr {
        for j in 0 .. snr {
            smat.set_v(i, j, rand::random::<f32>());
        }
    }
    smat.diag().show();
    smat.tri_u().show();
    smat.tri_l().show();
    println!("the cofactor when the indices are (0, 0)");
    smat.cofactors(0, 0).show();
    println!("the cofactor when the indices are (5, 5)");
    smat.cofactors(5, 5).show();
    println!("the cofactor when the indices are (2, 2)");
    smat.cofactors(2, 2).show();
    println!("adjoint(A) is ");
    smat.adjoint().show();
    println!("det(A) is {} by cramer's rule", smat.det_adj());
    println!("det(A) is {} by GE", smat.det_ge());
    println!("inverse(A) by adjoint matrix");
    smat.inv_adj().unwrap().show();
    println!("inverse(A) by GE");
    smat.inv_ge().unwrap().show();

    let sr = 100; // = 100;
    let mut smatx = SparseMatrix::<f32>::new(sr, 1);
    for i in 0 .. sr {
        smatx.set_v(i, 0, (i + 1) as f32)
    }

    let mut srmat = SparseMatrix::<f32>::new(sr, sr);
    for i in 0 .. sr {
        for j in 0 .. sr {
            srmat.set_v(i, j, rand::random::<f32>());
        }
    }
    // make it diagonally dominant to test the SOR method!
    srmat = srmat.add(&SparseMatrix::eye(sr).scalar_mul(2.0 * sr as f32));
    println!("A =");
    srmat.show();
    let sbmat = srmat.dot_mul(&smatx);
    println!("b =");
    sbmat.show();
    println!("real x is");
    smatx.show();
    let sxmat = SparseMatrix::solve_ge(&srmat, &sbmat).unwrap();
    println!("x_ calculated by GE is");
    sxmat.show();
    println!("x_ calculated by SOR is");
    let sor_x = SparseMatrix::solve_sor(&srmat, &sbmat, 0.95, 10);
    sor_x.show();
    println!("x_ calculated by GMRES is");
    let gmres_x = SparseMatrix::solve_gmres(&srmat, &sbmat, 5, 100);
    gmres_x.show();
    println!("(GE) ||A * x_ - x|| is");
    println!("{}", sxmat.sub(&smatx).norm2());
    println!("(SOR) ||A * x_ - x|| is");
    println!("{}", sor_x.sub(&smatx).norm2());
    println!("(GMRES) ||A * x_ - x|| is");
    println!("{}", gmres_x.sub(&smatx).norm2());
    println!("trace(inv(A) * A)  by GE is {}", srmat.inv_ge().unwrap().dot_mul(&srmat).trace());

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
