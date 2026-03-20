import * as assert from 'assert';
import * as vscode from 'vscode';

suite('اختبار الإضافة', () => {
    test('الإضافة مفعلة', async () => {
        const extension = vscode.extensions.getExtension('al-marjaa.al-marjaa-language');
        assert.ok(extension);
    });

    test('اللغة مسجلة', async () => {
        const doc = await vscode.workspace.openTextDocument({
            content: 'اطبع("مرحباً")؛',
            language: 'marjaa'
        });

        assert.strictEqual(doc.languageId, 'marjaa');
    });

    test('الأوامر مسجلة', async () => {
        const commands = await vscode.commands.getCommands(true);

        assert.ok(commands.includes('marjaa.run'));
        assert.ok(commands.includes('marjaa.debug'));
        assert.ok(commands.includes('marjaa.startServer'));
    });

    test('الإعدادات موجودة', () => {
        const config = vscode.workspace.getConfiguration('marjaa');

        assert.ok(config.has('executablePath'));
        assert.ok(config.has('enableLanguageServer'));
        assert.ok(config.has('format.enable'));
    });
});
