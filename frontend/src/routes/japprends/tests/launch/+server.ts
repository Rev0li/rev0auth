import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { testRuns } from '$lib/server/db/schema.js';
import { spawn } from 'child_process';
import { randomUUID } from 'crypto';

export const POST: RequestHandler = async ({ locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');

    const runId = randomUUID();
    const executedAt = Math.floor(Date.now() / 1000);

    const output = await new Promise<string>((resolve) => {
        const proc = spawn(
            process.env.CARGO_BIN ?? `${process.env.HOME}/.cargo/bin/cargo`,
            ['test', '-p', 'rev0auth-api', '--', '--format', 'json', '-Z', 'unstable-options'],
            { cwd: process.env.CARGO_MANIFEST_DIR ?? '/home/revoli/dev/rev0auth', timeout: 120_000 }
        );
        let buf = '';
        proc.stdout.on('data', (d: Buffer) => (buf += d.toString()));
        proc.stderr.on('data', (d: Buffer) => (buf += d.toString()));
        proc.on('close', () => resolve(buf));
    });

    // Parse test results from cargo output lines
    const cases: { name: string; ok: boolean }[] = [];
    for (const line of output.split('\n')) {
        if (line.startsWith('test ') && (line.includes('... ok') || line.includes('... FAILED'))) {
            const ok   = line.includes('... ok');
            const name = line.replace(/^test /, '').replace(/ \.\.\. (ok|FAILED)$/, '').trim();
            cases.push({ name, ok });
        }
    }

    const passed = cases.filter(c => c.ok).length;
    const total  = cases.length;

    await db.insert(testRuns).values({
        runId,
        executedAt,
        passed,
        total,
        cases: JSON.stringify(cases),
    });

    return json({ ok: true, runId, passed, total, cases });
};
