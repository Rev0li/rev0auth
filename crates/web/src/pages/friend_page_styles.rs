// Friend page styles as constant for modular extraction

pub const FRIEND_PAGE_STYLES: &str = r#"
        body {
            margin: 0;
            font-family: var(--font-sans);
            color: #132331;
            background:
                radial-gradient(circle at 15% 10%, #d9f0ff, transparent 40%),
                radial-gradient(circle at 85% 80%, #ffe7ca, transparent 40%),
                linear-gradient(135deg, #eef8ff 0%, #e6f7ee 100%);
            min-height: 100vh;
        }
        .container {
            max-width: 900px;
            margin: 0 auto;
            padding: 28px;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            margin-bottom: 32px;
            gap: 16px;
            flex-wrap: wrap;
        }
        .header h1 { margin: 0; font-size: clamp(1.8rem, 5vw, 2.5rem); }
        .header-meta {
            display: flex;
            flex-direction: column;
            gap: 10px;
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
            gap: 10px;
        }
        .header-action-row {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            justify-content: flex-end;
        }
        .logout-btn {
            padding: 8px 14px;
            background: rgba(255, 107, 59, 0.9);
            color: white;
            border: 0;
            border-radius: 10px;
            font-weight: 700;
            cursor: pointer;
            text-decoration: none;
            display: inline-block;
        }
        .logout-btn:hover { background: rgba(239, 78, 36, 1); }
        .profile-btn {
            padding: 8px 14px;
            background: rgba(13, 155, 115, 0.92);
            color: white;
            border: 0;
            border-radius: 10px;
            font-weight: 700;
            cursor: pointer;
            text-decoration: none;
            display: inline-block;
        }
        .profile-btn:hover { background: rgba(10, 132, 98, 1); }
        .chat-card {
            margin-bottom: 20px;
            border: 1px solid rgba(19, 35, 49, 0.14);
            border-radius: 14px;
            background: rgba(255, 255, 255, 0.9);
            padding: 16px;
            box-shadow: 0 12px 24px rgba(19, 35, 49, 0.09);
        }
        .chat-card h2 {
            margin: 0 0 10px;
            font-size: 1.1rem;
        }
        .onboarding-intro {
            margin-top: 0;
            opacity: .85;
        }
        .onboarding-label {
            display: block;
            font-weight: 700;
            margin-top: 10px;
        }
        .onboarding-field,
        .onboarding-textarea {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            padding: 9px;
            box-sizing: border-box;
            font: inherit;
            background: #fff;
        }
        .onboarding-textarea {
            min-height: 110px;
        }
        .actions-tight {
            margin-top: 10px;
        }
        .mood-label {
            font-weight: 700;
            opacity: .8;
        }
        .services-intro {
            margin-top: 0;
            opacity: .8;
        }
        .chat-intro {
            margin: 0 0 8px;
            opacity: .82;
        }
        .chat-card label {
            display: block;
            margin: 10px 0 6px;
            font-weight: 700;
        }
        .chat-card input,
        .chat-card textarea {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            padding: 9px;
            box-sizing: border-box;
            font: inherit;
            background: #fff;
        }
        .chat-card textarea {
            min-height: 110px;
            resize: vertical;
        }
        .chat-msg {
            margin-top: 10px;
            padding: 8px;
            border-radius: 8px;
            font-size: 0.85rem;
            display: none;
        }
        .chat-msg.ok {
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
            display: block;
        }
        .chat-msg.error {
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
            display: block;
        }
        .chat-history {
            margin-top: 12px;
            max-height: 300px;
            overflow: auto;
            display: grid;
            gap: 8px;
            padding-right: 4px;
        }
        .chat-bubble {
            max-width: 86%;
            padding: 9px 11px;
            border-radius: 12px;
            border: 1px solid rgba(19, 35, 49, 0.12);
            white-space: pre-wrap;
            line-height: 1.4;
            font-size: 0.9rem;
            box-shadow: 0 6px 14px rgba(19, 35, 49, 0.07);
        }
        .chat-bubble.mine {
            justify-self: end;
            background: #e8fff5;
            border-color: #b3ecd1;
        }
        .chat-bubble.theirs {
            justify-self: start;
            background: #fff;
        }
        .chat-meta {
            display: block;
            margin-top: 4px;
            font-size: 0.78rem;
            opacity: 0.72;
        }
        .card {
            background: rgba(255, 255, 255, 0.92);
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 20px;
            padding: 28px;
            box-shadow: 0 16px 45px rgba(19, 35, 49, 0.12);
            margin-bottom: 18px;
        }
        h2 { margin: 0 0 14px; font-size: 1.4rem; }
        .greeting { font-size: 1.1rem; line-height: 1.6; margin-bottom: 20px; }
        .greeting strong { color: #0d9b73; }
        .feature-list {
            list-style: none;
            padding: 0;
            margin: 0;
        }
        .feature-list li {
            padding: 10px 0;
            border-bottom: 1px solid rgba(19, 35, 49, 0.08);
            display: flex;
            align-items: center;
            gap: 10px;
        }
        .feature-list li:last-child { border-bottom: 0; }
        .feature-list li:before {
            content: "→";
            color: #ff6f3f;
            font-weight: 700;
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
        .remark-panel {
            display: none;
            margin-top: 14px;
            padding: 12px;
            border: 1px dashed rgba(19, 35, 49, 0.2);
            border-radius: 12px;
            background: rgba(243, 247, 250, 0.8);
        }
        .services {
            display: flex;
            flex-direction: column;
            gap: 14px;
            margin-top: 14px;
        }
        .service-card {
            border: 1px solid rgba(19, 35, 49, 0.12);
            border-radius: 12px;
            background: #fff;
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
            border-radius: 10px;
            border: 1px solid rgba(19, 35, 49, 0.12);
            background: #f3f7fa;
        }
        .service-card h3 {
            margin: 0 0 6px;
            font-size: 1rem;
        }
        .service-card p {
            margin: 0 0 10px;
            font-size: 0.9rem;
            opacity: 0.8;
        }
        .service-btn {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            background: rgba(13, 155, 115, 0.08);
            color: #0d9b73;
            font-weight: 700;
            padding: 9px 10px;
            cursor: pointer;
        }
        .service-btn:hover {
            background: rgba(13, 155, 115, 0.18);
        }
        .service-btn.locked {
            background: #f3f7fa;
            color: #4b5f71;
            border-color: rgba(19, 35, 49, 0.16);
        }
        .service-state {
            font-size: 0.84rem;
            margin: 8px 0;
            opacity: 0.84;
        }
        .service-input {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            padding: 8px;
            box-sizing: border-box;
            margin-bottom: 8px;
        }
        .service-msg {
            margin-top: 10px;
            padding: 8px;
            border-radius: 8px;
            font-size: 0.85rem;
            display: none;
        }
        .service-msg.ok {
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
        }
        .service-msg.error {
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
        }
        %%FRIEND_ONBOARDING_CSS%%
        %%FRIEND_SERVICES_CSS%%
        %%FRIEND_CHAT_CSS%%
        %%FRIEND_STATUS_CSS%%
        %%FRIEND_AVATAR_CSS%%
        @media (max-width: 900px) {
            .container {
                padding: 18px;
            }
            .header {
                flex-direction: column;
                align-items: stretch;
            }
            .header-actions {
                align-items: flex-start;
            }
            .header-action-row {
                justify-content: flex-start;
            }
            .service-card {
                aspect-ratio: 1 / 1;
                overflow: hidden;
            }
            .service-media {
                aspect-ratio: 1 / 1;
            }
        }
        .onboarding-modal {
            position: fixed;
            inset: 0;
            background: rgba(10, 20, 30, 0.6);
            display: none;
            align-items: center;
            justify-content: center;
            z-index: 90;
            padding: 16px;
        }
        .onboarding-card {
            width: 100%;
            max-width: 520px;
            background: #fff;
            border-radius: 14px;
            border: 1px solid rgba(19, 35, 49, 0.16);
            padding: 16px;
        }
        .onboarding-msg {
            margin-top: 10px;
            border-radius: 8px;
            padding: 8px;
            font-size: 0.9rem;
            display: none;
        }
        .onboarding-msg.ok {
            display: block;
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
        }
        .onboarding-msg.error {
            display: block;
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
        }
"#;
