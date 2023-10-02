// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

mod authentication_request;
pub use self::authentication_request::AuthenticationRequest;

#[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
mod automation_session;
#[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
pub use self::automation_session::AutomationSession;

mod back_forward_list;
pub use self::back_forward_list::BackForwardList;

mod back_forward_list_item;
pub use self::back_forward_list_item::BackForwardListItem;

mod color_chooser_request;
pub use self::color_chooser_request::ColorChooserRequest;

mod context_menu;
pub use self::context_menu::ContextMenu;

mod context_menu_item;
pub use self::context_menu_item::ContextMenuItem;

mod cookie_manager;
pub use self::cookie_manager::CookieManager;

#[cfg(feature = "v2_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_24")))]
mod device_info_permission_request;
#[cfg(feature = "v2_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_24")))]
pub use self::device_info_permission_request::DeviceInfoPermissionRequest;

mod download;
pub use self::download::Download;

#[cfg(feature = "v2_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
mod editor_state;
#[cfg(feature = "v2_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
pub use self::editor_state::EditorState;

mod favicon_database;
pub use self::favicon_database::FaviconDatabase;

mod file_chooser_request;
pub use self::file_chooser_request::FileChooserRequest;

mod find_controller;
pub use self::find_controller::FindController;

mod form_submission_request;
pub use self::form_submission_request::FormSubmissionRequest;

#[cfg(feature = "v2_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_26")))]
mod geolocation_manager;
#[cfg(feature = "v2_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_26")))]
pub use self::geolocation_manager::GeolocationManager;

mod geolocation_permission_request;
pub use self::geolocation_permission_request::GeolocationPermissionRequest;

mod hit_test_result;
pub use self::hit_test_result::HitTestResult;

#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
mod input_method_context;
#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
pub use self::input_method_context::InputMethodContext;

#[cfg(feature = "v2_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
mod install_missing_media_plugins_permission_request;
#[cfg(feature = "v2_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
pub use self::install_missing_media_plugins_permission_request::InstallMissingMediaPluginsPermissionRequest;

mod media_key_system_permission_request;
pub use self::media_key_system_permission_request::MediaKeySystemPermissionRequest;

mod navigation_policy_decision;
pub use self::navigation_policy_decision::NavigationPolicyDecision;

#[cfg(feature = "v2_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
mod notification;
#[cfg(feature = "v2_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
pub use self::notification::Notification;

#[cfg(feature = "v2_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
mod notification_permission_request;
#[cfg(feature = "v2_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
pub use self::notification_permission_request::NotificationPermissionRequest;

#[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
mod option_menu;
#[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
pub use self::option_menu::OptionMenu;

mod permission_request;
pub use self::permission_request::PermissionRequest;

#[cfg_attr(feature = "v2_32", deprecated = "Since 2.32")]
mod plugin;
#[cfg_attr(feature = "v2_32", deprecated = "Since 2.32")]
pub use self::plugin::Plugin;

#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
mod pointer_lock_permission_request;
#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
pub use self::pointer_lock_permission_request::PointerLockPermissionRequest;

mod policy_decision;
pub use self::policy_decision::PolicyDecision;

#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
mod print_custom_widget;
#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
pub use self::print_custom_widget::PrintCustomWidget;

mod print_operation;
pub use self::print_operation::PrintOperation;

mod response_policy_decision;
pub use self::response_policy_decision::ResponsePolicyDecision;

mod security_manager;
pub use self::security_manager::SecurityManager;

mod settings;
pub use self::settings::Settings;

mod uri_request;
pub use self::uri_request::URIRequest;

mod uri_response;
pub use self::uri_response::URIResponse;

mod uri_scheme_request;
pub use self::uri_scheme_request::URISchemeRequest;

mod uri_scheme_response;
pub use self::uri_scheme_response::URISchemeResponse;

#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
mod user_content_manager;
#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
pub use self::user_content_manager::UserContentManager;

#[cfg(feature = "v2_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
mod user_media_permission_request;
#[cfg(feature = "v2_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
pub use self::user_media_permission_request::UserMediaPermissionRequest;

#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
mod user_message;
#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
pub use self::user_message::UserMessage;

mod web_context;
pub use self::web_context::WebContext;

mod web_inspector;
pub use self::web_inspector::WebInspector;

mod web_resource;
pub use self::web_resource::WebResource;

mod web_view;
pub use self::web_view::WebView;

mod web_view_base;
pub use self::web_view_base::WebViewBase;

#[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
mod website_data_access_permission_request;
#[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
pub use self::website_data_access_permission_request::WebsiteDataAccessPermissionRequest;

