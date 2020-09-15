use legion::prelude::*;
use crate::driver::run_script;

pub type ComponentId = u64;

pub fn scripting_system(script: String, script2: String) -> Box<dyn Schedulable + 'static>{
    SystemBuilder::<()>::new("ScriptingSystem")
        .write_resource::<ComponentId>()
        .build(move |_commands, _world, _resource, _queries|{
            //@TODO: pass world to script
            run_script(script.clone(), _resource);
            run_script(script2.clone(), _resource);
            println!("Resource after running {}", **_resource);
        })
}
