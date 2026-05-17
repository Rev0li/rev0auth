pub const FRIEND_PAGE_STYLES: &str = r#"
        %%FRONTEND_SHARED_CSS%%
        body { margin: 0; min-height: 100vh; }

        /* ===== Navbar ===== */
        .navbar {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 12px 24px;
            background: var(--card);
            border-bottom: 1px solid var(--border);
            gap: 12px;
            position: sticky;
            top: 0;
            z-index: 50;
        }
        .navbar-brand {
            font-weight: 800;
            font-size: 0.9375rem;
            letter-spacing: -0.03em;
            color: var(--foreground);
            text-decoration: none;
        }
        .navbar-user { display: flex; align-items: center; gap: 10px; }
        .navbar-pseudo { font-size: 0.875rem; font-weight: 600; }
        .nav-avatar-wrap { position: relative; flex-shrink: 0; }
        .nav-avatar {
            width: 30px;
            height: 30px;
            border-radius: 50%;
            object-fit: cover;
            border: 1px solid var(--border);
            background: var(--muted);
            display: block;
            cursor: pointer;
        }
        .nav-avatar-fallback {
            display: none;
            width: 30px;
            height: 30px;
            border-radius: 50%;
            background: var(--foreground);
            color: var(--background);
            font-size: 0.75rem;
            font-weight: 700;
            align-items: center;
            justify-content: center;
            cursor: pointer;
        }
        .nav-btn {
            padding: 5px 12px;
            border-radius: var(--radius-md);
            font-size: 0.8125rem;
            font-weight: 500;
            cursor: pointer;
            text-decoration: none;
            transition: background 0.15s, color 0.15s;
            border: 1px solid var(--border);
            background: var(--card);
            color: var(--muted-foreground);
        }
        .nav-btn:hover { background: var(--muted); color: var(--foreground); }
        .nav-btn-logout { font-family: var(--font-sans); }

        /* ===== Hero ===== */
        .hero {
            padding: 52px 24px 40px;
            text-align: center;
        }
        .hero-title {
            margin: 0 0 10px;
            font-size: 2rem;
            font-weight: 800;
            letter-spacing: -0.04em;
        }
        .hero-sub {
            margin: 0 auto;
            max-width: 540px;
            font-size: 0.9375rem;
            color: var(--muted-foreground);
            line-height: 1.7;
        }

        /* ===== GitHub steps ===== */
        .hero-steps {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 10px;
            margin-top: 28px;
            flex-wrap: wrap;
        }
        .hero-step {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 4px;
            padding: 14px 20px;
            border-radius: var(--radius-xl);
            border: 1px solid var(--border);
            background: var(--card);
            text-decoration: none;
            color: var(--foreground);
            min-width: 110px;
            box-shadow: var(--shadow-soft);
            cursor: pointer;
            user-select: none;
            transition: border-color 0.15s, box-shadow 0.15s, transform 0.12s;
        }
        .hero-step:hover {
            border-color: var(--foreground);
            box-shadow: var(--shadow-soft);
            transform: translateY(-2px);
        }
        .hero-step-icon  { font-size: 1.5rem; line-height: 1; }
        .hero-step-label { font-size: 0.875rem; font-weight: 700; }
        .hero-step-hint  { font-size: 0.75rem; color: var(--muted-foreground); }
        .hero-step-arrow {
            font-size: 1.1rem;
            color: var(--muted-foreground);
            flex-shrink: 0;
            margin-top: -20px;
        }
        @media (max-width: 580px) {
            .hero-step-arrow { transform: rotate(90deg); margin-top: 0; }
        }

        /* ===== Page content ===== */
        .page-content {
            max-width: 860px;
            margin: 0 auto;
            padding: 0 20px 100px;
        }
        .section { margin-bottom: 28px; }
        .section-heading {
            margin: 0 0 4px;
            font-size: 0.9375rem;
            font-weight: 700;
            letter-spacing: -0.01em;
        }
        .section-sub {
            margin: 0 0 12px;
            font-size: 0.875rem;
            color: var(--muted-foreground);
        }

        /* ===== Support card ===== */
        .support-card {
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            background: var(--card);
            padding: 20px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 16px;
            flex-wrap: wrap;
            box-shadow: var(--shadow-soft);
        }
        .support-text { font-size: 0.875rem; color: var(--muted-foreground); line-height: 1.6; }
        .support-text p { margin: 0 0 4px; }
        .support-text p:last-child { margin: 0; }

        /* ===== Socials row ===== */
        .socials-row { display: flex; gap: 8px; flex-wrap: wrap; }
        .social-card {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 10px 16px;
            border-radius: var(--radius-xl);
            border: 1px solid var(--border);
            background: var(--card);
            text-decoration: none;
            color: var(--foreground);
            font-size: 0.875rem;
            font-weight: 600;
            box-shadow: var(--shadow-soft);
            transition: border-color 0.15s, box-shadow 0.15s;
        }
        .social-card:hover { border-color: var(--foreground); box-shadow: var(--shadow-soft); }
        .social-icon { font-size: 1.1rem; }

        /* ===== Member gallery ===== */
        .member-gallery {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
            gap: 8px;
        }
        .member-card {
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            background: var(--card);
            padding: 14px 10px;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 6px;
            text-align: center;
            cursor: pointer;
            box-shadow: var(--shadow-soft);
            transition: border-color 0.15s, box-shadow 0.15s;
        }
        .member-card:hover { border-color: var(--foreground); box-shadow: var(--shadow-soft); }
        .member-card-avatar-wrap { position: relative; }
        .member-card-avatar {
            width: 52px;
            height: 52px;
            border-radius: 50%;
            object-fit: cover;
            border: 1px solid var(--border);
            display: block;
        }
        .member-card-avatar-fallback {
            width: 52px;
            height: 52px;
            border-radius: 50%;
            background: var(--foreground);
            color: var(--background);
            font-size: 1.125rem;
            font-weight: 700;
            display: none;
            align-items: center;
            justify-content: center;
        }
        .member-card-pseudo { font-size: 0.8125rem; font-weight: 700; }
        .member-card-meta { display: flex; gap: 4px; align-items: center; flex-wrap: wrap; justify-content: center; }

        /* ===== Status / role chips ===== */
        .member-status,
        .member-badge {
            display: inline-flex;
            align-items: center;
            height: 18px;
            padding: 0 6px;
            border-radius: var(--radius-sm);
            font-size: 0.6875rem;
            font-weight: 600;
            border: 1px solid transparent;
        }
        .member-status.active   { background: var(--success-bg); color: var(--success); border-color: var(--success-border); }
        .member-status.pending  { background: #fff7ed; color: #c2410c; border-color: #fed7aa; }
        .member-status.inactive { background: var(--muted); color: var(--muted-foreground); border-color: var(--border); }
        .member-badge.admin     { background: var(--foreground); color: var(--background); }
        .member-badge.mod       { background: #581c87; color: #e9d5ff; }
        .member-badge.member,
        .member-badge.guest     { background: var(--muted); color: var(--muted-foreground); border-color: var(--border); }

        /* ===== Hero step preview popup ===== */
        .step-popup-overlay {
            display: none;
            position: fixed;
            inset: 0;
            background: rgba(0,0,0,0.55);
            z-index: 200;
            align-items: center;
            justify-content: center;
            padding: 20px;
            backdrop-filter: blur(3px);
        }
        .step-popup-overlay.open { display: flex; }
        .step-popup-panel {
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            width: 94vw;
            max-width: 1100px;
            height: 90vh;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            box-shadow: var(--shadow-hover);
            position: relative;
        }
        .step-popup-close {
            position: absolute;
            top: 10px;
            right: 10px;
            background: var(--muted);
            border: 1px solid var(--border);
            border-radius: var(--radius-md);
            width: 28px;
            height: 28px;
            cursor: pointer;
            font-size: 0.875rem;
            color: var(--muted-foreground);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 1;
            transition: background 0.1s, color 0.1s;
        }
        .step-popup-close:hover { background: var(--border); color: var(--foreground); }
        .step-popup-img-wrap {
            flex: 1;
            min-height: 0;
            background: var(--muted);
            overflow: hidden;
        }
        .step-popup-img {
            width: 100%;
            height: 100%;
            object-fit: contain;
            display: none;
        }
        .step-popup-img-placeholder {
            width: 100%;
            height: 100%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 5rem;
            background: linear-gradient(135deg, var(--muted) 0%, var(--card) 100%);
        }
        .step-popup-body {
            padding: 14px 18px 16px;
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 12px;
            border-top: 1px solid var(--border);
            flex-shrink: 0;
        }
        .step-popup-icon { font-size: 1.5rem; line-height: 1; flex-shrink: 0; }
        .step-popup-title { font-size: 1rem; font-weight: 700; }
        .step-popup-desc { font-size: 0.875rem; color: var(--muted-foreground); margin: 0; line-height: 1.4; flex: 1; }
        .step-popup-btn { flex-shrink: 0; white-space: nowrap; }

        /* ===== Responsive ===== */
        @media (max-width: 600px) {
            .hero { padding: 32px 16px 24px; }
            .hero-title { font-size: 1.5rem; }
            .member-gallery { grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); }
        }
        @media (max-width: 480px) {
            .navbar { padding: 8px 12px; }
            .navbar-pseudo { display: none; }
            .nav-btn { padding: 8px 10px; font-size: 0.75rem; min-height: 44px; }
            .hero { padding: 24px 12px 16px; }
            .hero-title { font-size: 1.25rem; }
            .page-content { padding: 0 12px 60px; }
            .hero-step { min-width: auto; }
        }
"#;
