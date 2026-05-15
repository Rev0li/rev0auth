pub const PROFILE_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body { margin: 0; min-height: 100vh; }

        /* ===== Navbar ===== */
        .profile-navbar {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 12px;
            padding: 10px 20px;
            border-bottom: 1px solid var(--border);
            background: var(--card);
            position: sticky;
            top: 0;
            z-index: 10;
        }
        .profile-navbar-left  { display: flex; align-items: center; gap: 10px; }
        .profile-navbar-right { display: flex; align-items: center; gap: 8px; }
        .profile-nav-avatar {
            width: 32px;
            height: 32px;
            border-radius: 50%;
            object-fit: cover;
            border: 1px solid var(--border);
            background: var(--muted);
            display: block;
        }
        .profile-nav-pseudo {
            font-size: 0.875rem;
            font-weight: 600;
        }
        .btn-back {
            display: inline-flex;
            align-items: center;
            gap: 4px;
            font-size: 0.8125rem;
            font-weight: 500;
            color: var(--muted-foreground);
            text-decoration: none;
            padding: 5px 10px;
            border-radius: var(--radius-md);
            border: 1px solid var(--border);
            background: var(--card);
            transition: background 0.1s, color 0.1s;
        }
        .btn-back:hover { background: var(--muted); color: var(--foreground); }
        .admin-note {
            padding: 4px 10px;
            border-radius: var(--radius-md);
            border: 1px solid #fde68a;
            background: #fffbeb;
            color: #92400e;
            font-size: 0.75rem;
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

        /* ===== Page layout ===== */
        .page {
            max-width: 600px;
            margin: 0 auto;
            padding: 20px 20px 80px;
        }
        .card {
            padding: 18px 20px;
            margin-bottom: 10px;
        }
        .card-danger { border-color: rgba(220,38,38,0.25); }

        h2 {
            margin: 0 0 14px;
            font-size: 0.9375rem;
            font-weight: 700;
        }

        /* ===== Info grid ===== */
        .info-grid  { display: grid; gap: 8px; }
        .info-item  { display: flex; align-items: center; gap: 8px; font-size: 0.875rem; }
        .info-label {
            min-width: 120px;
            color: var(--muted-foreground);
            font-size: 0.8125rem;
            font-weight: 600;
        }
        .info-val { color: var(--foreground); font-weight: 500; }

        /* ===== Avatar ===== */
        .avatar-section {
            display: flex;
            align-items: flex-start;
            gap: 16px;
            margin-bottom: 14px;
        }
        .avatar-current { flex-shrink: 0; }
        .avatar-preview {
            width: 68px;
            height: 68px;
            border-radius: 50%;
            border: 1px solid var(--border);
            background: var(--muted);
            object-fit: cover;
            display: block;
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
            color: var(--muted-foreground);
            margin: 10px 0 8px;
        }
        .default-avatar-grid { display: flex; gap: 10px; flex-wrap: wrap; }
        .default-avatar-btn {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 4px;
            border: 2px solid transparent;
            border-radius: var(--radius-lg);
            background: none;
            cursor: pointer;
            padding: 4px;
            transition: border-color 0.15s;
        }
        .default-avatar-btn:hover { border-color: var(--border); }
        .default-avatar-btn.selected { border-color: var(--foreground); }
        .default-avatar-btn img {
            width: 48px;
            height: 48px;
            border-radius: 50%;
            display: block;
        }
        .default-avatar-btn span { font-size: 0.7rem; color: var(--muted-foreground); }

        /* ===== Form fields ===== */
        .field-label {
            display: block;
            font-weight: 600;
            font-size: 0.8rem;
            color: var(--muted-foreground);
            margin: 12px 0 4px;
        }
        .field-input {
            width: 100%;
            border: 1px solid var(--border);
            border-radius: var(--radius-md);
            padding: 8px 10px;
            font: inherit;
            font-size: 0.9rem;
            background: var(--muted);
            color: var(--foreground);
            outline: none;
            transition: border-color 0.15s, box-shadow 0.15s;
        }
        .field-input:focus {
            border-color: var(--foreground);
            box-shadow: 0 0 0 3px rgba(10,10,10,0.1);
            background: var(--card);
        }
        select.field-input { cursor: pointer; }
        input[type="file"] { font-size: 0.875rem; margin: 6px 0; }

        /* ===== Buttons ===== */
        .actions { display: flex; gap: 8px; flex-wrap: wrap; margin-top: 14px; }
        .btn-profile-action {
            display: inline-flex;
            align-items: center;
            height: 34px;
            padding: 0 14px;
            border: 1px solid transparent;
            border-radius: var(--radius-md);
            font: 600 0.875rem/1 var(--font-sans);
            cursor: pointer;
            background: var(--primary);
            color: var(--primary-foreground);
            transition: opacity 0.15s;
        }
        .btn-profile-action:hover { opacity: 0.85; }
        .btn-profile-action.secondary {
            background: var(--card);
            border-color: var(--border);
            color: var(--foreground);
        }
        .btn-profile-action.secondary:hover { background: var(--muted); opacity: 1; }
        .btn-profile-action.danger {
            background: var(--destructive-bg);
            border-color: var(--destructive-border);
            color: var(--destructive);
        }

        /* ===== Status / feedback ===== */
        .msg {
            margin-top: 10px;
            font-size: 0.875rem;
            border-radius: var(--radius-md);
            padding: 8px 10px;
            display: none;
        }
        .ok  { display: block; }
        .down { display: block; }
        .meta { font-size: 0.875rem; color: var(--muted-foreground); }
        .admin-only { display: none; }

        /* ===== List box ===== */
        .list-box {
            margin-top: 12px;
            border: 1px solid var(--border);
            border-radius: var(--radius-lg);
            padding: 10px;
            background: var(--muted);
            font-size: 0.875rem;
        }
        .list-item {
            border: 1px solid var(--border);
            border-radius: var(--radius-md);
            padding: 8px 10px;
            margin-bottom: 6px;
            background: var(--card);
            font-size: 0.875rem;
        }
        .list-item:last-child { margin-bottom: 0; }
        .list-meta-gap { margin-top: 8px; }

        /* ===== Crypto addresses ===== */
        .crypto-addr-row {
            display: flex;
            align-items: baseline;
            gap: 8px;
            padding: 6px 0;
            border-bottom: 1px solid var(--border);
        }
        .crypto-addr-row:last-child { border-bottom: 0; }
        .crypto-addr-name {
            font-size: 0.8125rem;
            font-weight: 600;
            color: var(--muted-foreground);
            min-width: 60px;
        }
        .crypto-addr-val {
            font-size: 0.8rem;
            word-break: break-all;
            cursor: pointer;
            padding: 2px 6px;
            border-radius: var(--radius-sm);
            background: var(--muted);
            border: 1px solid var(--border);
            font-family: var(--font-mono);
            transition: background 0.1s;
        }
        .crypto-addr-val:hover { background: var(--border); }
        .crypto-addr-val.copied {
            background: var(--success-bg);
            border-color: var(--success-border);
            color: var(--success);
        }

        /* ===== Donation hint ===== */
        .donation-hint {
            display: flex;
            gap: 8px;
            align-items: flex-start;
            background: var(--muted);
            border: 1px solid var(--border);
            border-radius: var(--radius-md);
            padding: 9px 12px;
            font-size: 0.8125rem;
            line-height: 1.5;
            margin-top: 8px;
        }
        .donation-hint-icon {
            font-weight: 700;
            font-size: 0.875rem;
            color: var(--muted-foreground);
            flex-shrink: 0;
            margin-top: 1px;
        }

        /* ===== Services admin ===== */
        .services-admin-grid { display: flex; flex-direction: column; }
        .svc-admin-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 12px;
            padding: 10px 0;
            border-bottom: 1px solid var(--border);
        }
        .svc-admin-row:last-child { border-bottom: 0; }
        .svc-admin-label { font-weight: 600; font-size: 0.875rem; }
        .svc-req-badge {
            font-size: 0.75rem;
            font-weight: 500;
            color: var(--muted-foreground);
            margin-left: 6px;
        }
        .svc-admin-meta { font-size: 0.8rem; color: var(--muted-foreground); margin-top: 2px; }

        /* ===== Donation chips ===== */
        .don-list { display: flex; flex-direction: column; gap: 6px; margin-top: 10px; }
        .don-chip {
            display: flex;
            align-items: center;
            gap: 6px;
            flex-wrap: wrap;
            font-size: 0.8125rem;
            padding: 7px 10px;
            border-radius: var(--radius-md);
            border: 1px solid var(--border);
            background: var(--muted);
        }
        .don-chip.approved {
            border-color: var(--success-border);
            background: var(--success-bg);
        }
        .don-chip.refused {
            border-color: var(--destructive-border);
            background: var(--destructive-bg);
        }
        .don-ref     { font-weight: 700; color: var(--muted-foreground); }
        .don-sep     { color: var(--border); }
        .don-ok      { color: var(--success); font-weight: 600; }
        .don-ko      { color: var(--destructive);  font-weight: 600; }
        .don-pending { color: var(--muted-foreground); }

        /* ===== Secondary helper ===== */
        .secondary {
            background: var(--card);
            border: 1px solid var(--border);
            color: var(--foreground);
        }
"#;
