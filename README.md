# csg_surface_distance

A Rust Cargo crate for finding the distance between a point and a constructive solid geometry (CSG) surface

## Supported Surfaces

csg_surface_distance supports the following types of surfaces:

1. **Sphere**
   - Defined by its center coordinates `(x, y, z)` and radius.
   - Example:
     ```rust
     CSGSurface::Sphere { x: 0.0, y: 0.0, z: 0.0, radius: 1.0 }
     ```

2. **XPlane**
   - A plane perpendicular to the x-axis.
   - Defined by the x-coordinate of the plane.
   - Example:
     ```rust
     CSGSurface::XPlane { x: 2.0 }
     ```

3. **YPlane**
   - A plane perpendicular to the y-axis.
   - Defined by the y-coordinate of the plane.
   - Example:
     ```rust
     CSGSurface::YPlane { y: 2.0 }
     ```

4. **ZPlane**
   - A plane perpendicular to the z-axis.
   - Defined by the z-coordinate of the plane.
   - Example:
     ```rust
     CSGSurface::ZPlane { z: 2.0 }
     ```

5. **Plane**
   - A general plane defined by the equation `ax + by + cz + d = 0`.
   - Example:
     ```rust
     CSGSurface::Plane { a: 1.0, b: 1.0, c: 1.0, d: -3.0 }
     ```

6. **XAxisCylinder**
   - A cylinder aligned along the x-axis.
   - Defined by its center coordinates `(y, z)` and radius.
   - Example:
     ```rust
     CSGSurface::XAxisCylinder { y: 0.0, z: 0.0, radius: 1.0 }
     ```

7. **YAxisCylinder**
   - A cylinder aligned along the y-axis.
   - Defined by its center coordinates `(x, z)` and radius.
   - Example:
     ```rust
     CSGSurface::YAxisCylinder { x: 0.0, z: 0.0, radius: 1.0 }
     ```

8. **ZAxisCylinder**
   - A cylinder aligned along the z-axis.
   - Defined by its center coordinates `(x, y)` and radius.
   - Example:
     ```rust
     CSGSurface::ZAxisCylinder { x: 0.0, y: 0.0, radius: 1.0 }
     ```

9. **Quadric**
   - A general quadric surface defined by the equation `Ax^2 + By^2 + Cz^2 + Dxy + Eyz + Fxz + Gx + Hy + Jz + K = 0`.
   - Example:
     ```rust
     CSGSurface::Quadric { a: 1.0, b: 1.0, c: 1.0, d: 0.0, e: 0.0, f: 0.0, g: 0.0, h: 0.0, j: 0.0, k: -3.0 }
     ```

## Usage

To use the `csg_surface_distance` package, add it to your `Cargo.toml`:

```toml
[dependencies]
csg_surface_distance = "0.1.0"  # Replace with the actual version