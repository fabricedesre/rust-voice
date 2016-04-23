use libc::{c_int, c_char};

use super::pocketsphinx::ps_decoder_t;

#[allow(non_camel_case_types)] pub enum ps_search_iter_t {}
#[allow(non_camel_case_types)] pub enum ngram_model_t {}
#[allow(non_camel_case_types)] pub enum fsg_model_t {}

#[link(name="pocketsphinx")]
extern {

    pub fn ps_set_search(ps: *mut ps_decoder_t, name: *const c_char) -> c_int;
    pub fn ps_get_search(ps: *const ps_decoder_t) -> *const c_char;
    pub fn ps_unset_search(ps: *mut ps_decoder_t, name: *const c_char) -> c_int;

    pub fn ps_search_iter(ps: *const ps_decoder_t) -> *mut ps_search_iter_t;
    pub fn ps_search_iter_next(itor: *mut ps_search_iter_t) -> *mut ps_search_iter_t;
    pub fn ps_search_iter_val(itor: *mut ps_search_iter_t) -> *const c_char;
    pub fn ps_search_iter_free(itor: *mut ps_search_iter_t);

    pub fn ps_get_lm(ps: *const ps_decoder_t, name: *const c_char) -> *mut ngram_model_t;
    pub fn ps_set_lm(ps: *mut ps_decoder_t, name: *const c_char, lm: *mut ngram_model_t) -> c_int;
    pub fn ps_set_lm_file(ps: *mut ps_decoder_t, name: *const c_char, path: *const c_char) -> c_int;

    pub fn ps_get_fsg(ps: *const ps_decoder_t, name: *const c_char) -> *mut fsg_model_t;
    pub fn ps_set_fsg(ps: *mut ps_decoder_t, name: *const c_char, fsg: *const fsg_model_t) -> c_int;
    pub fn ps_set_jsgf_file(ps: *mut ps_decoder_t, name: *const c_char, path: *const c_char) -> c_int;
    pub fn ps_set_jsgf_string(ps: *mut ps_decoder_t, name: *const c_char, jsgf_string: *const c_char) -> c_int;

    pub fn ps_get_kws(ps: *const ps_decoder_t, name: *const c_char) -> *const c_char;
    pub fn ps_set_kws(ps: *mut ps_decoder_t, name: *const c_char, keyfile: *const c_char) -> c_int;
    pub fn ps_set_keyphrase(ps: *mut ps_decoder_t, name: *const c_char, keyphrase: *const c_char) -> c_int;

    pub fn ps_set_allphone(ps: *mut ps_decoder_t, name: *const c_char, lm: *mut ngram_model_t) -> c_int;
    pub fn ps_set_allphone_file(ps: *mut ps_decoder_t, name: *const c_char, path: *const c_char) -> c_int;

}
