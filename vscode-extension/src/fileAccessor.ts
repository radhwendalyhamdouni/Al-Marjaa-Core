/**
 * أداة الوصول للملفات
 * File Accessor Utility
 */

import { workspace, Uri } from 'vscode';

export function basename(path: string): string {
    const parts = path.split(/[\\/]/);
    return parts[parts.length - 1] || '';
}

export interface FileAccessor {
    readFile(path: string): Promise<string>;
    writeFile(path: string, content: string): Promise<void>;
    fileExists(path: string): Promise<boolean>;
}

export const fileAccessor: FileAccessor = {
    async readFile(path: string): Promise<string> {
        const uri = Uri.file(path);
        const content = await workspace.fs.readFile(uri);
        return Buffer.from(content).toString('utf-8');
    },

    async writeFile(path: string, content: string): Promise<void> {
        const uri = Uri.file(path);
        await workspace.fs.writeFile(uri, Buffer.from(content, 'utf-8'));
    },

    async fileExists(path: string): Promise<boolean> {
        try {
            const uri = Uri.file(path);
            await workspace.fs.stat(uri);
            return true;
        } catch {
            return false;
        }
    }
};
