use legion::*;
use crate::driver::run_script;

pub type ComponentId = u64;
pub type Scripts = Vec<String>;

#[system]
pub fn scripting(#[resource] scripts: &mut Scripts, #[resource] component_id: &mut ComponentId) {
    run_script(scripts[0].clone(), component_id);
    run_script(scripts[1].clone(), component_id);
    println!("Resource after running {}", *component_id);
}

//pub fn scripting_system(script: String, script2: String) -> systems::System {
//    SystemBuilder::<()>::new("ScriptingSystem")
//        .write_resource::<ComponentId>()
//        .build(move |_commands, _world, _resource, _queries|{
//            //@TODO: pass world to script
//            run_script(script.clone(), _resource);
//            run_script(script2.clone(), _resource);
//            println!("Resource after running {}", **_resource);
//        })
//}
