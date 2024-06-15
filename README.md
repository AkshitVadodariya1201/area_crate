# Basic Area Crate

This is a collection of some generally used area functions.

## Find the Area

### Example

#### Area of Circle
```rust
let radius = 12.0;
let answer = area_crate::area::circle(radius);
assert_eq!(452.38934211696005, answer);
```

## Functions

### Circle Area
```rust
pub fn circle(radius: f64) -> f64 {
    let pi = 3.14159265359;
    pi * radius * radius
}
```

### Triangle Area
```rust
pub fn triangle(base: f64, height: f64) -> f64 {
    0.5 * base * height
}
```

### Rectangle Area
```rust
pub fn rectangle(length: f64, breadth: f64) -> f64 {
    length * breadth
}
```

### Square Area
```rust
pub fn square(side: f64) -> f64 {
    side * side
}
```

### Parallelogram Area
```rust
pub fn parallelogram(base: f64, height: f64) -> f64 {
    base * height
}
```

### Trapezoid Area
```rust
pub fn trapezoid(base1: f64, base2: f64, height: f64) -> f64 {
    0.5 * (base1 + base2) * height
}
```

### Ellipse Area
```rust
pub fn ellipse(major_axis: f64, minor_axis: f64) -> f64 {
    let pi = 3.14159265359;
    pi * major_axis * minor_axis
}
```

### Sector Area
```rust
pub fn sector(radius: f64, angle: f64) -> f64 {
    let pi = 3.14159265359;
    0.5 * radius * radius * angle.to_radians()
}
```

### Rhombus Area
```rust
pub fn rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}
```

### Kite Area
```rust
pub fn kite(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}
```

### Regular Polygon Area
```rust
pub fn regular_polygon(perimeter: f64, apothem: f64) -> f64 {
    0.5 * perimeter * apothem
}
```

### Annulus Area
```rust
pub fn annulus(outer_radius: f64, inner_radius: f64) -> f64 {
    let pi = 3.14159265359;
    pi * (outer_radius * outer_radius - inner_radius * inner_radius)
}
```