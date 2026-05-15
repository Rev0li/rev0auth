// Avatar module for profile picture handling with fallback
pub const JS_FRIEND_AVATAR_MODULE: &str = r#"
function createFriendAvatarModule(ctx) {
    const { pseudo } = ctx;
    
    const headerAvatar = document.getElementById('header-avatar');

    function fallbackAvatar(pseudoValue) {
        const first = (pseudoValue || '?').charAt(0).toUpperCase() || '?';
        const svg = `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'>
            <defs>
                <linearGradient id='g' x1='0' y1='0' x2='1' y2='1'>
                    <stop offset='0%' stop-color='#0d9b73'/>
                    <stop offset='100%' stop-color='#132331'/>
                </linearGradient>
            </defs>
            <rect width='100' height='100' rx='50' fill='url(#g)'/>
            <text x='50' y='61' text-anchor='middle' font-size='44' font-family='Space Grotesk, Arial, sans-serif' fill='#ffffff'>${first}</text>
        </svg>`;
        return 'data:image/svg+xml;utf8,' + encodeURIComponent(svg);
    }

    function setHeaderAvatarSrc(hasAvatar) {
        if (hasAvatar) {
            headerAvatar.src = '/members/avatar/' + encodeURIComponent(pseudo) + '?t=' + Date.now();
            headerAvatar.onerror = () => {
                headerAvatar.onerror = null;
                headerAvatar.src = fallbackAvatar(pseudo);
            };
            return;
        }
        headerAvatar.src = fallbackAvatar(pseudo);
    }

    function loadAvatar() {
        // Show fallback immediately so the img is never blank or broken
        headerAvatar.src = fallbackAvatar(pseudo);
        // Probe the real avatar in the background; swap only on success
        const probe = new Image();
        probe.onload = () => { headerAvatar.src = probe.src; };
        probe.src = '/members/avatar/' + encodeURIComponent(pseudo) + '?t=' + Date.now();
    }

    return {
        fallbackAvatar,
        setHeaderAvatarSrc,
        loadAvatar
    };
}
"#;

pub const CSS_FRIEND_AVATAR_STYLES: &str = r#"
        .header-avatar {
            width: 32px;
            height: 32px;
            border-radius: 50%;
            border: 1px solid var(--border);
            background: var(--muted);
            object-fit: cover;
            display: block;
            flex-shrink: 0;
        }
"#;
