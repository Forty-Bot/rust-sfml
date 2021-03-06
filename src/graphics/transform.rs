use graphics::FloatRect;
use graphics::csfml_graphics_sys as ffi;
use system::Vector2f;
use system::raw_conv::{FromRaw, Raw};

/// Define a 3x3 transform matrix.
///
/// A `Transform` specifies how to translate,
/// rotate, scale, shear, project, whatever things.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Transform(pub ffi::sfTransform);

impl Transform {
    /// Create a new transform from a 3x3 matrix
    ///
    /// # Arguments
    ///
    /// * matrix - An array supplying the matrix
    ///
    ///   Here is an illustration of how the array elements correspond to the matrix elements:
    ///
    ///   ```text
    ///   [(0, 0), (0, 1), (0, 2),
    ///    (1, 0), (1, 1), (1, 2),
    ///    (2, 0), (2, 1), (2, 2)]
    ///   ```
    ///
    /// Return a new Transform
    pub fn new(matrix: [f32; 9]) -> Transform {
        unsafe {
            Transform(ffi::sfTransform_fromMatrix(matrix[0],
                                                  matrix[1],
                                                  matrix[2],
                                                  matrix[3],
                                                  matrix[4],
                                                  matrix[5],
                                                  matrix[6],
                                                  matrix[7],
                                                  matrix[8]))
        }
    }

    /// Return the matrix
    pub fn matrix(&self) -> [f32; 16] {
        unsafe {
            let matrix: [f32; 16] = [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                                     0.];
            ffi::sfTransform_getMatrix(&self.0, matrix.as_ptr() as *mut f32);
            matrix
        }
    }

    /// The identity transform (does nothing)
    pub fn identity() -> Self {
        unsafe { Transform(ffi::sfTransform_Identity) }
    }

    /// Return the inverse of a transform
    ///
    /// If the inverse cannot be computed, a new identity transform
    /// is returned.
    ///
    /// Return the inverse matrix
    pub fn inverse(&mut self) -> Transform {
        unsafe { Transform(ffi::sfTransform_getInverse(&self.0)) }
    }

    /// Combine two transforms
    ///
    /// The result is a transform that is equivalent to applying
    /// transform followed by other. Mathematically, it is
    /// equivalent to a matrix multiplication.
    ///
    /// # Arguments
    /// * other - Transform to combine to transform
    pub fn combine(&mut self, other: &mut Transform) {
        unsafe { ffi::sfTransform_combine(&mut self.0, &other.0) }
    }

    /// Combine a transform with a translation
    ///
    /// # Arguments
    /// * x - Offset to apply on X axis
    /// * y - Offset to apply on Y axis
    pub fn translate(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfTransform_translate(&mut self.0, x, y) }
    }

    /// Combine the current transform with a rotation
    ///
    /// # Arguments
    /// * angle - Rotation angle, in degrees
    pub fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfTransform_rotate(&mut self.0, angle) }
    }

    /// Combine the current transform with a rotation
    ///
    /// The center of rotation is provided for convenience as a second
    /// argument, so that you can build rotations around arbitrary points
    /// more easily (and efficiently) than the usual
    /// [translate(-center), rotate(angle), translate(center)].
    ///
    /// # Arguments
    /// * angle - Rotation angle, in degrees
    /// * center_x - X coordinate of the center of rotation
    /// * center_y - Y coordinate of the center of rotation
    pub fn rotate_with_center(&mut self, angle: f32, center_x: f32, center_y: f32) {
        unsafe { ffi::sfTransform_rotateWithCenter(&mut self.0, angle, center_x, center_y) }
    }

    /// Combine the current transform with a scaling
    ///
    /// # Arguments
    /// * scale_x - Scaling factor on the X axis
    /// * scale_y - Scaling factor on the Y axis
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe { ffi::sfTransform_scale(&mut self.0, scale_x, scale_y) }
    }

    /// Combine the current transform with a scaling
    ///
    /// The center of scaling is provided for convenience as a second
    /// argument, so that you can build scaling around arbitrary points
    /// more easily (and efficiently) than the usual
    /// [translate(-center), scale(factors), translate(center)]
    ///
    /// # Arguments
    /// * scale_x - Scaling factor on X axis
    /// * scale_y - Scaling factor on Y axis
    /// * center_x - X coordinate of the center of scaling
    /// * center_y - Y coordinate of the center of scaling
    pub fn scale_with_center(&mut self, scale_x: f32, scale_y: f32, center_x: f32, center_y: f32) {
        unsafe {
            ffi::sfTransform_scaleWithCenter(&mut self.0, scale_x, scale_y, center_x, center_y)
        }
    }

    /// Apply a transform to a 2D point
    ///
    /// # Arguments
    /// * point - Point to transform
    ///
    /// Return a transformed point
    pub fn transform_point(&mut self, point: &Vector2f) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfTransform_transformPoint(&self.0, point.raw())) }
    }

    /// Apply a transform to a rectangle
    ///
    /// Since SFML doesn't provide support for oriented rectangles,
    /// the result of this function is always an axis-aligned
    /// rectangle. Which means that if the transform contains a
    /// rotation, the bounding rectangle of the transformed rectangle
    /// is returned.
    ///
    /// # Arguments
    /// rectangle - Rectangle to transform
    ///
    /// Return the transformed rectangle
    pub fn transform_rect(&mut self, rectangle: &FloatRect) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfTransform_transformRect(&self.0, rectangle.raw())) }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}
