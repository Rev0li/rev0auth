pub const SIGNUP_PAGE_STYLES: &str = r#"
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: var(--font-sans, system-ui, sans-serif); background: var(--background); color: var(--foreground); min-height: 100vh; display: flex; align-items: center; justify-content: center; padding: 1.5rem; }
.page { width: 100%; max-width: 420px; }
.header { text-align: center; margin-bottom: 1.5rem; }
.header h1 { font-size: 1.5rem; font-weight: 700; letter-spacing: -0.03em; }
.header .subtitle { font-size: 0.875rem; color: var(--muted-foreground); margin-top: 0.35rem; }
.link { font-size: 0.8125rem; color: var(--muted-foreground); text-decoration: none; display: inline-block; margin-top: 0.5rem; }
.link:hover { color: var(--foreground); }
.card { background: var(--card); border: 1px solid var(--border); border-radius: var(--radius, 0.75rem); padding: 1.5rem; display: flex; flex-direction: column; gap: 0.875rem; }
label { font-size: 0.8125rem; font-weight: 600; color: var(--foreground); }
input[type=text], input[type=password] { width: 100%; padding: 0.5rem 0.75rem; border: 1px solid var(--border); border-radius: 0.375rem; background: var(--input, var(--background)); color: var(--foreground); font-size: 0.9rem; outline: none; transition: border-color 0.15s; }
input:focus { border-color: var(--foreground); }
.field { display: flex; flex-direction: column; gap: 0.3rem; }
.hint { font-size: 0.75rem; color: var(--muted-foreground); }
.hint-warn { font-size: 0.75rem; color: var(--warning, #b45309); }
.avatar-section label { display: block; margin-bottom: 0.5rem; }
.avatar-grid { display: grid; grid-template-columns: repeat(5, 1fr); gap: 0.5rem; }
.avatar-btn { background: var(--card); border: 2px solid var(--border); border-radius: 0.5rem; padding: 0.35rem; cursor: pointer; display: flex; flex-direction: column; align-items: center; gap: 0.2rem; transition: border-color 0.15s; }
.avatar-btn:hover { border-color: var(--muted-foreground); }
.avatar-btn.selected { border-color: var(--foreground); }
.avatar-btn img { width: 44px; height: 44px; border-radius: 50%; display: block; }
.avatar-btn span { font-size: 0.6rem; color: var(--muted-foreground); text-align: center; }
.btn { width: 100%; padding: 0.6rem 1rem; border: none; border-radius: 0.375rem; font-size: 0.9rem; font-weight: 600; cursor: pointer; transition: opacity 0.15s; }
.btn:disabled { opacity: 0.5; cursor: not-allowed; }
.btn-primary { background: var(--foreground); color: var(--background); }
.btn-primary:hover:not(:disabled) { opacity: 0.85; }
.result { font-size: 0.8125rem; padding: 0.5rem 0.75rem; border-radius: 0.375rem; display: none; }
.result.ok { background: var(--success-bg, #dcfce7); color: var(--success, #166534); display: block; }
.result.err { background: var(--error-bg, #fee2e2); color: var(--error, #991b1b); display: block; }
.error-page { text-align: center; padding: 2rem; }
.error-page h2 { font-size: 1.25rem; font-weight: 700; margin-bottom: 0.75rem; }
.error-page p { font-size: 0.875rem; color: var(--muted-foreground); }
"#;
