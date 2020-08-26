use legion::prelude::*;
use crate::driver::{create_script, update_script};

pub fn scripting_system(script: String) -> Box<dyn Schedulable + 'static> {
    let _ = create_script(script.clone());
    SystemBuilder::<()>::new("ScriptingSystem")
           .build(move |_commands, _world, _resource, _queries|{
               //@TODO: pass world to script
               update_script(script.clone());
           })
}
