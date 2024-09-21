// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use glib::{
	prelude::*,
	signal::{connect_raw, SignalHandlerId},
	translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
	#[doc(alias = "WebKitPrintCustomWidget")]
	pub struct PrintCustomWidget(Object<ffi::WebKitPrintCustomWidget, ffi::WebKitPrintCustomWidgetClass>);

	match fn {
		type_ => || ffi::webkit_print_custom_widget_get_type(),
	}
}

impl PrintCustomWidget {
	pub const NONE: Option<&'static PrintCustomWidget> = None;

	#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_print_custom_widget_new")]
	pub fn new(widget: &impl IsA<gtk::Widget>, title: &str) -> PrintCustomWidget {
		assert_initialized_main_thread!();
		unsafe {
			from_glib_full(ffi::webkit_print_custom_widget_new(
				widget.as_ref().to_glib_none().0,
				title.to_glib_none().0,
			))
		}
	}

	// rustdoc-stripper-ignore-next
	/// Creates a new builder-pattern struct instance to construct [`PrintCustomWidget`] objects.
	///
	/// This method returns an instance of [`PrintCustomWidgetBuilder`](crate::builders::PrintCustomWidgetBuilder) which can be used to create [`PrintCustomWidget`] objects.
	pub fn builder() -> PrintCustomWidgetBuilder {
		PrintCustomWidgetBuilder::new()
	}
}

#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
impl Default for PrintCustomWidget {
	fn default() -> Self {
		glib::object::Object::new::<Self>()
	}
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PrintCustomWidget`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PrintCustomWidgetBuilder {
	builder: glib::object::ObjectBuilder<'static, PrintCustomWidget>,
}

impl PrintCustomWidgetBuilder {
	fn new() -> Self {
		Self { builder: glib::object::Object::builder() }
	}

	#[cfg(feature = "v2_16")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
	#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
	pub fn title(self, title: impl Into<glib::GString>) -> Self {
		Self { builder: self.builder.property("title", title.into()) }
	}

	#[cfg(feature = "v2_16")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
	#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
	pub fn widget(self, widget: &impl IsA<gtk::Widget>) -> Self {
		Self { builder: self.builder.property("widget", widget.clone().upcast()) }
	}

	// rustdoc-stripper-ignore-next
	/// Build the [`PrintCustomWidget`].
	#[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
	pub fn build(self) -> PrintCustomWidget {
		self.builder.build()
	}
}

mod sealed {
	pub trait Sealed {}
	impl<T: super::IsA<super::PrintCustomWidget>> Sealed for T {}
}

pub trait PrintCustomWidgetExt: IsA<PrintCustomWidget> + sealed::Sealed + 'static {
	#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_print_custom_widget_get_title")]
	#[doc(alias = "get_title")]
	fn title(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::webkit_print_custom_widget_get_title(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_print_custom_widget_get_widget")]
	#[doc(alias = "get_widget")]
	fn widget(&self) -> Option<gtk::Widget> {
		unsafe {
			from_glib_none(ffi::webkit_print_custom_widget_get_widget(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
	#[cfg(feature = "v2_16")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
	#[doc(alias = "apply")]
	fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
		unsafe extern fn apply_trampoline<P: IsA<PrintCustomWidget>, F: Fn(&P) + 'static>(
			this: *mut ffi::WebKitPrintCustomWidget,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(PrintCustomWidget::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"apply\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					apply_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
	#[cfg(feature = "v2_16")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
	#[doc(alias = "update")]
	fn connect_update<F: Fn(&Self, &gtk::PageSetup, &gtk::PrintSettings) + 'static>(
		&self,
		f: F,
	) -> SignalHandlerId {
		unsafe extern fn update_trampoline<
			P: IsA<PrintCustomWidget>,
			F: Fn(&P, &gtk::PageSetup, &gtk::PrintSettings) + 'static,
		>(
			this: *mut ffi::WebKitPrintCustomWidget,
			page_setup: *mut gtk::ffi::GtkPageSetup,
			print_settings: *mut gtk::ffi::GtkPrintSettings,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(
				PrintCustomWidget::from_glib_borrow(this).unsafe_cast_ref(),
				&from_glib_borrow(page_setup),
				&from_glib_borrow(print_settings),
			)
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"update\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					update_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl<O: IsA<PrintCustomWidget>> PrintCustomWidgetExt for O {}
