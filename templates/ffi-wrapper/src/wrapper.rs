use std::ptr;
use super::bindings::{Vec3, LibError};
use super::error::FfiError;

/// Wrapper around a C Vec3 pointer that manages memory safely.
/// This type owns the memory and will call lib_vec3_destroy on drop.
#[derive(Debug)]
pub struct Vec3Wrapper {
    raw: *mut Vec3,
}

impl Vec3Wrapper {
    /// Creates a new Vec3 from x, y, z coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Result<Self, FfiError> {
        let mut vec_ptr: *mut Vec3 = ptr::null_mut();

        // SAFETY: Passing valid pointers, allocating new C memory that we own
        let result = unsafe {
            super::bindings::lib_vec3_create(x, y, z, &mut vec_ptr)
        };

        match result {
            LibError::Ok => {
                if vec_ptr.is_null() {
                    return Err(FfiError::NullPointer);
                }
                Ok(Self { raw: vec_ptr })
            }
            LibError::ErrorAllocation => Err(FfiError::AllocationFailed),
            LibError::ErrorInvalidInput => Err(FfiError::InvalidInput("invalid coordinates".into())),
            _ => Err(FfiError::Unknown(result as i32)),
        }
    }

    /// Creates a Vec3Wrapper from a raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `raw` is a valid, non-null pointer
    /// to a Vec3 allocated by the C library. The resulting Vec3Wrapper
    /// takes ownership and will call lib_vec3_destroy on drop.
    pub unsafe fn from_raw(raw: *mut Vec3) -> Self {
        assert!(!raw.is_null(), "Vec3Wrapper::from_raw: null pointer");
        Self { raw }
    }

    /// Returns the x coordinate.
    ///
    /// # Safety
    ///
    /// The underlying Vec3 must be valid and non-null.
    pub fn x(&self) -> f64 {
        // SAFETY: self.raw is guaranteed non-null and valid by construction
        // and the drop implementation ensures the pointer remains valid
        // for the lifetime of this Vec3Wrapper
        unsafe { (*self.raw).x }
    }

    /// Returns the y coordinate.
    pub fn y(&self) -> f64 {
        unsafe { (*self.raw).y }
    }

    /// Returns the z coordinate.
    pub fn z(&self) -> f64 {
        unsafe { (*self.raw).z }
    }

    /// Calculates the length (magnitude) of the vector.
    ///
    /// # Safety
    ///
    /// The underlying Vec3 must be valid and non-null.
    pub fn length(&self) -> f64 {
        // SAFETY: self.raw is valid, non-null, and points to initialized Vec3
        unsafe { super::bindings::lib_vec3_length(self.raw) }
    }

    /// Returns the raw pointer (for FFI boundary crossing).
    pub fn as_ptr(&self) -> *const Vec3 {
        self.raw
    }
}

/// SAFETY: Vec3Wrapper owns exclusive access to its raw pointer and
/// manages the memory via lib_vec3_destroy in Drop. The underlying C
/// library is thread-safe for read-only operations.
unsafe impl Send for Vec3Wrapper {}
unsafe impl Sync for Vec3Wrapper {}

impl Drop for Vec3Wrapper {
    fn drop(&mut self) {
        // SAFETY: self.raw is valid, non-null, and we have exclusive ownership
        // The C library will free the memory allocated in lib_vec3_create
        unsafe { super::bindings::lib_vec3_destroy(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_creation() {
        let vec = Vec3Wrapper::new(3.0, 4.0, 0.0).unwrap();
        assert!((vec.length() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_zero() {
        let vec = Vec3Wrapper::new(0.0, 0.0, 0.0).unwrap();
        assert!(vec.length() < 1e-10);
    }

    #[test]
    fn test_coordinates() {
        let vec = Vec3Wrapper::new(1.0, 2.0, 3.0).unwrap();
        assert!((vec.x() - 1.0).abs() < 1e-10);
        assert!((vec.y() - 2.0).abs() < 1e-10);
        assert!((vec.z() - 3.0).abs() < 1e-10);
    }
}