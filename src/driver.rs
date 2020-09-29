use std::ffi::CString;
use std::os::raw::{c_char, c_ulong, c_void};
use legion::storage::ComponentTypeId;
use crate::system::ExternalComponent;
use std::any::TypeId;

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

pub fn get_external_components_ids() -> [ComponentTypeId;2]{
    [ComponentTypeId { 
        type_id: TypeId::of::<ExternalComponent>(),
        ext_type_id: Some(666),
        name: "external component"
    }, ComponentTypeId { 
        type_id: TypeId::of::<ExternalComponent>(),
        ext_type_id: Some(777),
        name: "external component"
}]
}