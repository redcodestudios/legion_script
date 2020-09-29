
use legion::{
    storage::{ComponentTypeId, Archetype},
    world::World,
};
use crate::{
    system::{ExternalComponent},
    driver::{convert_bytes_into_pointer}
};
use std::os::raw::c_void;
use std::slice;

pub fn get_component_from_storage(world: &World, archetype: &Archetype, id: &ComponentTypeId) -> *const c_void{
    if !archetype.layout().has_component_by_id(*id) {
        panic!("Archetype's layout doesn't contain the required component id");
    }
    println!("{:?}", archetype.entities()); 
    let storage = world.components().get(*id).unwrap();
    println!("storage: {:?}", storage as *const _ as *const c_void);
    let (slice_ptr, len) = storage.get_raw(archetype.index()).expect("Failed to get raw component");
    let component: *const c_void;
    unsafe {
        let size = std::mem::size_of::<ExternalComponent>();
        let slice = slice::from_raw_parts(slice_ptr as *const _, size);
        component = convert_bytes_into_pointer(slice);
        println!("transmutei {:?}", component);
    }

    component
}