// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/wusyong/gir-files)
// DO NOT EDIT

use crate::MimeInfo;
use glib::{object::IsA, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitPlugin")]
    pub struct Plugin(Object<ffi::WebKitPlugin, ffi::WebKitPluginClass>);

    match fn {
        type_ => || ffi::webkit_plugin_get_type(),
    }
}

pub const NONE_PLUGIN: Option<&Plugin> = None;

pub trait PluginExt: 'static {
  #[cfg_attr(feature = "v2_32", deprecated = "Since 2.32")]
  #[doc(alias = "webkit_plugin_get_description")]
  #[doc(alias = "get_description")]
  fn description(&self) -> Option<glib::GString>;

  #[cfg_attr(feature = "v2_32", deprecated = "Since 2.32")]
  #[doc(alias = "webkit_plugin_get_mime_info_list")]
  #[doc(alias = "get_mime_info_list")]
  fn mime_info_list(&self) -> Vec<MimeInfo>;

  #[cfg_attr(feature = "v2_32", deprecated = "Since 2.32")]
  #[doc(alias = "webkit_plugin_get_name")]
  #[doc(alias = "get_name")]
  fn name(&self) -> Option<glib::GString>;

  #[cfg_attr(feature = "v2_32", deprecated = "Since 2.32")]
  #[doc(alias = "webkit_plugin_get_path")]
  #[doc(alias = "get_path")]
  fn path(&self) -> Option<glib::GString>;
}

impl<O: IsA<Plugin>> PluginExt for O {
  fn description(&self) -> Option<glib::GString> {
    unsafe {
      from_glib_none(ffi::webkit_plugin_get_description(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  fn mime_info_list(&self) -> Vec<MimeInfo> {
    unsafe {
      FromGlibPtrContainer::from_glib_none(ffi::webkit_plugin_get_mime_info_list(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  fn name(&self) -> Option<glib::GString> {
    unsafe { from_glib_none(ffi::webkit_plugin_get_name(self.as_ref().to_glib_none().0)) }
  }

  fn path(&self) -> Option<glib::GString> {
    unsafe { from_glib_none(ffi::webkit_plugin_get_path(self.as_ref().to_glib_none().0)) }
  }
}

impl fmt::Display for Plugin {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("Plugin")
  }
}
