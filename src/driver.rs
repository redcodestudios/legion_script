use std::ffi::CString;
use std::os::raw::{c_char, c_ulong};

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
