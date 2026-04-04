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
        .header-status {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            align-items: center;
        }
        .status-buttons {
            display: flex;
            gap: 10px;
            margin-top: 16px;
            flex-wrap: wrap;
        }
        .status-btn {
            padding: 8px 14px;
            border: 1px solid rgba(13, 155, 115, 0.3);
            border-radius: 8px;
            background: rgba(13, 155, 115, 0.05);
            color: #0d9b73;
            font-weight: 600;
            cursor: pointer;
            font-size: 0.9rem;
        }
        .status-btn:hover {
            background: rgba(13, 155, 115, 0.15);
        }
        .status-msg {
            margin-top: 10px;
            padding: 8px;
            border-radius: 8px;
            font-size: 0.85rem;
            display: none;
        }
        .status-msg.ok {
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
        }
        .status-msg.error {
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
        }
"#;
