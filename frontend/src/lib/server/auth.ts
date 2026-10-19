import argon2 from 'argon2';
import { randomBytes } from 'crypto';

export function hashPassword(plain: string): Promise<string> {
    return argon2.hash(plain);
}

export async function verifyPassword(plain: string, hash: string): Promise<boolean> {
    // hash vide (compte sans mot de passe après remove-password) ou hash corrompu :
    // argon2.verify lèverait une exception → refus propre plutôt que 500
    if (!hash) return false;
    try {
        return await argon2.verify(hash, plain);
    } catch {
        return false;
    }
}

export function generateToken(bytes = 32): string {
    return randomBytes(bytes).toString('hex');
}
