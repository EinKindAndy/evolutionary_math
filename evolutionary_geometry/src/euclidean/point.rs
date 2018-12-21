
#[derive(Debug, Copy, Clone)]
pub struct Point2D {
    pub x1: f32,
    pub x2: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Point3D {
    pub x1: f32,
    pub x2: f32,
    pub x3: f32,
}

impl Point2D {
    pub fn new(x1: f32, x2: f32) -> Point2D {
        Point2D { x1: x1, x2: x2 }
    }

}

impl Point3D {
    pub fn new(x1: f32, x2: f32, x3: f32) -> Point3D {
        Point3D { x1: x1, x2: x2, x3: x3 }
    }

}

#[derive(Debug, Copy, Clone)]
pub struct Vector2D {
    pub v1: f32,
    pub v2: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3D {
    pub v1: f32,
    pub v2: f32,
    pub v3: f32,
}

impl Vector2D {
    pub fn new(v1: f32, v2: f32) -> Vector2D {
        Vector2D { v1: v1, v2: v2 }
    }

    pub fn from_points(sp: Point2D, ep: Point2D) -> Vector2D {
        Vector2D { v1: ep.x1 - sp.x1, v2: ep.x2 - sp.x2 }
    }

    pub fn from_zoom(v: Vector2D, scale: f32) -> Vector2D {
        Vector2D { v1: v.v1 * scale, v2: v.v2 * scale }
    }

    pub fn norm2(&self) -> f32 {
        f32::sqrt(self.v1 * self.v1 + self.v2 * self.v2)
    }
}


impl Vector3D {
    pub fn new(v1: f32, v2: f32, v3: f32) -> Vector3D {
        Vector3D { v1: v1, v2: v2, v3: v3 }
    }

    pub fn from_points(sp: Point3D, ep: Point3D) -> Vector3D {
        Vector3D { v1: ep.x1 - sp.x1, v2: ep.x2 - sp.x2, v3: ep.x3 - sp.x3 }
    }

    pub fn norm2(&self) -> f32 {
        f32::sqrt(self.v1 * self.v1 + self.v2 * self.v2 + self.v3 * self.v3)
    }

    pub fn from_zoom(v: Vector3D, scale: f32) -> Vector3D {
        Vector3D { v1: v.v1 * scale, v2: v.v2 * scale, v3: v.v3 * scale }
    }

    pub fn multiply_vec(&self, v: Vector3D) -> f32 {
        self.v1 * v.v1 + self.v2 * v.v2 + self.v3 * v.v3
    }
}

#[test]
fn point_test() {
    let p2d = Point2D::new(0.0, 1.0);
    assert_eq!(p2d.x1, 0.0);
    assert_eq!(p2d.x2, 1.0);
    let p3d = Point3D::new(0.0, 1.1, 2.2);
    assert_eq!(p3d.x1, 0.0);
    assert_eq!(p3d.x2, 1.1);
    assert_eq!(p3d.x3, 2.2);
}


#[test]
fn vector_test() {
    let v2d = Vector2D::new(0.0, 1.0);
    assert_eq!(v2d.v1, 0.0);
    assert_eq!(v2d.v2, 1.0);
    let v3d = Vector3D::new(0.0, 1.1, 2.2);
    assert_eq!(v3d.v1, 0.0);
    assert_eq!(v3d.v2, 1.1);
    assert_eq!(v3d.v3, 2.2);

    let sp2d = Point2D::new(0.0, 0.0);
    let ep2d = Point2D::new(3.0, 4.0);
    let v2 = Vector2D::from_zoom(Vector2D::from_points(sp2d, ep2d), 1.0);
    assert_eq!(v2.v1, 3.0);
    assert_eq!(v2.v2, 4.0);
    assert_eq!(v2.norm2(), 5.0);

    let sp3d = Point3D::new(0.0, 0.0, 0.0);
    let ep3d = Point3D::new(3.0, 4.0, 0.0);
    let v3 = Vector3D::from_zoom(Vector3D::from_points(sp3d, ep3d), 1.0);
    assert_eq!(v3.v1, 3.0);
    assert_eq!(v3.v2, 4.0);
    assert_eq!(v3.v3, 0.0);
    assert_eq!(v3.norm2(), 5.0);
}