pub const FRONTEND_THEME_BOOT: &str = r#"
    <script>
        (function () {
            /* ── Custom theme (admin theme editor) ── */
            const THEME_KEYS = [
                '--font-sans','--font-mono',
                '--background','--foreground',
                '--card','--card-foreground',
                '--muted','--muted-foreground',
                '--primary','--primary-foreground','--primary-hover',
                '--secondary','--secondary-foreground',
                '--accent','--accent-foreground',
                '--border','--ring',
                '--success','--success-bg','--success-border',
                '--destructive','--destructive-bg','--destructive-border',
                '--warning',
                '--radius-sm','--radius-md','--radius-lg','--radius-xl','--radius-full',
                '--shadow-soft','--shadow-hover',
            ];
            function clearTheme(root) { THEME_KEYS.forEach(function(k){ root.style.removeProperty(k); }); }
            function applyTheme(theme) {
                var root = document.documentElement;
                clearTheme(root);
                if (!theme || typeof theme !== 'object') return;
                Object.entries(theme).forEach(function(kv) {
                    var k = kv[0], v = kv[1];
                    if (typeof k === 'string' && k.startsWith('--') && typeof v === 'string' && v.trim())
                        root.style.setProperty(k, v);
                });
            }
            function readTheme() {
                try {
                    var raw = localStorage.getItem('rev0auth_theme');
                    if (!raw) return null;
                    var p = JSON.parse(raw);
                    return p && typeof p === 'object' ? p : null;
                } catch(_){ return null; }
            }
            try {
                applyTheme(readTheme());
                window.addEventListener('storage', function(e){
                    if (e.key !== 'rev0auth_theme') return;
                    try { applyTheme(e.newValue ? JSON.parse(e.newValue) : null); } catch(_){ applyTheme(null); }
                });
                window.addEventListener('rev0auth:theme-update', function(){
                    try { applyTheme(readTheme()); } catch(_){ applyTheme(null); }
                });
            } catch(_) {}

            /* ── Dark / light mode ── */
            var saved = localStorage.getItem('rev0auth_color_scheme');
            var prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
            var isDark = saved === 'dark' || (saved === null && prefersDark);
            if (isDark)  document.documentElement.classList.add('dark');
            if (!isDark) document.documentElement.classList.add('light');

            /* ── Toggle button (injected once DOM is ready) ── */
            function injectToggleBtn() {
                if (document.getElementById('rev0-theme-toggle')) return;
                var btn = document.createElement('button');
                btn.id = 'rev0-theme-toggle';
                btn.className = 'rev0-theme-toggle';
                btn.title = 'Basculer le thème';
                btn.setAttribute('aria-label', 'Basculer clair / sombre');
                function syncIcon() {
                    btn.textContent = document.documentElement.classList.contains('dark') ? '☀' : '☾';
                }
                syncIcon();
                btn.addEventListener('click', function() {
                    var nowDark = document.documentElement.classList.contains('dark');
                    document.documentElement.classList.toggle('dark', !nowDark);
                    document.documentElement.classList.toggle('light', nowDark);
                    localStorage.setItem('rev0auth_color_scheme', nowDark ? 'light' : 'dark');
                    syncIcon();
                });
                document.body.appendChild(btn);
            }
            if (document.readyState === 'loading') {
                document.addEventListener('DOMContentLoaded', injectToggleBtn);
            } else {
                injectToggleBtn();
            }
        })();
    </script>
"#;

