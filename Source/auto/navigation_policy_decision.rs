// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use std::boxed::Box as Box_;

use glib::{
	prelude::*,
	signal::{connect_raw, SignalHandlerId},
	translate::*,
};

#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
use crate::NavigationAction;
use crate::{NavigationType, PolicyDecision, URIRequest};

glib::wrapper! {
	#[doc(alias = "WebKitNavigationPolicyDecision")]
	pub struct NavigationPolicyDecision(Object<ffi::WebKitNavigationPolicyDecision, ffi::WebKitNavigationPolicyDecisionClass>) @extends PolicyDecision;

	match fn {
		type_ => || ffi::webkit_navigation_policy_decision_get_type(),
	}
}

impl NavigationPolicyDecision {
	pub const NONE:Option<&'static NavigationPolicyDecision> = None;
}

mod sealed {
	pub trait Sealed {}
	impl<T:super::IsA<super::NavigationPolicyDecision>> Sealed for T {}
}

pub trait NavigationPolicyDecisionExt:
	IsA<NavigationPolicyDecision> + sealed::Sealed + 'static {
	#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_navigation_policy_decision_get_frame_name")]
	#[doc(alias = "get_frame_name")]
	fn frame_name(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::webkit_navigation_policy_decision_get_frame_name(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[cfg_attr(feature = "v2_6", deprecated = "Since 2.6")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_navigation_policy_decision_get_modifiers")]
	#[doc(alias = "get_modifiers")]
	fn modifiers(&self) -> u32 {
		unsafe {
			ffi::webkit_navigation_policy_decision_get_modifiers(self.as_ref().to_glib_none().0)
		}
	}

	#[cfg_attr(feature = "v2_6", deprecated = "Since 2.6")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_navigation_policy_decision_get_mouse_button")]
	#[doc(alias = "get_mouse_button")]
	fn mouse_button(&self) -> u32 {
		unsafe {
			ffi::webkit_navigation_policy_decision_get_mouse_button(self.as_ref().to_glib_none().0)
		}
	}

	#[cfg(feature = "v2_6")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
	#[doc(alias = "webkit_navigation_policy_decision_get_navigation_action")]
	#[doc(alias = "get_navigation_action")]
	fn navigation_action(&self) -> Option<NavigationAction> {
		unsafe {
			from_glib_none(ffi::webkit_navigation_policy_decision_get_navigation_action(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[cfg_attr(feature = "v2_6", deprecated = "Since 2.6")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_navigation_policy_decision_get_navigation_type")]
	#[doc(alias = "get_navigation_type")]
	fn navigation_type(&self) -> NavigationType {
		unsafe {
			from_glib(ffi::webkit_navigation_policy_decision_get_navigation_type(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[cfg_attr(feature = "v2_6", deprecated = "Since 2.6")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_navigation_policy_decision_get_request")]
	#[doc(alias = "get_request")]
	fn request(&self) -> Option<URIRequest> {
		unsafe {
			from_glib_none(ffi::webkit_navigation_policy_decision_get_request(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
	#[doc(alias = "frame-name")]
	fn connect_frame_name_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_frame_name_trampoline<
			P:IsA<NavigationPolicyDecision>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::WebKitNavigationPolicyDecision,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(NavigationPolicyDecision::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::frame-name\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					notify_frame_name_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg_attr(feature = "v2_6", deprecated = "Since 2.6")]
	#[doc(alias = "modifiers")]
	fn connect_modifiers_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_modifiers_trampoline<
			P:IsA<NavigationPolicyDecision>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::WebKitNavigationPolicyDecision,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(NavigationPolicyDecision::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::modifiers\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					notify_modifiers_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg_attr(feature = "v2_6", deprecated = "Since 2.6")]
	#[doc(alias = "mouse-button")]
	fn connect_mouse_button_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_mouse_button_trampoline<
			P:IsA<NavigationPolicyDecision>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::WebKitNavigationPolicyDecision,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(NavigationPolicyDecision::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::mouse-button\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					notify_mouse_button_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg(feature = "v2_6")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
	#[doc(alias = "navigation-action")]
	fn connect_navigation_action_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_navigation_action_trampoline<
			P:IsA<NavigationPolicyDecision>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::WebKitNavigationPolicyDecision,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(NavigationPolicyDecision::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::navigation-action\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					notify_navigation_action_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg_attr(feature = "v2_6", deprecated = "Since 2.6")]
	#[doc(alias = "navigation-type")]
	fn connect_navigation_type_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_navigation_type_trampoline<
			P:IsA<NavigationPolicyDecision>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::WebKitNavigationPolicyDecision,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(NavigationPolicyDecision::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::navigation-type\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					notify_navigation_type_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg_attr(feature = "v2_6", deprecated = "Since 2.6")]
	#[doc(alias = "request")]
	fn connect_request_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_request_trampoline<
			P:IsA<NavigationPolicyDecision>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::WebKitNavigationPolicyDecision,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(NavigationPolicyDecision::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::request\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					notify_request_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl<O:IsA<NavigationPolicyDecision>> NavigationPolicyDecisionExt for O {}
