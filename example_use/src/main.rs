use csg_surface_distance::{Point, Vector, CSGSurface};

fn main() {
    let point = Point { x: 1.0, y: 2.0, z: 3.0 };
    let vector = Vector { dx: 1.0, dy: 0.0, dz: 0.0 };

    let surface = CSGSurface::Sphere { x: 0.0, y: 0.0, z: 0.0, radius: 1.0 };

    if let Some(distance) = surface.distance_to_surface(&point, &vector) {
        println!("Distance to surface: {}", distance);
    } else {
        println!("No intersection with the surface.");
    }
}