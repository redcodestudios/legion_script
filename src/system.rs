use legion::prelude::*;
use crate::driver::run_script;

pub fn scripting_system(script: String) -> Box<dyn Schedulable + 'static>{
    SystemBuilder::<()>::new("ScriptingSystem")
           .build(move |_commands, _world, _resource, _queries|{
               //@TODO: pass world to script
               run_script(script.clone());
           })
}
