// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
	#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct SecurityOrigin(Shared<ffi::WebKitSecurityOrigin>);

	match fn {
		ref => |ptr| ffi::webkit_security_origin_ref(ptr),
		unref => |ptr| ffi::webkit_security_origin_unref(ptr),
		type_ => || ffi::webkit_security_origin_get_type(),
	}
}

impl SecurityOrigin {
	#[doc(alias = "webkit_security_origin_new")]
	pub fn new(protocol:&str, host:&str, port:u16) -> SecurityOrigin {
		assert_initialized_main_thread!();

		unsafe {
			from_glib_full(ffi::webkit_security_origin_new(
				protocol.to_glib_none().0,
				host.to_glib_none().0,
				port,
			))
		}
	}

	#[doc(alias = "webkit_security_origin_new_for_uri")]
	#[doc(alias = "new_for_uri")]
	pub fn for_uri(uri:&str) -> SecurityOrigin {
		assert_initialized_main_thread!();

		unsafe { from_glib_full(ffi::webkit_security_origin_new_for_uri(uri.to_glib_none().0)) }
	}

	#[doc(alias = "webkit_security_origin_get_host")]
	#[doc(alias = "get_host")]
	pub fn host(&self) -> Option<glib::GString> {
		unsafe { from_glib_none(ffi::webkit_security_origin_get_host(self.to_glib_none().0)) }
	}

	#[doc(alias = "webkit_security_origin_get_port")]
	#[doc(alias = "get_port")]
	pub fn port(&self) -> u16 {
		unsafe { ffi::webkit_security_origin_get_port(self.to_glib_none().0) }
	}

	#[doc(alias = "webkit_security_origin_get_protocol")]
	#[doc(alias = "get_protocol")]
	pub fn protocol(&self) -> Option<glib::GString> {
		unsafe { from_glib_none(ffi::webkit_security_origin_get_protocol(self.to_glib_none().0)) }
	}

	#[cfg_attr(feature = "v2_32", deprecated = "Since 2.32")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_security_origin_is_opaque")]
	pub fn is_opaque(&self) -> bool {
		unsafe { from_glib(ffi::webkit_security_origin_is_opaque(self.to_glib_none().0)) }
	}

	#[doc(alias = "webkit_security_origin_to_string")]
	#[doc(alias = "to_string")]
	pub fn to_str(&self) -> glib::GString {
		unsafe { from_glib_full(ffi::webkit_security_origin_to_string(self.to_glib_none().0)) }
	}
}

impl std::fmt::Display for SecurityOrigin {
	#[inline]
	fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result { f.write_str(&self.to_str()) }
}
