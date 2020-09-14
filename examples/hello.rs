use legion::prelude::*;
use legion_script::system::{scripting_system, ComponentId};


pub fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();

    let mut id_count = 0;
    resources.insert::<ComponentId>(id_count);
    
    let py_script = String::from("examples/python/hello.py");

    let mut schedule = Schedule::builder()
        .add_system(scripting_system(py_script))
        .build();

    schedule.execute(&mut world, &mut resources);
}
