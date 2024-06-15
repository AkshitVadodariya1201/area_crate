//! # Basic Area Crate
//! This is a collection of some generally used area function
//!

/// Find the Area
///
/// # Example
/// Area of Circle
/// ```
/// let radius = 12.0;
/// let answer = area_crate::area::circle(radius);
/// assert_eq!(452.38934211696005, answer);
/// ```
/// Area of Triangle
/// ```
/// let height = 20.0;
/// let width = 40.0;
/// let answer = area_crate::area::triangle(height, width);
/// assert_eq!(400.0, answer);
/// ```
/// Area of Square
/// ```
/// let length = 5;
/// let answer =  area_crate::area::square(length);
/// assert_eq!(25, answer);
/// ```
/// Area of Rectangle
/// ```
/// let height = 20;
/// let width = 40;
/// let answer = area_crate::area::rectangle(height, width);
/// assert_eq!(800, answer);
/// ```
///
/// Area of Trapezoid
/// ```
/// let base1 = 10.0;
/// let base2 = 20.0;
/// let height = 30.0;
/// let answer = area_crate::area::trapezoid(base1, base2, height);
/// assert_eq!(450.0, answer);
/// ```
///
/// Area of Parallelogram
/// ```
/// let base = 10.0;
/// let height = 20.0;
/// let answer = area_crate::area::parallelogram(base, height);
/// assert_eq!(200.0, answer);
/// ```
///
/// Area of Ellipse
/// ```
/// let major_axis = 10.0;
/// let minor_axis = 20.0;
/// let answer = area_crate::area::ellipse(major_axis, minor_axis);
/// assert_eq!(628.3185307179999, answer);
/// ```
///
/// Area of Sector
/// ```
/// let radius = 10.0;
/// let angle = 30.0;
/// let answer = area_crate::area::sector(radius, angle);
/// assert_eq!(26.179938779916665, answer);
/// ```
///
/// Area of Rhombus
/// ```
/// let diagonal1 = 10.0;
/// let diagonal2 = 20.0;
/// let answer = area_crate::area::rhombus(diagonal1, diagonal2);
/// assert_eq!(100.0, answer);
/// ```
///
/// Area of Kite
/// ```
/// let diagonal1 = 10.0;
/// let diagonal2 = 20.0;
/// let answer = area_crate::area::kite(diagonal1, diagonal2);
/// assert_eq!(100.0, answer);
/// ```
///
/// Area of Regular Polygon
/// ```
/// let perimeter = 10.0;
/// let apothem = 20.0;
/// let answer = area_crate::area::regular_polygon(perimeter, apothem);
/// assert_eq!(100.0, answer);
/// ```
///
/// Area of Annulus
/// ```
/// let outer_radius = 10.0;
/// let inner_radius = 5.0;
/// let answer = area_crate::area::annulus(outer_radius, inner_radius);
/// assert_eq!(235.61944901925, answer);
/// ```
///
/// # Note
/// This is a collection of some generally used area function
///
///
/// # Limitations
/// This crate is not yet fully implemented
///
/// # Future
/// More functions will be added in future
///
///
/// # Some other section
/// This is a collection of some generally used area function
///
///
pub mod area {
    pub fn circle(radius: f64) -> f64 {
        let pi = 3.14159265359;
        pi * radius * radius
    }

    pub fn triangle(height: f64, width: f64) -> f64 {
        let area = 0.5 * height * width;
        area
    }

    pub fn square(length: i32) -> i32 {
        length * length
    }

    pub fn rectangle(height: i32, width: i32) -> i32 {
        height * width
    }

    pub fn trapezoid(base1: f64, base2: f64, height: f64) -> f64 {
        0.5 * (base1 + base2) * height
    }

    pub fn parallelogram(base: f64, height: f64) -> f64 {
        base * height
    }

    pub fn ellipse(major_axis: f64, minor_axis: f64) -> f64 {
        let pi = 3.14159265359;
        pi * major_axis * minor_axis
    }

    pub fn sector(radius: f64, angle: f64) -> f64 {
        let pi = 3.14159265359;
        0.5 * radius * radius * angle * pi / 180.0
    }

    pub fn rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
        0.5 * diagonal1 * diagonal2
    }

    pub fn kite(diagonal1: f64, diagonal2: f64) -> f64 {
        0.5 * diagonal1 * diagonal2
    }

    pub fn regular_polygon(perimeter: f64, apothem: f64) -> f64 {
        0.5 * perimeter * apothem
    }

    pub fn annulus(outer_radius: f64, inner_radius: f64) -> f64 {
        let pi = 3.14159265359;
        pi * (outer_radius * outer_radius - inner_radius * inner_radius)
    }
}
