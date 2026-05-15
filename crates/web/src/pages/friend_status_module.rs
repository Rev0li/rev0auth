// Status module for member mood/status updates
pub const JS_FRIEND_STATUS_MODULE: &str = r#"
function createFriendStatusModule(ctx) {
    const { pseudo } = ctx;
    
    const statusMsg = document.getElementById('status-msg');

    async function setStatus(status) {
        try {
            const res = await fetch('/members/status', {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({
                    pseudo,
                    status
                })
            });
            const data = await res.json();

            statusMsg.className = 'status-msg ' + (data.ok ? 'ok' : 'error');
            statusMsg.textContent = data.message;
            statusMsg.style.display = 'block';

            setTimeout(() => {
                statusMsg.style.display = 'none';
            }, 3000);
            return data.ok;
        } catch (err) {
            statusMsg.className = 'status-msg error';
            statusMsg.textContent = 'Erreur: ' + err.message;
            statusMsg.style.display = 'block';
            return false;
        }
    }

    // Setup status buttons (null-guarded — module may be used on pages without these elements)
    const happyBtn = document.getElementById('happy-btn');
    if (happyBtn) happyBtn.addEventListener('click', async () => { await setStatus('content'); });

    const mehBtn = document.getElementById('meh-btn');
    if (mehBtn) mehBtn.addEventListener('click', async () => { await setStatus('bof'); });

    const questionBtn = document.getElementById('question-btn');
    if (questionBtn) questionBtn.addEventListener('click', async () => { await setStatus('question'); });

    return {
        setStatus
    };
}
"#;

pub const CSS_FRIEND_STATUS_STYLES: &str = r#"
        .header-status { display: flex; gap: 8px; flex-wrap: wrap; align-items: center; }
        .status-buttons { display: flex; gap: 6px; margin-top: 12px; flex-wrap: wrap; }
        .status-btn {
            display: inline-flex;
            align-items: center;
            height: 32px;
            padding: 0 13px;
            border: 1px solid var(--border);
            border-radius: var(--radius-md);
            background: var(--muted);
            color: var(--muted-foreground);
            font: 500 0.8125rem/1 var(--font-sans);
            cursor: pointer;
            transition: border-color 0.15s, background 0.15s, color 0.15s;
        }
        .status-btn:hover { background: var(--card); color: var(--foreground); border-color: var(--foreground); }
        .status-btn.actif   { border-color: var(--success-border); background: var(--success-bg); color: var(--success); }
        .status-btn.occupe  { border-color: #fed7aa; background: #fff7ed; color: #c2410c; }
        .status-btn.inactif { border-color: var(--border); background: var(--muted); color: var(--muted-foreground); }
        .status-msg {
            margin-top: 8px;
            padding: 7px 10px;
            border-radius: var(--radius-md);
            font-size: 0.875rem;
            display: none;
        }
        .status-msg.ok    { display: block; background: var(--success-bg); color: var(--success); border: 1px solid var(--success-border); }
        .status-msg.error { display: block; background: var(--destructive-bg);  color: var(--destructive);  border: 1px solid var(--destructive-border); }
"#;
