use legion::*;
use crate::driver::run_script;

pub type ComponentId = u64;
pub type Scripts = Vec<String>;

use log::*;

pub fn create_scripting_system(scripts: Vec<String>, id_count: &mut u64)->Box<dyn FnMut(&mut legion::World, &mut legion::Resources) + '_>{
    Box::new(move |_world: &mut legion::World, _resources|{
        debug!("World len do system {}", _world.len());
        crate::driver::run_script(_world, scripts[0].clone(), id_count);
        // run_script(_world, scripts[1].clone(), &mut 1);
    })
}

// #[system]
// pub fn scripting(#[resource] scripts: &mut Scripts, #[resource] component_id: &mut ComponentId) {
//     run_script(scripts[0].clone(), component_id);
//     run_script(scripts[1].clone(), component_id);
//     println!("Resource after running {}", *component_id);
// }

//pub fn scripting_system(script: String, script2: String) -> systems::System {
//    SystemBuilder::<()>::new("ScriptingSystem")
//        .write_resource::<ComponentId>()
//        .build(move |_commands, _subworld, _resource, _queries|{
//            //@TODO: pass world to script
//            run_script(script.clone(), _resource);
//            run_script(script2.clone(), _resource);
//            println!("Resource after running {}", **_resource);
//        })
//}

// pub fn local_scripting_system(script: String, script2: String) -> Box<dyn FnMut(&mut legion::world::World, &mut legion::Resources) -> ()> {
//     let mut sys = move |_world, _resources|{
//         run_script(_world, script.clone(), &mut 1);
//         run_script(_world, script2.clone(), &mut 1);
//     };

//     Box::new(sys)
// }
