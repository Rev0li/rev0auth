pub const HOME_PAGE_STYLES: &str = r#"
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
        .brand { margin-bottom: 32px; }
        .brand-mark {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 36px;
            height: 36px;
            border-radius: var(--radius-md);
            background: var(--foreground);
            color: var(--background);
            font-weight: 800;
            font-size: 1rem;
            letter-spacing: -0.03em;
            user-select: none;
        }
        .card { padding: 24px; }
        h1 {
            margin: 0 0 4px;
            font-size: 1.25rem;
            font-weight: 700;
            letter-spacing: -0.02em;
        }
        .hint {
            margin: 0 0 22px;
            font-size: 0.875rem;
            color: var(--muted-foreground);
        }
        .hidden { display: none; }
        #step-1 .btn, #step-2 .btn { margin-top: 10px; }
        .result { margin-top: 8px; }
        @media (max-width: 480px) {
            .page { padding: 24px 16px; }
            .card { padding: 16px; }
        }
"#;
