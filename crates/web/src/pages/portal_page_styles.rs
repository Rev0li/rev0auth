pub const PORTAL_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body { margin: 0; min-height: 100vh; }
        .page {
            max-width: 520px;
            margin: 0 auto;
            padding: 40px 24px 60px;
        }
        .header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 10px;
            margin-bottom: 24px;
        }
        .header h1 {
            margin: 0;
            font-size: 1.25rem;
            font-weight: 700;
            letter-spacing: -0.02em;
        }
        .card { padding: 24px; }
        .hint {
            margin: 0 0 20px;
            font-size: 0.875rem;
            color: var(--muted-foreground);
            line-height: 1.6;
        }
        .hint-warn {
            display: block;
            margin: 4px 0 0;
            font-size: 0.8125rem;
            color: var(--destructive);
        }
        .field { margin-bottom: 14px; }
        label {
            display: block;
            margin: 0 0 5px;
            font-size: 0.8125rem;
            font-weight: 600;
        }
        .label-optional {
            font-weight: 400;
            color: var(--muted-foreground);
        }
"#;
