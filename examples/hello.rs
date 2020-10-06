use legion::*;

use legion_script::{
    system::{scripting_system, Scripts, ComponentId},
    query::{get_external_components_ids},
    utils::{create_test_component_data},
    components::{Position, Rotation},
    query::{get_external_components}
};

use std::os::raw::c_void;
use simple_logger::{SimpleLogger};
use log::*;
use std::sync::Arc;
pub fn init_logger(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger::new()))
        .map(|()| log::set_max_level(level))
}

static mut world2: Arc<*const legion::world::World> = Arc::new(std::ptr::null());

pub fn main() {
    init_logger(LevelFilter::Trace).expect("Failed to create logger");

    // let c = ComponentTypeId{type_id: TypeId::of::<ExternalComponent>(), ext_type_id: Some(22), name: "meu_type" };

    let mut world = World::default();
    let mut resources = Resources::default();
    world2 = Arc::new(&world as *const legion::world::World);
    
    // let component_data = create_test_component_data();
    // let entities = world.extend(component_data); 
    
    
    // unsafe{
    //     let position = std::mem::transmute::<*const c_void, &Position>(components[0]);
    //     debug!("POSITION {:?}", position);
    // }
    // unsafe{
    //     let rotation = std::mem::transmute::<*const c_void, &Rotation>(components[1]);
    //     debug!("ROTATION {:?}", rotation);
    // }

    
    let id_count = 0;
    resources.insert::<ComponentId>(id_count);
    
    let scripts = vec![String::from("examples/python/hello.py")/*, String::from("examples/python/hello2.py")*/];
    resources.insert::<Scripts>(scripts);

    unsafe{
        let mut schedule = Schedule::builder()
        .add_system(scripting_system(world2.clone()))
        .build();
        
        schedule.execute(&mut world, &mut resources);
    } 
        std::thread::sleep(std::time::Duration::from_millis(500));

    let component_type_ids = get_external_components_ids();
    let mut components: Vec<*const c_void> = vec![];
    get_external_components(&world, component_type_ids.to_vec(), &mut components);
   
    debug!("COMPONENTS {:?}", components);
}
