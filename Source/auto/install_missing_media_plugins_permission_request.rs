// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::PermissionRequest;
use glib::{prelude::*, translate::*};

glib::wrapper! {
	#[doc(alias = "WebKitInstallMissingMediaPluginsPermissionRequest")]
	pub struct InstallMissingMediaPluginsPermissionRequest(Object<ffi::WebKitInstallMissingMediaPluginsPermissionRequest, ffi::WebKitInstallMissingMediaPluginsPermissionRequestClass>) @implements PermissionRequest;

	match fn {
		type_ => || ffi::webkit_install_missing_media_plugins_permission_request_get_type(),
	}
}

impl InstallMissingMediaPluginsPermissionRequest {
	pub const NONE: Option<&'static InstallMissingMediaPluginsPermissionRequest> = None;
}

mod sealed {
	pub trait Sealed {}
	impl<T: super::IsA<super::InstallMissingMediaPluginsPermissionRequest>> Sealed for T {}
}

pub trait InstallMissingMediaPluginsPermissionRequestExt:
	IsA<InstallMissingMediaPluginsPermissionRequest> + sealed::Sealed + 'static
{
	#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_install_missing_media_plugins_permission_request_get_description")]
	#[doc(alias = "get_description")]
	fn description(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(
				ffi::webkit_install_missing_media_plugins_permission_request_get_description(
					self.as_ref().to_glib_none().0,
				),
			)
		}
	}
}

impl<O: IsA<InstallMissingMediaPluginsPermissionRequest>>
	InstallMissingMediaPluginsPermissionRequestExt for O
{
}
