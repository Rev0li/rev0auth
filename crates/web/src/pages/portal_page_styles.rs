pub const PORTAL_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body {
            margin: 0;
            background: var(--bg-page);
            min-height: 100vh;
            -webkit-font-smoothing: antialiased;
        }
        .page {
            max-width: 560px;
            margin: 0 auto;
            padding: 32px 24px;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 20px;
            gap: 10px;
        }
        .header h1 {
            margin: 0;
            font-size: 1.375rem;
            font-weight: 700;
            letter-spacing: -0.02em;
        }
        .card {
            background: var(--color-panel);
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-lg);
            padding: 24px;
            box-shadow: 0 1px 3px rgba(15, 23, 42, 0.06);
        }
        .hint { margin: 0 0 20px; font-size: 0.875rem; color: var(--color-muted); }
        .field { margin-bottom: 14px; }
        label {
            display: block;
            margin: 0 0 5px;
            font-size: 0.8125rem;
            font-weight: 600;
        }
        .label-optional { font-weight: 400; color: var(--color-muted); }
        input, textarea {
            width: 100%;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 9px 11px;
            font: inherit;
            font-size: 0.9375rem;
            background: var(--color-panel);
            box-sizing: border-box;
            outline: none;
            transition: border-color 0.15s;
        }
        input:focus, textarea:focus {
            border-color: var(--color-accent);
            box-shadow: 0 0 0 3px var(--color-accent-bg);
        }
        textarea { min-height: 80px; resize: vertical; }
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
        .ok { display: block; }
        .down { display: block; }
        .link {
            display: block;
            margin-top: 18px;
            text-align: center;
            text-decoration: none;
            font-size: 0.875rem;
            color: var(--color-muted);
            font-weight: 500;
        }
        .link:hover { color: var(--color-ink); }
"#;
