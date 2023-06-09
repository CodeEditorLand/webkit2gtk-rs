// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WebKitFaviconDatabase")]
    pub struct FaviconDatabase(Object<ffi::WebKitFaviconDatabase, ffi::WebKitFaviconDatabaseClass>);

    match fn {
        type_ => || ffi::webkit_favicon_database_get_type(),
    }
}

impl FaviconDatabase {
        pub const NONE: Option<&'static FaviconDatabase> = None;
    
}

pub trait FaviconDatabaseExt: 'static {
    #[doc(alias = "webkit_favicon_database_clear")]
    fn clear(&self);

    #[doc(alias = "webkit_favicon_database_get_favicon")]
    #[doc(alias = "get_favicon")]
    fn favicon<P: FnOnce(Result<cairo::Surface, glib::Error>) + 'static>(&self, page_uri: &str, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P);

    
    fn favicon_future(&self, page_uri: &str) -> Pin<Box_<dyn std::future::Future<Output = Result<cairo::Surface, glib::Error>> + 'static>>;

    #[doc(alias = "webkit_favicon_database_get_favicon_uri")]
    #[doc(alias = "get_favicon_uri")]
    fn favicon_uri(&self, page_uri: &str) -> Option<glib::GString>;

    #[doc(alias = "favicon-changed")]
    fn connect_favicon_changed<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FaviconDatabase>> FaviconDatabaseExt for O {
    fn clear(&self) {
        unsafe {
            ffi::webkit_favicon_database_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn favicon<P: FnOnce(Result<cairo::Surface, glib::Error>) + 'static>(&self, page_uri: &str, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P) {
        
                let main_context = glib::MainContext::ref_thread_default();
                let is_main_context_owner = main_context.is_owner();
                let has_acquired_main_context = (!is_main_context_owner)
                    .then(|| main_context.acquire().ok())
                    .flatten();
                assert!(
                    is_main_context_owner || has_acquired_main_context.is_some(),
                    "Async operations only allowed if the thread is owning the MainContext"
                );
        
        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn favicon_trampoline<P: FnOnce(Result<cairo::Surface, glib::Error>) + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_favicon_database_get_favicon_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = favicon_trampoline::<P>;
        unsafe {
            ffi::webkit_favicon_database_get_favicon(self.as_ref().to_glib_none().0, page_uri.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn favicon_future(&self, page_uri: &str) -> Pin<Box_<dyn std::future::Future<Output = Result<cairo::Surface, glib::Error>> + 'static>> {

        let page_uri = String::from(page_uri);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.favicon(
                &page_uri,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    fn favicon_uri(&self, page_uri: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_favicon_database_get_favicon_uri(self.as_ref().to_glib_none().0, page_uri.to_glib_none().0))
        }
    }

    fn connect_favicon_changed<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn favicon_changed_trampoline<P: IsA<FaviconDatabase>, F: Fn(&P, &str, &str) + 'static>(this: *mut ffi::WebKitFaviconDatabase, page_uri: *mut libc::c_char, favicon_uri: *mut libc::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(FaviconDatabase::from_glib_borrow(this).unsafe_cast_ref(), &glib::GString::from_glib_borrow(page_uri), &glib::GString::from_glib_borrow(favicon_uri))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"favicon-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(favicon_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for FaviconDatabase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FaviconDatabase")
    }
}
