pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Vector {
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

pub enum CSGSurface {
    Sphere { x: f64, y: f64, z: f64, radius: f64 },
    XPlane { x: f64 },
    YPlane { y: f64 },
    ZPlane { z: f64 },
    Plane { a: f64, b: f64, c: f64, d: f64 }, // ax + by + cz + d = 0
    XAxisCylinder { radius: f64 },
    YAxisCylinder { radius: f64 },
    ZAxisCylinder { radius: f64 },
    XAxisCone { x: f64, y: f64, z: f64, angle: f64 }, // angle in radians
    YAxisCone { x: f64, y: f64, z: f64, angle: f64 }, // angle in radians
    ZAxisCone { x: f64, y: f64, z: f64, angle: f64 }, // angle in radians
    Quadric { a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, j: f64, k: f64 }, // Ax^2 + By^2 + Cz^2 + Dxy + Eyz + Fxz + Gx + Hy + Jz + K = 0
    XAxisTorus { x0: f64, y0: f64, z0: f64, a: f64, b: f64, c: f64 }, // Torus parallel to x-axis
}

impl CSGSurface {
    pub fn distance_to_surface(&self, point: &Point, vector: &Vector) -> Option<f64> {
        match self {
            CSGSurface::Sphere { x, y, z, radius } => {
                let distance_to_center = ((point.x - x).powi(2) + (point.y - y).powi(2) + (point.z - z).powi(2)).sqrt();
                let distance_to_surface = (distance_to_center - radius).abs();
                Some(distance_to_surface)
            }
            CSGSurface::XPlane { x } => {
                if vector.dx == 0.0 {
                    None
                } else {
                    let t = (x - point.x) / vector.dx;
                    if t >= 0.0 {
                        Some(t)
                    } else {
                        None
                    }
                }
            }
            CSGSurface::YPlane { y } => {
                if vector.dy == 0.0 {
                    None
                } else {
                    let t = (y - point.y) / vector.dy;
                    if t >= 0.0 {
                        Some(t)
                    } else {
                        None
                    }
                }
            }
            CSGSurface::ZPlane { z } => {
                if vector.dz == 0.0 {
                    None
                } else {
                    let t = (z - point.z) / vector.dz;
                    if t >= 0.0 {
                        Some(t)
                    } else {
                        None
                    }
                }
            }
            CSGSurface::Plane { a, b, c, d } => {
                let numerator = a * point.x + b * point.y + c * point.z + d;
                let denominator = (a.powi(2) + b.powi(2) + c.powi(2)).sqrt();
                Some((numerator / denominator).abs())
            }
            CSGSurface::XAxisCylinder { radius } => {
                let distance_to_axis = (point.y.powi(2) + point.z.powi(2)).sqrt();
                Some((distance_to_axis - radius).abs())
            }
            CSGSurface::YAxisCylinder { radius } => {
                let distance_to_axis = (point.x.powi(2) + point.z.powi(2)).sqrt();
                Some((distance_to_axis - radius).abs())
            }
            CSGSurface::ZAxisCylinder { radius } => {
                let distance_to_axis = (point.x.powi(2) + point.y.powi(2)).sqrt();
                Some((distance_to_axis - radius).abs())
            }
            CSGSurface::XAxisCone { x, y, z, angle } => {
                let tan_angle = angle.tan();
                let distance_to_apex = ((point.y - y).powi(2) + (point.z - z).powi(2)).sqrt();
                let distance_to_surface = ((point.x - x).abs() - distance_to_apex * tan_angle).abs();
                Some(distance_to_surface)
            }
            CSGSurface::YAxisCone { x, y, z, angle } => {
                let tan_angle = angle.tan();
                let distance_to_apex = ((point.x - x).powi(2) + (point.z - z).powi(2)).sqrt();
                let distance_to_surface = ((point.y - y).abs() - distance_to_apex * tan_angle).abs();
                Some(distance_to_surface)
            }
            CSGSurface::ZAxisCone { x, y, z, angle } => {
                let tan_angle = angle.tan();
                let distance_to_apex = ((point.x - x).powi(2) + (point.y - y).powi(2)).sqrt();
                let distance_to_surface = ((point.z - z).abs() - distance_to_apex * tan_angle).abs();
                Some(distance_to_surface)
            }
            CSGSurface::Quadric { a, b, c, d, e, f, g, h, j, k } => {
                // This is a simplified approach and may not be accurate for all cases
                let value = a * point.x.powi(2) + b * point.y.powi(2) + c * point.z.powi(2) +
                            d * point.x * point.y + e * point.y * point.z + f * point.x * point.z +
                            g * point.x + h * point.y + j * point.z + k;
                Some(value.abs())
            }
            CSGSurface::XAxisTorus { x0, y0, z0, a, b, c } => {
                let dx = point.x - x0;
                let dy = point.y - y0;
                let dz = point.z - z0;
                let distance_to_center = ((dy.powi(2) + dz.powi(2)).sqrt() - a).abs();
                let distance_to_surface = ((distance_to_center.powi(2) + dx.powi(2)).sqrt() - c).abs();
                Some(distance_to_surface)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    fn assert_approx_eq(a: Option<f64>, b: Option<f64>, epsilon: f64) {
        match (a, b) {
            (Some(a), Some(b)) => assert!((a - b).abs() < epsilon, "left: {:?}, right: {:?}", a, b),
            (None, None) => (),
            _ => panic!("left: {:?}, right: {:?}", a, b),
        }
    }

    #[test]
    fn test_distance_to_sphere() {
        let point = Point { x: 2.0, y: 2.0, z: 2.0 };
        let vector = Vector { dx: 0.0, dy: 0.0, dz: 0.0 };
        let surface = CSGSurface::Sphere { x: 1.0, y: 1.0, z: 1.0, radius: 1.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some((3.0_f64).sqrt() - 1.0), EPSILON);
    }

    #[test]
    fn test_distance_to_x_plane() {
        let point = Point { x: 1.0, y: 1.0, z: 1.0 };
        let vector = Vector { dx: 1.0, dy: 0.0, dz: 0.0 };
        let surface = CSGSurface::XPlane { x: 2.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some(1.0), EPSILON);
    }

    #[test]
    fn test_no_intersection_with_x_plane() {
        let point = Point { x: 1.0, y: 1.0, z: 1.0 };
        let vector = Vector { dx: 0.0, dy: 1.0, dz: 0.0 };
        let surface = CSGSurface::XPlane { x: 2.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), None, EPSILON);
    }

    #[test]
    fn test_distance_to_y_plane() {
        let point = Point { x: 1.0, y: 1.0, z: 1.0 };
        let vector = Vector { dx: 0.0, dy: 1.0, dz: 0.0 };
        let surface = CSGSurface::YPlane { y: 2.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some(1.0), EPSILON);
    }

    #[test]
    fn test_no_intersection_with_y_plane() {
        let point = Point { x: 1.0, y: 1.0, z: 1.0 };
        let vector = Vector { dx: 1.0, dy: 0.0, dz: 0.0 };
        let surface = CSGSurface::YPlane { y: 2.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), None, EPSILON);
    }

    #[test]
    fn test_distance_to_z_plane() {
        let point = Point { x: 1.0, y: 1.0, z: 1.0 };
        let vector = Vector { dx: 0.0, dy: 0.0, dz: 1.0 };
        let surface = CSGSurface::ZPlane { z: 2.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some(1.0), EPSILON);
    }

    #[test]
    fn test_no_intersection_with_z_plane() {
        let point = Point { x: 1.0, y: 1.0, z: 1.0 };
        let vector = Vector { dx: 1.0, dy: 0.0, dz: 0.0 };
        let surface = CSGSurface::ZPlane { z: 2.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), None, EPSILON);
    }

    #[test]
    fn test_distance_to_plane() {
        let point = Point { x: 1.0, y: 1.0, z: 1.0 };
        let vector = Vector { dx: 1.0, dy: 1.0, dz: 1.0 };
        let surface = CSGSurface::Plane { a: 1.0, b: 1.0, c: 1.0, d: -3.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some(0.0), EPSILON);
    }

    #[test]
    fn test_distance_to_x_axis_cylinder() {
        let point = Point { x: 1.0, y: 2.0, z: 2.0 };
        let vector = Vector { dx: 1.0, dy: 1.0, dz: 1.0 };
        let surface = CSGSurface::XAxisCylinder { radius: 1.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some((8.0_f64).sqrt() - 1.0), EPSILON);
    }

    #[test]
    fn test_distance_to_y_axis_cylinder() {
        let point = Point { x: 2.0, y: 1.0, z: 2.0 };
        let vector = Vector { dx: 1.0, dy: 1.0, dz: 1.0 };
        let surface = CSGSurface::YAxisCylinder { radius: 1.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some((8.0_f64).sqrt() - 1.0), EPSILON);
    }

    #[test]
    fn test_distance_to_z_axis_cylinder() {
        let point = Point { x: 2.0, y: 2.0, z: 1.0 };
        let vector = Vector { dx: 1.0, dy: 1.0, dz: 1.0 };
        let surface = CSGSurface::ZAxisCylinder { radius: 1.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some((8.0_f64).sqrt() - 1.0), EPSILON);
    }

    #[test]
    fn test_distance_to_x_axis_cone() {
        let point = Point { x: 1.0, y: 2.0, z: 2.0 };
        let vector = Vector { dx: 1.0, dy: 1.0, dz: 1.0 };
        let surface = CSGSurface::XAxisCone { x: 0.0, y: 0.0, z: 0.0, angle: std::f64::consts::FRAC_PI_4 }; // 45 degrees
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some((8.0_f64).sqrt()), EPSILON);
    }

    #[test]
    fn test_distance_to_y_axis_cone() {
        let point = Point { x: 2.0, y: 1.0, z: 2.0 };
        let vector = Vector { dx: 1.0, dy: 1.0, dz: 1.0 };
        let surface = CSGSurface::YAxisCone { x: 0.0, y: 0.0, z: 0.0, angle: std::f64::consts::FRAC_PI_4 }; // 45 degrees
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some((8.0_f64).sqrt()), EPSILON);
    }

    #[test]
    fn test_distance_to_z_axis_cone() {
        let point = Point { x: 2.0, y: 2.0, z: 1.0 };
        let vector = Vector { dx: 1.0, dy: 1.0, dz: 1.0 };
        let surface = CSGSurface::ZAxisCone { x: 0.0, y: 0.0, z: 0.0, angle: std::f64::consts::FRAC_PI_4 }; // 45 degrees
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some((8.0_f64).sqrt()), EPSILON);
    }

    #[test]
    fn test_distance_to_quadric() {
        let point = Point { x: 1.0, y: 1.0, z: 1.0 };
        let vector = Vector { dx: 1.0, dy: 1.0, dz: 1.0 };
        let surface = CSGSurface::Quadric { a: 1.0, b: 1.0, c: 1.0, d: 0.0, e: 0.0, f: 0.0, g: 0.0, h: 0.0, j: 0.0, k: -3.0 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some(0.0), EPSILON);
    }

    #[test]
    fn test_distance_to_x_axis_torus() {
        let point = Point { x: 2.0, y: 2.0, z: 2.0 };
        let vector = Vector { dx: 1.0, dy: 1.0, dz: 1.0 };
        let surface = CSGSurface::XAxisTorus { x0: 0.0, y0: 0.0, z0: 0.0, a: 1.0, b: 0.5, c: 0.5 };
        assert_approx_eq(surface.distance_to_surface(&point, &vector), Some((2.0_f64).sqrt() - 0.5), EPSILON);
    }
}