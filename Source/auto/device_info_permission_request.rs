// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use crate::PermissionRequest;

glib::wrapper! {
	#[doc(alias = "WebKitDeviceInfoPermissionRequest")]
	pub struct DeviceInfoPermissionRequest(Object<ffi::WebKitDeviceInfoPermissionRequest, ffi::WebKitDeviceInfoPermissionRequestClass>) @implements PermissionRequest;

	match fn {
		type_ => || ffi::webkit_device_info_permission_request_get_type(),
	}
}

impl DeviceInfoPermissionRequest {
	pub const NONE: Option<&'static DeviceInfoPermissionRequest> = None;
}
