pub const PROFILE_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body {
            margin: 0;
            background: var(--bg-page);
            min-height: 100vh;
            -webkit-font-smoothing: antialiased;
        }
        .profile-topbar {
            display: flex;
            align-items: center;
            gap: 14px;
            padding: 12px 20px;
            border-bottom: 1px solid var(--color-panel-border);
            background: var(--bg-card, #fff);
            position: sticky;
            top: 0;
            z-index: 10;
        }
        .btn-back {
            font-size: 0.875rem;
            font-weight: 600;
            color: var(--color-muted);
            text-decoration: none;
            padding: 5px 10px;
            border-radius: var(--radius-md);
            border: 1px solid var(--color-panel-border);
            background: var(--color-panel);
            transition: background 0.1s;
        }
        .btn-back:hover { background: var(--bg-page); color: var(--color-ink); }
        .admin-note {
            padding: 5px 10px;
            border-radius: var(--radius-md);
            border: 1px solid #fde68a;
            background: #fffbeb;
            color: #92400e;
            font-size: 0.8rem;
            font-weight: 600;
        }
        .admin-nav {
            max-width: 600px;
            margin: 0 auto 10px;
            padding: 0 20px;
            display: none;
            gap: 8px;
            flex-wrap: wrap;
            align-items: center;
        }
        .page {
            max-width: 600px;
            margin: 0 auto;
            padding: 20px 20px 80px;
        }
        .card {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-lg);
            background: var(--color-panel);
            padding: 18px 20px;
            margin-bottom: 12px;
            box-shadow: 0 1px 3px rgba(15,23,42,0.04);
        }
        .card-danger { border-color: rgba(220,38,38,0.25); }
        h2 {
            margin: 0 0 14px;
            font-size: 0.9375rem;
            font-weight: 700;
            color: var(--color-ink);
        }
        /* Info grid */
        .info-grid { display: grid; gap: 8px; }
        .info-item {
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 0.875rem;
        }
        .info-label {
            min-width: 120px;
            color: var(--color-muted);
            font-size: 0.8125rem;
            font-weight: 600;
        }
        .info-val { color: var(--color-ink); font-weight: 500; }
        /* Avatar section */
        .avatar-section {
            display: flex;
            align-items: flex-start;
            gap: 16px;
            margin-bottom: 14px;
        }
        .avatar-current { flex-shrink: 0; }
        .avatar-preview {
            width: 72px;
            height: 72px;
            display: block;
            border-radius: 50%;
            border: 2px solid var(--color-panel-border);
            background: var(--bg-page);
            object-fit: cover;
        }
        .avatar-preview[src=""] { display: none; }
        .avatar-actions-col {
            display: flex;
            flex-direction: column;
            gap: 6px;
            padding-top: 4px;
        }
        .default-avatar-label {
            font-size: 0.8125rem;
            font-weight: 600;
            color: var(--color-muted);
            margin: 10px 0 8px;
        }
        .default-avatar-grid {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
        }
        .default-avatar-btn {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 4px;
            border: 2px solid transparent;
            border-radius: 12px;
            background: none;
            cursor: pointer;
            padding: 4px;
            transition: border-color 0.15s;
        }
        .default-avatar-btn:hover { border-color: var(--color-accent-border); }
        .default-avatar-btn.selected { border-color: var(--color-accent); }
        .default-avatar-btn img {
            width: 52px;
            height: 52px;
            border-radius: 50%;
            display: block;
        }
        .default-avatar-btn span { font-size: 0.7rem; color: var(--color-muted); }
        /* Fields */
        .field-label {
            display: block;
            font-weight: 600;
            font-size: 0.8rem;
            color: var(--color-muted);
            margin: 12px 0 4px;
        }
        .field-input {
            width: 100%;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 8px 10px;
            box-sizing: border-box;
            font: inherit;
            font-size: 0.9rem;
            background: var(--bg-page);
            color: var(--color-ink);
            outline: none;
            transition: border-color 0.15s;
        }
        .field-input:focus { border-color: var(--color-accent); }
        select.field-input { cursor: pointer; }
        input[type="file"] { font-size: 0.875rem; margin: 6px 0; }
        /* Buttons */
        .actions { display: flex; gap: 8px; flex-wrap: wrap; margin-top: 14px; }
        .btn-profile-action {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 7px 14px;
            font: inherit;
            font-size: 0.875rem;
            font-weight: 600;
            cursor: pointer;
            background: var(--color-accent);
            color: #fff;
            border-color: transparent;
            transition: opacity 0.1s;
        }
        .btn-profile-action:hover { opacity: 0.85; }
        .btn-profile-action.secondary {
            background: var(--color-panel);
            border-color: var(--color-panel-border);
            color: var(--color-ink);
        }
        .btn-profile-action.danger {
            background: var(--color-danger-bg, #fee2e2);
            border-color: var(--color-danger-border, #fca5a5);
            color: var(--color-danger, #b91c1c);
        }
        /* Misc */
        .msg {
            margin-top: 10px;
            font-size: 0.875rem;
            border-radius: var(--radius-md);
            padding: 8px 10px;
            display: none;
        }
        .ok { display: block; background: var(--color-success-bg); color: var(--color-success); border: 1px solid var(--color-success-border); }
        .down { display: block; background: var(--color-danger-bg); color: var(--color-danger); border: 1px solid var(--color-danger-border); }
        .meta { font-size: 0.875rem; color: var(--color-muted); }
        .admin-only { display: none !important; }
        .list-box {
            margin-top: 12px;
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-md);
            padding: 10px;
            background: var(--bg-page);
            font-size: 0.875rem;
        }
        .list-item {
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-sm);
            padding: 8px 10px;
            margin-bottom: 6px;
            background: var(--color-panel);
            font-size: 0.875rem;
        }
        .list-item:last-child { margin-bottom: 0; }
        .list-meta-gap { margin-top: 8px; }
        .donation-proof-link { margin-top: 6px; font-size: 0.8125rem; color: var(--color-muted); }
        .secondary { background: var(--color-panel); border: 1px solid var(--color-panel-border); color: var(--color-ink); }
"#;
