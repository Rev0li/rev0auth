import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { gateRequest, consoleUrl, portalUrl, consoleWebAuthnOrigin, SONGSURF_EVENTS_PATH } from '../console-host.js';

// Console sous son propre nom (AUTH-11) : qui voit quoi selon l'hôte demandé.

const PORTAL  = 'rev0li.duckdns.org';
const CONSOLE = 'admin.rev0li.duckdns.org';

describe('console-host — sans ADMIN_ORIGIN (dev)', () => {
    beforeEach(() => {
        process.env.ORIGIN = 'http://localhost:5173';
        delete process.env.ADMIN_ORIGIN;
    });

    it('aucune séparation : la console reste sous /japprends', () => {
        expect(gateRequest('localhost:5173', '/japprends/dashboard', '')).toEqual({ kind: 'pass' });
        expect(gateRequest('localhost:5173', '/', '')).toEqual({ kind: 'pass' });
    });

    it('adresses relatives, WebAuthn sur le portail', () => {
        expect(consoleUrl('/japprends/login')).toBe('/japprends/login');
        expect(consoleWebAuthnOrigin()).toBe('http://localhost:5173');
    });
});

describe('console-host — console sous son propre nom', () => {
    beforeEach(() => {
        process.env.ORIGIN = `https://${PORTAL}`;
        process.env.ADMIN_ORIGIN = `https://${CONSOLE}`;
    });
    afterEach(() => { delete process.env.ADMIN_ORIGIN; });

    it.each([
        '/japprends',
        '/japprends/login',
        '/japprends/dashboard',
        '/japprends/passkey/auth-verify',
        '/japprends/users/alice',
    ])('portail public : %s introuvable', (path) => {
        expect(gateRequest(PORTAL, path, '')).toEqual({ kind: 'not-found' });
    });

    it('portail public : le reste du site passe', () => {
        for (const path of ['/', '/home/friend', '/members/profile', '/auth/verify', '/signup', '/japprendsX']) {
            expect(gateRequest(PORTAL, path, '')).toEqual({ kind: 'pass' });
        }
    });

    it('portail public : le NAS pousse toujours les évènements SongSurf', () => {
        expect(gateRequest(PORTAL, SONGSURF_EVENTS_PATH, '')).toEqual({ kind: 'pass' });
    });

    it('hôte absent ou inconnu : traité comme le portail public', () => {
        expect(gateRequest(null, '/japprends/dashboard', '')).toEqual({ kind: 'not-found' });
        expect(gateRequest('localhost:4173', '/japprends/login', '')).toEqual({ kind: 'not-found' });
        expect(gateRequest('evil.example', '/japprends/login', '')).toEqual({ kind: 'not-found' });
    });

    it('nom de la console : console, fichiers du site et déconnexion', () => {
        for (const path of ['/japprends/login', '/japprends/dashboard', '/_app/immutable/x.js', '/auth/logout']) {
            expect(gateRequest(CONSOLE, path, '')).toEqual({ kind: 'pass' });
        }
    });

    it('nom de la console : la racine mène à l’entrée de la console', () => {
        expect(gateRequest(CONSOLE, '/', '')).toEqual({ kind: 'redirect', location: `https://${CONSOLE}/japprends/login` });
    });

    it('nom de la console : le reste renvoie au portail public', () => {
        expect(gateRequest(CONSOLE, '/home/friend', '?x=1')).toEqual({ kind: 'redirect', location: `https://${PORTAL}/home/friend?x=1` });
        expect(gateRequest(CONSOLE, '/auth/verify', '?service=songsurf')).toEqual({ kind: 'redirect', location: `https://${PORTAL}/auth/verify?service=songsurf` });
    });

    it('adresses absolues ; WebAuthn sur le nom de la console', () => {
        expect(consoleUrl('/japprends/login')).toBe(`https://${CONSOLE}/japprends/login`);
        expect(portalUrl('/signup')).toBe(`https://${PORTAL}/signup`);
        expect(consoleWebAuthnOrigin()).toBe(`https://${CONSOLE}`);
    });
});
