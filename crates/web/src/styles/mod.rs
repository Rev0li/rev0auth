/// Shared CSS styles for all pages
pub const BASE_STYLE: &str = r#"
    body {
        margin: 0;
        font-family: "Space Grotesk", "Avenir Next", sans-serif;
        color: #132331;
    }
    .page {
        max-width: 1000px;
        margin: 0 auto;
        padding: 28px;
    }
    .card {
        background: rgba(255, 255, 255, 0.87);
        border: 1px solid rgba(19, 35, 49, 0.1);
        border-radius: 22px;
        padding: 24px;
        box-shadow: 0 18px 45px rgba(19, 35, 49, 0.15);
    }
    h1 { margin: 0 0 8px; font-size: clamp(2rem, 5vw, 3rem); }
    h2 { margin: 0 0 10px; }
    p { margin: 0; line-height: 1.55; }
    .cta { margin-top: 18px; display: flex; gap: 12px; flex-wrap: wrap; }
    .btn {
        text-decoration: none;
        font-weight: 700;
        border-radius: 12px;
        padding: 10px 14px;
        display: inline-block;
        border: 0;
        cursor: pointer;
    }
    .btn-primary { color: #fff; background: linear-gradient(120deg, #ff6f3f, #ed5529); }
    .btn-secondary {
        color: #132331;
        border: 1px solid rgba(19, 35, 49, 0.2);
        background: rgba(255, 255, 255, 0.8);
    }
    .link {
        text-decoration: none;
        border: 1px solid rgba(19, 35, 49, 0.16);
        padding: 8px 11px;
        border-radius: 10px;
        color: #132331;
        background: rgba(255, 255, 255, 0.9);
        font-weight: 700;
    }
    .hint { margin: 6px 0 0; opacity: .82; }
    label { display: block; margin: 10px 0 6px; font-weight: 700; }
    input, textarea {
        width: 100%;
        border: 1px solid rgba(19, 35, 49, 0.2);
        border-radius: 10px;
        padding: 10px;
        font: inherit;
        background: #fff;
    }
    textarea { min-height: 92px; resize: vertical; }
    .result {
        margin-top: 12px;
        border-radius: 10px;
        padding: 10px;
        font-size: .92rem;
        display: none;
    }
    .ok { display: block; background: #e8fff5; border: 1px solid #b3ecd1; }
    .down { display: block; background: #fff0ec; border: 1px solid #f3c2b4; }
"#;

pub fn home_gradient() -> &'static str {
    r#"
        background:
            radial-gradient(circle at 10% 5%, #ffe7cd, transparent 35%),
            radial-gradient(circle at 90% 0%, #d9f0ff, transparent 40%),
            linear-gradient(145deg, #eef7ff, #e8f8ef);
        min-height: 100vh;
    "#
}

pub fn portal_gradient() -> &'static str {
    r#"
        background:
            radial-gradient(circle at 12% 0%, #ffe7ca 0%, transparent 32%),
            radial-gradient(circle at 88% 12%, #d5ecff 0%, transparent 35%),
            linear-gradient(150deg, #eef8ff 0%, #e6f7ee 100%);
        min-height: 100vh;
    "#
}

pub fn portal_extra_style() -> &'static str {
    r#"
        .wrap { max-width: 1100px; margin: 0 auto; padding: 24px; }
        .top { display: flex; justify-content: space-between; align-items: center; gap: 10px; flex-wrap: wrap; }
        .grid { margin-top: 14px; display: grid; grid-template-columns: 1fr 1fr; gap: 14px; }
        @media (max-width: 900px) { .grid { grid-template-columns: 1fr; } }
    "#
}

pub fn dashboard_gradient() -> &'static str {
    r#"
        background:
            radial-gradient(circle at 7% 0%, #ffe8ce 0%, transparent 34%),
            radial-gradient(circle at 95% 15%, #d9ecff 0%, transparent 40%),
            linear-gradient(145deg, #eef7ff 0%, #e4f6ec 100%);
        min-height: 100vh;
    "#
}

pub fn dashboard_style() -> &'static str {
    r#"
        :root {
            --ink: #10202e;
            --panel: rgba(255, 255, 255, 0.9);
            --ok: #0d9b73;
            --down: #dc4f2f;
            --tab: #edf7ff;
            --tab-active: #ffdfd2;
        }
        * { box-sizing: border-box; }
        body {
            color: var(--ink);
        }
        .wrap {
            max-width: 1100px;
            margin: 0 auto;
            padding: 22px;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 12px;
            margin-bottom: 14px;
            flex-wrap: wrap;
        }
        .header h1 { margin: 0; font-size: clamp(1.5rem, 4vw, 2.4rem); }
        .chip {
            border: 1px solid rgba(17, 33, 48, 0.12);
            background: #f6fbff;
            border-radius: 999px;
            padding: 7px 12px;
            font-weight: 700;
            font-size: 0.85rem;
        }

        .tabs {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            margin: 6px 0 16px;
        }

        .tab-btn {
            border: 1px solid rgba(17, 33, 48, 0.13);
            background: var(--tab);
            color: var(--ink);
            border-radius: 10px;
            padding: 8px 12px;
            font-weight: 700;
            cursor: pointer;
        }

        .tab-btn.active {
            background: var(--tab-active);
            border-color: rgba(237, 86, 42, 0.35);
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
            box-shadow: 0 16px 38px rgba(17, 33, 48, 0.14);
            animation: reveal .45s ease both;
        }
        @keyframes reveal {
            from { opacity: 0; transform: translateY(12px); }
            to { opacity: 1; transform: translateY(0); }
        }
        .label { text-transform: uppercase; letter-spacing: .05em; font-size: .77rem; opacity: .75; }
        .state {
            margin-top: 8px;
            font-size: 1.4rem;
            font-weight: 800;
        }
        .ok { color: var(--ok); }
        .down { color: var(--down); }
        .meta { margin-top: 10px; font-size: .9rem; opacity: .85; line-height: 1.5; }
        .row {
            margin-top: 16px;
            background: var(--panel);
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 18px;
            padding: 16px;
        }
        .actions { display: flex; gap: 10px; flex-wrap: wrap; margin-top: 10px; }
        a.btn {
            text-decoration: none;
            border-radius: 10px;
            padding: 9px 12px;
            font-weight: 700;
            border: 1px solid rgba(17, 33, 48, 0.16);
            color: var(--ink);
            background: white;
        }

        .tab-page { display: none; }
        .tab-page.active { display: block; }

        .mini {
            margin-top: 10px;
            font-size: 0.88rem;
            line-height: 1.5;
            opacity: 0.85;
        }

        .timeline {
            list-style: none;
            padding: 0;
            margin: 10px 0 0;
            display: grid;
            gap: 7px;
        }

        .timeline li {
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 10px;
            background: #fff;
            padding: 8px 10px;
            font-size: 0.86rem;
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
    "#
}
