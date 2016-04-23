use libc::{c_char, c_void, size_t};

#[allow(non_camel_case_types)] pub enum hash_table_t {}

#[repr(C)]
pub struct hash_entry_t {
    key: *const c_char,
    len: size_t,
    val: *const c_void,
    next: *const hash_entry_t,
}

#[repr(C)]
pub struct hash_iter_t {
    pub ht: *const hash_table_t,
    pub ent: *const hash_entry_t,
    pub idx: size_t,
}

#[link(name="pocketsphinx")]
extern {

    pub fn hash_table_iter(h: *const hash_table_t) -> *const hash_iter_t;
    pub fn hash_table_iter_next(itor: *const hash_iter_t) -> *const hash_iter_t;
    pub fn hash_table_iter_free(itor: *const hash_iter_t);

}

// Macros
pub unsafe fn hash_entry_val(e: *const hash_entry_t) -> *const c_void { (*e).val }
pub unsafe fn hash_entry_key(e: *const hash_entry_t) -> *const c_char { (*e).key }
pub unsafe fn hash_entry_len(e: *const hash_entry_t) -> size_t { (*e).len }
