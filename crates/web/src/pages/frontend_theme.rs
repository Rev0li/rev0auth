pub const FRONTEND_THEME_BOOT: &str = r#"
    <script>
        (function () {
            const THEME_KEYS = [
                '--font-sans',
                '--color-ink',
                '--color-panel',
                '--color-panel-border',
                '--color-muted',
                '--color-success',
                '--color-danger',
                '--color-success-bg',
                '--color-danger-bg',
                '--color-success-border',
                '--color-danger-border',
                '--color-accent',
                '--color-accent-dark',
                '--color-accent-bg',
                '--color-accent-border',
                '--btn-primary-bg',
                '--btn-primary-hover',
                '--btn-secondary-bg',
                '--btn-secondary-border',
                '--btn-secondary-ink',
                '--bg-page',
                '--radius-sm',
                '--radius-md',
                '--radius-lg',
            ];

            function clearTheme(root) {
                THEME_KEYS.forEach((key) => {
                    root.style.removeProperty(key);
                });
            }

            function applyTheme(theme) {
                const root = document.documentElement;
                clearTheme(root);
                if (!theme || typeof theme !== 'object') return;
                Object.entries(theme).forEach(([key, value]) => {
                    if (typeof key === 'string' && key.startsWith('--') && typeof value === 'string' && value.trim()) {
                        root.style.setProperty(key, value);
                    }
                });
            }

            function readTheme() {
                const raw = localStorage.getItem('rev0auth_theme');
                if (!raw) return null;
                const parsed = JSON.parse(raw);
                return parsed && typeof parsed === 'object' ? parsed : null;
            }

            try {
                applyTheme(readTheme());
                window.addEventListener('storage', (event) => {
                    if (event.key !== 'rev0auth_theme') return;
                    try {
                        if (!event.newValue) {
                            applyTheme(null);
                            return;
                        }
                        const nextTheme = JSON.parse(event.newValue);
                        applyTheme(nextTheme);
                    } catch (_err) {
                        applyTheme(null);
                    }
                });
                window.addEventListener('rev0auth:theme-update', () => {
                    try {
                        applyTheme(readTheme());
                    } catch (_err) {
                        applyTheme(null);
                    }
                });
            } catch (_err) {
                // Ignore malformed local theme payloads.
            }
        })();
    </script>
"#;

pub const FRONTEND_SHARED_CSS: &str = r#"
        :root {
            --font-sans: "Inter", "system-ui", "-apple-system", sans-serif;
            --color-ink: #0f172a;
            --color-panel: #ffffff;
            --color-panel-border: #e2e8f0;
            --color-muted: #64748b;
            --color-success: #059669;
            --color-danger: #e11d48;
            --color-success-bg: #ecfdf5;
            --color-danger-bg: #fff1f2;
            --color-success-border: #a7f3d0;
            --color-danger-border: #fecdd3;
            --color-accent: #6366f1;
            --color-accent-dark: #4f46e5;
            --color-accent-bg: #eef2ff;
            --color-accent-border: #c7d2fe;
            --btn-primary-bg: #6366f1;
            --btn-primary-hover: #4f46e5;
            --btn-secondary-bg: #ffffff;
            --btn-secondary-border: #e2e8f0;
            --btn-secondary-ink: #0f172a;
            --bg-page: #f8fafc;
            --radius-sm: 6px;
            --radius-md: 8px;
            --radius-lg: 12px;
        }
        * {
            box-sizing: border-box;
        }
        body {
            font-family: var(--font-sans);
            color: var(--color-ink);
            -webkit-font-smoothing: antialiased;
        }
        .card {
            background: var(--color-panel);
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-lg);
            box-shadow: 0 1px 3px rgba(15, 23, 42, 0.06);
        }
        .meta,
        .hint,
        .mini {
            color: var(--color-muted);
            font-size: 0.875rem;
        }
        .primary,
        .btn-primary {
            color: #fff;
            background: var(--btn-primary-bg);
            border: 1px solid transparent;
        }
        .primary:hover,
        .btn-primary:hover {
            background: var(--btn-primary-hover);
        }
        .secondary,
        .btn-small,
        .tab-btn {
            background: var(--btn-secondary-bg);
            border: 1px solid var(--btn-secondary-border);
            color: var(--btn-secondary-ink);
            border-radius: var(--radius-md);
        }
        .ok {
            background: var(--color-success-bg);
            border: 1px solid var(--color-success-border);
            color: var(--color-success);
        }
        .down,
        .error {
            background: var(--color-danger-bg);
            border: 1px solid var(--color-danger-border);
            color: var(--color-danger);
        }
"#;
