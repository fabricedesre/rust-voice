use std::mem::transmute_copy;
use libc::c_void;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct anytype_t([u8; 8]);

impl anytype_t {
    pub unsafe fn as_ptr(&self) -> *const c_void { transmute_copy(&self.0) }
    pub unsafe fn as_i32(&self) -> *const c_void { transmute_copy(&self.0) }
    pub unsafe fn as_u32(&self) -> *const c_void { transmute_copy(&self.0) }
    pub unsafe fn as_f64(&self) -> *const c_void { transmute_copy(&self.0) }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct gnode_t {
    pub data: anytype_t,
    pub next: *const gnode_t,
}

#[allow(non_camel_case_types)]
pub type glist_t = *const gnode_t;
