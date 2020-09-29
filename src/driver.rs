use std::ffi::CString;
use std::os::raw::{c_char, c_ulong, c_void};

extern {
    fn C_run_python_file(source: *const c_char);
    fn C_RUN_PYSCRIPT(source: *const c_char, component_id: *mut c_ulong);

}

pub fn run_script(path: String, component_id: &mut u64) -> Result<(), &'static str> {
   unsafe {
        // C_run_python_file(CString::new(path).expect("Failed to convert to CStr").as_ptr());
       C_RUN_PYSCRIPT(CString::new(path).expect("Failed to convert to CStr").as_ptr(), component_id);
   }

   Ok(())
}


pub fn convert_bytes_into_pointer(slice: &[u8]) -> *const c_void{
    unsafe{
        std::mem::transmute::<[u8; 8], *const _>([slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7]])
    }
}