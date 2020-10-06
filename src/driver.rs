use std::ffi::CString;
use std::os::raw::{c_char, c_ulong, c_void};
use std::convert::TryInto;

use crate::c_api::World;

extern {
    // fn C_run_python_file(source: *const c_char);
    fn C_RUN_PYSCRIPT(world: *mut World, source: *const c_char, component_id: *mut c_ulong);
}

pub fn run_script(world: *mut legion::world::World, path: String, component_id: &mut u64) -> Result<(), &'static str> {
   unsafe {
       let boxed = Box::new(world);
        // C_run_python_file(CString::new(path).expect("Failed to convert to CStr").as_ptr());
       C_RUN_PYSCRIPT(Box::into_raw(boxed) as *mut World, CString::new(path).expect("Failed to convert to CStr").as_ptr(), component_id);
   }

   Ok(())
}


pub fn convert_bytes_into_pointer(slice: &[u8]) -> *const c_void{
    unsafe{
        let array: [u8;8] = slice[0..8].try_into().expect("Failed to cast to array, incorret length");
        std::mem::transmute::<[u8; 8], *const _>(array)
    }
}

