use legion_script::*;
use legion::*;
use legion::{storage::{ComponentTypeId, EntityLayout}};
use legion_script::{
    system::{ComponentId, Scripts, scripting_system,ComponentData, ExternalComponent},
    driver::{convert_bytes_into_pointer, get_external_components_ids},
    components::{Rotation, Position},
    utils::{create_test_component_data}
};

use std::os::raw::c_void;
use std::any::TypeId;
use std::slice;

#[test]
fn insert_entities_into_world(){
    let mut world = World::default();
    let mut entities: Vec<Entity> = Vec::new();
    for e in world.extend(create_test_component_data()){
        entities.push(*e);
    }

    for e in entities.iter() {
        assert_eq!(true, world.contains(*e));
    }
}

// #[test]
fn run_python_script() {
    let mut id_count = 0_u64;
    driver::run_script(
        String::from("tests/scripts/print.py"), &mut id_count);
    assert_eq!(0,0);
}


#[test]
fn raw_component(){
    // SimpleLogger::new().with_level(LevelFilter::Trace).init().expect("Failed to create logger");
    
    let mut world = World::default();
    let mut resources = Resources::default();

    
    let component_data = create_test_component_data();
    let entities = world.extend(component_data);

    let component_type_ids = get_external_components_ids();
    for archetype in world.archetypes() {
        println!("Archetype: {:?}", archetype);
        for id in component_type_ids.iter(){
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
                                assert_eq!(100, comp.x);
                                assert_eq!(50, comp.y);
                            } else if *id == component_type_ids[1] {
                                let comp = std::mem::transmute::<*const c_void, &Rotation>(test);
                                println!("CARALHO ROTATION {:?}", comp);
                                assert_eq!(50, comp.x);
                                // assert_eq!(true,false); // sanity test
                            }
                            println!("len: {}", len);
                    }
        
            }

        }
    }
    

    let id_count = 0;
    resources.insert::<ComponentId>(id_count);
    
    let scripts = vec![String::from("examples/python/hello.py"), String::from("examples/python/hello2.py")];
    resources.insert::<Scripts>(scripts);


    let mut schedule = Schedule::builder()
        .add_system(scripting_system())
        .build();

    schedule.execute(&mut world, &mut resources);
}