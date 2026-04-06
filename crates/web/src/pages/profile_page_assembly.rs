use super::{
    frontend_modules, frontend_theme, page_assembly, profile_account_deletion_module,
    profile_admin_navigator_module, profile_avatar_module, profile_donations_module,
    profile_edit_module, profile_info_module, profile_messages_module, profile_password_module,
    profile_page_styles,
};

pub fn assemble_profile_page(template: &str) -> String {
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
                placeholder: "%%PROFILE_PAGE_STYLES%%",
                content: profile_page_styles::PROFILE_PAGE_STYLES,
            },
            page_assembly::PageModule {
                placeholder: "%%COMMON_JS_UTILS%%",
                content: frontend_modules::JS_COMMON_UTILS,
            },
            page_assembly::PageModule {
                placeholder: "%%PROFILE_INFO_MODULE%%",
                content: profile_info_module::JS_PROFILE_INFO_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%PROFILE_EDIT_MODULE%%",
                content: profile_edit_module::JS_PROFILE_EDIT_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%PROFILE_AVATAR_MODULE%%",
                content: profile_avatar_module::JS_PROFILE_AVATAR_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%PROFILE_PASSWORD_MODULE%%",
                content: profile_password_module::JS_PROFILE_PASSWORD_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%PROFILE_MESSAGES_MODULE%%",
                content: profile_messages_module::JS_PROFILE_MESSAGES_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%PROFILE_DONATIONS_MODULE%%",
                content: profile_donations_module::JS_PROFILE_DONATIONS_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%PROFILE_ADMIN_NAVIGATOR_MODULE%%",
                content: profile_admin_navigator_module::JS_PROFILE_ADMIN_NAVIGATOR_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%PROFILE_ACCOUNT_DELETION_MODULE%%",
                content: profile_account_deletion_module::JS_PROFILE_ACCOUNT_DELETION_MODULE,
            },
        ],
    )
}
