use legion::*;

use legion::{
    storage::{Component,ComponentTypeId, ComponentSource, ArchetypeSource, ArchetypeWriter,ArchetypeIndex, EntityLayout, PackedStorage},
    query::{LayoutFilter, FilterResult},
};
use legion_script::{
    system::{scripting_system, test_query_system, Scripts, ComponentId, ComponentData, ExternalComponent},
    driver::{convert_bytes_into_pointer, get_external_components_ids},
    utils::{create_test_component_data},
    components::{Position, Rotation}
};

use std::os::raw::c_void;
use std::any::TypeId;
use std::slice;
use simple_logger::{SimpleLogger};
use log::*;


pub fn init_logger(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger::new()))
        .map(|()| log::set_max_level(level))
}

pub fn main() {
    init_logger(LevelFilter::Trace).expect("Failed to create logger");

    // let c = ComponentTypeId{type_id: TypeId::of::<ExternalComponent>(), ext_type_id: Some(22), name: "meu_type" };

    let mut world = World::default();
    let mut resources = Resources::default();
    
    let component_data = create_test_component_data();
    let entities = world.extend(component_data); 
    
    
    let component_type_ids = get_external_components_ids();

    for archetype in world.archetypes() {
        println!("Archetype: {:?}", archetype);
        for id in component_type_ids.iter() {
            println!("Getting id {:?}", id);
            if archetype.layout().has_component_by_id(*id) {
                println!("{:?}", archetype.entities()); 
                let storage = world.components().get(*id).unwrap();
                println!("storage: {:?}", storage as *const _ as *const c_void);
                let (slice_ptr, len) = storage.get_raw(archetype.index()).expect("Failed to get raw component");
                    unsafe {
                            
                            let size = std::mem::size_of::<ExternalComponent>();
                            let slice = slice::from_raw_parts(slice_ptr as *const _, size);
                            // println!("{:#x}", slice);
                            let size = size as isize;
                            for i in 1..=size {
                                print!("{:x}", *slice_ptr.offset(size-i)); 
                            }
                            println!("");
                            let test: *const c_void = convert_bytes_into_pointer(slice);
                            println!("transmutei {:?}", test);

                            if *id == component_type_ids[0] {
                                let comp = std::mem::transmute::<*const c_void, &Position>(test);
                                println!("CARALHO POSITION {:?}", comp);
                                
                            } else if *id == component_type_ids[1] {
                                let comp = std::mem::transmute::<*const c_void, &Rotation>(test);
                                println!("CARALHO ROTATION {:?}", comp);
                            }
                            // println!("t: {:?}", test);
                            // println!("ptr: {:#x}", *slice_ptr.offset(0));
                            // let slice = slice::from_raw_parts(slice_ptr as *const _, len);
                            // for comp_ptr in slice {
                            //     println!("Comp {:?}", comp_ptr);
                            // }
                            println!("len: {}", len);
                            // let comp_ptr = slice_ptr.offset(len * ) as *const usize;
                            // println!("ptr: {:?}", comp_ptr);
                            // println!("slice is: {:?}", slice[0]);
                            // let pos: &Position =  & *(comp_ptr as *const Position);
                            // let pos: Position = std::mem::transmute::<*const u8, Position>(comp_ptr);
                            // println!(" ------- pos-x: {}, pos-y: {} -------", pos.x, pos.y);
                    }
        
            }

        }
    }

    // let comp_storage = world.components().get(
    //     ComponentTypeId { 
    //         type_id: TypeId::of::<ExternalComponent>(),
    //         ext_type_id: Some(666),
    //         name: "external component"
    //     }
    // );


    let id_count = 0;
    resources.insert::<ComponentId>(id_count);
    
    let scripts = vec![String::from("examples/python/hello.py"), String::from("examples/python/hello2.py")];
    resources.insert::<Scripts>(scripts);


    let mut schedule = Schedule::builder()
        .add_system(scripting_system())
        .add_system(test_query_system())
        .build();

    schedule.execute(&mut world, &mut resources);
}
