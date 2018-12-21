
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

    let eyes = DenseMatrix::<f32>::eyes(5);
    eyes.show();

    DenseMatrix::<f32>::ones(3).show();

    DenseMatrix::<f32>::zeros(1).show();
    
    println!("Hello, world!");

}