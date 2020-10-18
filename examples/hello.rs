use legion::*;

use legion_script::{
    system::{create_scripting_system, ComponentId},
};

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
    
    let scripts = vec![String::from("examples/python/hello.py"), String::from("examples/python/hello2.py")];
    
    static mut ID_COUNT: u64 = 0;
    unsafe{
        resources.insert::<ComponentId>(ID_COUNT);
            
        let mut schedule = Schedule::builder()
            .add_thread_local_fn(create_scripting_system(scripts, &mut ID_COUNT))
            .build();   
        
        schedule.execute(&mut world, &mut resources);
    }
}

