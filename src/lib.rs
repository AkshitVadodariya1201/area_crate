//! # Basic Area Crate
//! This is a collection of some generally used area function
//!

/// This crate provides functions for calculating areas, perimeters, and volumes
/// of various geometric shapes.
///
/// # Features
/// - 2D shape area calculations
/// - 3D shape surface area and volume calculations
/// - Perimeter calculations
/// - Mathematical constants
/// - Generic type support
/// - Error handling for invalid inputs

pub mod constants {
    /// Mathematical constant π (pi) with high precision
    pub const PI: f64 = std::f64::consts::PI;

    /// Mathematical constant τ (tau) = 2π
    pub const TAU: f64 = std::f64::consts::TAU;

    /// Mathematical constant e (euler's number)
    pub const E: f64 = std::f64::consts::E;
}

// Error type for geometric calculations
#[derive(Debug)]
pub enum GeometryError {
    /// Input value is negative when it should be positive
    NegativeValue,
    /// Input value is zero when it should be non-zero
    ZeroValue,
    /// Invalid angle value
    InvalidAngle,
    /// Other general validation errors
    InvalidInput(String),
}

/// Module containing area calculation functions
pub mod area {
    use super::{constants::PI, GeometryError};

    /// Calculate the area of various geometric shapes.
    ///
    /// This module provides functions to calculate areas of common 2D geometric shapes.
    /// All functions perform input validation and return Result<f64, GeometryError>.
    ///
    /// # Examples
    ///
    /// Area of Circle:
    /// ```
    /// use area_crate::area;
    ///
    /// let radius = 12.0;
    /// let answer = area::circle(radius)?;
    /// assert_eq!(452.3893421169302, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// Area of Triangle:
    /// ```
    /// use area_crate::area;
    ///
    /// let height = 20.0;
    /// let width = 40.0;
    /// let answer = area::triangle(height, width)?;
    /// assert_eq!(400.0, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// Area of Square:
    /// ```
    /// use area_crate::area;
    ///
    /// let length = 5.0;
    /// let answer = area::square(length)?;
    /// assert_eq!(25.0, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// Area of Rectangle:
    /// ```
    /// use area_crate::area;
    ///
    /// let height = 20.0;
    /// let width = 40.0;
    /// let answer = area::rectangle(height, width)?;
    /// assert_eq!(800.0, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// Area of Trapezoid:
    /// ```
    /// use area_crate::area;
    ///
    /// let base1 = 10.0;
    /// let base2 = 20.0;
    /// let height = 30.0;
    /// let answer = area::trapezoid(base1, base2, height)?;
    /// assert_eq!(450.0, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// Area of Parallelogram:
    /// ```
    /// use area_crate::area;
    ///
    /// let base = 10.0;
    /// let height = 20.0;
    /// let answer = area::parallelogram(base, height)?;
    /// assert_eq!(200.0, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// Area of Ellipse:
    /// ```
    /// use area_crate::area;
    ///
    /// let major_axis = 10.0;
    /// let minor_axis = 20.0;
    /// let answer = area::ellipse(major_axis, minor_axis)?;
    /// assert_eq!(628.3185307179587, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// Area of Sector:
    /// ```
    /// use area_crate::area;
    ///
    /// let radius = 10.0;
    /// let angle = 30.0;
    /// let answer = area::sector(radius, angle)?;
    /// assert_eq!(26.17993877991494, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// Area of Rhombus:
    /// ```
    /// use area_crate::area;
    ///
    /// let diagonal1 = 10.0;
    /// let diagonal2 = 20.0;
    /// let answer = area::rhombus(diagonal1, diagonal2)?;
    /// assert_eq!(100.0, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// Area of Kite:
    /// ```
    /// use area_crate::area;
    ///
    /// let diagonal1 = 10.0;
    /// let diagonal2 = 20.0;
    /// let answer = area::kite(diagonal1, diagonal2)?;
    /// assert_eq!(100.0, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// Area of Regular Polygon:
    /// ```
    /// use area_crate::area;
    ///
    /// let perimeter = 10.0;
    /// let apothem = 20.0;
    /// let answer = area::regular_polygon(perimeter, apothem)?;
    /// assert_eq!(100.0, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// Area of Annulus:
    /// ```
    /// use area_crate::area;
    ///
    /// let outer_radius = 10.0;
    /// let inner_radius = 5.0;
    /// let answer = area::annulus(outer_radius, inner_radius)?;
    /// assert_eq!(235.61944901923448, answer);
    /// # Ok::<(), area_crate::GeometryError>(())
    /// ```
    ///
    /// # Error Handling
    /// All functions return `Result<f64, GeometryError>`. They will return errors in these cases:
    /// - `GeometryError::NegativeValue` if any input dimension is negative or zero
    /// - `GeometryError::InvalidAngle` if an angle is not in the valid range
    /// - `GeometryError::InvalidInput` for other validation errors (e.g., inner radius > outer radius)
    ///
    pub fn circle(radius: f64) -> Result<f64, GeometryError> {
        if radius <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(PI * radius * radius)
    }

