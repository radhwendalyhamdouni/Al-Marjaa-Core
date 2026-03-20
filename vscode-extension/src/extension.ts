/**
 * إضافة VS Code للغة المرجع البرمجية العربية
 * Al-Marjaa Arabic Programming Language VS Code Extension
 */

import * as path from 'path';
import { workspace, ExtensionContext, commands, window, StatusBarAlignment, StatusBarItem, Uri, Terminal, debug, DebugConfiguration, ProviderResult, CancellationToken, DebugConfigurationProvider, WorkspaceFolder, DebugAdapterDescriptorFactory, DebugAdapterExecutable, DebugAdapterDescriptor, DebugSession, DebugSessionCustomEvent } from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';

let client: LanguageClient | undefined;
let statusBarItem: StatusBarItem;
let terminal: Terminal | undefined;

export function activate(context: ExtensionContext) {
    console.log('تم تفعيل إضافة المرجع');

    // إنشاء شريط الحالة
    statusBarItem = window.createStatusBarItem(StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(check) المرجع';
    statusBarItem.tooltip = 'لغة المرجع البرمجية العربية';
    statusBarItem.command = 'marjaa.showOutput';
    statusBarItem.show();

    // تسجيل الأوامر
    registerCommands(context);

    // بدء خادم اللغة إذا كان مفعلاً
    const config = workspace.getConfiguration('marjaa');
    if (config.get<boolean>('enableLanguageServer', true)) {
        startLanguageServer(context);
    }

    // تسجيل مزود تكوين التصحيح
    const debugProvider = new MarjaaDebugConfigurationProvider();
    context.subscriptions.push(debug.registerDebugConfigurationProvider('marjaa', debugProvider));

    // تسجيل مصنع محول التصحيح
    const factory = new MarjaaDebugAdapterDescriptorFactory();
    context.subscriptions.push(debug.registerDebugAdapterDescriptorFactory('marjaa', factory));
}

function registerCommands(context: ExtensionContext) {
    // تشغيل البرنامج
    context.subscriptions.push(
        commands.registerCommand('marjaa.run', () => runMarjaaFile())
    );

    // تصحيح البرنامج
    context.subscriptions.push(
        commands.registerCommand('marjaa.debug', () => debugMarjaaFile())
    );

    // بدء خادم اللغة
    context.subscriptions.push(
        commands.registerCommand('marjaa.startServer', () => startLanguageServer(context))
    );

    // إيقاف خادم اللغة
    context.subscriptions.push(
        commands.registerCommand('marjaa.stopServer', async () => {
            if (client) {
                await client.stop();
                client = undefined;
                window.showInformationMessage('تم إيقاف خادم اللغة');
            }
        })
    );

    // إعادة تشغيل خادم اللغة
    context.subscriptions.push(
        commands.registerCommand('marjaa.restartServer', async () => {
            if (client) {
                await client.stop();
            }
            startLanguageServer(context);
            window.showInformationMessage('تم إعادة تشغيل خادم اللغة');
        })
    );

    // تجميع البرنامج
    context.subscriptions.push(
        commands.registerCommand('marjaa.compile', () => compileMarjaaFile())
    );

    // تنسيق الملف
    context.subscriptions.push(
        commands.registerCommand('marjaa.format', () => formatMarjaaFile())
    );

    // إظهار المخرجات
    context.subscriptions.push(
        commands.registerCommand('marjaa.showOutput', () => {
            if (client) {
                client.outputChannel.show();
            }
        })
    );
}

function getExecutablePath(): string {
    const config = workspace.getConfiguration('marjaa');
    return config.get<string>('executablePath', 'almarjaa');
}

function getTerminal(): Terminal {
    if (!terminal || terminal.exitStatus !== undefined) {
        terminal = window.createTerminal('المرجع');
    }
    return terminal;
}

async function runMarjaaFile() {
    const editor = window.activeTextEditor;
    if (!editor) {
        window.showErrorMessage('لا يوجد ملف مفتوح');
        return;
    }

    const document = editor.document;
    if (document.languageId !== 'marjaa') {
        window.showErrorMessage('الملف الحالي ليس ملف مرجع');
        return;
    }

    const filePath = document.uri.fsPath;
    const executable = getExecutablePath();

    const term = getTerminal();
    term.show();
    term.sendText(`"${executable}" run "${filePath}"`);
}

async function debugMarjaaFile() {
    const editor = window.activeTextEditor;
    if (!editor) {
        window.showErrorMessage('لا يوجد ملف مفتوح');
        return;
    }

    const document = editor.document;
    if (document.languageId !== 'marjaa') {
        window.showErrorMessage('الملف الحالي ليس ملف مرجع');
        return;
    }

    const config: DebugConfiguration = {
        type: 'marjaa',
        name: 'تصحيح المرجع',
        request: 'launch',
        program: document.uri.fsPath,
        stopOnEntry: true
    };

    await debug.startDebugging(workspace.workspaceFolders?.[0], config);
}

async function compileMarjaaFile() {
    const editor = window.activeTextEditor;
    if (!editor) {
        window.showErrorMessage('لا يوجد ملف مفتوح');
        return;
    }

    const document = editor.document;
    if (document.languageId !== 'marjaa') {
        window.showErrorMessage('الملف الحالي ليس ملف مرجع');
        return;
    }

    const filePath = document.uri.fsPath;
    const executable = getExecutablePath();

    const term = getTerminal();
    term.show();
    term.sendText(`"${executable}" compile "${filePath}"`);
}

async function formatMarjaaFile() {
    const editor = window.activeTextEditor;
    if (!editor) {
        return;
    }

    const document = editor.document;
    if (document.languageId !== 'marjaa') {
        return;
    }

    await commands.executeCommand('editor.action.formatDocument');
}

function startLanguageServer(context: ExtensionContext) {
    if (client) {
        return;
    }

    const config = workspace.getConfiguration('marjaa');
    const serverPath = config.get<string>('serverPath', '');

    // خيارات الخادم
    const serverOptions: ServerOptions = {
        run: {
            module: serverPath || context.asAbsolutePath(path.join('out', 'server.js')),
            transport: TransportKind.ipc
        },
        debug: {
            module: serverPath || context.asAbsolutePath(path.join('out', 'server.js')),
            transport: TransportKind.ipc,
            options: { execArgv: ['--nolazy', '--inspect=6009'] }
        }
    };

    // خيارات العميل
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'marjaa' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/*.mrj')
        },
        initializationOptions: {
            enableCompletion: config.get<boolean>('completion.enable', true),
            enableHover: config.get<boolean>('hover.enable', true),
            enableDiagnostics: config.get<boolean>('diagnostics.enable', true),
            enableSignatureHelp: config.get<boolean>('signatureHelp.enable', true)
        }
    };

    // إنشاء العميل وبدء الخادم
    client = new LanguageClient(
        'marjaaLanguageServer',
        'خادم لغة المرجع',
        serverOptions,
        clientOptions
    );

    client.start();
    window.showInformationMessage('بدأ خادم لغة المرجع');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

/**
 * مزود تكوين التصحيح
 */
class MarjaaDebugConfigurationProvider implements DebugConfigurationProvider {
    provideDebugConfigurations(folder: WorkspaceFolder | undefined): ProviderResult<DebugConfiguration[]> {
        return [
            {
                name: 'تشغيل البرنامج الحالي',
                type: 'marjaa',
                request: 'launch',
                program: '${file}',
                stopOnEntry: true
            },
            {
                name: 'تشغيل مع معاملات',
                type: 'marjaa',
                request: 'launch',
                program: '${file}',
                stopOnEntry: false,
                args: []
            }
        ];
    }

    resolveDebugConfiguration(folder: WorkspaceFolder | undefined, config: DebugConfiguration): ProviderResult<DebugConfiguration> {
        if (!config.type && !config.request && !config.name) {
            const editor = window.activeTextEditor;
            if (editor && editor.document.languageId === 'marjaa') {
                config.type = 'marjaa';
                config.name = 'تشغيل البرنامج';
                config.request = 'launch';
                config.program = '${file}';
                config.stopOnEntry = true;
            }
        }

        if (!config.program) {
            window.showErrorMessage('يجب تحديد ملف البرنامج');
            return undefined;
        }

        return config;
    }
}

/**
 * مصنع محول التصحيح
 */
class MarjaaDebugAdapterDescriptorFactory implements DebugAdapterDescriptorFactory {
    createDebugAdapterDescriptor(session: DebugSession, executable: DebugAdapterExecutable | undefined): ProviderResult<DebugAdapterDescriptor> {
        const config = workspace.getConfiguration('marjaa');
        const executablePath = config.get<string>('executablePath', 'almarjaa');

        return new DebugAdapterExecutable(executablePath, ['debug-adapter']);
    }
}
