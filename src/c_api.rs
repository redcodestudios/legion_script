extern crate legion;
extern crate easy_ffi;

use crate::component::{ComponentData, ExternalComponent};
use legion::storage::ComponentTypeId;
use easy_ffi::*;

use std::ffi::c_void;
use std::any::TypeId;
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

#[repr(C)]
pub struct CommandBuffer;

#[repr(C)]
pub struct ComponentsWrapper{
    data: *const c_void,
    len: usize,
}

ptr_ffi!(
    fn legion_create_entity(world_ptr: *mut World, component_data: *mut ComponentData) -> Result<*mut World, &'static str> {
        info!("Creating entity");
        unsafe {
            let world = (world_ptr as *mut legion::world::World).as_mut().expect("Failed to cast *mut World to &mut legion::systems::World");
            debug!("world len {}", world.len());
            world.extend((*component_data).clone());
            debug!("component data pointer {:?}", (*component_data).components);
            Ok(world as *mut legion::World as *mut World)
        }
    }
);

ptr_ffi!(
    fn legion_create_component_data(component_types: *const u32, number_components: u32, components: *const *const c_void) -> Result<*mut ComponentData, &'static str> {
        info!("Creating component data");
        unsafe {
            debug!("component types {:?}", std::slice::from_raw_parts(component_types, number_components as usize));
        }
        debug!("number of components {}", number_components);
        debug!("components address {:p}", components);
        let component_data = ComponentData { component_types, number_components, components, layout: legion::storage::EntityLayout::new() };
        let boxed = Box::new(component_data);

        Ok(Box::into_raw(boxed) as *mut ComponentData)
    }     
);

ptr_ffi!(
    fn get_component(world_ptr: *mut World, id: u32) -> Result<*const c_void, &'static str> {
        unsafe{
            let world = (world_ptr as *mut legion::world::World).as_mut().expect("Failed to cast *mut World to &mut legion::systems::World");
            let component_type_ids = vec![ComponentTypeId { 
                type_id: TypeId::of::<ExternalComponent>(),
                ext_type_id: Some(id),
                name: "external component"
            }];

            // Had to start with null because C was ignoring components[0] and [1]
            let mut components: Vec<*const c_void> = vec![std::ptr::null(),std::ptr::null()];
            crate::query::get_external_components(world, component_type_ids, &mut components);
        
            debug!("COMPONENTS {:?}", components);
            Ok(&ComponentsWrapper{
                data : components.as_ptr() as *const c_void,
                len : components.len()
            } as *const _ as *const c_void)
        }
    }
);
