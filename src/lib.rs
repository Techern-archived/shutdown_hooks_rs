//! Provides access to shutdown hooks
//! 
//! Currently, only adding new hooks is supported.
//! 
//! An unspecified future release will have a collection of hooks and have the ability to remove them

use std::os::raw::c_int;

/// Adds a callback function to be called on exit
#[allow(unused_assignments)] // Assigned in unsafe block
pub fn add_shutdown_hook(callback: extern fn()) -> bool {
    let mut result: c_int = 0;
    
    unsafe {
        result = atexit(callback);
    }
    
    if result == 0 {
        return true;
    } else {
        return false;
    }
}

extern {

    /// Calls the specified callback function on exit
    fn atexit(callback: extern fn()) -> c_int;

}
