use legion_script::*;
use legion::*;
use legion::world::World;

use legion_script::{
    system::{Scripts, scripting_system},
    system::ComponentId,
    query::{get_external_components_ids},
    components::{Rotation, Position},
    utils::{create_test_component_data},
    query::{get_external_components}
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
    let mut components: Vec<*const c_void> = vec![];
    
    get_external_components(&world, component_type_ids.to_vec(), &mut components);
    
    unsafe{
        let position = std::mem::transmute::<*const c_void, &Position>(components[0]);
        assert_eq!(100, position.x);
        assert_eq!(50, position.y);
    }
    unsafe{
        let rotation = std::mem::transmute::<*const c_void, &Rotation>(components[1]);
        assert_eq!(50, rotation.x);
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
