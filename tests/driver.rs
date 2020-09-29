use legion_script::*;
use legion::*;
use legion::{storage::{ComponentTypeId, EntityLayout}};
use legion_script::system::{ComponentId, Scripts, scripting_system,ComponentData, ExternalComponent};
use legion_script::driver::convert_bytes_into_pointer;

use std::os::raw::c_void;
use std::any::TypeId;
use std::slice;


// #[test]
fn run_python_script() {
    let mut id_count = 0_u64;
    driver::run_script(
        String::from("tests/scripts/print.py"), &mut id_count);
    assert_eq!(0,0);
}

#[derive(Debug)]
struct Rotation {
    x: u32
}
#[derive(Debug)]
#[repr(C)]
pub struct Position {
    pub x: u32,
    pub y: u32
}

#[test]
fn raw_component(){
    // SimpleLogger::new().with_level(LevelFilter::Trace).init().expect("Failed to create logger");
    
    let mut world = World::default();
    let mut resources = Resources::default();

    let component_types = [666, 777].as_ptr() as *const u32;
    
    let pos = Position{x: 100, y: 50};
    let rot = Rotation{x: 50};
    let pos_ptr = &pos as *const _ as *const c_void;
    let rot_ptr = &rot as *const _ as *const c_void;
    
    let comp_array = [pos_ptr, rot_ptr];
    let components: *const *const c_void = &comp_array as *const  *const _ as *const *const c_void;
    
    let layout = EntityLayout::new();
    let component_data = ComponentData {
        component_types: component_types, 
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

                            if *id == component_type_id {
                                let comp = std::mem::transmute::<*const c_void, &Position>(test);
                                println!("CARALHO POSITION {:?}", comp);
                                assert_eq!(100, comp.x);
                                assert_eq!(50, comp.y);
                            } else if *id == component_type_id2 {
                                let comp = std::mem::transmute::<*const c_void, &Rotation>(test);
                                println!("CARALHO ROTATION {:?}", comp);
                                assert_eq!(50, comp.x);
                                assert_eq!(true,false); // sanity test
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