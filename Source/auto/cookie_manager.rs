// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use std::{boxed::Box as Box_, pin::Pin};

use glib::{
	prelude::*,
	signal::{connect_raw, SignalHandlerId},
	translate::*,
};

use crate::{CookieAcceptPolicy, CookiePersistentStorage};

glib::wrapper! {
	#[doc(alias = "WebKitCookieManager")]
	pub struct CookieManager(Object<ffi::WebKitCookieManager, ffi::WebKitCookieManagerClass>);

	match fn {
		type_ => || ffi::webkit_cookie_manager_get_type(),
	}
}

impl CookieManager {
	pub const NONE:Option<&'static CookieManager> = None;
}

mod sealed {
	pub trait Sealed {}
	impl<T:super::IsA<super::CookieManager>> Sealed for T {}
}

pub trait CookieManagerExt: IsA<CookieManager> + sealed::Sealed + 'static {
	#[cfg(feature = "v2_20")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_20")))]
	#[doc(alias = "webkit_cookie_manager_add_cookie")]
	fn add_cookie<P:FnOnce(Result<(), glib::Error>) + 'static>(
		&self,
		cookie:&mut soup::Cookie,
		cancellable:Option<&impl IsA<gio::Cancellable>>,
		callback:P,
	) {
		let main_context = glib::MainContext::ref_thread_default();
		let is_main_context_owner = main_context.is_owner();
		let has_acquired_main_context =
			(!is_main_context_owner).then(|| main_context.acquire().ok()).flatten();
		assert!(
			is_main_context_owner || has_acquired_main_context.is_some(),
			"Async operations only allowed if the thread is owning the MainContext"
		);

		let user_data:Box_<glib::thread_guard::ThreadGuard<P>> =
			Box_::new(glib::thread_guard::ThreadGuard::new(callback));
		unsafe extern fn add_cookie_trampoline<P:FnOnce(Result<(), glib::Error>) + 'static>(
			_source_object:*mut glib::gobject_ffi::GObject,
			res:*mut gio::ffi::GAsyncResult,
			user_data:glib::ffi::gpointer,
		) {
			let mut error = std::ptr::null_mut();
			let _ = ffi::webkit_cookie_manager_add_cookie_finish(
				_source_object as *mut _,
				res,
				&mut error,
			);
			let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
			let callback:Box_<glib::thread_guard::ThreadGuard<P>> =
				Box_::from_raw(user_data as *mut _);
			let callback:P = callback.into_inner();
			callback(result);
		}
		let callback = add_cookie_trampoline::<P>;
		unsafe {
			ffi::webkit_cookie_manager_add_cookie(
				self.as_ref().to_glib_none().0,
				cookie.to_glib_none_mut().0,
				cancellable.map(|p| p.as_ref()).to_glib_none().0,
				Some(callback),
				Box_::into_raw(user_data) as *mut _,
			);
		}
	}

	#[cfg(feature = "v2_20")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_20")))]
	fn add_cookie_future(
		&self,
		cookie:&mut soup::Cookie,
	) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
		let mut cookie = cookie.clone();
		Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
			obj.add_cookie(&mut cookie, Some(cancellable), move |res| {
				send.resolve(res);
			});
		}))
	}

	#[cfg_attr(feature = "v2_16", deprecated = "Since 2.16")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_cookie_manager_delete_all_cookies")]
	fn delete_all_cookies(&self) {
		unsafe {
			ffi::webkit_cookie_manager_delete_all_cookies(self.as_ref().to_glib_none().0);
		}
	}

	#[cfg(feature = "v2_20")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_20")))]
	#[doc(alias = "webkit_cookie_manager_delete_cookie")]
	fn delete_cookie<P:FnOnce(Result<(), glib::Error>) + 'static>(
		&self,
		cookie:&mut soup::Cookie,
		cancellable:Option<&impl IsA<gio::Cancellable>>,
		callback:P,
	) {
		let main_context = glib::MainContext::ref_thread_default();
		let is_main_context_owner = main_context.is_owner();
		let has_acquired_main_context =
			(!is_main_context_owner).then(|| main_context.acquire().ok()).flatten();
		assert!(
			is_main_context_owner || has_acquired_main_context.is_some(),
			"Async operations only allowed if the thread is owning the MainContext"
		);

		let user_data:Box_<glib::thread_guard::ThreadGuard<P>> =
			Box_::new(glib::thread_guard::ThreadGuard::new(callback));
		unsafe extern fn delete_cookie_trampoline<P:FnOnce(Result<(), glib::Error>) + 'static>(
			_source_object:*mut glib::gobject_ffi::GObject,
			res:*mut gio::ffi::GAsyncResult,
			user_data:glib::ffi::gpointer,
		) {
			let mut error = std::ptr::null_mut();
			let _ = ffi::webkit_cookie_manager_delete_cookie_finish(
				_source_object as *mut _,
				res,
				&mut error,
			);
			let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
			let callback:Box_<glib::thread_guard::ThreadGuard<P>> =
				Box_::from_raw(user_data as *mut _);
			let callback:P = callback.into_inner();
			callback(result);
		}
		let callback = delete_cookie_trampoline::<P>;
		unsafe {
			ffi::webkit_cookie_manager_delete_cookie(
				self.as_ref().to_glib_none().0,
				cookie.to_glib_none_mut().0,
				cancellable.map(|p| p.as_ref()).to_glib_none().0,
				Some(callback),
				Box_::into_raw(user_data) as *mut _,
			);
		}
	}

	#[cfg(feature = "v2_20")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_20")))]
	fn delete_cookie_future(
		&self,
		cookie:&mut soup::Cookie,
	) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
		let mut cookie = cookie.clone();
		Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
			obj.delete_cookie(&mut cookie, Some(cancellable), move |res| {
				send.resolve(res);
			});
		}))
	}

	#[cfg_attr(feature = "v2_16", deprecated = "Since 2.16")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_cookie_manager_delete_cookies_for_domain")]
	fn delete_cookies_for_domain(&self, domain:&str) {
		unsafe {
			ffi::webkit_cookie_manager_delete_cookies_for_domain(
				self.as_ref().to_glib_none().0,
				domain.to_glib_none().0,
			);
		}
	}

	#[doc(alias = "webkit_cookie_manager_get_accept_policy")]
	#[doc(alias = "get_accept_policy")]
	fn accept_policy<P:FnOnce(Result<CookieAcceptPolicy, glib::Error>) + 'static>(
		&self,
		cancellable:Option<&impl IsA<gio::Cancellable>>,
		callback:P,
	) {
		let main_context = glib::MainContext::ref_thread_default();
		let is_main_context_owner = main_context.is_owner();
		let has_acquired_main_context =
			(!is_main_context_owner).then(|| main_context.acquire().ok()).flatten();
		assert!(
			is_main_context_owner || has_acquired_main_context.is_some(),
			"Async operations only allowed if the thread is owning the MainContext"
		);

		let user_data:Box_<glib::thread_guard::ThreadGuard<P>> =
			Box_::new(glib::thread_guard::ThreadGuard::new(callback));
		unsafe extern fn accept_policy_trampoline<
			P:FnOnce(Result<CookieAcceptPolicy, glib::Error>) + 'static,
		>(
			_source_object:*mut glib::gobject_ffi::GObject,
			res:*mut gio::ffi::GAsyncResult,
			user_data:glib::ffi::gpointer,
		) {
			let mut error = std::ptr::null_mut();
			let ret = ffi::webkit_cookie_manager_get_accept_policy_finish(
				_source_object as *mut _,
				res,
				&mut error,
			);
			let result =
				if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) };
			let callback:Box_<glib::thread_guard::ThreadGuard<P>> =
				Box_::from_raw(user_data as *mut _);
			let callback:P = callback.into_inner();
			callback(result);
		}
		let callback = accept_policy_trampoline::<P>;
		unsafe {
			ffi::webkit_cookie_manager_get_accept_policy(
				self.as_ref().to_glib_none().0,
				cancellable.map(|p| p.as_ref()).to_glib_none().0,
				Some(callback),
				Box_::into_raw(user_data) as *mut _,
			);
		}
	}

	fn accept_policy_future(
		&self,
	) -> Pin<
		Box_<dyn std::future::Future<Output = Result<CookieAcceptPolicy, glib::Error>> + 'static>,
	> {
		Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
			obj.accept_policy(Some(cancellable), move |res| {
				send.resolve(res);
			});
		}))
	}

	#[cfg(feature = "v2_20")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_20")))]
	#[doc(alias = "webkit_cookie_manager_get_cookies")]
	#[doc(alias = "get_cookies")]
	fn cookies<P:FnOnce(Result<Vec<soup::Cookie>, glib::Error>) + 'static>(
		&self,
		uri:&str,
		cancellable:Option<&impl IsA<gio::Cancellable>>,
		callback:P,
	) {
		let main_context = glib::MainContext::ref_thread_default();
		let is_main_context_owner = main_context.is_owner();
		let has_acquired_main_context =
			(!is_main_context_owner).then(|| main_context.acquire().ok()).flatten();
		assert!(
			is_main_context_owner || has_acquired_main_context.is_some(),
			"Async operations only allowed if the thread is owning the MainContext"
		);

		let user_data:Box_<glib::thread_guard::ThreadGuard<P>> =
			Box_::new(glib::thread_guard::ThreadGuard::new(callback));
		unsafe extern fn cookies_trampoline<
			P:FnOnce(Result<Vec<soup::Cookie>, glib::Error>) + 'static,
		>(
			_source_object:*mut glib::gobject_ffi::GObject,
			res:*mut gio::ffi::GAsyncResult,
			user_data:glib::ffi::gpointer,
		) {
			let mut error = std::ptr::null_mut();
			let ret = ffi::webkit_cookie_manager_get_cookies_finish(
				_source_object as *mut _,
				res,
				&mut error,
			);
			let result = if error.is_null() {
				Ok(FromGlibPtrContainer::from_glib_full(ret))
			} else {
				Err(from_glib_full(error))
			};
			let callback:Box_<glib::thread_guard::ThreadGuard<P>> =
				Box_::from_raw(user_data as *mut _);
			let callback:P = callback.into_inner();
			callback(result);
		}
		let callback = cookies_trampoline::<P>;
		unsafe {
			ffi::webkit_cookie_manager_get_cookies(
				self.as_ref().to_glib_none().0,
				uri.to_glib_none().0,
				cancellable.map(|p| p.as_ref()).to_glib_none().0,
				Some(callback),
				Box_::into_raw(user_data) as *mut _,
			);
		}
	}

	#[cfg(feature = "v2_20")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_20")))]
	fn cookies_future(
		&self,
		uri:&str,
	) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<soup::Cookie>, glib::Error>> + 'static>>
	{
		let uri = String::from(uri);
		Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
			obj.cookies(&uri, Some(cancellable), move |res| {
				send.resolve(res);
			});
		}))
	}

	#[cfg_attr(feature = "v2_16", deprecated = "Since 2.16")]
	#[allow(deprecated)]
	#[doc(alias = "webkit_cookie_manager_get_domains_with_cookies")]
	#[doc(alias = "get_domains_with_cookies")]
	fn domains_with_cookies<P:FnOnce(Result<Vec<glib::GString>, glib::Error>) + 'static>(
		&self,
		cancellable:Option<&impl IsA<gio::Cancellable>>,
		callback:P,
	) {
		let main_context = glib::MainContext::ref_thread_default();
		let is_main_context_owner = main_context.is_owner();
		let has_acquired_main_context =
			(!is_main_context_owner).then(|| main_context.acquire().ok()).flatten();
		assert!(
			is_main_context_owner || has_acquired_main_context.is_some(),
			"Async operations only allowed if the thread is owning the MainContext"
		);

		let user_data:Box_<glib::thread_guard::ThreadGuard<P>> =
			Box_::new(glib::thread_guard::ThreadGuard::new(callback));
		unsafe extern fn domains_with_cookies_trampoline<
			P:FnOnce(Result<Vec<glib::GString>, glib::Error>) + 'static,
		>(
			_source_object:*mut glib::gobject_ffi::GObject,
			res:*mut gio::ffi::GAsyncResult,
			user_data:glib::ffi::gpointer,
		) {
			let mut error = std::ptr::null_mut();
			let ret = ffi::webkit_cookie_manager_get_domains_with_cookies_finish(
				_source_object as *mut _,
				res,
				&mut error,
			);
			let result = if error.is_null() {
				Ok(FromGlibPtrContainer::from_glib_full(ret))
			} else {
				Err(from_glib_full(error))
			};
			let callback:Box_<glib::thread_guard::ThreadGuard<P>> =
				Box_::from_raw(user_data as *mut _);
			let callback:P = callback.into_inner();
			callback(result);
		}
		let callback = domains_with_cookies_trampoline::<P>;
		unsafe {
			ffi::webkit_cookie_manager_get_domains_with_cookies(
				self.as_ref().to_glib_none().0,
				cancellable.map(|p| p.as_ref()).to_glib_none().0,
				Some(callback),
				Box_::into_raw(user_data) as *mut _,
			);
		}
	}

	#[cfg_attr(feature = "v2_16", deprecated = "Since 2.16")]

	fn domains_with_cookies_future(
		&self,
	) -> Pin<
		Box_<dyn std::future::Future<Output = Result<Vec<glib::GString>, glib::Error>> + 'static>,
	> {
		Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
			obj.domains_with_cookies(Some(cancellable), move |res| {
				send.resolve(res);
			});
		}))
	}

	#[doc(alias = "webkit_cookie_manager_set_accept_policy")]
	fn set_accept_policy(&self, policy:CookieAcceptPolicy) {
		unsafe {
			ffi::webkit_cookie_manager_set_accept_policy(
				self.as_ref().to_glib_none().0,
				policy.into_glib(),
			);
		}
	}

	#[doc(alias = "webkit_cookie_manager_set_persistent_storage")]
	fn set_persistent_storage(&self, filename:&str, storage:CookiePersistentStorage) {
		unsafe {
			ffi::webkit_cookie_manager_set_persistent_storage(
				self.as_ref().to_glib_none().0,
				filename.to_glib_none().0,
				storage.into_glib(),
			);
		}
	}

	#[doc(alias = "changed")]
	fn connect_changed<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn changed_trampoline<P:IsA<CookieManager>, F:Fn(&P) + 'static>(
			this:*mut ffi::WebKitCookieManager,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(CookieManager::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"changed\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					changed_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl<O:IsA<CookieManager>> CookieManagerExt for O {}
