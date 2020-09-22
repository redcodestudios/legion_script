use legion::*;

use legion::{
    storage::{Component,ComponentTypeId, ComponentSource, ArchetypeSource, ArchetypeWriter, EntityLayout},
    query::{LayoutFilter, FilterResult},
};
use legion_script::system::{scripting_system, test_query_system, Scripts, ComponentId, Position, ComponentData};

use std::os::raw::c_void;
use std::any::TypeId;

pub fn main() {
    // let c = ComponentTypeId{type_id: TypeId::of::<ExternalComponent>(), ext_type_id: Some(22), name: "meu_type" };

    let mut world = World::default();
    let mut resources = Resources::default();


    // let teste = TestPTR {x: std::ptr::null()};
    
    // world.push((teste,));

    let pos = Box::<Position>::new(Position{x: 1, y: 2});
    let raw_pos = Box::into_raw(pos);
    println!("Raw pos: {:?}", raw_pos);
    let vec_pos = vec![raw_pos].as_ptr() as *const c_void;
    println!("Vec pos: {:?}", vec_pos);

    let component_data = ComponentData {
        // number_component_types: 1,
        component_types: vec![1].as_ptr() as *const u32,
        // component_data_sizes: vec![2].as_ptr(),
        number_components: 1,
        components: vec_pos,
        layout: EntityLayout::new(),
    };
    let mut entities: Vec<Entity> = Vec::new();
    for e in world.extend(vec![(component_data,)]){
        entities.push(*e);
    }

    for e in entities.iter() {
        assert_eq!(true, world.contains(*e));
    }

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
