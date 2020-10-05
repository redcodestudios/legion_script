extern crate easy_ffi;

use legion::{
    storage::{ComponentTypeId, Archetype},
};

use crate::{
    component::{ExternalComponent},
    driver::{convert_bytes_into_pointer},
    component::{ComponentData}
};
use std::os::raw::c_void;
use std::slice;

use std::any::TypeId;

fn get_component_from_storage(world: &legion::world::World, archetype: &Archetype, id: &ComponentTypeId) -> *const c_void{
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

pub fn get_external_components(world: &legion::world::World, component_type_ids: Vec<ComponentTypeId>,components: &mut Vec<*const c_void>){
    for archetype in world.archetypes() {
        println!("Archetype: {:?}", archetype);
        for id in component_type_ids.iter() {
            let component: *const c_void = get_component_from_storage(&world, archetype, id); 
            println!("Getting id {:?}", id);
            components.push(component);           
        }
    }
}

pub fn get_external_components_ids() -> [ComponentTypeId;2]{
    [ComponentTypeId { 
        type_id: TypeId::of::<ExternalComponent>(),
        ext_type_id: Some(666),
        name: "external component"
    }, ComponentTypeId { 
        type_id: TypeId::of::<ExternalComponent>(),
        ext_type_id: Some(777),
        name: "external component"
    }]
}
