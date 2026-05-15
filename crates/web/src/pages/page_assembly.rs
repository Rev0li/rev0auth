pub struct PageModule {
    pub placeholder: &'static str,
    pub content: &'static str,
}

pub fn assemble_template(template: &str, modules: &[PageModule]) -> String {
    let mut output = template.to_string();
    // Two passes: first pass substitutes page-level placeholders (which may introduce
    // nested ones like %%FRONTEND_SHARED_CSS%% inside page style constants);
    // second pass resolves those nested placeholders.
    for _ in 0..2 {
        for module in modules {
            output = output.replace(module.placeholder, module.content);
        }
    }
    output
}
