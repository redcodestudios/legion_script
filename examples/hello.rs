use legion::*;

use legion_script::{
    system::{/*local_scripting_system, */Scripts, ComponentId},
    query::{get_external_components_ids},
    utils::{create_test_component_data},
    components::{Position, Rotation},
    query::{get_external_components},
    driver::run_script,
};

use std::os::raw::c_void;
use simple_logger::{SimpleLogger};
use log::*;


pub fn init_logger(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger::new()))
        .map(|()| log::set_max_level(level))
}

pub fn main() {
    init_logger(LevelFilter::Trace).expect("Failed to create logger");

    let mut world = World::default();
    let mut resources = Resources::default();
    
    let id_count = 0;
    resources.insert::<ComponentId>(id_count);
    
    let scripts = vec![String::from("examples/python/hello.py"), String::from("examples/python/hello2.py")];

    let mut schedule = Schedule::builder()
        .add_thread_local_fn(move |_world, _resources|{
            debug!("World len do system {}", _world.len());
            run_script(_world, scripts[0].clone(), &mut 1);
        // run_script(_world, scripts[1].clone(), &mut 1);
        })
        .build();

    schedule.execute(&mut world, &mut resources);
}
