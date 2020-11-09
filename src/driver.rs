use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::{c_char, c_ulong, c_void};

use crate::c_api::World;
use log::*;
extern "C" {
    fn C_RUN_PYSCRIPT(world: *mut World, source: *const c_char, component_id: *mut c_ulong);
}

pub fn run_script(
    world: &mut legion::world::World,
    path: String,
    component_id: &mut u64,
) -> Result<(), &'static str> {
    unsafe {
        C_RUN_PYSCRIPT(
            world as *mut legion::World as *mut World,
            CString::new(path)
                .expect("Failed to convert to CStr")
                .as_ptr(),
            component_id,
        );
    }

    Ok(())
}

pub fn convert_bytes_into_pointer(slice: &[u8]) -> *const c_void {
    debug!("Slice vec to transmute {:?}", slice);
    unsafe {
        let array: [u8; 8] = slice[0..8]
            .try_into()
            .expect("Failed to cast to array, incorret length");
        std::mem::transmute::<[u8; 8], *const _>(array)
    }
}
