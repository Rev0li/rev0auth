pub struct PageModule {
    pub placeholder: &'static str,
    pub content: &'static str,
}

pub fn assemble_template(template: &str, modules: &[PageModule]) -> String {
    let mut output = template.to_string();
    for module in modules {
        output = output.replace(module.placeholder, module.content);
    }
    output
}
