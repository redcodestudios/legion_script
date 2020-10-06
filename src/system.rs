use legion::*;

use crate::{
    driver::run_script,
};

pub type ComponentId = u64;
pub type Scripts = Vec<String>;
use std::sync::Arc;
#[system]
pub fn scripting(#[state] world: &mut Arc<*const legion::world::World>, #[resource] scripts: &mut Scripts, #[resource] component_id: &mut ComponentId) {
    println!("scripting - start");
    unsafe{
        let world: &mut *const legion::world::World = Arc::get_mut(world).unwrap(); 
        run_script(*world as *mut legion::world::World, scripts[0].clone(), component_id);
    }
    // run_script(scripts[1].clone(), component_id);
    println!("scripting - end - Resource after running {}", *component_id);
}
