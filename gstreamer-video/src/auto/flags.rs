// This file was generated by gir (cf27827) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

bitflags! {
    pub struct VideoChromaSite: u32 {
        const VIDEO_CHROMA_SITE_UNKNOWN = 0;
        const VIDEO_CHROMA_SITE_NONE = 1;
        const VIDEO_CHROMA_SITE_H_COSITED = 2;
        const VIDEO_CHROMA_SITE_V_COSITED = 4;
        const VIDEO_CHROMA_SITE_ALT_LINE = 8;
        const VIDEO_CHROMA_SITE_COSITED = 6;
        const VIDEO_CHROMA_SITE_JPEG = 1;
        const VIDEO_CHROMA_SITE_MPEG2 = 2;
        const VIDEO_CHROMA_SITE_DV = 14;
    }
}

#[doc(hidden)]
impl ToGlib for VideoChromaSite {
    type GlibType = ffi::GstVideoChromaSite;

    fn to_glib(&self) -> ffi::GstVideoChromaSite {
        ffi::GstVideoChromaSite::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoChromaSite> for VideoChromaSite {
    fn from_glib(value: ffi::GstVideoChromaSite) -> VideoChromaSite {
        skip_assert_initialized!();
        VideoChromaSite::from_bits_truncate(value.bits())
    }
}

impl StaticType for VideoChromaSite {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_chroma_site_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoChromaSite {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoChromaSite {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstVideoChromaSite::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for VideoChromaSite {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct VideoFlags: u32 {
        const VIDEO_FLAG_NONE = 0;
        const VIDEO_FLAG_VARIABLE_FPS = 1;
        const VIDEO_FLAG_PREMULTIPLIED_ALPHA = 2;
    }
}

#[doc(hidden)]
impl ToGlib for VideoFlags {
    type GlibType = ffi::GstVideoFlags;

    fn to_glib(&self) -> ffi::GstVideoFlags {
        ffi::GstVideoFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFlags> for VideoFlags {
    fn from_glib(value: ffi::GstVideoFlags) -> VideoFlags {
        skip_assert_initialized!();
        VideoFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for VideoFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstVideoFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for VideoFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct VideoFormatFlags: u32 {
        const VIDEO_FORMAT_FLAG_YUV = 1;
        const VIDEO_FORMAT_FLAG_RGB = 2;
        const VIDEO_FORMAT_FLAG_GRAY = 4;
        const VIDEO_FORMAT_FLAG_ALPHA = 8;
        const VIDEO_FORMAT_FLAG_LE = 16;
        const VIDEO_FORMAT_FLAG_PALETTE = 32;
        const VIDEO_FORMAT_FLAG_COMPLEX = 64;
        const VIDEO_FORMAT_FLAG_UNPACK = 128;
        const VIDEO_FORMAT_FLAG_TILED = 256;
    }
}

#[doc(hidden)]
impl ToGlib for VideoFormatFlags {
    type GlibType = ffi::GstVideoFormatFlags;

    fn to_glib(&self) -> ffi::GstVideoFormatFlags {
        ffi::GstVideoFormatFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFormatFlags> for VideoFormatFlags {
    fn from_glib(value: ffi::GstVideoFormatFlags) -> VideoFormatFlags {
        skip_assert_initialized!();
        VideoFormatFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for VideoFormatFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_format_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoFormatFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoFormatFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstVideoFormatFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for VideoFormatFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct VideoMultiviewFlags: u32 {
        const VIDEO_MULTIVIEW_FLAGS_NONE = 0;
        const VIDEO_MULTIVIEW_FLAGS_RIGHT_VIEW_FIRST = 1;
        const VIDEO_MULTIVIEW_FLAGS_LEFT_FLIPPED = 2;
        const VIDEO_MULTIVIEW_FLAGS_LEFT_FLOPPED = 4;
        const VIDEO_MULTIVIEW_FLAGS_RIGHT_FLIPPED = 8;
        const VIDEO_MULTIVIEW_FLAGS_RIGHT_FLOPPED = 16;
        const VIDEO_MULTIVIEW_FLAGS_HALF_ASPECT = 16384;
        const VIDEO_MULTIVIEW_FLAGS_MIXED_MONO = 32768;
    }
}

#[doc(hidden)]
impl ToGlib for VideoMultiviewFlags {
    type GlibType = ffi::GstVideoMultiviewFlags;

    fn to_glib(&self) -> ffi::GstVideoMultiviewFlags {
        ffi::GstVideoMultiviewFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoMultiviewFlags> for VideoMultiviewFlags {
    fn from_glib(value: ffi::GstVideoMultiviewFlags) -> VideoMultiviewFlags {
        skip_assert_initialized!();
        VideoMultiviewFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for VideoMultiviewFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_multiview_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoMultiviewFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoMultiviewFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstVideoMultiviewFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for VideoMultiviewFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

