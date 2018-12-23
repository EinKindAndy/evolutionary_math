use rand::prelude::*;
use evolutionary_algebra::linear_algebra::matrix::dense::DenseMatrix;

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

}