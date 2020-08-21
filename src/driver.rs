use std::ffi::CString;
use std::os::raw::c_char;

extern {
    fn C_run_python_file(source: *const c_char);
}

pub fn run_script(path: String) -> Result<(), &'static str> {
   unsafe {
        C_run_python_file(CString::new(path).expect("Failed to convert to CStr").as_ptr());
   }

   Ok(())
}
