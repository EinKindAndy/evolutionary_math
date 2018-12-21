use super::point::{Point3D, Vector3D};

#[allow(dead_code)]
pub struct Hexahedron {
    pub p0: Point3D,
    p1: Point3D,
    p2: Point3D,
    p3: Point3D,
    p4: Point3D,
    p5: Point3D,
    p6: Point3D,
    p7: Point3D,
    p8: Point3D,
    pub f_t: f32,
    pub f_b: f32,
    pub f_n: f32,
    pub f_s: f32,
    pub f_w: f32,
    pub f_e: f32,
    pub vol: f32,
    pub nv_t: Vector3D,
    pub nv_b: Vector3D,
    pub nv_n: Vector3D,
    pub nv_s: Vector3D,
    pub nv_w: Vector3D,
    pub nv_e: Vector3D,
}

fn calc_si(p1: Point3D, p2: Point3D, 
           p3: Point3D, p4: Point3D) -> Vector3D {
    // 1->1; 5->2; 8->3; 4->4
    let dxa = p3.x1 - p1.x1;
    let dxb = p2.x1 - p4.x1;
    let dya = p3.x2 - p1.x2;
    let dyb = p2.x2 - p4.x2;
    let dza = p3.x3 - p1.x3;
    let dzb = p2.x3 - p4.x3;
    Vector3D { v1: (dza*dyb - dya*dzb) / 2.0, v2: (dxa*dzb - dza*dxb) / 2.0, v3: (dya*dxb - dxa*dyb) / 2.0 }
}

fn calc_rmid(p1: Point3D, p2: Point3D, 
             p3: Point3D, p4: Point3D) -> Vector3D {
    let p0 = Point3D::new(0.0, 0.0, 0.0);
    let v1 = Vector3D::from_points(p0, p1);
    let v2 = Vector3D::from_points(p0, p2);
    let v3 = Vector3D::from_points(p0, p3);
    let v4 = Vector3D::from_points(p0, p4);
    Vector3D { v1: (v1.v1 + v2.v1 + v3.v1 + v4.v1) / 4.0, 
               v2: (v1.v2 + v2.v2 + v3.v2 + v4.v2) / 4.0, 
               v3: (v1.v3 + v2.v3 + v3.v3 + v4.v3) / 4.0 }
}

impl Hexahedron {
    pub fn new(p1: Point3D,
               p2: Point3D,
               p3: Point3D,
               p4: Point3D,
               p5: Point3D,
               p6: Point3D,
               p7: Point3D,
               p8: Point3D) -> Hexahedron {
        let p0 = Point3D { 
            x1: (p1.x1 + p2.x1 + p3.x1 + p4.x1 + p5.x1 + p6.x1 + p7.x1 + p8.x1) / 8.0, 
            x2: (p1.x2 + p2.x2 + p3.x2 + p4.x2 + p5.x2 + p6.x2 + p7.x2 + p8.x2) / 8.0,
            x3: (p1.x3 + p2.x3 + p3.x3 + p4.x3 + p5.x3 + p6.x3 + p7.x3 + p8.x3) / 8.0 };
        let st = calc_si(p5, p6, p7, p8);
        let sb = calc_si(p1, p2, p3, p4);
        let ss = calc_si(p1, p5, p8, p4);
        let se = calc_si(p1, p2, p6, p5);
        let sn = calc_si(p2, p6, p7, p3);
        let sw = calc_si(p4, p3, p7, p8);
        let f_t = st.norm2();
        let f_b = sb.norm2();
        let f_s = ss.norm2();
        let f_e = se.norm2();
        let f_n = sn.norm2();
        let f_w = sw.norm2();
        let nv_t = Vector3D::from_zoom(st, 1.0 / f_t);
        let nv_b = Vector3D::from_zoom(sb, 1.0 / f_b);
        let nv_s = Vector3D::from_zoom(ss, 1.0 / f_s);
        let nv_e = Vector3D::from_zoom(se, 1.0 / f_e);
        let nv_n = Vector3D::from_zoom(sn, 1.0 / f_n);
        let nv_w = Vector3D::from_zoom(sw, 1.0 / f_w);
        let rt = calc_rmid(p5, p6, p7, p8);
        let rb = calc_rmid(p1, p2, p3, p4);
        let rs = calc_rmid(p1, p5, p8, p4);
        let re = calc_rmid(p1, p2, p6, p5);
        let rn = calc_rmid(p2, p6, p7, p3);
        let rw = calc_rmid(p4, p3, p7, p8);
        let vol = rt.multiply_vec(nv_t) * f_t +
                  rb.multiply_vec(nv_b) * f_b +
                  rs.multiply_vec(nv_s) * f_s +
                  re.multiply_vec(nv_e) * f_e +
                  rn.multiply_vec(nv_n) * f_n +
                  rw.multiply_vec(nv_w) * f_w;
        Hexahedron {
            p0: p0,
            p1: p1,
            p2: p2,
            p3: p3,
            p4: p4,
            p5: p5,
            p6: p6,
            p7: p7,
            p8: p8,
            f_t: f_t,
            f_b: f_b,
            f_n: f_n,
            f_s: f_s,
            f_w: f_w,
            f_e: f_e,
            vol: vol,
            nv_t: nv_t,
            nv_b: nv_b,
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
        println!("p5: {:?}", self.p5);
        println!("p6: {:?}", self.p6);
        println!("p7: {:?}", self.p7);
        println!("p8: {:?}", self.p8);
        println!("f_t: {}", self.f_t);
        println!("f_b: {}", self.f_b);
        println!("f_n: {}", self.f_n);
        println!("f_s: {}", self.f_s);
        println!("f_w: {}", self.f_w);
        println!("f_e: {}", self.f_e);
        println!("nv_t: {:?}", self.nv_t);
        println!("nv_b: {:?}", self.nv_b);
        println!("nv_n: {:?}", self.nv_n);
        println!("nv_s: {:?}", self.nv_s);
        println!("nv_w: {:?}", self.nv_w);
        println!("nv_e: {:?}", self.nv_e);
        println!("vol: {}", self.vol);
    }

}
