use legion::world::World;
use legion::*;

use legion_script::{
    query::get_external_components,
    utils::{create_test_component_data, create_test_component_ids, Position, Rotation},
};

use std::os::raw::c_void;

#[test]
fn insert_entities_into_world() {
    let mut world = World::default();
    let mut entities: Vec<Entity> = Vec::new();
    for e in world.extend(create_test_component_data()) {
        entities.push(*e);
    }

    for e in entities.iter() {
        assert_eq!(true, world.contains(*e));
    }
}

// #[test]
// fn run_python_script() {
//     let mut id_count = 0_u64;
//     driver::run_script(
//         String::from("tests/scripts/print.py"), &mut id_count);
//     assert_eq!(0,0);
// }

#[test]
fn transmute_components_from_world() {
    let mut world = World::default();

    let component_data = create_test_component_data();
    let entities = world.extend(component_data);

    let component_type_ids = create_test_component_ids();
    let mut components: Vec<*const c_void> = vec![];

    get_external_components(&world, component_type_ids.to_vec(), &mut components);

    unsafe {
        let position = std::mem::transmute::<*const c_void, &Position>(components[0]);
        assert_eq!(100, position.x);
        assert_eq!(50, position.y);
    }
    unsafe {
        let rotation = std::mem::transmute::<*const c_void, &Rotation>(components[1]);
        assert_eq!(50, rotation.x);
    }
}

#[test]
fn get_multiple_components() {
    let mut world = World::default();

    for _ in 0..10 {
        world.extend(create_test_component_data());
    }

    let component_type_ids = create_test_component_ids();
    let mut components: Vec<*const c_void> = vec![];

    let vec_with_position_id = vec![component_type_ids.to_vec()[0]];
    get_external_components(&world, vec_with_position_id, &mut components);

    unsafe {
        for i in 0..10 {
            let position = std::mem::transmute::<*const c_void, &Position>(components[i]);
            assert_eq!(100, position.x);
            assert_eq!(50, position.y);
        }
    }
}
