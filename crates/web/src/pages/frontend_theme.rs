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
                '--btn-primary-from',
                '--btn-primary-to',
                '--btn-secondary-bg',
                '--btn-secondary-border',
                '--btn-secondary-ink',
                '--bg-a',
                '--bg-b',
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
            --font-sans: "Space Grotesk", "Avenir Next", sans-serif;
            --color-ink: #132331;
            --color-panel: rgba(255, 255, 255, 0.92);
            --color-panel-border: rgba(19, 35, 49, 0.1);
            --color-muted: #4b5f71;
            --color-success: #0d9b73;
            --color-danger: #ef4e24;
            --color-success-bg: #e8fff5;
            --color-danger-bg: #fff0ec;
            --color-success-border: #b3ecd1;
            --color-danger-border: #f3c2b4;
            --btn-primary-from: #ff6b3b;
            --btn-primary-to: #ef4e24;
            --btn-secondary-bg: #f2f9ff;
            --btn-secondary-border: rgba(19, 35, 49, 0.15);
            --btn-secondary-ink: #132331;
            --bg-a: #eef8ff;
            --bg-b: #e6f7ee;
            --radius-md: 10px;
            --radius-lg: 18px;
        }
        * {
            box-sizing: border-box;
        }
        body {
            font-family: var(--font-sans);
            color: var(--color-ink);
        }
        .card {
            background: var(--color-panel);
            border: 1px solid var(--color-panel-border);
            border-radius: var(--radius-lg);
        }
        .meta,
        .hint,
        .mini {
            color: var(--color-muted);
        }
        .primary,
        .btn-primary {
            color: #fff;
            background: linear-gradient(120deg, var(--btn-primary-from), var(--btn-primary-to));
        }
        .secondary,
        .btn-small,
        .tab-btn {
            background: var(--btn-secondary-bg);
            border-color: var(--btn-secondary-border);
            color: var(--btn-secondary-ink);
            border-radius: var(--radius-md);
        }
        .ok {
            background: var(--color-success-bg);
            border-color: var(--color-success-border);
            color: var(--color-success);
        }
        .down,
        .error {
            background: var(--color-danger-bg);
            border-color: var(--color-danger-border);
            color: var(--color-danger);
        }
"#;
