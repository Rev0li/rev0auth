pub const FRIEND_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body {
            margin: 0;
            background: var(--bg-page);
            min-height: 100vh;
            -webkit-font-smoothing: antialiased;
        }
        .container {
            max-width: 860px;
            margin: 0 auto;
            padding: 28px 20px;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            margin-bottom: 24px;
            gap: 14px;
            flex-wrap: wrap;
        }
        .header h1 {
            margin: 0;
            font-size: 1.375rem;
            font-weight: 700;
            letter-spacing: -0.02em;
        }
        .header-meta {
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
        .header-status {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            align-items: center;
        }
        .header-actions {
            display: flex;
            flex-direction: column;
            align-items: flex-end;
            gap: 8px;
        }
        .header-action-row {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            justify-content: flex-end;
        }
        .logout-btn {
            padding: 7px 13px;
            background: var(--color-danger-bg);
            color: var(--color-danger);
            border: 1px solid var(--color-danger-border);
            border-radius: var(--radius-md);
            font-weight: 600;
            font-size: 0.875rem;
            cursor: pointer;
            text-decoration: none;
            display: inline-flex;
            align-items: center;
            transition: opacity 0.15s;
        }
        .logout-btn:hover { opacity: 0.85; }
        .profile-btn {
            padding: 7px 13px;
            background: var(--color-accent-bg);
            color: var(--color-accent);
            border: 1px solid var(--color-accent-border);
            border-radius: var(--radius-md);
            font-weight: 600;
            font-size: 0.875rem;
            cursor: pointer;
            text-decoration: none;
            display: inline-flex;
            align-items: center;
            transition: opacity 0.15s;
        }
        .profile-btn:hover { opacity: 0.85; }
        .onboarding-intro { margin-top: 0; color: var(--color-muted); font-size: 0.875rem; }
        .onboarding-label { display: block; font-weight: 600; font-size: 0.8125rem; margin-top: 10px; }
        .onboarding-field,
        .onboarding-textarea {
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
        .onboarding-field:focus,
        .onboarding-textarea:focus { border-color: var(--color-accent); }
        .onboarding-textarea { min-height: 110px; resize: vertical; }
        .actions-tight { margin-top: 10px; }
        .mood-label { font-weight: 600; font-size: 0.875rem; color: var(--color-muted); }
        .services-intro { margin-top: 0; color: var(--color-muted); font-size: 0.875rem; }
        .card {
            background: var(--color-panel);
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-lg);
            padding: 20px;
            box-shadow: 0 1px 3px rgba(15, 23, 42, 0.06);
            margin-bottom: 14px;
        }
        h2 { margin: 0 0 12px; font-size: 1rem; font-weight: 600; }
        .greeting { font-size: 0.9375rem; line-height: 1.6; margin-bottom: 16px; color: var(--color-muted); }
        .greeting strong { color: var(--color-ink); }
        .feature-list { list-style: none; padding: 0; margin: 0; }
        .feature-list li {
            padding: 9px 0;
            border-bottom: 1px solid var(--color-panel-border);
            display: flex;
            align-items: center;
            gap: 10px;
            font-size: 0.9rem;
        }
        .feature-list li:last-child { border-bottom: 0; }
        .feature-list li:before {
            content: "→";
            color: var(--color-accent);
            font-weight: 700;
        }
        .status-buttons { display: flex; gap: 8px; margin-top: 14px; flex-wrap: wrap; }
        .status-btn {
            padding: 7px 13px;
            border: 1px solid var(--color-success-border);
            border-radius: var(--radius-md);
            background: var(--color-success-bg);
            color: var(--color-success);
            font-weight: 600;
            cursor: pointer;
            font-size: 0.875rem;
            transition: opacity 0.1s;
        }
        .status-btn:hover { opacity: 0.8; }
        .status-msg {
            margin-top: 10px;
            padding: 8px 10px;
            border-radius: var(--radius-md);
            font-size: 0.875rem;
            display: none;
        }
        .status-msg.ok { display: block; }
        .status-msg.error { display: block; }
        .remark-panel {
            display: none;
            margin-top: 12px;
            padding: 12px;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            background: var(--bg-page);
        }
        .services { display: flex; flex-direction: column; gap: 12px; margin-top: 12px; }
        .service-card {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-lg);
            background: var(--color-panel);
            padding: 14px;
            width: 100%;
            box-sizing: border-box;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
        .service-media {
            width: 100%;
            aspect-ratio: 16 / 9;
            height: auto;
            object-fit: cover;
            border-radius: var(--radius-md);
            border: 1px solid var(--color-panel-border);
            background: var(--bg-page);
        }
        .service-card h3 { margin: 0 0 5px; font-size: 0.9375rem; font-weight: 600; }
        .service-card p { margin: 0 0 10px; font-size: 0.875rem; color: var(--color-muted); }
        .service-btn {
            width: 100%;
            border: 1px solid var(--color-success-border);
            border-radius: var(--radius-md);
            background: var(--color-success-bg);
            color: var(--color-success);
            font-weight: 600;
            padding: 8px 10px;
            cursor: pointer;
            font-size: 0.875rem;
            transition: opacity 0.1s;
        }
        .service-btn:hover { opacity: 0.8; }
        .service-btn.locked {
            background: var(--bg-page);
            color: var(--color-muted);
            border-color: var(--color-panel-border);
        }
        .service-state { font-size: 0.8125rem; margin: 6px 0; color: var(--color-muted); }
        .service-input {
            width: 100%;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 8px 10px;
            box-sizing: border-box;
            font: inherit;
            font-size: 0.875rem;
            margin-bottom: 8px;
            outline: none;
            transition: border-color 0.15s;
        }
        .service-input:focus { border-color: var(--color-accent); }
        .service-msg { margin-top: 8px; padding: 8px 10px; border-radius: var(--radius-md); font-size: 0.875rem; display: none; }
        .service-msg.ok { display: block; }
        .service-msg.error { display: block; }
        %%FRIEND_ONBOARDING_CSS%%
        %%FRIEND_SERVICES_CSS%%
        %%FRIEND_CHAT_CSS%%
        %%FRIEND_STATUS_CSS%%
        %%FRIEND_AVATAR_CSS%%
        @media (max-width: 900px) {
            .container { padding: 16px; }
            .header { flex-direction: column; align-items: stretch; }
            .header-actions { align-items: flex-start; }
            .header-action-row { justify-content: flex-start; }
            .service-card { aspect-ratio: 1 / 1; overflow: hidden; }
            .service-media { aspect-ratio: 1 / 1; }
        }
        .onboarding-modal {
            position: fixed;
            inset: 0;
            background: rgba(15, 23, 42, 0.55);
            display: none;
            align-items: center;
            justify-content: center;
            z-index: 90;
            padding: 16px;
        }
        .onboarding-card {
            width: 100%;
            max-width: 500px;
            background: var(--color-panel);
            border-radius: var(--radius-lg);
            border: 1px solid var(--color-panel-border);
            padding: 20px;
            box-shadow: 0 4px 24px rgba(15, 23, 42, 0.12);
        }
        .onboarding-msg {
            margin-top: 10px;
            border-radius: var(--radius-md);
            padding: 8px 10px;
            font-size: 0.875rem;
            display: none;
        }
        .onboarding-msg.ok { display: block; }
        .onboarding-msg.error { display: block; }
"#;
