extern crate legion;

extern crate easy_ffi;

use std::os::raw::c_void;
use easy_ffi::*;

use std::any::TypeId;

use legion::{
    Entity,
    storage::{Component,ComponentTypeId, ComponentSource, ArchetypeSource, ArchetypeWriter, EntityLayout},
    query::{LayoutFilter, FilterResult},
};

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

struct ComponentDataFilter;

impl LayoutFilter for ComponentDataFilter {
    fn matches_layout(&self, components: &[ComponentTypeId]) -> FilterResult {
        FilterResult::Match(components.is_empty())
    }
}


struct ExternalComponent;


#[repr(C)]
struct ComponentData {
    number_component_types: u32,
    component_types: *const u32,
    component_data_sizes: *const u32,
    number_components: u32,
    components: *const *const c_void,
    layout: EntityLayout,
}

impl ArchetypeSource for ComponentData {
    type Filter = ComponentDataFilter;

    fn filter(&self) -> Self::Filter {
        Self::Filter{}
    }

    fn layout(&mut self) -> EntityLayout {
        self.layout.clone()
    }

}

impl ComponentSource for ComponentData {
    
    fn push_components<'b>(&mut self, writer: &mut ArchetypeWriter<'b>, entities: impl Iterator<Item = Entity>) {
        for entity in entities {
            writer.push(entity);
        }

        for component_index in 0..self.number_components{
            unsafe {
                let mut storage = writer.claim_components_unknown(
                    ComponentTypeId {
                        type_id: TypeId::of::<ExternalComponent>(),
                        ext_type_id: Some(*(self.component_types.offset(component_index as isize))),
                        name: "external component"
                    }
                );
            
                storage.extend_memcopy_raw(*(self.components).offset(component_index as isize) as *mut u8, 1)
            }
        }
    }
}
