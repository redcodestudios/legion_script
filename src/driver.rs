use std::ffi::CString;
use std::os::raw::c_char;

extern {
    fn LUA_ON_CREATE(source: *const c_char);
    fn LUA_ON_UPDATE(source: *const c_char);
}

pub fn create_script(path: String) -> Result<(), &'static str> {
   unsafe {
        LUA_ON_CREATE(CString::new(path).expect("Failed to convert to CStr").as_ptr());
   }

   Ok(())
}

pub fn update_script(path: String) -> Result<(), &'static str> {
   unsafe {
        LUA_ON_UPDATE(CString::new(path).expect("Failed to convert to CStr").as_ptr());
   }

   Ok(())
}

