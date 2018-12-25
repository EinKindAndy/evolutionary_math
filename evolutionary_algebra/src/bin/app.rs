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