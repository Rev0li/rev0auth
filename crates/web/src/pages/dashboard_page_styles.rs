pub const DASHBOARD_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body { margin: 0; min-height: 100vh; }

        /* ===== Layout ===== */
        .wrap {
            max-width: 1100px;
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
            font-size: 1.25rem;
            font-weight: 800;
            letter-spacing: -0.03em;
        }

        /* ===== Status grid (Admin / User / API cards) ===== */
        .grid {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 10px;
            margin-bottom: 12px;
        }
        .grid .card {
            padding: 16px 18px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 10px;
        }
        .label {
            font-size: 0.75rem;
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 0.06em;
            color: var(--muted-foreground);
        }
        @media (max-width: 560px) {
            .grid { grid-template-columns: 1fr; }
        }

        /* ===== Chip / badge ===== */
        .chip {
            display: inline-flex;
            align-items: center;
            height: 24px;
            padding: 0 10px;
            border: 1px solid var(--border);
            background: var(--muted);
            border-radius: 999px;
            font: 600 0.75rem/1 var(--font-sans);
            color: var(--muted-foreground);
            white-space: nowrap;
        }

        /* ===== Tabs — MyCss pill style ===== */
        .tabs {
            display: flex;
            gap: 0;
            margin: 0 0 20px;
            background: var(--muted);
            border-radius: var(--radius-lg);
            padding: 3px;
            width: fit-content;
            flex-wrap: wrap;
        }
        .tab-btn {
            border: none;
            background: transparent;
            color: var(--muted-foreground);
            border-radius: var(--radius-md);
            padding: 6px 14px;
            font: 500 0.8125rem/1 var(--font-sans);
            cursor: pointer;
            white-space: nowrap;
            transition: background 0.15s, color 0.15s, box-shadow 0.15s;
            border: 1px solid transparent;
        }
        .tab-btn:hover { color: var(--foreground); }
        .tab-btn.active {
            background: var(--card);
            color: var(--foreground);
            box-shadow: var(--shadow-soft);
            border-color: var(--border);
            font-weight: 600;
        }

        /* ===== Tab pages ===== */
        .tab-page { display: none; }
        .tab-page.active { display: block; }

        /* ===== Row / section ===== */
        .row {
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            background: var(--card);
            padding: 16px 18px;
            margin-bottom: 12px;
            box-shadow: var(--shadow-soft);
        }
        strong { font-weight: 700; font-size: 0.9rem; }

        /* ===== State indicators ===== */
        .state {
            display: inline-flex;
            align-items: center;
            height: 22px;
            padding: 0 8px;
            border-radius: var(--radius-md);
            font-size: 0.75rem;
            font-weight: 600;
            border: 1px solid transparent;
        }
        .state.ok    { background: var(--success-bg); color: var(--success); border-color: var(--success-border); }
        .state.down  { background: var(--destructive-bg);  color: var(--destructive);  border-color: var(--destructive-border); }
        .state.wait  { background: #fff7ed; color: #c2410c; border-color: #fed7aa; }
        .state.mini  { font-size: 0.6875rem; height: 18px; }

        /* ===== Member gallery ===== */
        .member-gallery {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
            gap: 8px;
            margin-top: 12px;
        }
        .member-card {
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            background: var(--muted);
            padding: 12px 10px;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 5px;
            text-align: center;
            cursor: pointer;
            transition: border-color 0.15s, box-shadow 0.15s;
        }
        .member-card:hover { border-color: var(--foreground); box-shadow: var(--shadow-soft); }
        .member-card-pending { border-color: rgba(251,191,36,0.4); background: rgba(251,191,36,0.04); }
        .badge-pending {
            display: inline-block;
            font-size: 0.65rem;
            font-weight: 700;
            background: rgba(251,191,36,0.15);
            color: #ca8a04;
            border: 1px solid rgba(251,191,36,0.35);
            border-radius: 9999px;
            padding: 1px 6px;
            vertical-align: middle;
            margin-left: 4px;
        }
        .member-card-avatar-wrap { position: relative; }
        .member-card-avatar {
            width: 46px;
            height: 46px;
            border-radius: 50%;
            object-fit: cover;
            border: 1px solid var(--border);
            display: block;
        }
        .member-card-avatar-fallback {
            width: 46px;
            height: 46px;
            border-radius: 50%;
            background: var(--foreground);
            color: var(--background);
            font-size: 1rem;
            font-weight: 700;
            display: none;
            align-items: center;
            justify-content: center;
        }
        .member-card-pseudo { font-size: 0.8rem; font-weight: 700; }
        .member-card-meta   { display: flex; gap: 4px; flex-wrap: wrap; justify-content: center; }

        /* ===== Status / role badges ===== */
        .member-status,
        .member-badge {
            display: inline-flex;
            align-items: center;
            height: 16px;
            padding: 0 5px;
            border-radius: var(--radius-sm);
            font-size: 0.625rem;
            font-weight: 700;
            letter-spacing: 0.01em;
            border: 1px solid transparent;
        }
        .member-status.active   { background: var(--success-bg); color: var(--success); border-color: var(--success-border); }
        .member-status.pending  { background: #fff7ed; color: #c2410c; border-color: #fed7aa; }
        .member-status.inactive { background: var(--muted); color: var(--muted-foreground); border-color: var(--border); }
        .member-badge.admin  { background: var(--foreground); color: var(--background); }
        .member-badge.mod    { background: #581c87; color: #e9d5ff; }
        .member-badge.member,
        .member-badge.guest  { background: var(--muted); color: var(--muted-foreground); border-color: var(--border); }

        /* ===== User profile modal ===== */
        .user-profile-modal {
            display: none;
            position: fixed;
            inset: 0;
            background: rgba(0,0,0,0.5);
            z-index: 500;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }
        .user-profile-modal.open { display: flex; }
        .user-profile-panel {
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            width: 100%;
            max-width: 480px;
            max-height: 88vh;
            overflow-y: auto;
            padding: 22px;
            box-shadow: var(--shadow-hover);
        }
        .upm-header {
            display: flex;
            align-items: flex-start;
            justify-content: space-between;
            gap: 12px;
            margin-bottom: 16px;
        }
        .upm-title { font-size: 1rem; font-weight: 700; margin: 0; }
        .upm-close {
            background: none;
            border: none;
            cursor: pointer;
            font-size: 1rem;
            color: var(--muted-foreground);
            padding: 4px;
            border-radius: var(--radius-sm);
            line-height: 1;
        }
        .upm-close:hover { color: var(--foreground); }
        .upm-avatar-row {
            display: flex;
            align-items: center;
            gap: 14px;
            margin-bottom: 16px;
        }
        .upm-avatar {
            width: 60px;
            height: 60px;
            border-radius: 50%;
            border: 1px solid var(--border);
            object-fit: cover;
            background: var(--muted);
        }
        .upm-avatar-fallback {
            width: 60px;
            height: 60px;
            border-radius: 50%;
            background: var(--foreground);
            color: var(--background);
            font-size: 1.375rem;
            font-weight: 700;
            display: none;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
        }
        .upm-meta { display: grid; gap: 4px; }
        .upm-pseudo { font-weight: 700; font-size: 1rem; }
        .upm-badges { display: flex; gap: 4px; flex-wrap: wrap; }

        /* Generic info rows inside modal */
        .upm-section { margin-bottom: 14px; }
        .upm-section-title {
            font-size: 0.75rem;
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 0.06em;
            color: var(--muted-foreground);
            margin: 0 0 8px;
        }
        .upm-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 8px;
            font-size: 0.875rem;
            padding: 5px 0;
            border-bottom: 1px solid var(--border);
        }
        .upm-row:last-child { border-bottom: 0; }
        .upm-row-label { color: var(--muted-foreground); font-size: 0.8125rem; }
        .upm-row-val   { font-weight: 500; text-align: right; }
        .upm-actions { display: flex; gap: 6px; flex-wrap: wrap; margin-top: 14px; }
        .upm-msg { margin-top: 8px; font-size: 0.8125rem; min-height: 18px; }

        /* ===== Service request badges ===== */
        .req-badge {
            display: inline-flex;
            align-items: center;
            height: 18px;
            padding: 0 6px;
            border-radius: var(--radius-sm);
            font-size: 0.65rem;
            font-weight: 700;
            background: var(--muted);
            color: var(--muted-foreground);
            border: 1px solid var(--border);
        }
        .req-badge.pending { background: #fff7ed; color: #c2410c; border-color: #fed7aa; }

        /* ===== Community wall (admin) ===== */
        .admin-wall-post {
            display: flex;
            align-items: flex-start;
            justify-content: space-between;
            gap: 8px;
            padding: 8px 0;
            border-bottom: 1px solid var(--border);
            font-size: 0.875rem;
        }
        .admin-wall-post:last-child { border-bottom: 0; }
        .wall-post-pseudo { font-weight: 700; margin-right: 5px; }

        /* ===== Messaging panel ===== */
        .msg-admin-layout {
            display: grid;
            grid-template-columns: 220px 1fr;
            gap: 12px;
            min-height: 300px;
        }
        .msg-thread-list {
            border: 1px solid var(--border);
            border-radius: var(--radius-lg);
            overflow-y: auto;
            max-height: 380px;
            background: var(--muted);
        }
        .msg-thread-wrap {
            display: flex;
            align-items: stretch;
            border-bottom: 1px solid var(--border);
            position: relative;
        }
        .msg-thread-wrap:last-child { border-bottom: 0; }
        .msg-thread-wrap:hover .msg-thread-delete { opacity: 1; }
        .msg-thread-row {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 10px 12px;
            border: none;
            background: none;
            cursor: pointer;
            flex: 1;
            min-width: 0;
            text-align: left;
            transition: background 0.1s;
        }
        .msg-thread-row:hover { background: var(--card); }
        .msg-thread-row.active { background: var(--card); }
        .msg-thread-delete {
            opacity: 0;
            flex-shrink: 0;
            border: none;
            background: none;
            color: var(--muted-foreground);
            cursor: pointer;
            font-size: 0.7rem;
            padding: 0 10px;
            transition: opacity 0.15s, color 0.15s, background 0.15s;
            border-left: 1px solid var(--border);
        }
        .msg-thread-delete:hover { color: var(--destructive); background: var(--destructive-bg); opacity: 1; }
        .msg-thread-avatar {
            width: 34px;
            height: 34px;
            border-radius: 50%;
            background: var(--foreground);
            color: var(--background);
            font-size: 0.75rem;
            font-weight: 700;
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
        }
        .msg-thread-info { flex: 1; min-width: 0; }
        .msg-thread-name {
            font-size: 0.8125rem;
            font-weight: 700;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            display: flex;
            align-items: center;
            gap: 5px;
        }
        .msg-thread-preview {
            font-size: 0.75rem;
            color: var(--muted-foreground);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            margin-top: 2px;
        }
        .msg-thread-time { font-size: 0.65rem; color: var(--muted-foreground); flex-shrink: 0; }
        .msg-unread-badge {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            min-width: 16px;
            height: 16px;
            border-radius: 8px;
            background: var(--destructive);
            color: #fff;
            font-size: 0.6rem;
            font-weight: 700;
            padding: 0 4px;
        }
        .msg-admin-panel {
            display: flex;
            flex-direction: column;
            border: 1px solid var(--border);
            border-radius: var(--radius-lg);
            overflow: hidden;
            background: var(--muted);
        }
        .msg-conversation {
            flex: 1;
            overflow-y: auto;
            padding: 12px;
            display: flex;
            flex-direction: column;
            gap: 6px;
            min-height: 200px;
            max-height: 280px;
        }
        .msg-empty {
            text-align: center;
            color: var(--muted-foreground);
            font-size: 0.875rem;
            padding: 20px 0;
            margin: auto;
        }
        .msg-bubble {
            max-width: 76%;
            padding: 7px 11px;
            border-radius: var(--radius-xl);
            word-break: break-word;
        }
        .msg-bubble.mine {
            align-self: flex-end;
            background: var(--foreground);
            color: var(--background);
            border-bottom-right-radius: var(--radius-sm);
        }
        .msg-bubble.theirs {
            align-self: flex-start;
            background: var(--card);
            border: 1px solid var(--border);
            border-bottom-left-radius: var(--radius-sm);
        }
        .msg-bubble-text { font-size: 0.875rem; line-height: 1.45; white-space: pre-wrap; }
        .msg-bubble-meta { font-size: 0.68rem; opacity: 0.55; margin-top: 3px; text-align: right; }
        .msg-compose {
            border-top: 1px solid var(--border);
            padding: 10px;
            background: var(--card);
            flex-shrink: 0;
        }
        .msg-compose-row { display: flex; gap: 8px; align-items: flex-end; }
        .msg-compose-input {
            flex: 1;
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            padding: 7px 13px;
            font: inherit;
            font-size: 1rem;
            background: var(--muted);
            color: var(--foreground);
            resize: none;
            max-height: 90px;
            outline: none;
            transition: border-color 0.15s;
            line-height: 1.4;
        }
        .msg-compose-input:focus { border-color: var(--foreground); background: var(--card); }
        .msg-compose-send {
            width: 36px;
            height: 36px;
            border-radius: 50%;
            background: var(--foreground);
            color: var(--background);
            border: none;
            cursor: pointer;
            font-size: 0.9rem;
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
            transition: opacity 0.15s;
        }
        .msg-compose-send:disabled { opacity: 0.4; }
        .msg-reply-status { font-size: 0.8rem; margin-top: 6px; min-height: 16px; color: var(--muted-foreground); }

        /* ===== Emoji picker (admin compose) ===== */
        .emoji-pick {
            border: none;
            background: none;
            cursor: pointer;
            font-size: 1.15rem;
            padding: 4px;
            border-radius: var(--radius-sm);
            transition: background 0.1s;
            line-height: 1;
            color: inherit;
        }
        .emoji-pick:hover { background: var(--muted); }
        .msg-emoji-wrap { position: relative; flex-shrink: 0; }
        .msg-emoji-btn {
            width: 34px;
            height: 34px;
            border-radius: 50%;
            border: 1px solid var(--border);
            background: var(--muted);
            cursor: pointer;
            font-size: 0.95rem;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: background 0.1s;
        }
        .msg-emoji-btn:hover { background: var(--border); }
        .msg-emoji-panel {
            position: absolute;
            bottom: 40px;
            left: 0;
            width: 272px;
            max-height: 200px;
            overflow-y: auto;
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: var(--radius-lg);
            padding: 8px;
            display: none;
            grid-template-columns: repeat(6, 1fr);
            gap: 2px;
            box-shadow: var(--shadow-hover);
            z-index: 200;
        }
        .msg-emoji-panel.open { display: grid; }

        /* ===== Donations ===== */
        .donation-item {
            border: 1px solid var(--border);
            border-radius: var(--radius-lg);
            padding: 12px 14px;
            margin-bottom: 8px;
            background: var(--muted);
            font-size: 0.875rem;
        }
        .donation-item:last-child { margin-bottom: 0; }
        .donation-header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            gap: 10px;
            flex-wrap: wrap;
            margin-bottom: 8px;
        }
        .donation-pseudo { font-weight: 700; }
        .donation-proof-link {
            font-size: 0.8125rem;
            color: var(--accent);
            text-decoration: none;
            font-weight: 500;
        }
        .donation-proof-link:hover { text-decoration: underline; }
        .donation-actions { display: flex; gap: 6px; flex-wrap: wrap; margin-top: 8px; }

        /* ===== Signup queue ===== */
        .queue-item {
            border: 1px solid var(--border);
            border-radius: var(--radius-lg);
            padding: 12px 14px;
            margin-bottom: 8px;
            background: var(--muted);
        }
        .queue-item:last-child { margin-bottom: 0; }
        .queue-pseudo { font-weight: 700; font-size: 0.9375rem; }
        .queue-meta   { font-size: 0.8125rem; color: var(--muted-foreground); margin: 4px 0 10px; }
        .queue-actions { display: flex; gap: 6px; flex-wrap: wrap; }
        .queue-msg { font-size: 0.8125rem; margin-top: 6px; min-height: 18px; }

        /* ===== Sweep log ===== */
        .sweep-log {
            margin-top: 12px;
            padding: 12px;
            background: var(--foreground);
            color: var(--background);
            border-radius: var(--radius-lg);
            min-height: 140px;
            white-space: pre-wrap;
            font: 0.8rem/1.5 var(--font-mono);
            overflow-x: auto;
        }

        /* ===== Admin wall ===== */
        .admin-wall-list { display: flex; flex-direction: column; }

        /* ===== Temp password modal ===== */
        .temp-pw-modal {
            display: none;
            position: fixed;
            inset: 0;
            background: rgba(0,0,0,0.5);
            z-index: 600;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }
        .temp-pw-modal.open { display: flex; }
        .temp-pw-panel {
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            padding: 24px;
            width: 100%;
            max-width: 360px;
            box-shadow: var(--shadow-hover);
        }
        .temp-pw-panel h3 { margin: 0 0 14px; font-size: 1rem; font-weight: 700; }
        .temp-pw-code {
            font: 700 1.125rem/1.5 var(--font-mono);
            background: var(--muted);
            border: 1px solid var(--border);
            border-radius: var(--radius-md);
            padding: 10px 14px;
            letter-spacing: 0.02em;
            word-break: break-all;
            cursor: pointer;
            margin-bottom: 12px;
            transition: background 0.1s;
        }
        .temp-pw-code:hover { background: var(--border); }
        .temp-pw-hint { font-size: 0.8125rem; color: var(--muted-foreground); margin-bottom: 14px; }
        .temp-pw-actions { display: flex; justify-content: flex-end; }

        /* ===== Monitor status colors ===== */
        .monitor-ok   { color: var(--success); font-weight: 600; }
        .monitor-down { color: var(--destructive);  font-weight: 600; }

        /* ===== Status badges (OK/KO) ===== */
        .badge-ok, .badge-ko {
            display: inline-flex;
            align-items: center;
            height: 18px;
            padding: 0 6px;
            border-radius: var(--radius-sm);
            font-size: 0.65rem;
            font-weight: 700;
            flex-shrink: 0;
            border: 1px solid transparent;
        }
        .badge-ok { background: var(--success-bg); color: var(--success); border-color: var(--success-border); }
        .badge-ko { background: var(--destructive-bg); color: var(--destructive); border-color: var(--destructive-border); }

        /* ===== Chain checks / endpoint items ===== */
        .chain-grid { display: grid; gap: 2px; margin-top: 8px; }
        .endpoint-item {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 8px;
            padding: 5px 0;
            border-bottom: 1px solid var(--border);
            font-size: 0.8125rem;
        }
        .endpoint-item:last-child { border-bottom: 0; }

        /* ===== Endpoint accordion sections ===== */
        .endpoint-grid { display: grid; gap: 5px; margin-top: 8px; }
        .ep-section {
            border: 1px solid var(--border);
            border-radius: var(--radius-lg);
            overflow: hidden;
            background: var(--muted);
        }
        .ep-section-head {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 8px 12px;
            cursor: pointer;
            font-size: 0.8125rem;
            font-weight: 600;
            user-select: none;
            gap: 8px;
            transition: background 0.15s;
        }
        .ep-section-head:hover { background: var(--card); }
        .ep-section-label { flex: 1; }
        .ep-section-count { font-weight: 400; color: var(--muted-foreground); }
        .ep-chevron { color: var(--muted-foreground); transition: transform 0.2s ease; font-size: 0.9rem; }
        .ep-section.open .ep-chevron { transform: rotate(90deg); }
        .ep-section-items {
            display: none;
            padding: 6px 12px 10px;
            background: var(--card);
            border-top: 1px solid var(--border);
        }
        .ep-section.open .ep-section-items { display: block; }

        /* ===== Test history accordion ===== */
        .tests-history { display: grid; gap: 5px; }
        .test-run {
            border: 1px solid var(--border);
            border-radius: var(--radius-lg);
            overflow: hidden;
            background: var(--muted);
        }
        .test-head {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 9px 12px;
            cursor: pointer;
            font-size: 0.8125rem;
            font-weight: 500;
            user-select: none;
            gap: 8px;
            transition: background 0.15s;
        }
        .test-head:hover { background: var(--card); }
        .test-head.ok   { border-left: 3px solid var(--success); }
        .test-head.fail { border-left: 3px solid var(--destructive); }
        .test-head-chevron {
            color: var(--muted-foreground);
            font-size: 0.75rem;
            flex-shrink: 0;
            transition: transform 0.2s ease;
        }
        .test-run.open .test-head-chevron { transform: rotate(180deg); }
        .test-cases {
            display: none;
            list-style: none;
            margin: 0;
            padding: 8px 14px 10px;
            background: var(--card);
            border-top: 1px solid var(--border);
            gap: 3px;
        }
        .test-run.open .test-cases { display: grid; }
        .case-ok   { font-size: 0.8125rem; color: var(--success); padding: 1px 0; }
        .case-fail { font-size: 0.8125rem; color: var(--destructive); padding: 1px 0; }

        /* ===== Invitations ===== */
        .inv-gen-form {
            display: flex;
            gap: 0.5rem;
            align-items: flex-end;
            flex-wrap: wrap;
            margin-bottom: 1rem;
        }
        .inv-gen-form input {
            flex: 1;
            min-width: 180px;
            padding: 0.4rem 0.65rem;
            border: 1px solid var(--border);
            border-radius: 0.375rem;
            background: var(--input, var(--background));
            color: var(--foreground);
            font-size: 0.85rem;
        }
        .inv-gen-form button {
            padding: 0.4rem 0.85rem;
            background: var(--foreground);
            color: var(--background);
            border: none;
            border-radius: 0.375rem;
            font-size: 0.85rem;
            font-weight: 600;
            cursor: pointer;
        }
        .inv-gen-msg {
            font-size: 0.8125rem;
            padding: 0.4rem 0.6rem;
            border-radius: 0.3rem;
            margin-bottom: 0.75rem;
        }
        .inv-gen-msg.ok { background: var(--success-bg, #dcfce7); color: var(--success, #166534); }
        .inv-gen-msg.err { background: var(--error-bg, #fee2e2); color: var(--error, #991b1b); }
        .invites-empty { font-size: 0.875rem; color: var(--muted-foreground); }

        .inv-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0.75rem 0;
            border-bottom: 1px solid var(--border);
            gap: 1rem;
            flex-wrap: wrap;
        }
        .inv-row:last-child { border-bottom: none; }

        .inv-meta {
            display: flex;
            flex-direction: column;
            gap: 0.25rem;
            min-width: 0;
        }
        .inv-meta-top {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            flex-wrap: wrap;
        }
        .inv-badge {
            font-size: 0.7rem;
            font-weight: 700;
            padding: 0.15rem 0.5rem;
            border-radius: 999px;
            white-space: nowrap;
        }
        .inv-badge.active  { background: #dcfce7; color: #166534; }
        .inv-badge.used    { background: #e0e7ff; color: #3730a3; }
        .inv-badge.expired { background: #f3f4f6; color: #6b7280; }
        .inv-note  { font-size: 0.8125rem; color: var(--foreground); font-weight: 500; }
        .inv-dates { font-size: 0.75rem; color: var(--muted-foreground); }

        .inv-actions { display: flex; gap: 0.4rem; flex-shrink: 0; }
        .inv-copy-btn, .inv-revoke-btn {
            font-size: 0.75rem;
            padding: 0.25rem 0.6rem;
            border-radius: 0.3rem;
            cursor: pointer;
            border: 1px solid var(--border);
            background: var(--card);
            color: var(--foreground);
            white-space: nowrap;
        }
        .inv-revoke-btn { color: var(--error, #991b1b); border-color: var(--error, #991b1b); }

        /* ===== Responsive ===== */
        @media (max-width: 700px) {
            .msg-admin-layout { grid-template-columns: 1fr; }
            .msg-thread-list  { max-height: 180px; }
            .member-gallery   { grid-template-columns: repeat(auto-fill, minmax(100px, 1fr)); }
            .tabs { width: 100%; }
        }
        @media (max-width: 480px) {
            .tab-btn { min-height: 44px; padding: 8px 12px; }
            .member-gallery { grid-template-columns: repeat(auto-fill, minmax(90px, 1fr)); }
        }
"#;
