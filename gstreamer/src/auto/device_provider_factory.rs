// This file was generated by gir (6a48033) from gir-files (???)
// DO NOT EDIT

use DeviceProvider;
use Object;
use Rank;
use ffi;
use glib;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DeviceProviderFactory(Object<ffi::GstDeviceProviderFactory>): Object;

    match fn {
        get_type => || ffi::gst_device_provider_factory_get_type(),
    }
}

impl DeviceProviderFactory {
    pub fn get(&self) -> Option<DeviceProvider> {
        unsafe {
            from_glib_full(ffi::gst_device_provider_factory_get(self.to_glib_none().0))
        }
    }

    pub fn get_device_provider_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gst_device_provider_factory_get_device_provider_type(self.to_glib_none().0))
        }
    }

    pub fn get_metadata(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_device_provider_factory_get_metadata(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_metadata_keys(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_provider_factory_get_metadata_keys(self.to_glib_none().0))
        }
    }

    pub fn has_classes<'a, P: Into<Option<&'a str>>>(&self, classes: P) -> bool {
        let classes = classes.into();
        let classes = classes.to_glib_none();
        unsafe {
            from_glib(ffi::gst_device_provider_factory_has_classes(self.to_glib_none().0, classes.0))
        }
    }

    pub fn has_classesv(&self, classes: &[&str]) -> bool {
        unsafe {
            from_glib(ffi::gst_device_provider_factory_has_classesv(self.to_glib_none().0, classes.to_glib_none().0))
        }
    }

    pub fn find(name: &str) -> Option<DeviceProviderFactory> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_device_provider_factory_find(name.to_glib_none().0))
        }
    }

    pub fn get_by_name(factoryname: &str) -> Option<DeviceProvider> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_device_provider_factory_get_by_name(factoryname.to_glib_none().0))
        }
    }

    pub fn list_get_device_providers(minrank: Rank) -> Vec<DeviceProviderFactory> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_provider_factory_list_get_device_providers(minrank.to_glib()))
        }
    }
}

unsafe impl Send for DeviceProviderFactory {}
unsafe impl Sync for DeviceProviderFactory {}
