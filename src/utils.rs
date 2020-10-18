use crate::component::ComponentData;

use std::os::raw::c_void;
use legion::storage::{EntityLayout,ComponentTypeId};
use crate::component::ExternalComponent;

use std::any::TypeId;

#[derive(Debug)]
pub struct Rotation {
    pub x: u32
}

#[derive(Debug)]
#[repr(C)]
pub struct Position {
    pub x: u32,
    pub y: u32
}

pub fn create_test_component_data() -> ComponentData{
    let component_types = [666, 777].as_ptr() as *const u32;
    
    static pos: Position = Position{x: 100, y: 50};
    static rot: Rotation= Rotation{x: 50};
    let pos_ptr = &pos as *const _ as *const c_void;
    let rot_ptr = &rot as *const _ as *const c_void;
    
    let comp_array = [pos_ptr, rot_ptr];
    let components: *const *const c_void = &comp_array as *const  *const _ as *const *const c_void;
    
    let layout = EntityLayout::new();
    let component_data = ComponentData {
        component_types: component_types, 
        number_components: 2,
        components: components,
        layout: layout,
    };
    component_data
}

pub fn create_test_component_ids() -> [ComponentTypeId;2]{
    [ComponentTypeId { 
        type_id: TypeId::of::<ExternalComponent>(),
        ext_type_id: Some(666),
        name: "external component"
    },
    ComponentTypeId { 
        type_id: TypeId::of::<ExternalComponent>(),
        ext_type_id: Some(777),
        name: "external component"
    }]
}