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

    return {
        fallbackAvatar,
        setHeaderAvatarSrc
    };
}
"#;

pub const CSS_FRIEND_AVATAR_STYLES: &str = r#"
        .header-avatar {
            width: 62px;
            height: 62px;
            border-radius: 50%;
            border: 2px solid rgba(19, 35, 49, 0.16);
            background: #f3f7fa;
            object-fit: cover;
            box-shadow: 0 6px 14px rgba(19, 35, 49, 0.16);
        }
"#;