    pub fn triangle(height: f64, width: f64) -> Result<f64, GeometryError> {
        if height <= 0.0 || width <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(0.5 * height * width)
    }

    pub fn square(length: f64) -> Result<f64, GeometryError> {
        if length <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(length * length)
    }

    pub fn rectangle(height: f64, width: f64) -> Result<f64, GeometryError> {
        if height <= 0.0 || width <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(height * width)
    }

    pub fn trapezoid(base1: f64, base2: f64, height: f64) -> Result<f64, GeometryError> {
        if base1 <= 0.0 || base2 <= 0.0 || height <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(0.5 * (base1 + base2) * height)
    }

    pub fn parallelogram(base: f64, height: f64) -> Result<f64, GeometryError> {
        if base <= 0.0 || height <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(base * height)
    }

    pub fn ellipse(major_axis: f64, minor_axis: f64) -> Result<f64, GeometryError> {
        if major_axis <= 0.0 || minor_axis <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(PI * major_axis * minor_axis)
    }

    pub fn sector(radius: f64, angle: f64) -> Result<f64, GeometryError> {
        if radius <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        if angle <= 0.0 || angle > 360.0 {
            return Err(GeometryError::InvalidAngle);
        }
        Ok(0.5 * radius * radius * angle * PI / 180.0)
    }

    pub fn rhombus(diagonal1: f64, diagonal2: f64) -> Result<f64, GeometryError> {
        if diagonal1 <= 0.0 || diagonal2 <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(0.5 * diagonal1 * diagonal2)
    }

    pub fn kite(diagonal1: f64, diagonal2: f64) -> Result<f64, GeometryError> {
        if diagonal1 <= 0.0 || diagonal2 <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(0.5 * diagonal1 * diagonal2)
    }

    pub fn regular_polygon(perimeter: f64, apothem: f64) -> Result<f64, GeometryError> {
        if perimeter <= 0.0 || apothem <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(0.5 * perimeter * apothem)
    }

    pub fn annulus(outer_radius: f64, inner_radius: f64) -> Result<f64, GeometryError> {
        if outer_radius <= 0.0 || inner_radius <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        if inner_radius >= outer_radius {
            return Err(GeometryError::InvalidInput(
                "Inner radius must be smaller than outer radius".to_string(),
            ));
        }
        Ok(PI * (outer_radius * outer_radius - inner_radius * inner_radius))
    }

    /// Calculates the area of a pentagon given the side length
    pub fn pentagon(side_length: f64) -> Result<f64, GeometryError> {
        if side_length <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(0.25 * (5.0 * (5.0 + 2.0 * (5.0_f64).sqrt())).sqrt() * side_length * side_length)
    }

