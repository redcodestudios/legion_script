use legion::prelude::*;
use legion_script::system::scripting_system;

pub fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();

    let lua_script = String::from("examples/lua/hello.lua");

    let mut schedule = Schedule::builder()
        .add_system(scripting_system(lua_script))
        .build();

    schedule.execute(&mut world, &mut resources);
}
