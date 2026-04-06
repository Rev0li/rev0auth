use super::{
    dashboard_chat_module, dashboard_donations_module, dashboard_queue_module,
    dashboard_status_module, dashboard_testing_module, dashboard_users_module, frontend_modules,
    dashboard_theme_editor_module, dashboard_page_styles, frontend_theme, page_assembly,
};

pub fn assemble_dashboard_page(template: &str) -> String {
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
                placeholder: "%%DASHBOARD_CHAT_MODULE%%",
                content: dashboard_chat_module::JS_DASHBOARD_CHAT_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%DASHBOARD_USERS_MODULE%%",
                content: dashboard_users_module::JS_DASHBOARD_USERS_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%DASHBOARD_DONATIONS_MODULE%%",
                content: dashboard_donations_module::JS_DASHBOARD_DONATIONS_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%DASHBOARD_TESTING_MODULE%%",
                content: dashboard_testing_module::JS_DASHBOARD_TESTING_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%DASHBOARD_QUEUE_MODULE%%",
                content: dashboard_queue_module::JS_DASHBOARD_QUEUE_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%DASHBOARD_STATUS_MODULE%%",
                content: dashboard_status_module::JS_DASHBOARD_STATUS_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%DASHBOARD_THEME_EDITOR_MODULE%%",
                content: dashboard_theme_editor_module::JS_DASHBOARD_THEME_EDITOR_MODULE,
            },
            page_assembly::PageModule {
                placeholder: "%%DASHBOARD_PAGE_STYLES%%",
                content: dashboard_page_styles::DASHBOARD_PAGE_STYLES,
            },
        ],
    )
}
