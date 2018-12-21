extern crate evolutionary_geometry;

use evolutionary_geometry::euclidean::{Point2D, Point3D, Vector2D, Vector3D, Hexahedron, Tetragon};

use std::collections::HashMap;

#[allow(dead_code)]
pub struct TetragonCell {
    id: usize,
    nn_id: Option<usize>,
    ns_id: Option<usize>,
    nw_id: Option<usize>,
    ne_id: Option<usize>,
    dim: usize,
    dim_values: Vec<f32>,
    geometry: Tetragon,
}

#[allow(dead_code)]
pub struct HexahedronCell {
    id: usize,
    nt_id: Option<usize>,
    nb_id: Option<usize>,
    nn_id: Option<usize>,
    ns_id: Option<usize>,
    nw_id: Option<usize>,
    ne_id: Option<usize>,
    dim: usize,
    dim_values: Vec<f32>,
    geometry: Hexahedron,
}

impl TetragonCell {
    pub fn new(dim: usize, 
               id: usize,
               ne_id: Option<usize>,
               nw_id: Option<usize>,
               nn_id: Option<usize>,
               ns_id: Option<usize>,
               p1: Point2D, p2: Point2D,
               p3: Point2D, p4: Point2D) -> TetragonCell
    {
        let mut values = Vec::with_capacity(dim);
        values.resize(dim, 0.0);
        TetragonCell {
            id: id,
            nn_id: nn_id,
            ns_id: ns_id,
            nw_id: nw_id,
            ne_id: ne_id,
            dim: dim,
            dim_values: values,
            geometry: Tetragon::new(p1, p2, p3, p4),
        }
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn update_val(&mut self, v: f32, at: usize) {
        self.dim_values[at] = v;
    }

    pub fn get_val(&self, at: usize) -> f32 {
        self.dim_values[at]
    }

    pub fn nn(&self) -> Option<usize> {
        self.nn_id
    }

    pub fn ns(&self) -> Option<usize> {
        self.ns_id
    }

    pub fn nw(&self) -> Option<usize> {
        self.nw_id
    }

    pub fn ne(&self) -> Option<usize> {
        self.ne_id
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn volume(&self) -> f32 {
        self.geometry.vol
    }

    pub fn face_n(&self) -> f32 {
        self.geometry.f_n
    }

    pub fn face_s(&self) -> f32 {
        self.geometry.f_s
    }

    pub fn face_w(&self) -> f32 {
        self.geometry.f_w
    }

    pub fn face_e(&self) -> f32 {
        self.geometry.f_e
    }

    pub fn inner_p(&self) -> Point2D {
        self.geometry.p0
    }

    pub fn nvec_n(&self) -> Vector2D {
        self.geometry.nv_n
    }

    pub fn nvec_s(&self) -> Vector2D {
        self.geometry.nv_s
    }

    pub fn nvec_w(&self) -> Vector2D {
        self.geometry.nv_w
    }

    pub fn nvec_e(&self) -> Vector2D {
        self.geometry.nv_e
    }

}

#[test]
fn tetragon_cell_test() {
    let p1 = Point2D::new(0.0, 0.0);
    let p2 = Point2D::new(2.0, 0.0);
    let p3 = Point2D::new(2.0, 2.0);
    let p4 = Point2D::new(0.0, 2.0);
    let mut cell = TetragonCell::new(3, 0,
                                 None, None, None, None,
                                 p1, p2, p3, p4,);
    assert!(cell.volume() == 4.0);
    assert!(cell.dim() == 3);

    cell.update_val(0.5, 2);
    assert!(cell.get_val(2) == 0.5);
}


impl HexahedronCell {
    pub fn new(dim: usize, 
               id: usize,
               ne_id: Option<usize>,
               nw_id: Option<usize>,
               nn_id: Option<usize>,
               ns_id: Option<usize>,
               nt_id: Option<usize>,
               nb_id: Option<usize>,
               p1: Point3D, p2: Point3D, p3: Point3D, p4: Point3D,
               p5: Point3D, p6: Point3D, p7: Point3D, p8: Point3D) -> HexahedronCell
    {
        let mut values = Vec::with_capacity(dim);
        values.resize(dim, 0.0);
        HexahedronCell {
            id: id,
            nn_id: nn_id,
            ns_id: ns_id,
            nw_id: nw_id,
            ne_id: ne_id,
            nt_id: nt_id,
            nb_id: nb_id,
            dim: dim,
            dim_values: values,
            geometry: Hexahedron::new(p1, p2, p3, p4, p5, p6, p7, p8),
        }
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn update_val(&mut self, v: f32, at: usize) {
        self.dim_values[at] = v;
    }

    pub fn get_val(&self, at: usize) -> f32 {
        self.dim_values[at]
    }

    pub fn nn(&self) -> Option<usize> {
        self.nn_id
    }

    pub fn ns(&self) -> Option<usize> {
        self.ns_id
    }

    pub fn nw(&self) -> Option<usize> {
        self.nw_id
    }

    pub fn ne(&self) -> Option<usize> {
        self.ne_id
    }

    pub fn nt(&self) -> Option<usize> {
        self.nt_id
    }

    pub fn nb(&self) -> Option<usize> {
        self.nb_id
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn volume(&self) -> f32 {
        self.geometry.vol
    }

    pub fn face_n(&self) -> f32 {
        self.geometry.f_n
    }

    pub fn face_s(&self) -> f32 {
        self.geometry.f_s
    }

    pub fn face_w(&self) -> f32 {
        self.geometry.f_w
    }

    pub fn face_e(&self) -> f32 {
        self.geometry.f_e
    }

    pub fn face_t(&self) -> f32 {
        self.geometry.f_t
    }

    pub fn face_b(&self) -> f32 {
        self.geometry.f_b
    }

    pub fn inner_p(&self) -> Point3D {
        self.geometry.p0
    }

    pub fn nvec_n(&self) -> Vector3D {
        self.geometry.nv_n
    }

    pub fn nvec_s(&self) -> Vector3D {
        self.geometry.nv_s
    }

    pub fn nvec_w(&self) -> Vector3D {
        self.geometry.nv_w
    }

    pub fn nvec_e(&self) -> Vector3D {
        self.geometry.nv_e
    }

    pub fn nvec_t(&self) -> Vector3D {
        self.geometry.nv_t
    }

    pub fn nvec_b(&self) -> Vector3D {
        self.geometry.nv_b
    }

}

#[test]
fn hexahedron_cell_test() {
    let p_1 = Point3D::new(2.0, 0.0, 0.0);
    let p_2 = Point3D::new(2.0, 2.0, 0.0);
    let p_3 = Point3D::new(0.0, 2.0, 0.0);
    let p_4 = Point3D::new(0.0, 0.0, 0.0);
    let p_5 = Point3D::new(2.0, 0.0, 2.0);
    let p_6 = Point3D::new(2.0, 2.0, 2.0);
    let p_7 = Point3D::new(0.0, 2.0, 2.0);
    let p_8 = Point3D::new(0.0, 0.0, 2.0);

    let mut cell = HexahedronCell::new(4, 0,
                                   None, None, None, None, None, None,
                                   p_1, p_2, p_3, p_4,
                                   p_5, p_6, p_7, p_8);
    assert!(cell.volume() == 8.0);
    assert!(cell.dim() == 4);

    cell.update_val(0.5, 2);
    assert!(cell.get_val(2) == 0.5);
}

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