use libc::{c_int, c_char};

use super::hash_table::*;

#[allow(non_camel_case_types)] pub enum jsgf_t {}
#[allow(non_camel_case_types)] pub type jsgf_rule_iter_t = hash_iter_t;

/// These definitions are extracted from implementation headers,
/// they are not publicly available and thus are subject to change
/// without notification. Use them at your own risk.
pub mod internal {

    use libc::{c_int, c_char};
    use super::super::glist::glist_t;

    #[repr(C)]
    pub struct jsgf_rhs_t {
        pub atoms: glist_t,
        pub alt: *const jsgf_rhs_t,
    }

    #[repr(C)]
    pub struct jsgf_atom_t {
        pub name: *const c_char,
        pub tags: glist_t,
        pub weight: f32,
    }

    #[repr(C)]
    pub struct jsgf_rule_s {
        pub refcnt: c_int,
        pub name: *const c_char,
        pub is_public: c_int,
        pub rhs: *const jsgf_rhs_t,
    }

}

#[allow(non_camel_case_types)] pub enum jsgf_rule_t {}
#[allow(non_camel_case_types)] pub enum jsfg_rule_iter_t {}

#[link(name="pocketsphinx")]
extern {

    pub fn jsgf_grammar_new(parent: *const jsgf_t) -> *mut jsgf_t;
    pub fn jsgf_parse_file(filename: *const c_char, parent: *const jsgf_t) -> *mut jsgf_t;
    pub fn jsgf_parse_string(s: *const c_char, parent: *const jsgf_t) -> *mut jsgf_t;
    pub fn jsgf_grammar_name(jsgf: *const jsgf_t) -> *const c_char;
    pub fn jsgf_grammar_free(jsgf: *mut jsgf_t);
    pub fn jsgf_rule_iter(grammar: *const jsgf_t) -> *mut jsgf_rule_iter_t;
    pub fn jsgf_get_rule(grammar: *const jsgf_t, name: *const c_char) -> *const jsgf_rule_t;
    pub fn jsgf_get_public_rule(grammar: *const jsgf_t) -> *const jsgf_rule_t;
    pub fn jsgf_rule_name(rule: *const jsgf_rule_t) -> *const c_char;
    pub fn jsgf_rule_public(rule: *const jsgf_rule_t) -> c_int;
    //pub fn jsgf_build_fsg(grammar: *const jsgf_t, rule: *const jsgf_rule_t, lmath: *const logmath_t, lw: f32) -> *mut fsg_model_t;
    //pub fn jsgf_build_fsg_raw(grammar: *const jsgf_t, rule: *const jsgf_rule_t, lmath: *const logmath_t, lw: f32) -> *mut fsg_model_t;
    //pub fn jsgf_read_file(file: *const c_char, lmath: *const logmath_t, lw: f32) -> *mut fsg_model_t;
    //pub fn jsgf_read_string(s: *const c_char, lmath: *const logmath_t, lw: f32) -> *mut fsg_model_t;
    //pub fn jsgf_write_fsg(grammar: *const jsgf_t, rule: *const jsgf_rule_t, outfh: *mut libc::FILE) -> c_int;

}

// Following functions are Rust implementation of C macros
pub unsafe fn jsgf_rule_iter_next(it: *const jsgf_rule_iter_t) -> *const jsgf_rule_iter_t {
    hash_table_iter_next(it)
}
pub unsafe fn jsgf_rule_iter_rule(it: *const jsgf_rule_iter_t) -> *const jsgf_rule_t {
    hash_entry_val((*it).ent) as *const jsgf_rule_t
}
pub unsafe fn jsgf_rule_iter_free(it: *const jsgf_rule_iter_t) {
    hash_table_iter_free(it);
}
