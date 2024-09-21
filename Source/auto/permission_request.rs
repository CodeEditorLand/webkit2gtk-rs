// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};

glib::wrapper! {
	#[doc(alias = "WebKitPermissionRequest")]
	pub struct PermissionRequest(Interface<ffi::WebKitPermissionRequest, ffi::WebKitPermissionRequestIface>);

	match fn {
		type_ => || ffi::webkit_permission_request_get_type(),
	}
}

impl PermissionRequest {
	pub const NONE: Option<&'static PermissionRequest> = None;
}

mod sealed {
	pub trait Sealed {}
	impl<T: super::IsA<super::PermissionRequest>> Sealed for T {}
}

pub trait PermissionRequestExt: IsA<PermissionRequest> + sealed::Sealed + 'static {
	#[doc(alias = "webkit_permission_request_allow")]
	fn allow(&self) {
		unsafe {
			ffi::webkit_permission_request_allow(self.as_ref().to_glib_none().0);
		}
	}

	#[doc(alias = "webkit_permission_request_deny")]
	fn deny(&self) {
		unsafe {
			ffi::webkit_permission_request_deny(self.as_ref().to_glib_none().0);
		}
	}
}

impl<O: IsA<PermissionRequest>> PermissionRequestExt for O {}
