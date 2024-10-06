// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::translate::*;

use crate::ITPFirstParty;

glib::wrapper! {
	#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct ITPThirdParty(Shared<ffi::WebKitITPThirdParty>);

	match fn {
		ref => |ptr| ffi::webkit_itp_third_party_ref(ptr),
		unref => |ptr| ffi::webkit_itp_third_party_unref(ptr),
		type_ => || ffi::webkit_itp_third_party_get_type(),
	}
}

impl ITPThirdParty {
	#[doc(alias = "webkit_itp_third_party_get_domain")]
	#[doc(alias = "get_domain")]
	pub fn domain(&self) -> Option<glib::GString> {
		unsafe { from_glib_none(ffi::webkit_itp_third_party_get_domain(self.to_glib_none().0)) }
	}

	#[doc(alias = "webkit_itp_third_party_get_first_parties")]
	#[doc(alias = "get_first_parties")]
	pub fn first_parties(&self) -> Vec<ITPFirstParty> {
		unsafe {
			FromGlibPtrContainer::from_glib_none(ffi::webkit_itp_third_party_get_first_parties(
				self.to_glib_none().0,
			))
		}
	}
}
