pub const DASHBOARD_PAGE_STYLES: &str = r#"
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
            margin: 0;
            font-family: var(--font-sans);
            color: var(--ink);
            background:
                radial-gradient(circle at 7% 0%, #ffe8ce 0%, transparent 34%),
                radial-gradient(circle at 95% 15%, #d9ecff 0%, transparent 40%),
                linear-gradient(145deg, #eef7ff 0%, #e4f6ec 100%);
            min-height: 100vh;
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
        .tests-history {
            margin-top: 12px;
            display: grid;
            gap: 8px;
        }
        .test-run {
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 10px;
            background: #fff;
            padding: 10px;
        }
        .test-head {
            font-weight: 700;
            font-size: 0.9rem;
        }
        .test-cases {
            margin: 6px 0 0;
            padding-left: 16px;
            font-size: 0.85rem;
            opacity: 0.88;
        }

        code {
            background: #f2f9ff;
            border: 1px solid #d4e7f8;
            border-radius: 6px;
            padding: 1px 6px;
            font-size: 0.84em;
        }

        .user-card {
            border: 1px solid rgba(17, 33, 48, 0.13);
            border-radius: 10px;
            padding: 12px;
            margin: 8px 0;
            background: #fff;
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 10px;
            flex-wrap: wrap;
        }

        .user-card.clickable {
            cursor: pointer;
            transition: transform .15s ease, box-shadow .2s ease;
        }

        .user-card.clickable:hover {
            transform: translateY(-1px);
            box-shadow: 0 8px 20px rgba(17, 33, 48, 0.1);
        }

        .user-info { flex: 1; }
        .user-name { font-weight: 700; }
        .user-meta { font-size: 0.85rem; opacity: 0.75; }
        .user-actions { display: flex; gap: 6px; flex-wrap: wrap; }

        .btn-small {
            padding: 6px 10px;
            border: 1px solid rgba(17, 33, 48, 0.16);
            border-radius: 6px;
            background: #fff;
            color: var(--ink);
            font-weight: 600;
            font-size: 0.85rem;
            cursor: pointer;
        }

        .btn-small:hover { background: #f0f5fa; }
        .btn-small.danger {
            color: #dc4f2f;
            border-color: #fcc5b7;
        }

        .btn-small.danger:hover { background: #fff0ed; }

        .btn-small.grant {
            color: #0d9b73;
            border-color: #a9e2ce;
        }

        .onboarding-panel {
            display: none;
            margin: 0 0 16px;
            border: 2px solid rgba(237, 86, 42, 0.22);
            background: linear-gradient(135deg, rgba(255, 255, 255, 0.96), rgba(255, 245, 240, 0.96));
            box-shadow: 0 14px 28px rgba(237, 86, 42, 0.12);
        }

        .onboarding-panel h2 {
            margin-top: 0;
        }

        .form-group textarea {
            width: 100%;
            min-height: 110px;
            padding: 8px;
            border: 1px solid rgba(17, 33, 48, 0.16);
            border-radius: 8px;
            font: inherit;
            box-sizing: border-box;
            resize: vertical;
        }

        .btn-small.warn {
            color: #8a5a00;
            border-color: #f6d08a;
            background: #fff9ea;
        }

        .form-group {
            margin-bottom: 10px;
        }

        .form-group label {
            display: block;
            font-weight: 600;
            font-size: 0.9rem;
            margin-bottom: 4px;
        }

        .form-group input {
            width: 100%;
            padding: 8px;
            border: 1px solid rgba(17, 33, 48, 0.16);
            border-radius: 8px;
            font-size: 0.9rem;
            box-sizing: border-box;
        }

        .stats-strip {
            display: grid;
            grid-template-columns: repeat(4, minmax(140px, 1fr));
            gap: 8px;
            margin-top: 10px;
        }

        .stat-box {
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 10px;
            background: #fff;
            padding: 10px;
        }

        .stat-k {
            font-size: 0.75rem;
            text-transform: uppercase;
            letter-spacing: .05em;
            opacity: .72;
        }

        .stat-v {
            margin-top: 4px;
            font-size: 1.15rem;
            font-weight: 800;
        }

        .endpoint-grid {
            margin-top: 10px;
            display: grid;
            gap: 8px;
        }

        .chain-grid {
            margin-top: 10px;
            display: grid;
            gap: 8px;
        }

        .endpoint-item {
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 10px;
            background: #fff;
            padding: 8px 10px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 8px;
            flex-wrap: wrap;
        }

        .endpoint-meta {
            font-size: .85rem;
            opacity: .8;
        }

        .badge-ok,
        .badge-ko {
            border-radius: 999px;
            padding: 3px 8px;
            font-size: .78rem;
            font-weight: 700;
        }

        .badge-ok {
            color: #085e48;
            background: #dff7ee;
            border: 1px solid #9bdcc7;
        }

        .badge-ko {
            color: #7e2a18;
            background: #ffe8e1;
            border: 1px solid #f6b7a6;
        }
        .chat-admin-wrap {
            margin-top: 10px;
            border: 1px solid rgba(17, 33, 48, 0.12);
            border-radius: 12px;
            background: #fff;
            padding: 10px;
        }
        .chat-admin-layout {
            display: grid;
            grid-template-columns: 240px minmax(0, 1fr);
            gap: 10px;
        }
        .chat-admin-threads {
            border: 1px solid rgba(17, 33, 48, 0.12);
            border-radius: 10px;
            background: #f8fbfe;
            padding: 8px;
            max-height: 520px;
            overflow: auto;
        }
        .chat-admin-thread {
            width: 100%;
            border: 1px solid rgba(17, 33, 48, 0.12);
            border-radius: 10px;
            background: #fff;
            padding: 8px 10px;
            margin-bottom: 8px;
            cursor: pointer;
            text-align: left;
        }
        .chat-admin-thread.active {
            border-color: rgba(13, 155, 115, 0.35);
            background: #e8fff5;
        }
        .chat-admin-thread-name {
            font-weight: 700;
            font-size: 0.92rem;
        }
        .chat-admin-thread-meta {
            margin-top: 4px;
            font-size: 0.8rem;
            opacity: 0.8;
            line-height: 1.35;
        }
        .chat-admin-history {
            max-height: 340px;
            overflow: auto;
            display: grid;
            gap: 8px;
            padding-right: 4px;
        }
        .chat-admin-item {
            border: 1px solid rgba(17, 33, 48, 0.12);
            border-radius: 10px;
            background: #fff;
            padding: 8px;
        }
        .chat-admin-head {
            display: flex;
            justify-content: space-between;
            gap: 8px;
            flex-wrap: wrap;
            font-size: 0.84rem;
            opacity: 0.82;
        }
        .chat-admin-body {
            margin-top: 6px;
            white-space: pre-wrap;
            line-height: 1.45;
            font-size: 0.9rem;
        }
        .chat-admin-compose {
            margin-top: 10px;
            border-top: 1px dashed rgba(17, 33, 48, 0.18);
            padding-top: 10px;
            display: grid;
            gap: 8px;
        }
        .chat-admin-compose input,
        .chat-admin-compose textarea {
            width: 100%;
            padding: 8px;
            border: 1px solid rgba(17, 33, 48, 0.16);
            border-radius: 8px;
            font: inherit;
            box-sizing: border-box;
            background: #fff;
        }
        .chat-admin-compose textarea {
            min-height: 88px;
            resize: vertical;
        }
        .chat-admin-msg {
            font-size: 0.88rem;
            display: none;
        }
        .chat-admin-panel {
            border: 1px solid rgba(17, 33, 48, 0.12);
            border-radius: 10px;
            background: #fff;
            padding: 8px;
        }

        @media (max-width: 900px) {
            .grid { grid-template-columns: 1fr; }
            .stats-strip { grid-template-columns: 1fr 1fr; }
            .chat-admin-layout { grid-template-columns: 1fr; }
            .chat-admin-threads { max-height: 180px; }
            .service-card {
                aspect-ratio: 1 / 1;
                overflow: hidden;
            }
            .service-media {
                height: auto;
                aspect-ratio: 1 / 1;
                margin-bottom: 8px;
            }
        }
    "#;
