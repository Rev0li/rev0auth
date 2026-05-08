pub const DASHBOARD_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        * { box-sizing: border-box; }
        body {
            margin: 0;
            background: var(--bg-page);
            min-height: 100vh;
            -webkit-font-smoothing: antialiased;
        }
        .wrap {
            max-width: 1140px;
            margin: 0 auto;
            padding: 24px 20px;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 12px;
            margin-bottom: 20px;
            flex-wrap: wrap;
        }
        .header h1 {
            margin: 0;
            font-size: 1.375rem;
            font-weight: 700;
            letter-spacing: -0.02em;
        }
        .chip {
            border: 1px solid var(--color-panel-border);
            background: var(--color-panel);
            border-radius: 999px;
            padding: 5px 12px;
            font-weight: 600;
            font-size: 0.8125rem;
            color: var(--color-muted);
        }
        .tabs {
            display: flex;
            gap: 4px;
            flex-wrap: wrap;
            margin: 0 0 20px;
            border-bottom: 1px solid var(--color-panel-border);
            padding-bottom: 0;
        }
        .tab-btn {
            border: none;
            background: transparent;
            color: var(--color-muted);
            border-radius: 0;
            padding: 8px 14px;
            font-weight: 500;
            font-size: 0.875rem;
            cursor: pointer;
            border-bottom: 2px solid transparent;
            margin-bottom: -1px;
            transition: color 0.15s, border-color 0.15s;
        }
        .tab-btn:hover { color: var(--color-ink); }
        .tab-btn.active {
            color: var(--color-accent);
            border-bottom-color: var(--color-accent);
            font-weight: 600;
        }
        .tab-page { display: none; }
        .tab-page.active { display: block; }
        .grid {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 12px;
        }
        .card {
            background: var(--color-panel);
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-lg);
            padding: 16px;
            box-shadow: 0 1px 3px rgba(15, 23, 42, 0.06);
        }
        .row {
            margin-top: 12px;
            background: var(--color-panel);
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-lg);
            padding: 16px;
            box-shadow: 0 1px 3px rgba(15, 23, 42, 0.06);
        }
        .label {
            text-transform: uppercase;
            letter-spacing: 0.06em;
            font-size: 0.7rem;
            font-weight: 600;
            color: var(--color-muted);
        }
        .state {
            margin-top: 6px;
            font-size: 1.25rem;
            font-weight: 700;
        }
        .ok { color: var(--color-success); }
        .down { color: var(--color-danger); }
        .meta { margin-top: 8px; font-size: 0.875rem; color: var(--color-muted); line-height: 1.5; }
        .mini { font-size: 0.8125rem; color: var(--color-muted); line-height: 1.5; margin-top: 8px; }
        .actions { display: flex; gap: 8px; flex-wrap: wrap; margin-top: 10px; }
        .actions-no-top { margin-top: 0; }
        .actions-tight { margin-top: 6px; }
        a.btn {
            text-decoration: none;
            border-radius: var(--radius-md);
            padding: 7px 12px;
            font-weight: 600;
            font-size: 0.875rem;
            border: 1px solid var(--color-panel-border);
            color: var(--color-ink);
            background: var(--color-panel);
            cursor: pointer;
            display: inline-flex;
            align-items: center;
            transition: background 0.1s;
        }
        a.btn:hover { background: var(--bg-page); }
        .btn-small {
            padding: 5px 10px;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-sm);
            background: var(--color-panel);
            color: var(--color-ink);
            font-weight: 600;
            font-size: 0.8125rem;
            cursor: pointer;
            transition: background 0.1s;
        }
        .btn-small:hover { background: var(--bg-page); }
        .btn-small.danger {
            color: var(--color-danger);
            border-color: var(--color-danger-border);
            background: var(--color-danger-bg);
        }
        .btn-small.danger:hover { opacity: 0.85; }
        .btn-small.grant {
            color: var(--color-success);
            border-color: var(--color-success-border);
            background: var(--color-success-bg);
        }
        .btn-small.warn {
            color: #92400e;
            border-color: #fde68a;
            background: #fffbeb;
        }
        code {
            background: var(--bg-page);
            border: 1px solid var(--color-panel-border);
            border-radius: 4px;
            padding: 1px 5px;
            font-size: 0.82em;
            font-family: "ui-monospace", "SF Mono", monospace;
        }
        .timeline {
            list-style: none;
            padding: 0;
            margin: 10px 0 0;
            display: grid;
            gap: 6px;
        }
        .timeline li {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            background: var(--color-panel);
            padding: 7px 10px;
            font-size: 0.8125rem;
            color: var(--color-muted);
        }
        .tests-history {
            margin-top: 12px;
            display: grid;
            gap: 8px;
        }
        .test-run {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            background: var(--color-panel);
            padding: 10px;
        }
        .test-head {
            font-weight: 600;
            font-size: 0.875rem;
        }
        .test-head.ok { color: var(--color-success); }
        .test-head.fail { color: var(--color-error, #e55); }
        .test-cases {
            margin: 6px 0 0;
            padding-left: 16px;
            font-size: 0.8125rem;
            color: var(--color-muted);
            list-style: none;
        }
        .test-cases li {
            opacity: 0;
            animation: fadeInRow 0.18s ease forwards;
        }
        @keyframes fadeInRow {
            from { opacity: 0; transform: translateX(-6px); }
            to   { opacity: 1; transform: translateX(0); }
        }
        .case-ok  { color: var(--color-success); }
        .case-fail { color: var(--color-error, #e55); font-weight: 600; }
        .user-card {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 11px 12px;
            margin: 6px 0;
            background: var(--color-panel);
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 10px;
            flex-wrap: wrap;
        }
        .user-card.clickable {
            cursor: pointer;
            transition: box-shadow 0.15s;
        }
        .user-card.clickable:hover {
            box-shadow: 0 2px 8px rgba(15, 23, 42, 0.08);
        }
        .user-info { flex: 1; min-width: 0; }
        .user-name { font-weight: 600; font-size: 0.9375rem; }
        .user-meta { font-size: 0.8125rem; color: var(--color-muted); margin-top: 2px; }
        .user-actions { display: flex; gap: 6px; flex-wrap: wrap; }
        .user-card-title { margin: 0 0 8px; }
        .onboarding-panel {
            display: none;
            margin: 0 0 14px;
            border: 1px solid var(--color-accent-border);
            background: var(--color-accent-bg);
        }
        .onboarding-panel h2 { margin-top: 0; font-size: 1rem; }
        .form-group {
            margin-bottom: 10px;
        }
        .form-group-top { margin-top: 10px; }
        .form-group label {
            display: block;
            font-weight: 600;
            font-size: 0.8125rem;
            color: var(--color-ink);
            margin-bottom: 4px;
        }
        .form-group input,
        .form-group textarea {
            width: 100%;
            padding: 8px 10px;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            font: inherit;
            font-size: 0.875rem;
            box-sizing: border-box;
            background: var(--color-panel);
            outline: none;
            transition: border-color 0.15s;
        }
        .form-group input:focus,
        .form-group textarea:focus { border-color: var(--color-accent); }
        .form-group textarea {
            min-height: 110px;
            resize: vertical;
        }
        .onboarding-msg {
            margin-top: 8px;
            font-size: 0.875rem;
            display: none;
        }
        .theme-editor-grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 10px;
            margin-top: 10px;
        }
        .field-label {
            display: block;
            font-weight: 600;
            font-size: 0.8125rem;
            margin-bottom: 5px;
        }
        .field-input,
        .field-select,
        .field-textarea {
            width: 100%;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 8px 10px;
            box-sizing: border-box;
            font: inherit;
            font-size: 0.875rem;
            background: var(--color-panel);
            outline: none;
            transition: border-color 0.15s;
        }
        .field-input:focus,
        .field-select:focus,
        .field-textarea:focus { border-color: var(--color-accent); }
        .field-textarea { margin-top: 10px; min-height: 120px; resize: vertical; }
        .mini-top { margin-top: 8px; }
        .preview-grid {
            margin-top: 10px;
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }
        .preview-card { padding: 14px; }
        .preview-title { margin: 8px 0 4px; }
        .preview-feedback { margin-top: 8px; padding: 8px; border: 1px solid; border-radius: var(--radius-md); }
        .preview-input-label { display: block; margin-top: 8px; font-weight: 600; }
        .request-row,
        .donation-row {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 8px 10px;
            margin: 6px 0;
            background: var(--color-panel);
        }
        .request-actions { margin-top: 6px; }
        .donation-actions { margin-top: 6px; display: flex; gap: 8px; flex-wrap: wrap; }
        .user-request-badges { margin-top: 4px; color: #92400e; font-size: 0.8125rem; }
        .card-title-reset { margin-top: 0; }
        .queue-title { margin: 0 0 6px; }
        .queue-meta { margin: 0; color: var(--color-muted); font-size: 0.875rem; }
        .test-path { word-break: break-all; font-size: 0.75rem; }
        .test-result-row { display: flex; gap: 8px; margin-bottom: 6px; align-items: center; }
        .test-result-status { font-weight: 700; font-size: 0.8125rem; }
        .test-result-status.ok { color: var(--color-success); }
        .test-result-status.down { color: var(--color-danger); }
        .test-result-name { font-size: 0.8125rem; flex: 1; color: var(--color-muted); }
        .test-result-time { font-size: 0.75rem; color: var(--color-muted); }
        .theme-token-row { margin-top: 10px; }
        .theme-token-label { display: block; font-weight: 600; font-size: 0.8125rem; }
        .theme-token-key { color: var(--color-muted); font-weight: 400; }
        .theme-token-input {
            width: 100%;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 8px 10px;
            box-sizing: border-box;
            font: inherit;
            font-size: 0.875rem;
            background: var(--color-panel);
        }
        .donation-item-title { margin: 0 0 4px; }
        .donation-photo-link { font-size: 0.8rem; margin: 0; color: var(--color-muted); }
        .stats-strip {
            display: grid;
            grid-template-columns: repeat(4, minmax(120px, 1fr));
            gap: 8px;
            margin-top: 10px;
        }
        .stat-box {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            background: var(--color-panel);
            padding: 12px;
        }
        .stat-k {
            font-size: 0.7rem;
            text-transform: uppercase;
            letter-spacing: 0.06em;
            font-weight: 600;
            color: var(--color-muted);
        }
        .stat-v {
            margin-top: 4px;
            font-size: 1.25rem;
            font-weight: 700;
            letter-spacing: -0.02em;
        }
        .endpoint-grid { margin-top: 10px; display: grid; gap: 6px; }
        .chain-grid { margin-top: 10px; display: grid; gap: 6px; }
        .endpoint-item {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            background: var(--color-panel);
            padding: 8px 10px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 8px;
            flex-wrap: wrap;
        }
        .endpoint-meta { font-size: 0.8125rem; color: var(--color-muted); }
        .badge-ok,
        .badge-ko {
            border-radius: 999px;
            padding: 2px 8px;
            font-size: 0.75rem;
            font-weight: 600;
        }
        .badge-ok {
            color: var(--color-success);
            background: var(--color-success-bg);
            border: 1px solid var(--color-success-border);
        }
        .badge-ko {
            color: var(--color-danger);
            background: var(--color-danger-bg);
            border: 1px solid var(--color-danger-border);
        }
        /* ===== Messenger-style messages ===== */
        .msg-admin-layout {
            display: grid;
            grid-template-columns: 260px 1fr;
            gap: 12px;
            height: 520px;
        }
        .msg-thread-list {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            background: var(--color-panel);
            overflow-y: auto;
            display: flex;
            flex-direction: column;
        }
        .msg-thread-row {
            display: grid;
            grid-template-columns: 40px 1fr auto;
            align-items: center;
            gap: 10px;
            padding: 10px 12px;
            border: none;
            border-bottom: 1px solid var(--color-panel-border);
            background: none;
            cursor: pointer;
            text-align: left;
            transition: background 0.1s;
        }
        .msg-thread-row:last-child { border-bottom: none; }
        .msg-thread-row:hover { background: var(--bg-page); }
        .msg-thread-row.active { background: var(--color-accent-bg); }
        .msg-thread-avatar {
            width: 38px;
            height: 38px;
            border-radius: 50%;
            background: var(--color-accent, #4a9eff);
            color: #fff;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 700;
            font-size: 0.85rem;
            flex-shrink: 0;
        }
        .msg-thread-name {
            font-weight: 600;
            font-size: 0.875rem;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .msg-thread-preview {
            font-size: 0.75rem;
            color: var(--color-muted);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .msg-thread-time { font-size: 0.72rem; color: var(--color-muted); white-space: nowrap; }
        .msg-unread-badge {
            display: inline-block;
            background: var(--color-accent, #4a9eff);
            color: #fff;
            border-radius: 10px;
            padding: 1px 6px;
            font-size: 0.72rem;
            font-weight: 700;
        }
        .msg-empty { text-align: center; color: var(--color-muted); font-size: 0.875rem; padding: 20px; margin: auto; }
        .msg-admin-panel {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            background: var(--color-panel);
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }
        .msg-conversation {
            flex: 1;
            overflow-y: auto;
            padding: 14px;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
        .msg-bubble {
            max-width: 72%;
            padding: 8px 12px;
            border-radius: 16px;
            word-break: break-word;
            white-space: pre-wrap;
        }
        .msg-bubble.mine {
            align-self: flex-end;
            background: var(--color-accent, #4a9eff);
            color: #fff;
            border-bottom-right-radius: 4px;
        }
        .msg-bubble.theirs {
            align-self: flex-start;
            background: var(--bg-page);
            border: 1px solid var(--color-panel-border);
            border-bottom-left-radius: 4px;
        }
        .msg-bubble-text { font-size: 0.875rem; line-height: 1.45; }
        .msg-bubble-meta { font-size: 0.72rem; opacity: 0.65; margin-top: 3px; text-align: right; }
        .msg-compose {
            border-top: 1px solid var(--color-panel-border);
            padding: 10px;
            background: var(--bg-page);
            flex-shrink: 0;
        }
        .msg-compose-row {
            display: flex;
            gap: 8px;
            align-items: flex-end;
        }
        .msg-compose-input {
            flex: 1;
            border: 1px solid var(--color-panel-border);
            border-radius: 20px;
            padding: 8px 14px;
            font: inherit;
            font-size: 0.875rem;
            background: var(--color-panel);
            resize: none;
            max-height: 90px;
            outline: none;
            transition: border-color 0.15s;
            line-height: 1.4;
        }
        .msg-compose-input:focus { border-color: var(--color-accent); }
        .msg-compose-send {
            width: 38px;
            height: 38px;
            border-radius: 50%;
            background: var(--color-accent, #4a9eff);
            color: #fff;
            border: none;
            cursor: pointer;
            font-size: 1rem;
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
            transition: opacity 0.15s;
        }
        .msg-compose-send:disabled { opacity: 0.45; }
        .msg-reply-status { font-size: 0.8rem; margin-top: 6px; min-height: 16px; }

        /* ===== Member gallery ===== */
        .member-gallery {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
            gap: 14px;
            margin-top: 6px;
        }
        .member-card {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            background: var(--color-panel);
            padding: 14px 10px 10px;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 6px;
            cursor: pointer;
            transition: box-shadow 0.15s, border-color 0.15s;
        }
        .member-card:hover {
            border-color: var(--color-accent-border);
            box-shadow: 0 4px 14px rgba(0,0,0,0.1);
        }
        .member-card-avatar-wrap {
            position: relative;
            width: 54px;
            height: 54px;
            border-radius: 50%;
            overflow: hidden;
            flex-shrink: 0;
        }
        .member-card-avatar {
            width: 100%;
            height: 100%;
            object-fit: cover;
            display: block;
        }
        .member-card-avatar-fallback {
            position: absolute;
            inset: 0;
            display: none;
            align-items: center;
            justify-content: center;
            background: var(--color-accent, #4a9eff);
            color: #fff;
            font-weight: 700;
            font-size: 1.1rem;
        }
        .member-card-pseudo {
            font-weight: 700;
            font-size: 0.9rem;
            text-align: center;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            max-width: 100%;
        }
        .member-card-meta {
            font-size: 0.75rem;
            color: var(--color-muted);
            display: flex;
            align-items: center;
            gap: 5px;
            flex-wrap: wrap;
            justify-content: center;
        }
        .member-status { font-size: 0.65rem; }
        .member-status.active { color: #0d9b73; }
        .member-status.inactive { color: var(--color-muted); }
        .member-status.pending { color: #e8a000; }
        .member-badge {
            font-size: 0.7rem;
            padding: 1px 6px;
            border-radius: 8px;
            font-weight: 600;
        }
        .member-badge.admin { background: #fee2e2; color: #b91c1c; }
        .member-badge.mod { background: #fef3c7; color: #92400e; }
        .member-badge.member { background: #d1fae5; color: #065f46; }
        .member-badge.guest { background: var(--color-accent-bg); color: var(--color-muted); }
        .member-card-actions {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
            justify-content: center;
            margin-top: 4px;
        }

        @media (max-width: 900px) {
            .grid { grid-template-columns: 1fr; }
            .stats-strip { grid-template-columns: 1fr 1fr; }
            .msg-admin-layout { grid-template-columns: 1fr; height: auto; }
            .msg-thread-list { max-height: 200px; }
            .msg-conversation { min-height: 220px; }
            .member-gallery { grid-template-columns: repeat(auto-fill, minmax(130px, 1fr)); }
            .service-card { aspect-ratio: 1 / 1; overflow: hidden; }
            .service-media { height: auto; aspect-ratio: 1 / 1; margin-bottom: 8px; }
        }
    "#;
