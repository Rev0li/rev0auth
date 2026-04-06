use super::{
    friend_avatar_module, friend_chat_module, friend_onboarding_module, friend_services_module,
    friend_status_module, frontend_modules, frontend_theme, page_assembly,
};

pub fn assemble_friend_page(template: &str) -> String {
    page_assembly::assemble_template(
        template,
        &[
            page_assembly::PageModule {
                placeholder: "%%FRONTEND_THEME_BOOT%%",
                content: frontend_theme::FRONTEND_THEME_BOOT,
            },
            page_assembly::PageModule {
                placeholder: "%%FRONTEND_SHARED_CSS%%",
                content: frontend_theme::FRONTEND_SHARED_CSS,
            },
            page_assembly::PageModule {
                placeholder: "%%COMMON_JS_UTILS%%",
                content: frontend_modules::JS_COMMON_UTILS,
            },
            page_assembly::PageModule {
                placeholder: "%%FRIEND_ONBOARDING_CSS%%",
                content: friend_onboarding_module::CSS_FRIEND_ONBOARDING_STYLES,
            },
            page_assembly::PageModule {
                placeholder: "%%FRIEND_ONBOARDING_JS%%",
                content: friend_onboarding_module::JS_FRIEND_ONBOARDING_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%FRIEND_SERVICES_CSS%%",
                content: friend_services_module::CSS_FRIEND_SERVICES_STYLES,
            },
            page_assembly::PageModule {
                placeholder: "%%FRIEND_SERVICES_JS%%",
                content: friend_services_module::JS_FRIEND_SERVICES_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%FRIEND_CHAT_CSS%%",
                content: friend_chat_module::CSS_FRIEND_CHAT_STYLES,
            },
            page_assembly::PageModule {
                placeholder: "%%FRIEND_CHAT_JS%%",
                content: friend_chat_module::JS_FRIEND_CHAT_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%FRIEND_STATUS_CSS%%",
                content: friend_status_module::CSS_FRIEND_STATUS_STYLES,
            },
            page_assembly::PageModule {
                placeholder: "%%FRIEND_STATUS_JS%%",
                content: friend_status_module::JS_FRIEND_STATUS_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%FRIEND_AVATAR_CSS%%",
                content: friend_avatar_module::CSS_FRIEND_AVATAR_STYLES,
            },
            page_assembly::PageModule {
                placeholder: "%%FRIEND_AVATAR_JS%%",
                content: friend_avatar_module::JS_FRIEND_AVATAR_MODULE,
            },
        ],
    )
}