pub const FRONTEND_SHARED_CSS: &str = r#"
        /* ===== Design tokens — light ===== */
        :root {
            --font-sans: 'Geist', ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
            --font-mono: 'Geist Mono', ui-monospace, 'SF Mono', Consolas, 'Liberation Mono', monospace;

            --background:           #F7F4EF;
            --foreground:           #111111;
            --card:                 #FFFFFF;
            --card-foreground:      #111111;
            --muted:                #FCFBF8;
            --muted-foreground:     #6B6B6B;
            --primary:              #E8B7C4;
            --primary-foreground:   #111111;
            --primary-hover:        #DCA2B5;
            --secondary:            #FCFBF8;
            --secondary-foreground: #111111;
            --accent:               #6EDAD3;
            --accent-foreground:    #111111;
            --border:               rgba(17,17,17,0.08);
            --ring:                 rgba(232,183,196,0.35);

            --success:              #65B48A;
            --success-bg:           rgba(101,180,138,0.12);
            --success-border:       rgba(101,180,138,0.35);
            --destructive:          #D96B6B;
            --destructive-bg:       rgba(217,107,107,0.12);
            --destructive-border:   rgba(217,107,107,0.35);
            --warning:              #D9A84C;

            --radius-sm:   0.5rem;
            --radius-md:   0.875rem;
            --radius-lg:   1.125rem;
            --radius-xl:   1.75rem;
            --radius-full: 9999px;

            --shadow-soft:  0 4px 24px rgba(0,0,0,0.06);
            --shadow-hover: 0 8px 30px rgba(0,0,0,0.10);
        }

        /* ===== Design tokens — dark (media) ===== */
        @media (prefers-color-scheme: dark) {
            :root:not(.light) {
                --background:           #111111;
                --foreground:           #F7F4EF;
                --card:                 #1A1A1A;
                --card-foreground:      #F7F4EF;
                --muted:                #222222;
                --muted-foreground:     #8A8A8A;
                --secondary:            #222222;
                --secondary-foreground: #F7F4EF;
                --border:               rgba(255,255,255,0.08);
                --shadow-soft:          0 4px 24px rgba(0,0,0,0.30);
                --shadow-hover:         0 8px 30px rgba(0,0,0,0.45);
            }
        }

        /* ===== Design tokens — dark (class) ===== */
        html.dark {
            --background:           #111111;
            --foreground:           #F7F4EF;
            --card:                 #1A1A1A;
            --card-foreground:      #F7F4EF;
            --muted:                #222222;
            --muted-foreground:     #8A8A8A;
            --secondary:            #222222;
            --secondary-foreground: #F7F4EF;
            --border:               rgba(255,255,255,0.08);
            --shadow-soft:          0 4px 24px rgba(0,0,0,0.30);
            --shadow-hover:         0 8px 30px rgba(0,0,0,0.45);
        }

        /* ===== Design tokens — light (class override) ===== */
        html.light {
            --background:           #F7F4EF;
            --foreground:           #111111;
            --card:                 #FFFFFF;
            --card-foreground:      #111111;
            --muted:                #FCFBF8;
            --muted-foreground:     #6B6B6B;
            --secondary:            #FCFBF8;
            --secondary-foreground: #111111;
            --border:               rgba(17,17,17,0.08);
            --shadow-soft:          0 4px 24px rgba(0,0,0,0.06);
            --shadow-hover:         0 8px 30px rgba(0,0,0,0.10);
        }

        /* ===== Reset ===== */
        *, *::before, *::after { box-sizing: border-box; }
        body {
            font-family: var(--font-sans);
            font-size: 0.9375rem;
            line-height: 1.5;
            color: var(--foreground);
            background: var(--background);
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
            margin: 0;
        }

        /* ===== Card ===== */
        .card {
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            box-shadow: var(--shadow-soft);
        }

        /* ===== Form (centralized) ===== */
        input:not([type="file"]),
        textarea,
        select {
            width: 100%;
            border: 1px solid var(--border);
            border-radius: var(--radius-md);
            padding: 8px 12px;
            font: inherit;
            font-size: 1rem;
            background: var(--muted);
            color: var(--foreground);
            outline: none;
            transition: border-color 0.15s, box-shadow 0.15s;
        }
        input:not([type="file"]):focus,
        textarea:focus,
        select:focus {
            border-color: var(--primary);
            box-shadow: 0 0 0 3px var(--ring);
            background: var(--card);
        }
        textarea { min-height: 80px; resize: vertical; }
        select { cursor: pointer; }
        input[type="file"] { font-size: 0.875rem; margin: 6px 0; }
        label {
            display: block;
            margin: 0 0 5px;
            font-size: 0.8125rem;
            font-weight: 600;
        }
        .field { margin-bottom: 14px; }

        /* ===== Buttons ===== */
        .btn-action,
        .btn-primary {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
            height: 40px;
            padding: 0 22px;
            font: 600 0.9375rem/1 var(--font-sans);
            color: var(--primary-foreground);
            background: var(--primary);
            border: 1px solid transparent;
            border-radius: var(--radius-full);
            cursor: pointer;
            white-space: nowrap;
            text-decoration: none;
            transition: background 0.15s, box-shadow 0.15s, transform 0.12s;
            box-shadow: var(--shadow-soft);
        }
        .btn-action:hover, .btn-primary:hover {
            background: var(--primary-hover);
            box-shadow: var(--shadow-hover);
            transform: scale(1.02);
        }
        .btn-action:active, .btn-primary:active { transform: scale(0.98); }
        .btn-action:disabled, .btn-primary:disabled { opacity: 0.45; pointer-events: none; }

        /* Full-width variant used in auth forms */
        .btn { width: 100%; height: 40px; border: none; border-radius: var(--radius-full); font: 600 0.9375rem/1 var(--font-sans); cursor: pointer; transition: background 0.15s, box-shadow 0.15s; }

        .btn-small,
        .btn-secondary {
            display: inline-flex;
            align-items: center;
            gap: 5px;
            height: 32px;
            padding: 0 14px;
            font: 500 0.8125rem/1 var(--font-sans);
            color: var(--foreground);
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: var(--radius-full);
            cursor: pointer;
            white-space: nowrap;
            text-decoration: none;
            transition: background 0.15s, box-shadow 0.12s;
        }
        .btn-small:hover, .btn-secondary:hover {
            background: var(--muted);
            box-shadow: var(--shadow-soft);
        }
        .btn-small.grant  { color: var(--success);     border-color: var(--success-border); }
        .btn-small.revoke { color: var(--destructive);  border-color: var(--destructive-border); }

        /* ===== State chips ===== */
        .ok {
            display: block;
            background: var(--success-bg);
            border: 1px solid var(--success-border);
            color: var(--success);
            border-radius: var(--radius-md);
            padding: 6px 10px;
        }
        .down, .error {
            display: block;
            background: var(--destructive-bg);
            border: 1px solid var(--destructive-border);
            color: var(--destructive);
            border-radius: var(--radius-md);
            padding: 6px 10px;
        }

        /* Form feedback wrapper — hidden until .ok / .down added by JS */
        .result {
            margin-top: 10px;
            font-size: 0.875rem;
            display: none;
        }

        /* ===== Typography helpers ===== */
        .meta, .hint, .mini { color: var(--muted-foreground); font-size: 0.875rem; }

        /* ===== Nav link ===== */
        .link {
            display: block;
            margin-top: 18px;
            text-align: center;
            text-decoration: none;
            font-size: 0.875rem;
            color: var(--muted-foreground);
            font-weight: 500;
            transition: color 0.15s;
        }
        .link:hover { color: var(--foreground); }

        /* ===== Theme toggle button ===== */
        .rev0-theme-toggle {
            position: fixed;
            bottom: 20px;
            left: 20px;
            z-index: 9999;
            width: 36px;
            height: 36px;
            border-radius: 50%;
            border: 1px solid var(--border);
            background: var(--card);
            color: var(--foreground);
            font-size: 1rem;
            line-height: 1;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            box-shadow: var(--shadow-soft);
            transition: background 0.15s, box-shadow 0.15s, transform 0.12s;
            padding: 0;
        }
        .rev0-theme-toggle:hover {
            background: var(--muted);
            box-shadow: var(--shadow-hover);
            transform: scale(1.08);
        }
"#;
