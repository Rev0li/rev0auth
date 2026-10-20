import { describe, it, expect } from 'vitest';
import { usernameOf, adminView, PSEUDO_RE, type User } from '../users.js';

describe('modèle utilisateur', () => {
    it('le username est le pseudo en minuscules, sans espaces autour', () => {
        expect(usernameOf('  Revo ')).toBe('revo');
    });

    it('valide les pseudos comme l’inscription', () => {
        expect(PSEUDO_RE.test('revo_42')).toBe(true);
        expect(PSEUDO_RE.test('ab')).toBe(false);
        expect(PSEUDO_RE.test('avec espace')).toBe(false);
    });

    it('la vue admin garde la forme d’avant (pseudo affiché, date en secondes)', () => {
        const user = {
            id: 'u1', name: 'Revo', email: 'revo@local.invalid', emailVerified: false, image: null,
            createdAt: new Date('2026-05-01T10:00:00Z'), updatedAt: new Date(),
            username: 'revo', displayUsername: 'Revo', role: 'admin',
            active: true, approved: true, bio: '', accessJellyfin: true, accessSongsurf: false,
            requestJellyfin: false, requestSongsurf: true, githubUsername: 'rev0li', linkedinName: null,
        } satisfies User;
        const view = adminView(user);
        expect(view.pseudo).toBe('Revo');
        expect(view.createdAt).toBe(Date.parse('2026-05-01T10:00:00Z') / 1000);
        expect(view).not.toHaveProperty('email');
        expect(view).not.toHaveProperty('id');
    });
});
