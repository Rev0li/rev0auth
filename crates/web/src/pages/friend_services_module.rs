// Services module for GitHub, Jellyfin, Songsurf access requests
pub const JS_FRIEND_SERVICES_MODULE: &str = r#"
function createFriendServicesModule(ctx) {
    const { pseudo } = ctx;
    
    const accessState = {
        github: false,
        jellyfin: false,
        songsurf: false,
        requestGithub: false,
        requestJellyfin: false,
        requestSongsurf: false
    };

    const serviceMsg = document.getElementById('service-msg');

    function setServiceMsg(ok, message) {
        serviceMsg.className = 'service-msg ' + (ok ? 'ok' : 'error');
        serviceMsg.textContent = message;
        serviceMsg.style.display = 'block';
    }

    function renderServiceButtons() {
        const songsurfBtn = document.getElementById('songsurf-btn');
        const jellyfinBtn = document.getElementById('jellyfin-btn');
        const githubBtn = document.getElementById('github-btn');

        const songsurfState = document.getElementById('songsurf-state');
        const jellyfinState = document.getElementById('jellyfin-state');
        const githubState = document.getElementById('github-state');

        if (accessState.songsurf) {
            songsurfState.textContent = 'Etat: ACCES OUVERT';
            songsurfBtn.textContent = 'Ouvrir Songsurf';
            songsurfBtn.classList.remove('locked');
        } else if (accessState.requestSongsurf) {
            songsurfState.textContent = 'Etat: demande envoyee, en attente admin';
            songsurfBtn.textContent = 'Demande Songsurf envoyee';
            songsurfBtn.classList.add('locked');
        } else {
            songsurfState.textContent = 'Etat: verrouille';
            songsurfBtn.textContent = 'Demander acces Songsurf';
            songsurfBtn.classList.add('locked');
        }

        if (accessState.jellyfin) {
            jellyfinState.textContent = 'Etat: ACCES OUVERT';
            jellyfinBtn.textContent = 'Ouvrir Jellyfin';
            jellyfinBtn.classList.remove('locked');
        } else if (accessState.requestJellyfin) {
            jellyfinState.textContent = 'Etat: demande envoyee, en attente admin';
            jellyfinBtn.textContent = 'Demande Jellyfin envoyee';
            jellyfinBtn.classList.add('locked');
        } else {
            jellyfinState.textContent = 'Etat: verrouille';
            jellyfinBtn.textContent = 'Demander acces Jellyfin';
            jellyfinBtn.classList.add('locked');
        }

        if (accessState.github) {
            githubState.textContent = 'Etat: ACCES OUVERT';
            githubBtn.textContent = 'Ouvrir GitHub';
            githubBtn.classList.remove('locked');
        } else if (accessState.requestGithub) {
            githubState.textContent = 'Etat: demande envoyee, en attente admin';
            githubBtn.textContent = 'Demande GitHub envoyee';
            githubBtn.classList.add('locked');
        } else {
            githubState.textContent = 'Etat: verrouille';
            githubBtn.textContent = 'J\'ai mis une star, demander acces GitHub';
            githubBtn.classList.add('locked');
        }
    }

    async function loadMemberAccessState() {
        try {
            const res = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(pseudo), { cache: 'no-store' });
            const data = await res.json();
            accessState.github = !!data.access_github;
            accessState.jellyfin = !!data.access_jellyfin;
            accessState.songsurf = !!data.access_songsurf;
            accessState.requestGithub = !!data.request_github;
            accessState.requestJellyfin = !!data.request_jellyfin;
            accessState.requestSongsurf = !!data.request_songsurf;

            if (data.github_username) {
                document.getElementById('github-username').value = data.github_username;
            }
            renderServiceButtons();
        } catch (err) {
            setServiceMsg(false, 'Impossible de charger l\'etat des acces: ' + err.message);
        }
    }

    async function requestService(service, extraPayload) {
        try {
            const res = await fetch('/members/access/request', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify(Object.assign({ pseudo, service }, extraPayload || {}))
            });
            const data = await res.json();
            setServiceMsg(!!data.ok, data.message || 'Reponse recue.');
            if (data.ok) {
                await loadMemberAccessState();
            }
        } catch (err) {
            setServiceMsg(false, 'Erreur: ' + err.message);
        }
    }

    // Setup service button handlers
    document.getElementById('songsurf-btn').addEventListener('click', async () => {
        if (accessState.songsurf) {
            window.location.href = 'https://revoli-songsurf.duckdns.org';
            return;
        }
        await requestService('songsurf');
    });

    document.getElementById('jellyfin-btn').addEventListener('click', async () => {
        if (accessState.jellyfin) {
            window.location.href = 'https://revoli-jellyfin.duckdns.org';
            return;
        }
        await requestService('jellyfin');
    });

    document.getElementById('github-btn').addEventListener('click', async () => {
        if (accessState.github) {
            window.location.href = 'https://github.com/Rev0li';
            return;
        }

        const githubUsername = document.getElementById('github-username').value.trim();
        if (!githubUsername) {
            setServiceMsg(false, 'Renseigne ton username GitHub avant la demande.');
            return;
        }

        await requestService('github', {
            github_username: githubUsername,
            starred: true
        });
    });

    // Load initial state
    loadMemberAccessState();

    return {
        setServiceMsg,
        renderServiceButtons,
        loadMemberAccessState,
        requestService,
        accessState
    };
}
"#;

pub const CSS_FRIEND_SERVICES_STYLES: &str = r#"
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
        @media (max-width: 900px) {
            .service-card {
                aspect-ratio: 1 / 1;
                overflow: hidden;
            }
            .service-media {
                aspect-ratio: 1 / 1;
            }
        }
"#;
