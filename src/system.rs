use legion::*;
use crate::driver::run_script;
use crate::filter::ExternalLayoutFilter;
use legion::{
    storage::{ComponentTypeId, PackedStorage,UnknownComponentStorage, ArchetypeSource, ArchetypeWriter, EntityLayout},
    
};

use std::any::TypeId;
use std::os::raw::c_void;
use std::fmt::Debug;

pub type ComponentId = u64;
pub type Scripts = Vec<String>;

#[derive(Debug)]
#[repr(C)]
pub struct Position {
    pub x: u32,
    pub y: u32
}


#[repr(C)]
#[derive(Debug)]
pub struct ComponentData {
    // pub number_component_types: u32,
    pub component_types: *const u32,
    // pub component_data_sizes: *const u32,
    pub number_components: u32,
    pub components: *const *const c_void,
    pub layout: EntityLayout,
}

impl ArchetypeSource for ComponentData {
    type Filter = ExternalLayoutFilter;

    fn filter(&self) -> Self::Filter {
        println!("filter - start");
        let filter = Self::Filter{};
        println!("filter - end");
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



struct ComponentDataLayout;

pub struct ExternalComponent {
   _private: *const c_void,
}

unsafe impl Send for ExternalComponent{}
unsafe impl Sync for ExternalComponent {}

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
#[system]
pub fn scripting(#[resource] scripts: &mut Scripts, #[resource] component_id: &mut ComponentId) {
    println!("scripting - start");
    run_script(scripts[0].clone(), component_id);
    run_script(scripts[1].clone(), component_id);
    println!("scripting - end - Resource after running {}", *component_id);
}

#[system(for_each)]
pub fn test_query(data: &ComponentData) {
    println!("aaaaaaaaaaa");
    println!("aaaaaaa {:?}", data);
    println!("aaaaaaa");
   
    unsafe {
    
        // let components = std::slice::from_raw_parts(data.components, 1 as usize);
        // println!("BBBBBBBBB {:?}", *(*(data.components).offset(1)));
        let data: &Position =  & *(data.components as *const Position);

         // let pos = std::mem::transmute::<*const c_void, Position>((data.components));
         println!("Pos x: {:?}", data.x);
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
