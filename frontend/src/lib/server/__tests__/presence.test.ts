import { describe, it, expect, beforeEach } from 'vitest';
import { touch, presence, clearPresence, ONLINE_WINDOW_S } from '../presence.js';

const T = 1_800_000_000;

describe('présence en direct', () => {
    beforeEach(clearPresence);

    it('en ligne si activité récente, par endroit, le plus récent d\'abord', () => {
        touch('u1', 'alice', 'member', 'portail', T - 600);
        touch('u1', 'alice', 'member', 'songsurf', T - 30);
        touch('u2', 'Revo', 'admin', 'console', T - 10);
        const p = presence(T);
        expect(p.map((x) => [x.pseudo, x.online])).toEqual([['Revo', true], ['alice', true]]);
        expect(p[1].places).toEqual([
            { place: 'songsurf', last: T - 30, online: true },
            { place: 'portail', last: T - 600, online: false },
        ]);
    });

    it('hors ligne après la fenêtre, oublié après 24 h', () => {
        touch('u1', 'alice', 'member', 'portail', T - ONLINE_WINDOW_S - 1);
        touch('u2', 'bob', 'member', 'portail', T - 25 * 3600);
        expect(presence(T).map((x) => [x.pseudo, x.online])).toEqual([['alice', false]]);
    });

    it('le pseudo suit la dernière activité', () => {
        touch('u1', 'alice', 'member', 'portail', T - 5);
        touch('u1', 'Alice', 'mod', 'portail', T);
        expect(presence(T)[0]).toMatchObject({ pseudo: 'Alice', role: 'mod' });
    });
});
