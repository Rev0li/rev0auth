pub const TDD_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body {
            margin: 0;
            min-height: 100vh;
            font-family: var(--font-sans);
        }
        .wrap {
            max-width: 1140px;
            margin: 0 auto;
            padding: 24px;
        }
        .top {
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 12px;
            flex-wrap: wrap;
            margin-bottom: 20px;
        }
        h1 {
            margin: 0;
            font-size: clamp(1.5rem, 4vw, 2.25rem);
            font-weight: 800;
            letter-spacing: -0.04em;
        }
        .chip {
            border: 1px solid var(--border);
            background: var(--muted);
            border-radius: 999px;
            padding: 6px 12px;
            font-size: 0.8rem;
            font-weight: 600;
            color: var(--muted-foreground);
        }
        .grid {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 12px;
        }
        .card {
            padding: 16px;
        }
        .card h2 {
            margin: 0 0 8px;
            font-size: 0.9375rem;
            font-weight: 700;
        }
        .state {
            font-size: 1.25rem;
            font-weight: 800;
            margin-bottom: 6px;
        }
        .ok   { color: var(--success); }
        .down { color: var(--destructive); }
        .wait { color: var(--warning); }
        .mini { font-size: 0.875rem; color: var(--muted-foreground); line-height: 1.5; }
        .row {
            margin-top: 14px;
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            background: var(--card);
            padding: 16px;
            box-shadow: var(--shadow-soft);
        }
        .actions {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            margin-top: 10px;
        }
        .suite-list { display: grid; gap: 10px; margin-top: 12px; }
        .suite {
            border: 1px solid var(--border);
            border-radius: var(--radius-lg);
            background: var(--muted);
            padding: 12px;
        }
        .suite-top {
            display: flex;
            justify-content: space-between;
            gap: 10px;
            flex-wrap: wrap;
            align-items: center;
        }
        .suite-name { font-weight: 700; font-size: 0.9rem; }
        .suite-command {
            margin-top: 6px;
            font: 0.8125rem/1.5 var(--font-mono);
            background: var(--muted);
            border: 1px solid var(--border);
            border-radius: var(--radius-md);
            padding: 8px 10px;
            overflow-x: auto;
            color: var(--foreground);
        }
        .log {
            margin-top: 10px;
            padding: 12px;
            background: var(--foreground);
            color: var(--background);
            border-radius: var(--radius-lg);
            min-height: 150px;
            white-space: pre-wrap;
            font: 0.8rem/1.5 var(--font-mono);
        }
        code {
            background: var(--muted);
            border: 1px solid var(--border);
            border-radius: var(--radius-sm);
            padding: 1px 6px;
            font: 0.84em var(--font-mono);
        }
        .btn-songsurf {
            background: rgba(232,183,196,0.15);
            border: 1px solid rgba(232,183,196,0.45);
            color: #E8B7C4;
            font-weight: 700;
        }
        .btn-songsurf:hover {
            background: rgba(232,183,196,0.28);
            border-color: rgba(232,183,196,0.7);
        }
        @media (max-width: 860px) {
            .grid { grid-template-columns: 1fr 1fr; }
        }
        @media (max-width: 540px) {
            .grid { grid-template-columns: 1fr; }
        }
"#;
