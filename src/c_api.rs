extern crate legion;

extern crate easy_ffi;

use easy_ffi::*;

use std::ffi::c_void;

use crate::component::ComponentData;
use crate::utils::create_test_component_data;

use log::*;

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
unsafe impl Sync for World {}
unsafe impl Send for World {}

ptr_ffi!(
    fn legion_world_new() -> Result<*mut World, &'static str> {
        info!("Creating world");
        let world = Box::new(legion::World::default());
        Ok(Box::into_raw(world) as *mut World)
    }
);

ptr_ffi!(
    fn legion_create_entity(world_ptr: *mut World, component_data: *mut ComponentData) -> Result<*mut World, &'static str> {
        info!("Creating entity");
        unsafe {
            debug!("Raw World Pointer from C in Rust: {:?}", world_ptr);
            let world = (world_ptr as *mut legion::World).as_mut().expect("Failed to cast *mut World to &mut legion::World");
            // let component_data_ref = component_data.as_ref().expect("Failed to get component data reference");
            // world.extend((*component_data_ref).clone());
            debug!("AiAI");
            world.extend(create_test_component_data());
            let boxed = Box::new(world);
            Ok(Box::into_raw(boxed) as *mut World)
        }
    }
);

ptr_ffi!(
    fn legion_create_component_data(component_types: *const u32, number_components: u32, components: *const *const c_void) -> Result<*mut ComponentData, &'static str> {
        info!("Creating component data");
        debug!("{}", number_components);
        let component_data = ComponentData { component_types, number_components, components, layout: legion::storage::EntityLayout::new() };
        let boxed = Box::new(component_data);

         Ok(Box::into_raw(boxed) as *mut ComponentData)
    }     
);
