// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use std::boxed::Box as Box_;

use glib::{
	prelude::*,
	signal::{connect_raw, SignalHandlerId},
	translate::*,
};

use crate::PermissionRequest;

glib::wrapper! {
	#[doc(alias = "WebKitUserMediaPermissionRequest")]
	pub struct UserMediaPermissionRequest(Object<ffi::WebKitUserMediaPermissionRequest, ffi::WebKitUserMediaPermissionRequestClass>) @implements PermissionRequest;

	match fn {
		type_ => || ffi::webkit_user_media_permission_request_get_type(),
	}
}

impl UserMediaPermissionRequest {
	pub const NONE:Option<&'static UserMediaPermissionRequest> = None;
}

mod sealed {
	pub trait Sealed {}
	impl<T:super::IsA<super::UserMediaPermissionRequest>> Sealed for T {}
}

pub trait UserMediaPermissionRequestExt:
	IsA<UserMediaPermissionRequest> + sealed::Sealed + 'static {
	#[cfg(feature = "v2_8")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
	#[doc(alias = "is-for-audio-device")]
	fn is_for_audio_device(&self) -> bool {
		ObjectExt::property(self.as_ref(), "is-for-audio-device")
	}

	#[cfg(feature = "v2_8")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
	#[doc(alias = "is-for-video-device")]
	fn is_for_video_device(&self) -> bool {
		ObjectExt::property(self.as_ref(), "is-for-video-device")
	}

	#[cfg(feature = "v2_8")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
	#[doc(alias = "is-for-audio-device")]
	fn connect_is_for_audio_device_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_is_for_audio_device_trampoline<
			P:IsA<UserMediaPermissionRequest>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::WebKitUserMediaPermissionRequest,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(UserMediaPermissionRequest::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::is-for-audio-device\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					notify_is_for_audio_device_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg(feature = "v2_8")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
	#[doc(alias = "is-for-video-device")]
	fn connect_is_for_video_device_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_is_for_video_device_trampoline<
			P:IsA<UserMediaPermissionRequest>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::WebKitUserMediaPermissionRequest,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(UserMediaPermissionRequest::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::is-for-video-device\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					notify_is_for_video_device_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl<O:IsA<UserMediaPermissionRequest>> UserMediaPermissionRequestExt for O {}
