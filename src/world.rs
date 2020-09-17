extern crate legion;

extern crate easy_ffi;

use std::os::raw::c_void;
use easy_ffi::*;

easy_ffi!(void_ffi =>
    |err| {
        println!("{}", err);
        ()
    }
    |panic_val| {
        match panic_val.downcast_ref::<&'static str>() {
            Some(s) => println!("panic: {}", s),
            None => println!("unknown panic!"),
        }
        ()
    }
);

easy_ffi!(ptr_ffi =>
    |err| {
        println!("{}", err);
        std::ptr::null_mut()
    }
    |panic_val| {
        match panic_val.downcast_ref::<&'static str>() {
            Some(s) => println!("panic: {}", s),
            None => println!("panic: Unknown")
        }
        std::ptr::null_mut()
    }
);

#[repr(C)]
pub struct World;

ptr_ffi!(
    fn legion_world_new() -> Result<*mut World, &'static str> {
        let world = Box::new(legion::World::default());
        Ok(Box::into_raw(world) as *mut World)
    }
);
