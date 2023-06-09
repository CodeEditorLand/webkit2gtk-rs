// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use crate::AutoplayPolicy;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitWebsitePolicies")]
    pub struct WebsitePolicies(Object<ffi::WebKitWebsitePolicies, ffi::WebKitWebsitePoliciesClass>);

    match fn {
        type_ => || ffi::webkit_website_policies_get_type(),
    }
}

impl WebsitePolicies {
        pub const NONE: Option<&'static WebsitePolicies> = None;
    

    #[doc(alias = "webkit_website_policies_new")]
    pub fn new() -> WebsitePolicies {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_website_policies_new())
        }
    }

    //#[doc(alias = "webkit_website_policies_new_with_policies")]
    //#[doc(alias = "new_with_policies")]
    //pub fn with_policies(first_policy_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> WebsitePolicies {
    //    unsafe { TODO: call ffi:webkit_website_policies_new_with_policies() }
    //}

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`WebsitePolicies`] objects.
            ///
            /// This method returns an instance of [`WebsitePoliciesBuilder`](crate::builders::WebsitePoliciesBuilder) which can be used to create [`WebsitePolicies`] objects.
            pub fn builder() -> WebsitePoliciesBuilder {
                WebsitePoliciesBuilder::default()
            }
        
}

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
impl Default for WebsitePolicies {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`WebsitePolicies`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct WebsitePoliciesBuilder {
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    autoplay: Option<AutoplayPolicy>,
}

impl WebsitePoliciesBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`WebsitePoliciesBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`WebsitePolicies`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> WebsitePolicies {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v2_30", feature = "dox"))]
if let Some(ref autoplay) = self.autoplay {
                properties.push(("autoplay", autoplay));
            }
        glib::Object::new::<WebsitePolicies>(&properties)

    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    pub fn autoplay(mut self, autoplay: AutoplayPolicy) -> Self {
        self.autoplay = Some(autoplay);
        self
    }
}

pub trait WebsitePoliciesExt: 'static {
    #[doc(alias = "webkit_website_policies_get_autoplay_policy")]
    #[doc(alias = "get_autoplay_policy")]
    fn autoplay_policy(&self) -> AutoplayPolicy;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn autoplay(&self) -> AutoplayPolicy;
}

impl<O: IsA<WebsitePolicies>> WebsitePoliciesExt for O {
    fn autoplay_policy(&self) -> AutoplayPolicy {
        unsafe {
            from_glib(ffi::webkit_website_policies_get_autoplay_policy(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn autoplay(&self) -> AutoplayPolicy {
        glib::ObjectExt::property(self.as_ref(), "autoplay")
    }
}

impl fmt::Display for WebsitePolicies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebsitePolicies")
    }
}
