import { describe, it, expect } from 'vitest';
import { auth } from '../auth-v2.js';

// Les comptes ne sont créés que côté serveur (invitation, admin) : BetterAuth
// doit refuser toute inscription directe.

describe('BetterAuth — inscription directe', () => {
    it('est désactivée dans la configuration', () => {
        expect(auth.options.emailAndPassword?.disableSignUp).toBe(true);
    });

    it('refuse signUpEmail', async () => {
        await expect(
            auth.api.signUpEmail({
                body: { email: 'test@local.invalid', password: 'motdepasse123', name: 'test' },
            }),
        ).rejects.toThrow('Email and password sign up is not enabled');
    });
});
