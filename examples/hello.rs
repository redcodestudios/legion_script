use legion::*;
use legion_script::system::{scripting_system, Scripts, ComponentId};

use std::os::raw::c_void;


#[derive(Clone, Debug, PartialEq)]
struct TestPTR {
    x: *const c_void, 
}

unsafe impl Send for TestPTR {}
unsafe impl Sync for TestPTR {}

pub fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();


    let teste = TestPTR {x: std::ptr::null()};
    
    world.push((teste,));

    let id_count = 0;
    resources.insert::<ComponentId>(id_count);
    
    let scripts = vec![String::from("examples/python/hello.py"), String::from("examples/python/hello2.py")];
    resources.insert::<Scripts>(scripts);


    let mut schedule = Schedule::builder()
        .add_system(scripting_system())
        .build();

    schedule.execute(&mut world, &mut resources);
}
