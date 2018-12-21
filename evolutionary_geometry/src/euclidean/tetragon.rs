use super::point::{Point2D, Vector2D};

#[allow(dead_code)]
pub struct Tetragon {
    pub p0: Point2D,
    p1: Point2D,
    p2: Point2D,
    p3: Point2D,
    p4: Point2D,
    pub f_n: f32,
    pub f_s: f32,
    pub f_w: f32,
    pub f_e: f32,
    pub vol: f32,
    pub nv_n: Vector2D,
    pub nv_s: Vector2D,
    pub nv_w: Vector2D,
    pub nv_e: Vector2D,
}

fn calc_si(p1: Point2D, p2: Point2D, scale: f32) -> Vector2D {
    Vector2D::from_zoom(Vector2D { v1: p2.x2 - p1.x2, v2: p1.x1 - p2.x1 }, scale)
}

impl Tetragon {
    pub fn new(p1: Point2D,
               p2: Point2D,
               p3: Point2D,
               p4: Point2D) -> Tetragon {
        let p0 = Point2D { x1: (p1.x1 + p2.x1 + p3.x1 + p4.x1) / 4.0, x2: (p1.x2 + p2.x2 + p3.x2 + p4.x2) / 4.0 };
        let ss = calc_si(p1, p2, 1.0);
        let se = calc_si(p2, p3, 1.0);
        let sn = calc_si(p3, p4, 1.0);
        let sw = calc_si(p4, p1, 1.0);
        let f_s = ss.norm2();
        let f_e = se.norm2();
        let f_n = sn.norm2();
        let f_w = sw.norm2();
        let nv_s = Vector2D::from_zoom(ss, 1.0 / f_s);
        let nv_e = Vector2D::from_zoom(se, 1.0 / f_e);
        let nv_n = Vector2D::from_zoom(sn, 1.0 / f_n);
        let nv_w = Vector2D::from_zoom(sw, 1.0 / f_w);
        Tetragon {
            p0: p0,
            p1: p1,
            p2: p2,
            p3: p3,
            p4: p4,
            f_n: f_n,
            f_s: f_s,
            f_w: f_w,
            f_e: f_e,
            vol: ((p1.x1 - p3.x1)*(p2.x2 - p4.x2) + (p4.x1 - p2.x1)*(p1.x2 - p3.x2)) / 2.0,
            nv_n: nv_n,
            nv_s: nv_s,
            nv_w: nv_w,
            nv_e: nv_e,
        }
    }

    pub fn show(&self) {
        println!("p0: {:?}", self.p0);
        println!("p1: {:?}", self.p1);
        println!("p2: {:?}", self.p2);
        println!("p3: {:?}", self.p3);
        println!("p4: {:?}", self.p4);
        println!("f_n: {}", self.f_n);
        println!("f_s: {}", self.f_s);
        println!("f_w: {}", self.f_w);
        println!("f_e: {}", self.f_e);
        println!("nv_n: {:?}", self.nv_n);
        println!("nv_s: {:?}", self.nv_s);
        println!("nv_w: {:?}", self.nv_w);
        println!("nv_e: {:?}", self.nv_e);
        println!("vol: {}", self.vol);
    }

}


#[test]
fn tetragon_test() {
    let p1 = Point2D::new(0.0, 0.0);
    let p2 = Point2D::new(2.0, 0.0);
    let p3 = Point2D::new(2.0, 2.0);
    let p4 = Point2D::new(0.0, 2.0);
    let tetragon = Tetragon::new(p1, p2, p3, p4);
    tetragon.show();
    assert_eq!(2 + 2, 4);
}