#[cfg(feature = "v2_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
mod website_data_manager;
#[cfg(feature = "v2_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
pub use self::website_data_manager::WebsiteDataManager;

#[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
mod website_policies;
#[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
pub use self::website_policies::WebsitePolicies;

mod window_properties;
pub use self::window_properties::WindowProperties;

#[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
mod application_info;
#[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
pub use self::application_info::ApplicationInfo;

#[cfg(feature = "v2_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
mod credential;
#[cfg(feature = "v2_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
pub use self::credential::Credential;

#[cfg(feature = "v2_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_26")))]
mod geolocation_position;
#[cfg(feature = "v2_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_26")))]
pub use self::geolocation_position::GeolocationPosition;

#[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
mod itp_first_party;
#[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
pub use self::itp_first_party::ITPFirstParty;

#[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
mod itp_third_party;
#[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
pub use self::itp_third_party::ITPThirdParty;

#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
mod input_method_underline;
#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
pub use self::input_method_underline::InputMethodUnderline;

mod javascript_result;
pub use self::javascript_result::JavascriptResult;

#[cfg(feature = "v2_34")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_34")))]
mod memory_pressure_settings;
#[cfg(feature = "v2_34")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_34")))]
pub use self::memory_pressure_settings::MemoryPressureSettings;

mod mime_info;
pub use self::mime_info::MimeInfo;

#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
mod navigation_action;
#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
pub use self::navigation_action::NavigationAction;

#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
mod network_proxy_settings;
#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
pub use self::network_proxy_settings::NetworkProxySettings;

#[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
mod option_menu_item;
#[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
pub use self::option_menu_item::OptionMenuItem;

#[cfg(feature = "v2_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_24")))]
mod script_dialog;
#[cfg(feature = "v2_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_24")))]
pub use self::script_dialog::ScriptDialog;

#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
mod security_origin;
#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
pub use self::security_origin::SecurityOrigin;

#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
mod user_script;
#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
pub use self::user_script::UserScript;

#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
mod user_style_sheet;
#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
pub use self::user_style_sheet::UserStyleSheet;

#[cfg(feature = "v2_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_12")))]
mod web_view_session_state;
#[cfg(feature = "v2_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_12")))]
pub use self::web_view_session_state::WebViewSessionState;

#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
mod website_data;
#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
pub use self::website_data::WebsiteData;

mod enums;
#[cfg(feature = "v2_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
pub use self::enums::AuthenticationScheme;
#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
pub use self::enums::AutomationBrowsingContextPresentation;
#[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
pub use self::enums::AutoplayPolicy;
pub use self::enums::CacheModel;
pub use self::enums::ContextMenuAction;
pub use self::enums::CookieAcceptPolicy;
pub use self::enums::CookiePersistentStorage;
#[cfg(feature = "v2_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
pub use self::enums::CredentialPersistence;
pub use self::enums::DownloadError;
pub use self::enums::FaviconDatabaseError;
#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
pub use self::enums::HardwareAccelerationPolicy;
#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
pub use self::enums::InputPurpose;
pub use self::enums::InsecureContentEvent;
pub use self::enums::JavascriptError;
pub use self::enums::LoadEvent;
#[cfg(feature = "v2_34")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_34")))]
pub use self::enums::MediaCaptureState;
pub use self::enums::NavigationType;
pub use self::enums::NetworkError;
#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
pub use self::enums::NetworkProxyMode;
pub use self::enums::PluginError;
pub use self::enums::PolicyDecisionType;
pub use self::enums::PolicyError;
pub use self::enums::PrintError;
pub use self::enums::PrintOperationResponse;
#[cfg(feature = "v2_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_4")))]
#[allow(deprecated)]
pub use self::enums::ProcessModel;
pub use self::enums::SaveMode;
pub use self::enums::ScriptDialogType;
pub use self::enums::SnapshotError;
pub use self::enums::SnapshotRegion;
pub use self::enums::TLSErrorsPolicy;
#[cfg(feature = "v2_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_24")))]
pub use self::enums::UserContentFilterError;
#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
pub use self::enums::UserContentInjectedFrames;
#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
pub use self::enums::UserMessageError;
#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
pub use self::enums::UserScriptInjectionTime;
#[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
pub use self::enums::UserStyleLevel;
#[cfg(feature = "v2_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_20")))]
pub use self::enums::WebProcessTerminationReason;

