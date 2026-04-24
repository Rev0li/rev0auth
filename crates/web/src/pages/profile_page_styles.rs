pub const PROFILE_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body {
            margin: 0;
            background: var(--bg-page);
            min-height: 100vh;
            -webkit-font-smoothing: antialiased;
        }
        .page {
            max-width: 680px;
            margin: 0 auto;
            padding: 32px 20px;
        }
        .page-header {
            margin-bottom: 24px;
        }
        .page-header h1 {
            margin: 0 0 4px;
            font-size: 1.375rem;
            font-weight: 700;
            letter-spacing: -0.02em;
        }
        .card {
            padding: 20px;
            margin-bottom: 12px;
        }
        h1 { margin-top: 0; }
        h2 {
            margin: 0 0 14px;
            font-size: 1rem;
            font-weight: 600;
            color: var(--color-ink);
        }
        label {
            display: block;
            font-weight: 600;
            font-size: 0.8125rem;
            margin: 0 0 5px;
            color: var(--color-ink);
        }
        .field { margin-bottom: 14px; }
        input, textarea, select {
            width: 100%;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 8px 10px;
            box-sizing: border-box;
            font: inherit;
            font-size: 0.9375rem;
            background: var(--color-panel);
            color: var(--color-ink);
            outline: none;
            transition: border-color 0.15s;
        }
        input:focus, textarea:focus, select:focus {
            border-color: var(--color-accent);
            box-shadow: 0 0 0 3px var(--color-accent-bg);
        }
        textarea { min-height: 90px; resize: vertical; }
        .danger-outline {
            border-color: var(--color-danger);
        }
        .meta-topless { margin-top: 0; }
        .list-meta-gap { margin-top: 8px; }
        .message-body { margin-top: 6px; white-space: pre-wrap; font-size: 0.9rem; }
        .actions-compact { margin-top: 8px; }
        .donation-proof-link { margin-top: 6px; font-size: 0.8125rem; color: var(--color-muted); }
        .actions {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            margin-top: 14px;
        }
        button, a.btn {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 8px 14px;
            font: inherit;
            font-size: 0.9rem;
            font-weight: 600;
            cursor: pointer;
            text-decoration: none;
            background: var(--color-panel);
            color: var(--color-ink);
            transition: background 0.1s;
            display: inline-flex;
            align-items: center;
        }
        button:hover, a.btn:hover { background: var(--bg-page); }
        .primary {
            background: var(--color-accent);
            border-color: transparent;
            color: #fff;
        }
        .primary:hover { background: var(--color-accent-dark); }
        .secondary {
            background: var(--color-panel);
            border-color: var(--color-panel-border);
            color: var(--color-ink);
        }
        .msg {
            margin-top: 10px;
            font-size: 0.875rem;
            border-radius: var(--radius-md);
            padding: 8px 10px;
            display: none;
        }
        .ok { display: block; }
        .down { display: block; }
        .meta { font-size: 0.875rem; color: var(--color-muted); }
        .admin-note {
            margin-top: 10px;
            padding: 9px 12px;
            border-radius: var(--radius-md);
            border: 1px solid #fde68a;
            background: #fffbeb;
            color: #92400e;
            font-size: 0.875rem;
            display: none;
        }
        .admin-nav {
            margin-top: 10px;
            display: none;
            gap: 8px;
            flex-wrap: wrap;
            align-items: center;
        }
        .avatar-preview {
            width: 80px;
            height: 80px;
            display: none;
            border-radius: 50%;
            border: 1px solid var(--color-panel-border);
            background: var(--bg-page);
            margin-bottom: 10px;
            object-fit: cover;
        }
        .admin-only { display: none; }
        .list-box {
            margin-top: 12px;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 4px;
            background: var(--bg-page);
        }
        .list-item {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-sm);
            padding: 8px 10px;
            margin-bottom: 6px;
            background: var(--color-panel);
            font-size: 0.875rem;
        }
        .list-item:last-child { margin-bottom: 0; }
"#;
