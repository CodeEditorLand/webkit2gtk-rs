// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
	#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct ApplicationInfo(Shared<ffi::WebKitApplicationInfo>);

	match fn {
		ref => |ptr| ffi::webkit_application_info_ref(ptr),
		unref => |ptr| ffi::webkit_application_info_unref(ptr),
		type_ => || ffi::webkit_application_info_get_type(),
	}
}

impl ApplicationInfo {
	#[doc(alias = "webkit_application_info_new")]
	pub fn new() -> ApplicationInfo {
		assert_initialized_main_thread!();
		unsafe { from_glib_full(ffi::webkit_application_info_new()) }
	}

	#[doc(alias = "webkit_application_info_get_name")]
	#[doc(alias = "get_name")]
	pub fn name(&self) -> Option<glib::GString> {
		unsafe { from_glib_none(ffi::webkit_application_info_get_name(self.to_glib_none().0)) }
	}

	#[doc(alias = "webkit_application_info_get_version")]
	#[doc(alias = "get_version")]
	pub fn version(&self) -> (u64, u64, u64) {
		unsafe {
			let mut major = std::mem::MaybeUninit::uninit();
			let mut minor = std::mem::MaybeUninit::uninit();
			let mut micro = std::mem::MaybeUninit::uninit();
			ffi::webkit_application_info_get_version(
				self.to_glib_none().0,
				major.as_mut_ptr(),
				minor.as_mut_ptr(),
				micro.as_mut_ptr(),
			);
			(major.assume_init(), minor.assume_init(), micro.assume_init())
		}
	}

	#[doc(alias = "webkit_application_info_set_name")]
	pub fn set_name(&self, name: &str) {
		unsafe {
			ffi::webkit_application_info_set_name(self.to_glib_none().0, name.to_glib_none().0);
		}
	}

	#[doc(alias = "webkit_application_info_set_version")]
	pub fn set_version(&self, major: u64, minor: u64, micro: u64) {
		unsafe {
			ffi::webkit_application_info_set_version(self.to_glib_none().0, major, minor, micro);
		}
	}
}

#[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
impl Default for ApplicationInfo {
	fn default() -> Self {
		Self::new()
	}
}
