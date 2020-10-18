extern crate easy_ffi;

use legion::{
    storage::{ComponentTypeId, Archetype},
};

use crate::{
    component::{ExternalComponent},
    driver::{convert_bytes_into_pointer},
};
use std::os::raw::c_void;
use std::slice;

use log::*;

fn get_component_from_storage(world: &legion::world::World, archetype: &Archetype, id: &ComponentTypeId) -> *const c_void {
    trace!("Get component from storage - start");
    if !archetype.layout().has_component_by_id(*id) {
        //@TODO: change to a Result, this will break if we have two archetypes
        panic!("Archetype's layout doesn't contain the required component id");
    }

    debug!("Archetype entities {:?}", archetype.entities());

    let storage = world.components().get(*id).unwrap();
    
    let (slice_ptr, len) = storage.get_raw(archetype.index()).expect("Failed to get raw component");
    let component: *const c_void;
    unsafe {
        let size = std::mem::size_of::<ExternalComponent>();
        let slice = slice::from_raw_parts(slice_ptr as *const _, size);
        component = convert_bytes_into_pointer(slice);
        info!("Transmutei {:?}", component);
    }
    
    trace!("Get component from storage - end");
    component
}

pub fn get_external_components(world: &legion::world::World, component_type_ids: Vec<ComponentTypeId>,components: &mut Vec<*const c_void>){
    trace!("Get external components - start");
    for archetype in world.archetypes() {
        debug!("Archetype: {:?}\n", archetype);
        for id in component_type_ids.iter() {
            info!("Getting component with id = {:?}", id);
            //@TODO: validate Result before pushing to the vec, what result? see TODO above
            let component: *const c_void = get_component_from_storage(&world, archetype, id); 
            components.push(component);           
        }
    }
    trace!("Get external components - end");
}

