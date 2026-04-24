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

    // Setup status buttons
    document.getElementById('happy-btn').addEventListener('click', async () => {
        await setStatus('content');
    });

    document.getElementById('meh-btn').addEventListener('click', async () => {
        await setStatus('bof');
    });

    document.getElementById('question-btn').addEventListener('click', async () => {
        await setStatus('question');
    });

    return {
        setStatus
    };
}
"#;

pub const CSS_FRIEND_STATUS_STYLES: &str = r#"
        .header-status { display: flex; gap: 8px; flex-wrap: wrap; align-items: center; }
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
        .status-msg.ok { display: block; background: var(--color-success-bg); color: var(--color-success); border: 1px solid var(--color-success-border); }
        .status-msg.error { display: block; background: var(--color-danger-bg); color: var(--color-danger); border: 1px solid var(--color-danger-border); }
"#;