    /// Calculates the area of a hexagon given the side length
    pub fn hexagon(side_length: f64) -> Result<f64, GeometryError> {
        if side_length <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(3.0 * 3.0_f64.sqrt() * side_length * side_length / 2.0)
    }

    /// Calculates the area of any regular polygon
    pub fn polygon(sides: u32, side_length: f64) -> Result<f64, GeometryError> {
        if sides < 3 {
            return Err(GeometryError::InvalidInput(
                "Polygon must have at least 3 sides".to_string(),
            ));
        }
        if side_length <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        let n = sides as f64;
        Ok(n * side_length * side_length / (4.0 * (PI / n).tan()))
    }

    /// Calculates the area of a circular segment
    pub fn circular_segment(radius: f64, angle_degrees: f64) -> Result<f64, GeometryError> {
        if radius <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        if angle_degrees <= 0.0 || angle_degrees > 360.0 {
            return Err(GeometryError::InvalidAngle);
        }
        let angle_rad = angle_degrees * PI / 180.0;
        Ok(0.5 * radius * radius * (angle_rad - angle_rad.sin()))
    }
}

/// Module containing perimeter calculation functions
pub mod perimeter {
    use super::{constants::PI, GeometryError};

    pub fn circle(radius: f64) -> Result<f64, GeometryError> {
        if radius <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(2.0 * PI * radius)
    }

    pub fn rectangle(length: f64, width: f64) -> Result<f64, GeometryError> {
        if length <= 0.0 || width <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(2.0 * (length + width))
    }

    pub fn triangle(a: f64, b: f64, c: f64) -> Result<f64, GeometryError> {
        if a <= 0.0 || b <= 0.0 || c <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        if a + b <= c || b + c <= a || a + c <= b {
            return Err(GeometryError::InvalidInput(
                "Invalid triangle sides".to_string(),
            ));
        }
        Ok(a + b + c)
    }

    pub fn regular_polygon(sides: u32, side_length: f64) -> Result<f64, GeometryError> {
        if sides < 3 {
            return Err(GeometryError::InvalidInput(
                "Polygon must have at least 3 sides".to_string(),
            ));
        }
        if side_length <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(sides as f64 * side_length)
    }
}

/// Module containing surface area calculations for 3D shapes
pub mod surface_area {
    use super::{constants::PI, GeometryError};

    pub fn cube(side: f64) -> Result<f64, GeometryError> {
        if side <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(6.0 * side * side)
    }

    pub fn sphere(radius: f64) -> Result<f64, GeometryError> {
        if radius <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(4.0 * PI * radius * radius)
    }

    pub fn cylinder(radius: f64, height: f64) -> Result<f64, GeometryError> {
        if radius <= 0.0 || height <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(2.0 * PI * radius * (radius + height))
    }

    pub fn cone(radius: f64, height: f64) -> Result<f64, GeometryError> {
        if radius <= 0.0 || height <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        let slant_height = (radius * radius + height * height).sqrt();
        Ok(PI * radius * (radius + slant_height))
    }
}

/// Module containing volume calculations for 3D shapes
pub mod volume {
    use super::{constants::PI, GeometryError};

    pub fn cube(side: f64) -> Result<f64, GeometryError> {
        if side <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(side * side * side)
    }

    pub fn sphere(radius: f64) -> Result<f64, GeometryError> {
        if radius <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(4.0 / 3.0 * PI * radius * radius * radius)
    }

    pub fn cylinder(radius: f64, height: f64) -> Result<f64, GeometryError> {
        if radius <= 0.0 || height <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(PI * radius * radius * height)
    }

    pub fn cone(radius: f64, height: f64) -> Result<f64, GeometryError> {
        if radius <= 0.0 || height <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(PI * radius * radius * height / 3.0)
    }

    pub fn rectangular_prism(length: f64, width: f64, height: f64) -> Result<f64, GeometryError> {
        if length <= 0.0 || width <= 0.0 || height <= 0.0 {
            return Err(GeometryError::NegativeValue);
        }
        Ok(length * width * height)
    }
}
