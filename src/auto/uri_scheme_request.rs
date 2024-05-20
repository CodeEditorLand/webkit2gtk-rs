// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

#[cfg(feature = "v2_36")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_36")))]
use crate::URISchemeResponse;
use crate::WebView;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "WebKitURISchemeRequest")]
    pub struct URISchemeRequest(Object<ffi::WebKitURISchemeRequest, ffi::WebKitURISchemeRequestClass>);

    match fn {
        type_ => || ffi::webkit_uri_scheme_request_get_type(),
    }
}

impl URISchemeRequest {
  pub const NONE: Option<&'static URISchemeRequest> = None;
}

mod sealed {
  pub trait Sealed {}
  impl<T: super::IsA<super::URISchemeRequest>> Sealed for T {}
}

pub trait URISchemeRequestExt: IsA<URISchemeRequest> + sealed::Sealed + 'static {
  #[doc(alias = "webkit_uri_scheme_request_finish")]
  fn finish(
    &self,
    stream: &impl IsA<gio::InputStream>,
    stream_length: i64,
    content_type: Option<&str>,
  ) {
    unsafe {
      ffi::webkit_uri_scheme_request_finish(
        self.as_ref().to_glib_none().0,
        stream.as_ref().to_glib_none().0,
        stream_length,
        content_type.to_glib_none().0,
      );
    }
  }

  #[cfg(feature = "v2_2")]
  #[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
  #[doc(alias = "webkit_uri_scheme_request_finish_error")]
  fn finish_error(&self, error: &mut glib::Error) {
    unsafe {
      ffi::webkit_uri_scheme_request_finish_error(
        self.as_ref().to_glib_none().0,
        error.to_glib_none_mut().0,
      );
    }
  }

  #[cfg(feature = "v2_36")]
  #[cfg_attr(docsrs, doc(cfg(feature = "v2_36")))]
  #[doc(alias = "webkit_uri_scheme_request_finish_with_response")]
  fn finish_with_response(&self, response: &impl IsA<URISchemeResponse>) {
    unsafe {
      ffi::webkit_uri_scheme_request_finish_with_response(
        self.as_ref().to_glib_none().0,
        response.as_ref().to_glib_none().0,
      );
    }
  }

  #[cfg(feature = "v2_40")]
  #[cfg_attr(docsrs, doc(cfg(feature = "v2_40")))]
  #[doc(alias = "webkit_uri_scheme_request_get_http_body")]
  #[doc(alias = "get_http_body")]
  fn http_body(&self) -> Option<gio::InputStream> {
    unsafe {
      from_glib_full(ffi::webkit_uri_scheme_request_get_http_body(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[cfg(feature = "v2_36")]
  #[cfg_attr(docsrs, doc(cfg(feature = "v2_36")))]
  #[doc(alias = "webkit_uri_scheme_request_get_http_headers")]
  #[doc(alias = "get_http_headers")]
  fn http_headers(&self) -> Option<soup::MessageHeaders> {
    unsafe {
      from_glib_none(ffi::webkit_uri_scheme_request_get_http_headers(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[cfg(feature = "v2_36")]
  #[cfg_attr(docsrs, doc(cfg(feature = "v2_36")))]
  #[doc(alias = "webkit_uri_scheme_request_get_http_method")]
  #[doc(alias = "get_http_method")]
  fn http_method(&self) -> Option<glib::GString> {
    unsafe {
      from_glib_none(ffi::webkit_uri_scheme_request_get_http_method(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[doc(alias = "webkit_uri_scheme_request_get_path")]
  #[doc(alias = "get_path")]
  fn path(&self) -> Option<glib::GString> {
    unsafe {
      from_glib_none(ffi::webkit_uri_scheme_request_get_path(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[doc(alias = "webkit_uri_scheme_request_get_scheme")]
  #[doc(alias = "get_scheme")]
  fn scheme(&self) -> Option<glib::GString> {
    unsafe {
      from_glib_none(ffi::webkit_uri_scheme_request_get_scheme(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[doc(alias = "webkit_uri_scheme_request_get_uri")]
  #[doc(alias = "get_uri")]
  fn uri(&self) -> Option<glib::GString> {
    unsafe {
      from_glib_none(ffi::webkit_uri_scheme_request_get_uri(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[doc(alias = "webkit_uri_scheme_request_get_web_view")]
  #[doc(alias = "get_web_view")]
  fn web_view(&self) -> Option<WebView> {
    unsafe {
      from_glib_none(ffi::webkit_uri_scheme_request_get_web_view(
        self.as_ref().to_glib_none().0,
      ))
    }
  }
}

impl<O: IsA<URISchemeRequest>> URISchemeRequestExt for O {}
