// This file was generated by gir (6a48033) from gir-files (???)
// DO NOT EDIT

use Bin;
use ClockTime;
use DebugGraphDetails;
use DebugLevel;
use Element;
use Error;
#[cfg(feature = "v1_12")]
use StackTraceFlags;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::mem;
use std::ptr;


pub fn debug_bin_to_dot_data<P: IsA<Bin>>(bin: &P, details: DebugGraphDetails) -> Option<String> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gst_debug_bin_to_dot_data(bin.to_glib_none().0, details.to_glib()))
    }
}

pub fn debug_bin_to_dot_file<P: IsA<Bin>>(bin: &P, details: DebugGraphDetails, file_name: &str) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_bin_to_dot_file(bin.to_glib_none().0, details.to_glib(), file_name.to_glib_none().0);
    }
}

pub fn debug_bin_to_dot_file_with_ts<P: IsA<Bin>>(bin: &P, details: DebugGraphDetails, file_name: &str) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_bin_to_dot_file_with_ts(bin.to_glib_none().0, details.to_glib(), file_name.to_glib_none().0);
    }
}

pub fn debug_get_default_threshold() -> DebugLevel {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_debug_get_default_threshold())
    }
}

#[cfg(feature = "v1_12")]
pub fn debug_get_stack_trace(flags: StackTraceFlags) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_debug_get_stack_trace(flags.to_glib()))
    }
}

pub fn debug_is_active() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_debug_is_active())
    }
}

pub fn debug_is_colored() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_debug_is_colored())
    }
}

pub fn debug_print_stack_trace() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_print_stack_trace();
    }
}

pub fn debug_set_active(active: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_active(active.to_glib());
    }
}

pub fn debug_set_colored(colored: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_colored(colored.to_glib());
    }
}

pub fn debug_set_default_threshold(level: DebugLevel) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_default_threshold(level.to_glib());
    }
}

pub fn debug_set_threshold_for_name(name: &str, level: DebugLevel) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_threshold_for_name(name.to_glib_none().0, level.to_glib());
    }
}

pub fn debug_set_threshold_from_string(list: &str, reset: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_threshold_from_string(list.to_glib_none().0, reset.to_glib());
    }
}

pub fn debug_unset_threshold_for_name(name: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_unset_threshold_for_name(name.to_glib_none().0);
    }
}

pub fn parse_bin_from_description(bin_description: &str, ghost_unlinked_pads: bool) -> Result<Option<Bin>, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_bin_from_description(bin_description.to_glib_none().0, ghost_unlinked_pads.to_glib(), &mut error);
        if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
    }
}

pub fn parse_launch(pipeline_description: &str) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launch(pipeline_description.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
    }
}

pub fn parse_launchv(argv: &[&str]) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launchv(argv.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
    }
}

pub fn update_registry() -> Result<(), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        glib::error::BoolError::from_glib(ffi::gst_update_registry(), "Failed to update registry")
    }
}

pub fn util_get_timestamp() -> ClockTime {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_get_timestamp()
    }
}

pub fn util_group_id_next() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_group_id_next()
    }
}

pub fn util_seqnum_next() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_seqnum_next()
    }
}

pub fn version() -> (u32, u32, u32, u32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut major = mem::uninitialized();
        let mut minor = mem::uninitialized();
        let mut micro = mem::uninitialized();
        let mut nano = mem::uninitialized();
        ffi::gst_version(&mut major, &mut minor, &mut micro, &mut nano);
        (major, minor, micro, nano)
    }
}

pub fn version_string() -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_version_string())
    }
}
