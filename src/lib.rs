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

#[allow(unused_imports)]
mod tests {

    use super::add_shutdown_hook;

    #[test]
    fn test_add_shutdown_hook_should_work() {
        assert!(add_shutdown_hook(get_in_the_bowl));
    }
    
    #[allow(dead_code)]
    extern fn get_in_the_bowl() {
        assert!(1 + 2 == 3); //You get in the bowl
    }

}