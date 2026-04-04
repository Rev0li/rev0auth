pub const JS_COMMON_UTILS: &str = r#"
function bindEnterToClick(inputId, buttonId) {
    const input = document.getElementById(inputId);
    const button = document.getElementById(buttonId);
    if (!input || !button) return;
    input.addEventListener('keydown', (event) => {
        if (event.key === 'Enter') {
            event.preventDefault();
            button.click();
        }
    });
}

function escapeHtml(value) {
    return String(value || '')
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/\"/g, '&quot;')
        .replace(/'/g, '&#39;');
}
"#;
