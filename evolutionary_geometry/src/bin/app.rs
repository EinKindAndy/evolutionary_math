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