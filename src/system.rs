use legion::*;
use legion::world::SubWorld;
use crate::driver::run_script;

use legion::{
    storage::{Component,ComponentTypeId, ComponentSource, ArchetypeSource, ArchetypeWriter, EntityLayout},
    query::{LayoutFilter, FilterResult},
};

use std::any::TypeId;
use std::os::raw::c_void;
use std::fmt::Debug;

pub type ComponentId = u64;
pub type Scripts = Vec<String>;

#[derive(Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32
}

pub struct ComponentDataFilter;

#[repr(C)]
#[derive(Debug)]
pub struct ComponentData {
    // pub number_component_types: u32,
    pub component_types: *const u32,
    // pub component_data_sizes: *const u32,
    pub number_components: u32,
    pub components: *const c_void,
    pub layout: EntityLayout,
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

unsafe impl Send for ComponentData {}
unsafe impl Sync for ComponentData {}


impl LayoutFilter for ComponentDataFilter {
    fn matches_layout(&self, components: &[ComponentTypeId]) -> FilterResult {
        FilterResult::Match(components.is_empty())
    }
}

struct ComponentDataLayout;

struct ExternalComponent;

impl storage::ComponentSource for ComponentData {
    
    fn push_components<'b>(&mut self, writer: &mut ArchetypeWriter<'b>, entities: impl Iterator<Item = Entity>) {
        for e in entities {
            writer.push(e);
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
            
                storage.extend_memcopy_raw((self.components).offset(component_index as isize) as *mut u8, 1)
            }
        }
    }
}
#[system]
pub fn scripting(#[resource] scripts: &mut Scripts, #[resource] component_id: &mut ComponentId) {
    run_script(scripts[0].clone(), component_id);
    run_script(scripts[1].clone(), component_id);
    println!("Resource after running {}", *component_id);
}

#[system(for_each)]
pub fn test_query(data: &ComponentData) {
    println!("aaaaaaaaaaa");
    println!("aaaaaaa {:?}", data);
   
    unsafe {
    
        // let components = std::slice::from_raw_parts(data.components, 1 as usize);
        println!("{:?}", *(data.components));

         let pos = std::mem::transmute::<*const c_void, Position>((data.components));
         println!("Pos x: {:?}", pos);
    }


    // for a in query.iter_mut(world) {
        // println!("{:?}", a);
    // }
}
// let mut world = World::default();

//     let components = vec![
//         (Pos(1., 2., 3.), Rot(0.1, 0.2, 0.3)),
//         (Pos(4., 5., 6.), Rot(0.4, 0.5, 0.6)),
//     ];

//     let mut entities: Vec<Entity> = Vec::new();
//     for e in world.extend(components) {
//         entities.push(*e);
//     }

//     for e in entities.iter() {
//         assert_eq!(true, world.contains(*e));
//     }

//     for e in entities.iter() {
//         world.remove(*e);
//         assert_eq!(false, world.contains(*e));
//     }

//pub fn scripting_system(script: String, script2: String) -> systems::System {
//    SystemBuilder::<()>::new("ScriptingSystem")
//        .write_resource::<ComponentId>()
//        .build(move |_commands, _world, _resource, _queries|{
//            //@TODO: pass world to script
//            run_script(script.clone(), _resource);
//            run_script(script2.clone(), _resource);
//            println!("Resource after running {}", **_resource);
//        })
//}
