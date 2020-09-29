use legion::*;

use legion::{
    storage::{Component,ComponentTypeId, ComponentSource, ArchetypeSource, ArchetypeWriter,ArchetypeIndex, EntityLayout, PackedStorage},
    query::{LayoutFilter, FilterResult},
};
use legion_script::{
    system::{scripting_system, test_query_system, Scripts, ComponentId, ComponentData, ExternalComponent},
    driver::{convert_bytes_into_pointer, get_external_components_ids},
    utils::{create_test_component_data},
    components::{Position, Rotation},
    query::{get_component_from_storage}
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
            let component: *const c_void = get_component_from_storage(&world, archetype, id); 
            println!("Getting id {:?}", id);
            
            if *id == component_type_ids[0] {
                unsafe{
                    let position = std::mem::transmute::<*const c_void, &Position>(component);
                    println!("CARALHO POSITION {:?}", position);
                    assert_eq!(100, position.x);
                    assert_eq!(50, position.y);
                }
            } else if *id == component_type_ids[1] {
                unsafe{
                    let rotation = std::mem::transmute::<*const c_void, &Rotation>(component);
                    println!("CARALHO ROTATION {:?}", rotation);
                    assert_eq!(50, rotation.x);
                    // assert_eq!(true,false); // sanity test
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
