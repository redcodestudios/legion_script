use legion::*;

use legion::{
    storage::{Component,ComponentTypeId, ComponentSource, ArchetypeSource, ArchetypeWriter,ArchetypeIndex, EntityLayout, PackedStorage},
    query::{LayoutFilter, FilterResult},
};
use legion_script::system::{scripting_system, test_query_system, Scripts, ComponentId, Position, ComponentData, ExternalComponent};

use std::os::raw::c_void;
use std::any::TypeId;
use std::slice;

struct Rotation {
    x: u32
}

pub fn main() {
    // let c = ComponentTypeId{type_id: TypeId::of::<ExternalComponent>(), ext_type_id: Some(22), name: "meu_type" };

    let mut world = World::default();
    let mut resources = Resources::default();

    let component_types = [666, 777].as_ptr() as *const u32;
    
    let pos = Position{x: 100, y: 50};
    let rot = Rotation{x: 50};

    let pos_ptr = &pos as *const _ as *const c_void;
    let rot_ptr = &rot as *const _ as *const c_void;

    println!("pos ptr: {:?}", pos_ptr);
    println!("rot ptr: {:?}", rot_ptr);
    
    let comp_array = [pos_ptr, rot_ptr];
    println!("comp array: {:?}", &comp_array as *const _);
    let components: *const *const c_void = &comp_array as *const  *const _ as *const *const c_void;
    println!("components: {:?}", components);
    
    unsafe {
    	println!("OFFSET {:?}", (*(components.offset(1) as *const _) as *const _));
    	// let data: &Position =  & *((components.offset(0) as *const c_void) as *const Position);
    	// println!("component ptr {}", data.x);
    }
    // let components: &Position = unsafe { &mut *(data as *mut State) };
    let layout = EntityLayout::new();
    let component_data = ComponentData {
        // number_component_types: 1,
        component_types: component_types, 
        // component_data_sizes: vec![2].as_ptr(),
        number_components: 2,
        components: components,
        layout: layout,
    };
    let mut entities: Vec<Entity> = Vec::new();
    for e in world.extend(component_data){
        entities.push(*e);
    }

    for e in entities.iter() {
        assert_eq!(true, world.contains(*e));
    }
    
    let component_type_id = ComponentTypeId { 
            type_id: TypeId::of::<ExternalComponent>(),
            ext_type_id: Some(666),
            name: "external component"
    }
    ;
    let component_type_id2 = ComponentTypeId { 
            type_id: TypeId::of::<ExternalComponent>(),
            ext_type_id: Some(777),
            name: "external component"
    };

    for archetype in world.archetypes() {
        println!("Archetype: {:?}", archetype);
        for id in &[component_type_id, component_type_id2] {
            println!("Getting id {:?}", id);
            if(archetype.layout().has_component_by_id(*id)) {
                println!("{:?}", archetype.entities()); 
                let storage = world.components().get(*id).unwrap();
                println!("storage: {:?}", storage as *const _ as *const c_void);
                let (slice_ptr, len) = storage.get_raw(archetype.index()).expect("Failed to get raw component");
                    unsafe {
                            // let test = std::mem::transmute::<*const u8, *const c_void>(slice_ptr); 
                            // println!("t: {:?}", test);
                            println!("ptr: {:?}", slice_ptr);
                            println!("len: {}", len);
                            // let comp_ptr = slice_ptr.offset(len * ) as *const usize;
                            // println!("ptr: {:?}", comp_ptr);
                            // let slice = slice::from_raw_parts(comp_ptr as *const _, len);
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
