pub const ADMIN_LOGIN_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body {
            margin: 0;
            background: var(--bg-page);
            min-height: 100vh;
            -webkit-font-smoothing: antialiased;
        }
        .page {
            max-width: 400px;
            margin: 0 auto;
            padding: 40px 24px;
            display: flex;
            flex-direction: column;
            justify-content: center;
            min-height: 100vh;
        }
        .brand {
            margin-bottom: 28px;
        }
        .brand-badge {
            display: inline-block;
            background: var(--color-ink);
            color: #fff;
            font-size: 0.75rem;
            font-weight: 700;
            letter-spacing: 0.08em;
            text-transform: uppercase;
            padding: 4px 8px;
            border-radius: var(--radius-sm);
        }
        .card {
            background: var(--color-panel);
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-lg);
            padding: 28px;
            box-shadow: 0 1px 3px rgba(15, 23, 42, 0.06);
        }
        h1 { margin: 0 0 6px; font-size: 1.25rem; font-weight: 700; letter-spacing: -0.02em; }
        .hint { margin: 0 0 22px; font-size: 0.875rem; color: var(--color-muted); }
        .field { margin-bottom: 14px; }
        label {
            display: block;
            margin: 0 0 5px;
            font-size: 0.8125rem;
            font-weight: 600;
        }
        input {
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
            background: var(--color-accent);
            color: #fff;
            transition: background 0.15s;
        }
        .btn:hover { background: var(--color-accent-dark); }
        .result {
            margin-top: 12px;
            border-radius: var(--radius-md);
            padding: 9px 11px;
            font-size: 0.875rem;
            display: none;
        }
        .ok { display: block; }
        .down { display: block; }
        .challenge-grid {
            margin-top: 10px;
            display: grid;
            grid-template-columns: repeat(3, minmax(0, 1fr));
            gap: 6px;
        }
        .challenge-label {
            margin-top: 14px;
            font-size: 0.8125rem;
            font-weight: 600;
            color: var(--color-muted);
        }
        .challenge-btn {
            border: 1px solid var(--color-panel-border);
            background: var(--color-panel);
            color: var(--color-ink);
            border-radius: var(--radius-md);
            padding: 8px;
            font-weight: 600;
            font-size: 0.875rem;
            cursor: pointer;
            transition: background 0.1s, border-color 0.1s;
        }
        .challenge-btn:hover { background: var(--bg-page); }
        .challenge-btn.selected {
            border-color: var(--color-accent-border);
            background: var(--color-accent-bg);
            color: var(--color-accent);
        }
        .trap-zone {
            position: absolute;
            left: -10000px;
            top: -10000px;
            width: 1px;
            height: 1px;
            overflow: hidden;
        }
"#;
