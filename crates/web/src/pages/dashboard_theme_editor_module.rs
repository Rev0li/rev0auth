pub const JS_DASHBOARD_THEME_EDITOR_MODULE: &str = r#"
function createDashboardThemeEditorModule() {
    const editableTokens = [
        { key: '--color-ink', label: 'Texte principal', sample: '#132331' },
        { key: '--color-muted', label: 'Texte secondaire', sample: '#4b5f71' },
        { key: '--color-panel', label: 'Fond carte', sample: 'rgba(255, 255, 255, 0.92)' },
        { key: '--color-panel-border', label: 'Bordure carte', sample: 'rgba(19, 35, 49, 0.1)' },
        { key: '--color-success', label: 'Success texte', sample: '#0d9b73' },
        { key: '--color-danger', label: 'Danger texte', sample: '#ef4e24' },
        { key: '--color-success-bg', label: 'Success fond', sample: '#e8fff5' },
        { key: '--color-danger-bg', label: 'Danger fond', sample: '#fff0ec' },
        { key: '--btn-primary-from', label: 'Bouton primary start', sample: '#ff6b3b' },
        { key: '--btn-primary-to', label: 'Bouton primary end', sample: '#ef4e24' },
        { key: '--btn-secondary-bg', label: 'Bouton secondary fond', sample: '#f2f9ff' },
        { key: '--btn-secondary-border', label: 'Bouton secondary bordure', sample: 'rgba(19, 35, 49, 0.15)' },
        { key: '--btn-secondary-ink', label: 'Bouton secondary texte', sample: '#132331' },
        { key: '--bg-a', label: 'Fond gradient A', sample: '#eef8ff' },
        { key: '--bg-b', label: 'Fond gradient B', sample: '#e6f7ee' },
    ];
    const PRESET_KEY = 'rev0auth_theme_presets';
    const ACTIVE_PRESET_KEY = 'rev0auth_theme_active_preset';
    let previewBaseline = null;

    function currentTokenValue(token) {
        const value = getComputedStyle(document.documentElement).getPropertyValue(token);
        return (value || '').trim();
    }

    function setThemeMsg(ok, message) {
        const el = document.getElementById('theme-editor-msg');
        if (!el) return;
        el.className = 'mini ' + (ok ? 'ok' : 'down');
        el.textContent = message;
    }

    function broadcastThemeUpdate() {
        window.dispatchEvent(new Event('rev0auth:theme-update'));
    }

    function readPresets() {
        try {
            const raw = localStorage.getItem(PRESET_KEY);
            if (!raw) return {};
            const parsed = JSON.parse(raw);
            return parsed && typeof parsed === 'object' ? parsed : {};
        } catch (_err) {
            return {};
        }
    }

    function writePresets(presets) {
        localStorage.setItem(PRESET_KEY, JSON.stringify(presets));
    }

    function activePresetName() {
        return localStorage.getItem(ACTIVE_PRESET_KEY) || '';
    }

    function setActivePresetName(name) {
        if (!name) {
            localStorage.removeItem(ACTIVE_PRESET_KEY);
            return;
        }
        localStorage.setItem(ACTIVE_PRESET_KEY, name);
    }

    function collectThemePayload() {
        const payload = {};
        editableTokens.forEach((token) => {
            const input = document.getElementById('theme-token-' + token.key.replace(/[^a-z0-9]/gi, '-'));
            if (!input) return;
            const value = String(input.value || '').trim();
            if (value) payload[token.key] = value;
        });
        return payload;
    }

    function applyThemePayload(payload) {
        const root = document.documentElement;
        Object.entries(payload).forEach(([token, value]) => {
            root.style.setProperty(token, value);
        });
    }

    function clearThemePayload() {
        const root = document.documentElement;
        editableTokens.forEach((token) => {
            root.style.removeProperty(token.key);
        });
    }

    function readStoredTheme() {
        try {
            const raw = localStorage.getItem('rev0auth_theme');
            if (!raw) return null;
            const parsed = JSON.parse(raw);
            return parsed && typeof parsed === 'object' ? parsed : null;
        } catch (_err) {
            return null;
        }
    }

    function renderPresetSelect() {
        const select = document.getElementById('theme-preset-select');
        if (!select) return;
        const presets = readPresets();
        const names = Object.keys(presets).sort((a, b) => a.localeCompare(b));
        const active = activePresetName();
        const options = ['<option value="">(Preset custom)</option>'].concat(
            names.map((name) => {
                const selected = name === active ? ' selected' : '';
                return '<option value="' + name + '"' + selected + '>' + name + '</option>';
            })
        );
        select.innerHTML = options.join('');
    }

    function renderEditor() {
        const container = document.getElementById('theme-editor-list');
        if (!container) return;
        container.innerHTML = editableTokens.map((token) => {
            const id = 'theme-token-' + token.key.replace(/[^a-z0-9]/gi, '-');
            const value = currentTokenValue(token.key) || token.sample;
            return '<div class="theme-token-row">'
                + '<label class="theme-token-label" for="' + id + '">' + token.label + ' <span class="theme-token-key">(' + token.key + ')</span></label>'
                + '<input class="theme-token-input" id="' + id + '" value="' + value + '" />'
                + '</div>';
        }).join('');
        editableTokens.forEach((token) => {
            const input = document.getElementById('theme-token-' + token.key.replace(/[^a-z0-9]/gi, '-'));
            if (!input) return;
            input.addEventListener('input', () => {
                const payload = collectThemePayload();
                applyThemePayload(payload);
                setActivePresetName('');
                renderPresetSelect();
            });
        });
    }

    function saveTheme() {
        const payload = collectThemePayload();
        localStorage.setItem('rev0auth_theme', JSON.stringify(payload));
        applyThemePayload(payload);
        broadcastThemeUpdate();
        previewBaseline = null;
        setActivePresetName('');
        renderPresetSelect();
        setThemeMsg(true, 'Theme sauvegarde en local. Il sera applique sur toutes les pages frontend.');
    }

    function saveAsPreset() {
        const input = document.getElementById('theme-preset-name');
        const selected = document.getElementById('theme-preset-select');
        const fallbackSelectedName = String((selected && selected.value) || '').trim();
        const name = String((input && input.value) || '').trim() || fallbackSelectedName;
        if (!name) {
            setThemeMsg(false, 'Donne un nom de preset avant de sauvegarder.');
            return;
        }
        const presets = readPresets();
        presets[name] = collectThemePayload();
        writePresets(presets);
        setActivePresetName(name);
        if (input) input.value = name;
        renderPresetSelect();
        setThemeMsg(true, 'Preset "' + name + '" sauvegarde.');
    }

    function updateSelectedPreset() {
        const select = document.getElementById('theme-preset-select');
        const name = String((select && select.value) || '').trim();
        if (!name) {
            setThemeMsg(false, 'Choisis un preset a ecraser.');
            return;
        }
        const presets = readPresets();
        if (!presets[name]) {
            setThemeMsg(false, 'Preset introuvable.');
            return;
        }

        const payload = collectThemePayload();
        presets[name] = payload;
        writePresets(presets);
        localStorage.setItem('rev0auth_theme', JSON.stringify(payload));
        applyThemePayload(payload);
        broadcastThemeUpdate();
        setActivePresetName(name);
        const input = document.getElementById('theme-preset-name');
        if (input) input.value = name;
        renderPresetSelect();
        setThemeMsg(true, 'Preset "' + name + '" mis a jour.');
    }

    function applyPreset(name) {
        const presets = readPresets();
        const payload = presets[name];
        if (!payload || typeof payload !== 'object') {
            setThemeMsg(false, 'Preset introuvable.');
            return;
        }
        localStorage.setItem('rev0auth_theme', JSON.stringify(payload));
        applyThemePayload(payload);
        broadcastThemeUpdate();
        setActivePresetName(name);
        renderEditor();
        renderPresetSelect();
        setThemeMsg(true, 'Preset "' + name + '" applique.');
    }

    function applySelectedPreset() {
        const select = document.getElementById('theme-preset-select');
        const name = String((select && select.value) || '').trim();
        if (!name) {
            setThemeMsg(false, 'Choisis un preset a appliquer.');
            return;
        }
        applyPreset(name);
    }

    function previewTheme() {
        if (previewBaseline === null) {
            previewBaseline = readStoredTheme();
        }
        const payload = collectThemePayload();
        applyThemePayload(payload);
        setThemeMsg(true, 'Preview applique uniquement sur cette page (non sauvegarde).');
    }

    function resetPreview() {
        clearThemePayload();
        const stored = readStoredTheme();
        if (stored) {
            applyThemePayload(stored);
        }
        renderEditor();
        previewBaseline = null;
        setThemeMsg(true, 'Preview reset: retour au theme sauvegarde.');
    }

    function deleteSelectedPreset() {
        const select = document.getElementById('theme-preset-select');
        const name = String((select && select.value) || '').trim();
        if (!name) {
            setThemeMsg(false, 'Choisis un preset a supprimer.');
            return;
        }
        const presets = readPresets();
        if (!presets[name]) {
            setThemeMsg(false, 'Preset introuvable.');
            return;
        }
        delete presets[name];
        writePresets(presets);
        if (activePresetName() === name) {
            setActivePresetName('');
        }
        renderPresetSelect();
        setThemeMsg(true, 'Preset "' + name + '" supprime.');
    }

    function resetTheme() {
        localStorage.removeItem('rev0auth_theme');
        document.documentElement.removeAttribute('style');
        broadcastThemeUpdate();
        setActivePresetName('');
        renderEditor();
        renderPresetSelect();
        setThemeMsg(true, 'Theme local supprime. Recharge la page pour revenir au style par defaut.');
    }

    function exportTheme() {
        const payload = collectThemePayload();
        const text = JSON.stringify(payload, null, 2);
        const output = document.getElementById('theme-editor-export');
        if (output) output.value = text;
        if (navigator.clipboard && navigator.clipboard.writeText) {
            navigator.clipboard.writeText(text).catch(() => {});
        }
        setThemeMsg(true, 'JSON theme exporte (copie presse-papiers si autorise).');
    }

    function importTheme() {
        const input = document.getElementById('theme-editor-export');
        const raw = String((input && input.value) || '').trim();
        if (!raw) {
            setThemeMsg(false, 'Colle un JSON theme avant import.');
            return;
        }
        try {
            const payload = JSON.parse(raw);
            if (!payload || typeof payload !== 'object') {
                setThemeMsg(false, 'JSON invalide.');
                return;
            }
            localStorage.setItem('rev0auth_theme', JSON.stringify(payload));
            applyThemePayload(payload);
            broadcastThemeUpdate();
            setActivePresetName('');
            renderEditor();
            renderPresetSelect();
            setThemeMsg(true, 'Theme importe et applique.');
        } catch (_err) {
            setThemeMsg(false, 'Impossible de parser le JSON theme.');
        }
    }

    function bindEvents() {
        const saveBtn = document.getElementById('theme-editor-save');
        const resetBtn = document.getElementById('theme-editor-reset');
        const exportBtn = document.getElementById('theme-editor-export-btn');
        const importBtn = document.getElementById('theme-editor-import-btn');
        const savePresetBtn = document.getElementById('theme-preset-save');
        const updatePresetBtn = document.getElementById('theme-preset-update');
        const applyPresetBtn = document.getElementById('theme-preset-apply');
        const deletePresetBtn = document.getElementById('theme-preset-delete');
        const previewBtn = document.getElementById('theme-preview-apply');
        const resetPreviewBtn = document.getElementById('theme-preview-reset');
        const presetSelect = document.getElementById('theme-preset-select');
        if (saveBtn) saveBtn.addEventListener('click', saveTheme);
        if (resetBtn) resetBtn.addEventListener('click', resetTheme);
        if (exportBtn) exportBtn.addEventListener('click', exportTheme);
        if (importBtn) importBtn.addEventListener('click', importTheme);
        if (savePresetBtn) savePresetBtn.addEventListener('click', saveAsPreset);
        if (updatePresetBtn) updatePresetBtn.addEventListener('click', updateSelectedPreset);
        if (applyPresetBtn) applyPresetBtn.addEventListener('click', applySelectedPreset);
        if (deletePresetBtn) deletePresetBtn.addEventListener('click', deleteSelectedPreset);
        if (previewBtn) previewBtn.addEventListener('click', previewTheme);
        if (resetPreviewBtn) resetPreviewBtn.addEventListener('click', resetPreview);
        if (presetSelect) {
            presetSelect.addEventListener('change', () => {
                const input = document.getElementById('theme-preset-name');
                if (input) input.value = String(presetSelect.value || '');
            });
        }
    }

    function initThemeEditor() {
        renderEditor();
        renderPresetSelect();
        bindEvents();
    }

    return {
        initThemeEditor,
        saveTheme,
        saveAsPreset,
        updateSelectedPreset,
        applyPreset,
        previewTheme,
        resetPreview,
        resetTheme,
        exportTheme,
        importTheme,
    };
}
"#;
