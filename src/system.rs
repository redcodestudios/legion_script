use legion::*;

use crate::{
    driver::run_script,
};

pub type ComponentId = u64;
pub type Scripts = Vec<String>;

#[system]
pub fn scripting(#[state]world: &mut legion::world::World, #[resource] scripts: &mut Scripts, #[resource] component_id: &mut ComponentId) {
    println!("scripting - start");
    run_script(world, scripts[0].clone(), component_id);
    // run_script(scripts[1].clone(), component_id);
    println!("scripting - end - Resource after running {}", *component_id);
}
