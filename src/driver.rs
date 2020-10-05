use std::ffi::CString;
use std::os::raw::{c_char, c_ulong, c_void};
use std::convert::TryInto;

extern {
    // fn C_run_python_file(source: *const c_char);
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
        let array: [u8;8] = slice[0..8].try_into().expect("Failed to cast to array, incorret length");
        std::mem::transmute::<[u8; 8], *const _>(array)
    }
}

