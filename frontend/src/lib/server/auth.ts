import argon2 from 'argon2';
import { randomBytes, createHmac } from 'crypto';

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

// Native TOTP (RFC 6238) — HMAC-SHA1, 30s period, 6 digits, window ±1
function base32Decode(input: string): Buffer {
    const alphabet = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
    const s = input.toUpperCase().replace(/=+$/, '').replace(/\s/g, '');
    let bits = 0, value = 0;
    const output: number[] = [];
    for (const char of s) {
        const idx = alphabet.indexOf(char);
        if (idx < 0) continue;
        value = (value << 5) | idx;
        bits += 5;
        if (bits >= 8) { output.push((value >>> (bits - 8)) & 0xff); bits -= 8; }
    }
    return Buffer.from(output);
}

function totpCode(secret: string, counter: number): string {
    const key = base32Decode(secret);
    const buf = Buffer.alloc(8);
    buf.writeBigInt64BE(BigInt(counter));
    const hmac = createHmac('sha1', key).update(buf).digest();
    const offset = hmac[hmac.length - 1] & 0x0f;
    const code = ((hmac[offset] & 0x7f) << 24 | hmac[offset+1] << 16 | hmac[offset+2] << 8 | hmac[offset+3]) % 1_000_000;
    return code.toString().padStart(6, '0');
}

export function verifyTotp(secret: string, token: string): boolean {
    if (!/^\d{6}$/.test(token)) return false;
    try {
        const counter = Math.floor(Date.now() / 1000 / 30);
        return [-1, 0, 1].some(drift => totpCode(secret, counter + drift) === token);
    } catch {
        return false;
    }
}
