// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::translate::*;

use crate::{NavigationType, URIRequest};

glib::wrapper! {
	#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct NavigationAction(Boxed<ffi::WebKitNavigationAction>);

	match fn {
		copy => |ptr| ffi::webkit_navigation_action_copy(mut_override(ptr)),
		free => |ptr| ffi::webkit_navigation_action_free(ptr),
		type_ => || ffi::webkit_navigation_action_get_type(),
	}
}

impl NavigationAction {
	#[cfg(feature = "v2_40")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_40")))]
	#[doc(alias = "webkit_navigation_action_get_frame_name")]
	#[doc(alias = "get_frame_name")]
	pub fn frame_name(&mut self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::webkit_navigation_action_get_frame_name(self.to_glib_none_mut().0))
		}
	}

	#[doc(alias = "webkit_navigation_action_get_modifiers")]
	#[doc(alias = "get_modifiers")]
	pub fn modifiers(&self) -> u32 {
		unsafe { ffi::webkit_navigation_action_get_modifiers(mut_override(self.to_glib_none().0)) }
	}

	#[doc(alias = "webkit_navigation_action_get_mouse_button")]
	#[doc(alias = "get_mouse_button")]
	pub fn mouse_button(&self) -> u32 {
		unsafe {
			ffi::webkit_navigation_action_get_mouse_button(mut_override(self.to_glib_none().0))
		}
	}

	#[doc(alias = "webkit_navigation_action_get_navigation_type")]
	#[doc(alias = "get_navigation_type")]
	pub fn navigation_type(&self) -> NavigationType {
		unsafe {
			from_glib(ffi::webkit_navigation_action_get_navigation_type(mut_override(
				self.to_glib_none().0,
			)))
		}
	}

	#[doc(alias = "webkit_navigation_action_get_request")]
	#[doc(alias = "get_request")]
	pub fn request(&self) -> Option<URIRequest> {
		unsafe {
			from_glib_none(ffi::webkit_navigation_action_get_request(mut_override(
				self.to_glib_none().0,
			)))
		}
	}

	#[cfg(feature = "v2_20")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_20")))]
	#[doc(alias = "webkit_navigation_action_is_redirect")]
	pub fn is_redirect(&mut self) -> bool {
		unsafe { from_glib(ffi::webkit_navigation_action_is_redirect(self.to_glib_none_mut().0)) }
	}

	#[doc(alias = "webkit_navigation_action_is_user_gesture")]
	pub fn is_user_gesture(&self) -> bool {
		unsafe {
			from_glib(ffi::webkit_navigation_action_is_user_gesture(mut_override(
				self.to_glib_none().0,
			)))
		}
	}
}
