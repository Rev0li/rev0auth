// Profile page styles as constant for modular extraction

pub const PROFILE_PAGE_STYLES: &str = r#"
        body {
            margin: 0;
            font-family: var(--font-sans);
            color: #132331;
            background:
                radial-gradient(circle at 10% 5%, #ffe7cd, transparent 35%),
                radial-gradient(circle at 90% 0%, #d9f0ff, transparent 40%),
                linear-gradient(145deg, #eef7ff, #e8f8ef);
            min-height: 100vh;
        }
        .page {
            max-width: 760px;
            margin: 0 auto;
            padding: 24px;
        }
        .card {
            background: rgba(255, 255, 255, 0.92);
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 18px;
            padding: 20px;
            box-shadow: 0 12px 34px rgba(19, 35, 49, 0.14);
            margin-bottom: 14px;
        }
        h1 { margin-top: 0; }
        label { display: block; font-weight: 700; margin: 10px 0 6px; }
        input, textarea {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            padding: 9px;
            box-sizing: border-box;
            font: inherit;
            background: #fff;
        }
        textarea { min-height: 90px; resize: vertical; }
        .actions {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            margin-top: 12px;
        }
        button, a.btn {
            border: 0;
            border-radius: 10px;
            padding: 9px 13px;
            font-weight: 700;
            cursor: pointer;
            text-decoration: none;
        }
        .primary { background: linear-gradient(120deg, #ff6b3b, #ef4e24); color: #fff; }
        .secondary { background: #f2f9ff; color: #132331; border: 1px solid rgba(19, 35, 49, 0.15); }
        .msg {
            margin-top: 10px;
            font-size: 0.9rem;
            border-radius: 8px;
            padding: 8px;
            display: none;
        }
        .ok { display: block; background: #e8fff5; border: 1px solid #b3ecd1; color: #0d9b73; }
        .down { display: block; background: #fff0ec; border: 1px solid #f3c2b4; color: #ef4e24; }
        .meta { font-size: 0.9rem; opacity: 0.82; }
        .admin-note {
            margin-top: 10px;
            padding: 9px 10px;
            border-radius: 10px;
            border: 1px solid #f6d08a;
            background: #fff9ea;
            color: #6d4b00;
            font-size: 0.88rem;
            display: none;
        }
        .admin-nav {
            margin-top: 10px;
            display: none;
            gap: 10px;
            flex-wrap: wrap;
            align-items: center;
        }
        .avatar-preview {
            width: 100%;
            max-width: 280px;
            display: none;
            border-radius: 14px;
            border: 1px solid rgba(19, 35, 49, 0.16);
            background: #f3f7fa;
            margin-bottom: 10px;
        }
        .admin-only {
            display: none;
        }
        .list-box {
            margin-top: 12px;
            border: 1px solid rgba(19, 35, 49, 0.12);
            border-radius: 10px;
            padding: 10px;
            background: rgba(255, 255, 255, 0.85);
        }
        .list-item {
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 8px;
            padding: 8px;
            margin-bottom: 8px;
            background: #fff;
        }
        .list-item:last-child {
            margin-bottom: 0;
        }
"#;
