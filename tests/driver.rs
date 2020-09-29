use legion_script::*;
use legion::*;
use legion::world::World;

use legion_script::{
    system::{ComponentId, Scripts, scripting_system},
    driver::{get_external_components_ids},
    components::{Rotation, Position},
    utils::{create_test_component_data},
    query::{get_component_from_storage}
};

use std::os::raw::c_void;

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
    let mut world = World::default();
    let mut resources = Resources::default();

    
    let component_data = create_test_component_data();
    let entities = world.extend(component_data);

    let component_type_ids = get_external_components_ids();
    for archetype in world.archetypes() {
        println!("Archetype: {:?}", archetype);
        assert_eq!(world.archetypes().len(), 1);
        for id in component_type_ids.iter(){
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
    
    

    let id_count = 0;
    resources.insert::<ComponentId>(id_count);
    
    let scripts = vec![String::from("examples/python/hello.py"), String::from("examples/python/hello2.py")];
    resources.insert::<Scripts>(scripts);


    let mut schedule = Schedule::builder()
        .add_system(scripting_system())
        .build();

    schedule.execute(&mut world, &mut resources);
}