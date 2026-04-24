pub const HOME_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body {
            margin: 0;
            background: var(--bg-page);
            min-height: 100vh;
        }
        .page {
            max-width: 420px;
            margin: 0 auto;
            padding: 40px 24px;
            display: flex;
            flex-direction: column;
            justify-content: center;
            min-height: 100vh;
        }
        .brand {
            margin-bottom: 32px;
        }
        .brand-mark {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 40px;
            height: 40px;
            border-radius: var(--radius-md);
            background: var(--color-accent);
            color: #fff;
            font-weight: 800;
            font-size: 1.1rem;
            letter-spacing: -0.02em;
        }
        .card {
            padding: 28px;
        }
        h1 {
            margin: 0 0 6px;
            font-size: 1.375rem;
            font-weight: 700;
            letter-spacing: -0.02em;
        }
        .hint {
            margin: 0 0 24px;
            font-size: 0.875rem;
            color: var(--color-muted);
        }
        label {
            display: block;
            margin: 0 0 6px;
            font-size: 0.875rem;
            font-weight: 600;
            color: var(--color-ink);
        }
        .field {
            margin-bottom: 16px;
        }
        input {
            width: 100%;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 9px 11px;
            font: inherit;
            font-size: 0.9375rem;
            background: #fff;
            color: var(--color-ink);
            transition: border-color 0.15s;
            outline: none;
        }
        input:focus {
            border-color: var(--color-accent);
            box-shadow: 0 0 0 3px var(--color-accent-bg);
        }
        .btn {
            margin-top: 8px;
            border: none;
            border-radius: var(--radius-md);
            padding: 10px 16px;
            font: inherit;
            font-size: 0.9375rem;
            font-weight: 600;
            cursor: pointer;
            width: 100%;
            transition: background 0.15s;
        }
        .btn-primary {
            background: var(--color-accent);
            color: #fff;
        }
        .btn-primary:hover { background: var(--color-accent-dark); }
        .result {
            margin-top: 12px;
            border-radius: var(--radius-md);
            padding: 9px 11px;
            font-size: 0.875rem;
            display: none;
        }
        .hidden { display: none; }
        .ok { display: block; }
        .down { display: block; }
        .link {
            display: block;
            margin-top: 20px;
            text-align: center;
            text-decoration: none;
            font-size: 0.875rem;
            color: var(--color-muted);
            font-weight: 500;
        }
        .link:hover { color: var(--color-ink); }
"#;
