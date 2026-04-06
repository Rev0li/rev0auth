pub const PORTAL_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body {
            margin: 0;
            font-family: var(--font-sans);
            color: #132331;
            background:
                radial-gradient(circle at 12% 0%, #ffe7ca 0%, transparent 32%),
                radial-gradient(circle at 88% 12%, #d5ecff 0%, transparent 35%),
                linear-gradient(150deg, #eef8ff 0%, #e6f7ee 100%);
            min-height: 100vh;
        }
        .page {
            max-width: 600px;
            margin: 0 auto;
            padding: 28px;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 16px;
            gap: 10px;
        }
        .header h1 { margin: 0; font-size: clamp(1.5rem, 5vw, 2rem); }
        .card {
            background: rgba(255, 255, 255, 0.9);
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 20px;
            padding: 28px;
            box-shadow: 0 16px 36px rgba(19, 35, 49, 0.14);
        }
        .hint { margin: 12px 0 24px; opacity: .82; font-size: 0.95rem; }
        label { display: block; margin: 14px 0 6px; font-weight: 700; }
        .label-optional { font-size: 0.85rem; font-weight: 400; opacity: 0.75; }
        input, textarea {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 10px;
            padding: 10px;
            font: inherit;
            background: #fff;
            box-sizing: border-box;
        }
        textarea { min-height: 80px; resize: vertical; }
        .btn {
            margin-top: 16px;
            border: 0;
            border-radius: 10px;
            padding: 11px 16px;
            font-weight: 700;
            cursor: pointer;
            width: 100%;
        }
        .btn-primary { color: #fff; background: linear-gradient(120deg, #ff6b3b, #ef4e24); }
        .result {
            margin-top: 14px;
            border-radius: 10px;
            padding: 10px;
            font-size: .92rem;
            display: none;
        }
        .ok { display: block; background: #e8fff5; border: 1px solid #b3ecd1; }
        .down { display: block; background: #fff0ec; border: 1px solid #f3c2b4; }
        .link {
            display: block;
            margin-top: 18px;
            text-align: center;
            text-decoration: none;
            color: #132331;
            font-weight: 700;
        }
"#;