mod flags;
#[cfg(feature = "v2_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
pub use self::flags::EditorTypingAttributes;
pub use self::flags::FindOptions;
pub use self::flags::HitTestResultContext;
#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
pub use self::flags::InputHints;
pub use self::flags::SnapshotOptions;
#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
pub use self::flags::WebsiteDataTypes;

pub(crate) mod traits {
    pub use super::authentication_request::AuthenticationRequestExt;
    #[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
    pub use super::automation_session::AutomationSessionExt;
    pub use super::back_forward_list::BackForwardListExt;
    pub use super::back_forward_list_item::BackForwardListItemExt;
    pub use super::color_chooser_request::ColorChooserRequestExt;
    pub use super::context_menu::ContextMenuExt;
    pub use super::context_menu_item::ContextMenuItemExt;
    pub use super::cookie_manager::CookieManagerExt;
    pub use super::download::DownloadExt;
    #[cfg(feature = "v2_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
    pub use super::editor_state::EditorStateExt;
    pub use super::favicon_database::FaviconDatabaseExt;
    pub use super::file_chooser_request::FileChooserRequestExt;
    pub use super::find_controller::FindControllerExt;
    pub use super::form_submission_request::FormSubmissionRequestExt;
    #[cfg(feature = "v2_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_26")))]
    pub use super::geolocation_manager::GeolocationManagerExt;
    pub use super::hit_test_result::HitTestResultExt;
    #[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
    pub use super::input_method_context::InputMethodContextExt;
    #[cfg(feature = "v2_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
    #[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
    pub use super::install_missing_media_plugins_permission_request::InstallMissingMediaPluginsPermissionRequestExt;
    pub use super::navigation_policy_decision::NavigationPolicyDecisionExt;
    #[cfg(feature = "v2_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
    pub use super::notification::NotificationExt;
    #[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
    pub use super::option_menu::OptionMenuExt;
    pub use super::permission_request::PermissionRequestExt;
    #[cfg_attr(feature = "v2_32", deprecated = "Since 2.32")]
    pub use super::plugin::PluginExt;
    pub use super::policy_decision::PolicyDecisionExt;
    #[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
    pub use super::print_custom_widget::PrintCustomWidgetExt;
    pub use super::print_operation::PrintOperationExt;
    pub use super::response_policy_decision::ResponsePolicyDecisionExt;
    pub use super::security_manager::SecurityManagerExt;
    pub use super::settings::SettingsExt;
    pub use super::uri_request::URIRequestExt;
    pub use super::uri_response::URIResponseExt;
    pub use super::uri_scheme_request::URISchemeRequestExt;
    pub use super::uri_scheme_response::URISchemeResponseExt;
    #[cfg(feature = "v2_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_6")))]
    pub use super::user_content_manager::UserContentManagerExt;
    #[cfg(feature = "v2_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
    pub use super::user_media_permission_request::UserMediaPermissionRequestExt;
    #[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
    pub use super::user_message::UserMessageExt;
    pub use super::web_context::WebContextExt;
    pub use super::web_inspector::WebInspectorExt;
    pub use super::web_resource::WebResourceExt;
    pub use super::web_view::WebViewExt;
    #[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
    pub use super::website_data_access_permission_request::WebsiteDataAccessPermissionRequestExt;
    #[cfg(feature = "v2_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
    pub use super::website_data_manager::WebsiteDataManagerExt;
    #[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
    pub use super::website_policies::WebsitePoliciesExt;
    pub use super::window_properties::WindowPropertiesExt;
}
pub(crate) mod builders {
    #[cfg(feature = "v2_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
    pub use super::automation_session::AutomationSessionBuilder;
    pub use super::color_chooser_request::ColorChooserRequestBuilder;
    pub use super::download::DownloadBuilder;
    pub use super::find_controller::FindControllerBuilder;
    pub use super::hit_test_result::HitTestResultBuilder;
    #[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
    pub use super::print_custom_widget::PrintCustomWidgetBuilder;
    pub use super::print_operation::PrintOperationBuilder;
    pub use super::settings::SettingsBuilder;
    pub use super::uri_request::URIRequestBuilder;
    pub use super::uri_scheme_response::URISchemeResponseBuilder;
    #[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
    pub use super::user_message::UserMessageBuilder;
    pub use super::web_context::WebContextBuilder;
    pub use super::web_view::WebViewBuilder;
    #[cfg(feature = "v2_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
    pub use super::website_data_manager::WebsiteDataManagerBuilder;
    #[cfg(feature = "v2_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_30")))]
    pub use super::website_policies::WebsitePoliciesBuilder;
    pub use super::window_properties::WindowPropertiesBuilder;
}
