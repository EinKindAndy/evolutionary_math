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

    let negones = ones.sub(&twos);
    negones.show();
    negones.neg().show();
    threes.show();
    twos.show();
    ones.show();

    ones.concat(&twos).show();

    let mut mat = DenseMatrix::<f32>::new(3, 4);
    for i in 0 .. 3 {
        for j in 0 .. 4 {
            mat.set_v(i, j, (i * 4 + j) as f32);
        }
    }

    mat.show();

    let matt = mat.t();
    matt.show();

    let sevens = ones.scalar_mul(7.0);
    sevens.show();
    sevens.diag().show();
    sevens.tri_l().show();
    sevens.tri_u().show();

    let nr = 15;
    let mut matx = DenseMatrix::<f32>::new(nr, 1);
    for i in 0 .. nr {
        matx.set_v(i, 0, (i + 1) as f32)
    }
    matx.show();

    let matb = ones.dot_mul(&matx);
    matb.show();

    let eye5 = DenseMatrix::<f32>::eye(5);
    eye5.show();

    DenseMatrix::<f32>::ones(3).show();

    DenseMatrix::<f32>::zeros(1).show();

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
    println!("calculated by GE x_ is");
    xmat.show();
    println!("A * x_ is");
    rmat.dot_mul(&xmat).show();
    println!("A * x_ - x is");
    xmat.sub(&matx).show();
    println!("||A * x_ - x|| is");
    println!("{}", xmat.sub(&matx).norm2());

    rmat.diag().show();
    rmat.tri_u().show();
    rmat.tri_l().show();
    println!("");
    rmat.cofactors(0, 0).show();
    println!("");
    rmat.cofactors(5, 5).show();
    println!("");
    rmat.cofactors(2, 2).show();
    
    println!("");
    //Not recommand: cramer or calculated by adjoint matrix
    //println!("{}", rmat.det_adj());
    //rmat.adjoint().show();
    //println!("inverse by adjoint matrix");
    //let inv = rmat.inv_adj().unwrap();
    //inv.show();
    println!("inverse by GE");
    rmat.inv_ge().unwrap().show();
    println!("");
    rmat.inv_ge().unwrap().dot_mul(&rmat).show();
    println!("{}", rmat.inv_ge().unwrap().dot_mul(&rmat).norm2());
    println!("Hello, world!");

}