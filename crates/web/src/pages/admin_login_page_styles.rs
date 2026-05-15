pub const ADMIN_LOGIN_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body { margin: 0; min-height: 100vh; }
        .page {
            max-width: 400px;
            margin: 0 auto;
            padding: 40px 24px;
            display: flex;
            flex-direction: column;
            justify-content: center;
            min-height: 100vh;
        }
        .brand { margin-bottom: 28px; }
        .brand-badge {
            display: inline-block;
            background: var(--foreground);
            color: var(--background);
            font-size: 0.6875rem;
            font-weight: 700;
            letter-spacing: 0.1em;
            text-transform: uppercase;
            padding: 3px 8px;
            border-radius: var(--radius-sm);
        }
        .card { padding: 24px; }
        h1 {
            margin: 0 0 4px;
            font-size: 1.25rem;
            font-weight: 700;
            letter-spacing: -0.02em;
        }
        .hint {
            margin: 0 0 20px;
            font-size: 0.875rem;
            color: var(--muted-foreground);
        }
        /* TOTP / challenge */
        .challenge-label {
            margin-top: 14px;
            font-size: 0.8125rem;
            font-weight: 600;
            color: var(--muted-foreground);
        }
        .challenge-grid {
            margin-top: 8px;
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 6px;
        }
        .challenge-btn {
            border: 1px solid var(--border);
            background: var(--card);
            color: var(--foreground);
            border-radius: var(--radius-md);
            padding: 8px;
            font: 600 0.875rem/1 var(--font-sans);
            cursor: pointer;
            transition: background 0.1s, border-color 0.1s;
        }
        .challenge-btn:hover { background: var(--muted); }
        .challenge-btn.selected {
            border-color: var(--foreground);
            background: var(--foreground);
            color: var(--background);
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
