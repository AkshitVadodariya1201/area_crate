# Area Crate v1.0.0

A comprehensive Rust library for geometric calculations, providing functions for calculating areas, perimeters, surface areas, and volumes of various shapes.

## Features

- 2D shape area calculations (circle, triangle, square, rectangle, trapezoid, parallelogram, ellipse, etc.)
- 3D shape surface area calculations (cube, sphere, cylinder, cone)
- 3D shape volume calculations
- Perimeter calculations for 2D shapes
- Error handling for invalid inputs
- High-precision mathematical constants
- Well-documented API
- Zero dependencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
area_crate = "1.0.0"
```

## Usage Examples

### Area Calculations

```rust
use area_crate::{area, GeometryError};

fn main() -> Result<(), GeometryError> {
    // Calculate circle area
    let radius = 12.0;
    let circle_area = area::circle(radius)?;
    assert_eq!(452.3893421169302, circle_area);

    // Calculate pentagon area
    let pentagon_area = area::pentagon(10.0)?;
    println!("Pentagon area: {}", pentagon_area);

    // Calculate polygon area (hexagon)
    let hexagon_area = area::polygon(6, 10.0)?;
    println!("Hexagon area: {}", hexagon_area);

    Ok(())
}
```

### Surface Area and Volume Calculations

```rust
use area_crate::{surface_area, volume, GeometryError};

fn main() -> Result<(), GeometryError> {
    // Calculate sphere surface area
    let radius = 5.0;
    let surface = surface_area::sphere(radius)?;
    println!("Sphere surface area: {}", surface);

    // Calculate cylinder volume
    let height = 10.0;
    let volume = volume::cylinder(radius, height)?;
    println!("Cylinder volume: {}", volume);

    Ok(())
}
```

### Using Constants

```rust
use area_crate::constants::{PI, TAU, E};

fn main() {
    println!("π = {}", PI);
    println!("τ = {}", TAU);
    println!("e = {}", E);
}
```

## Available Functions

### Areas (2D)

- Circle
- Triangle
- Square
- Rectangle
- Trapezoid
- Parallelogram
- Ellipse
- Sector
- Rhombus
- Kite
- Regular polygon
- Annulus
- Pentagon
- Hexagon
- N-sided polygon
- Circular segment

### Perimeters

- Circle
- Rectangle
- Triangle
- Regular polygon

### Surface Areas (3D)

- Cube
- Sphere
- Cylinder
- Cone

### Volumes (3D)

- Cube
- Sphere
- Cylinder
- Cone
- Rectangular prism

## Error Handling

All functions now return a `Result` type, properly handling invalid inputs:

```rust
use area_crate::area;

fn main() {
    // This will return an error because radius cannot be negative
    let result = area::circle(-5.0);
    assert!(result.is_err());
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details
