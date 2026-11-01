import { describe, it, expect, vi, beforeEach } from 'vitest';

// Garde-fous de la console (AUTH-12, AUTH-27) : l'admin connecté ne peut ni
// changer son propre rôle, ni se désactiver, ni supprimer son compte ; il peut
// le faire pour les autres. C'est aussi ce qui garantit qu'il reste un admin.

const users = vi.hoisted(() => ({
    findUserByPseudo: vi.fn(),
    updateUser: vi.fn(async () => {}),
    deleteUser: vi.fn(async () => {}),
    ROLES: ['guest', 'member', 'mod', 'admin'],
}));
vi.mock('$lib/server/users.js', () => users);
vi.mock('$lib/server/audit.js', () => ({ writeAudit: vi.fn(async () => {}) }));

const { PUT, DELETE } = await import('../../../routes/japprends/users/[pseudo]/+server.js');

const ADMIN = { memberSession: null, adminSession: { pseudo: 'Revo' } } as unknown as App.Locals;

function put(pseudo: string, body: unknown) {
    return PUT({
        locals: ADMIN,
        params: { pseudo },
        request: new Request('http://x', { method: 'PUT', body: JSON.stringify(body) }),
    } as never) as Promise<Response>;
}
function del(pseudo: string) {
    return DELETE({ locals: ADMIN, params: { pseudo } } as never) as Promise<Response>;
}

beforeEach(() => {
    vi.clearAllMocks();
    users.findUserByPseudo.mockImplementation(async (p: string) => ({ id: `id-${p}`, name: p === 'revo' ? 'Revo' : p }));
});

describe('changer un rôle', () => {
    it.each(['member', 'mod', 'admin'])('donne le rôle %s à un autre membre', async (role) => {
        const r = await put('alice', { role });
        expect(r.status).toBe(200);
        expect(users.updateUser).toHaveBeenCalledWith('id-alice', { role });
    });

    it('refuse un rôle inconnu', async () => {
        const r = await put('alice', { role: 'superadmin' });
        expect(r.status).toBe(400);
        expect(users.updateUser).not.toHaveBeenCalled();
    });

    it('refuse de changer son propre rôle', async () => {
        const r = await put('revo', { role: 'member' });
        expect(r.status).toBe(409);
        expect(users.updateUser).not.toHaveBeenCalled();
    });

    it('refuse de se désactiver soi-même', async () => {
        const r = await put('revo', { active: false });
        expect(r.status).toBe(409);
        expect(users.updateUser).not.toHaveBeenCalled();
    });
});

describe('supprimer un compte', () => {
    it('supprime un autre membre', async () => {
        const r = await del('alice');
        expect(r.status).toBe(200);
        expect(users.deleteUser).toHaveBeenCalledWith('id-alice');
    });

    it('refuse de supprimer son propre compte', async () => {
        const r = await del('revo');
        expect(r.status).toBe(409);
        expect(users.deleteUser).not.toHaveBeenCalled();
    });
});
