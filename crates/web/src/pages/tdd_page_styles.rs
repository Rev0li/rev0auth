pub const TDD_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        :root {
            --ink: #10202e;
            --panel: rgba(255, 255, 255, 0.92);
            --ok: #0d9b73;
            --down: #dc4f2f;
            --wait: #927d2a;
            --bg1: #fff0df;
            --bg2: #dcedff;
        }
        * { box-sizing: border-box; }
        body {
            margin: 0;
            font-family: var(--font-sans);
            color: var(--ink);
            background:
                radial-gradient(circle at 8% 0%, var(--bg1) 0%, transparent 34%),
                radial-gradient(circle at 92% 10%, var(--bg2) 0%, transparent 38%),
                linear-gradient(145deg, #f4f8ff 0%, #edf8f0 100%);
            min-height: 100vh;
        }
        .wrap {
            max-width: 1180px;
            margin: 0 auto;
            padding: 24px;
        }
        .top {
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 12px;
            flex-wrap: wrap;
            margin-bottom: 16px;
        }
        h1 { margin: 0; font-size: clamp(1.6rem, 4vw, 2.6rem); }
        .chip {
            border: 1px solid rgba(17, 33, 48, 0.12);
            background: #f8fbff;
            border-radius: 999px;
            padding: 8px 12px;
            font-size: 0.86rem;
            font-weight: 700;
        }
        .grid {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 14px;
        }
        .card {
            background: var(--panel);
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 18px;
            padding: 18px;
            box-shadow: 0 16px 38px rgba(17, 33, 48, 0.12);
        }
        .card h2 {
            margin: 0 0 8px;
            font-size: 1.05rem;
        }
        .state {
            font-size: 1.35rem;
            font-weight: 800;
            margin-bottom: 8px;
        }
        .ok { color: var(--ok); }
        .down { color: var(--down); }
        .wait { color: var(--wait); }
        .mini { font-size: 0.92rem; line-height: 1.5; opacity: 0.85; }
        .row {
            margin-top: 16px;
            background: var(--panel);
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 18px;
            padding: 16px;
        }
        .actions {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            margin-top: 10px;
        }
        button, a.btn {
            border: 1px solid rgba(17, 33, 48, 0.16);
            background: #fff;
            color: var(--ink);
            border-radius: 10px;
            padding: 9px 12px;
            font-weight: 700;
            cursor: pointer;
            text-decoration: none;
        }
        .suite-list {
            display: grid;
            gap: 10px;
            margin-top: 12px;
        }
        .suite {
            background: #fff;
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 12px;
            padding: 12px;
        }
        .suite-top {
            display: flex;
            justify-content: space-between;
            gap: 10px;
            flex-wrap: wrap;
            align-items: center;
        }
        .suite-name { font-weight: 800; }
        .suite-command {
            margin-top: 6px;
            font-size: 0.88rem;
            font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
            background: #f7fafe;
            border: 1px solid #d9e8f5;
            border-radius: 8px;
            padding: 8px 10px;
            overflow-x: auto;
        }
        .log {
            margin-top: 10px;
            padding: 12px;
            background: #0f1a24;
            color: #d9e6f2;
            border-radius: 12px;
            min-height: 160px;
            white-space: pre-wrap;
            font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
            font-size: 0.84rem;
            line-height: 1.45;
        }
        code {
            background: #f2f9ff;
            border: 1px solid #d4e7f8;
            border-radius: 6px;
            padding: 1px 6px;
            font-size: 0.84em;
        }
        @media (max-width: 900px) {
            .grid { grid-template-columns: 1fr; }
        }
"#;
