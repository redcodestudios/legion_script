use legion::*;

use crate::{
    filter::ExternalLayoutFilter,
};

use legion::{
    storage::{ComponentTypeId, PackedStorage, UnknownComponentStorage, ArchetypeSource, ArchetypeWriter, EntityLayout},
};

use std::any::TypeId;
use std::os::raw::c_void;
use std::fmt::Debug;

pub struct ExternalComponent {
   _private: *const c_void,
}

unsafe impl Send for ExternalComponent{}
unsafe impl Sync for ExternalComponent{}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ComponentData {
    pub component_types: *const u32,
    pub number_components: u32,
    pub components: *const *const c_void,
    pub layout: EntityLayout,
}

impl ArchetypeSource for ComponentData {
    type Filter = ExternalLayoutFilter;

    fn filter(&self) -> Self::Filter {
        // println!("filter - start");
        let filter = Self::Filter{};
        // println!("filter - end");
        filter
    }

    fn layout(&mut self) -> EntityLayout {
        let constructor = || {
            let storage = Box::new(PackedStorage::<ExternalComponent>::default()) as Box<dyn UnknownComponentStorage>;
            println!("REGISTERING storage: {:?}", &*storage as *const _ as *const c_void);
            return storage
        };
        let mut ids: Vec<ComponentTypeId> = Vec::new();
        unsafe{

                for component_index in 0..self.number_components {
                    println!("ext id: {}", *(self.component_types).offset(component_index as isize) as u32 );
                    let id = ComponentTypeId {
                        type_id: TypeId::of::<ExternalComponent>(),
                        ext_type_id: Some(*(self.component_types).offset(component_index as isize) as u32),
                        name: "external component"
                    };
                    ids.push(id);
                
                    self.layout.register_component_raw(id, constructor);
            }
        }    
        println!("layout - start");
        let layout = self.layout.clone();
        println!("layout - end");
        layout
    }

}

unsafe impl Send for ComponentData {}
unsafe impl Sync for ComponentData {}

impl storage::IntoComponentSource for ComponentData{
    type Source = Self; 
    fn into(self)-> Self{
        self
    }
}
impl storage::ComponentSource for ComponentData {
    
    fn push_components<'b>(&mut self, writer: &mut ArchetypeWriter<'b>, entities: impl Iterator<Item = Entity>) {
        println!("storage - push components _ start");
        let mut ids: Vec<ComponentTypeId> = Vec::new();
        
        unsafe{ 
            for component_index in 0..self.number_components{
                let id = ComponentTypeId {
                    type_id: TypeId::of::<ExternalComponent>(),
                    ext_type_id: Some(*(self.component_types.offset(component_index as isize))),
                    name: "external component"
                };
                ids.push(id);
                
                // self.layout.register_component_raw(id,constructor);
                
            }
        }
        
        for id in ids{
            println!("Layout has the component id [{:?}], name [{}]? {}", id.ext_type_id, id.name, self.layout.has_component_by_id(id));
        }
        for e in entities {
            writer.push(e);
            println!("Creating entity - {:?}", e);
            break;
        }
        
        
        for component_index in 0..self.number_components{
            println!("storing components - #{}", component_index);
            unsafe {
                let mut unkown_component_writer = writer.claim_components_unknown(
                    ComponentTypeId {
                        type_id: TypeId::of::<ExternalComponent>(),
                        ext_type_id: Some(*(self.component_types.offset(component_index as isize)) as u32),
                        name: "external component"
                    }
                );
                println!("unknown_component_writer_storage: {:?}", unkown_component_writer.components as *const _ as *const c_void);
                let comp_ptr = *self.components.offset(component_index as isize);
                let black_magic: *const *const c_void = &[comp_ptr] as *const *const c_void;
                println!("pushing black_magic_ptr: {:?}", black_magic);
                unkown_component_writer.extend_memcopy_raw(black_magic as *mut u8, 1);
            }
        }
        println!("storage - push components _ end");
    }
}
