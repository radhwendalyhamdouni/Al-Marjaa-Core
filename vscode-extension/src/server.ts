/**
 * خادم لغة المرجع
 * Al-Marjaa Language Server
 */

import {
    createConnection,
    TextDocuments,
    ProposedFeatures,
    InitializeParams,
    DidChangeConfigurationNotification,
    CompletionItem,
    CompletionItemKind,
    TextDocumentPositionParams,
    TextDocumentSyncKind,
    InitializeResult,
    Hover,
    MarkupKind,
    Diagnostic,
    DiagnosticSeverity,
    Position,
    Range,
    SignatureHelp,
    SignatureInformation,
    ParameterInformation,
    DocumentFormattingParams,
    TextEdit,
    Definition,
    Location,
    ReferenceParams,
    DocumentSymbolParams,
    SymbolInformation,
    SymbolKind,
    WorkspaceEdit,
    RenameParams
} from 'vscode-languageserver/node';
import { TextDocument } from 'vscode-languageserver-textdocument';

// إنشاء الاتصال
const connection = createConnection(ProposedFeatures.all);

// إنشاء مدير المستندات
const documents: TextDocuments<TextDocument> = new TextDocuments(TextDocument);

let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;

// الكلمات المفتاحية العربية مع وصفها
const arabicKeywords: Map<string, { description: string; category: string }> = new Map([
    // الكلمات المفتاحية الأساسية
    ['دالة', { description: 'تعريف دالة جديدة', category: 'keyword' }],
    ['أرجع', { description: 'إرجاع قيمة من الدالة', category: 'keyword' }],
    ['ارجع', { description: 'إرجاع قيمة من الدالة', category: 'keyword' }],
    ['إذا', { description: 'جملة شرطية', category: 'keyword' }],
    ['اذا', { description: 'جملة شرطية', category: 'keyword' }],
    ['وإلا', { description: 'بديل في الجملة الشرطية', category: 'keyword' }],
    ['والا', { description: 'بديل في الجملة الشرطية', category: 'keyword' }],
    ['وإذا', { description: 'شرط إضافي', category: 'keyword' }],
    ['واذا', { description: 'شرط إضافي', category: 'keyword' }],
    ['طالما', { description: 'حلقة طالما الشرط صحيح', category: 'keyword' }],
    ['لكل', { description: 'حلقة للتكرار على عناصر', category: 'keyword' }],
    ['في', { description: 'للتكرار داخل مجموعة', category: 'keyword' }],
    ['توقف', { description: 'إيقاف الحلقة', category: 'keyword' }],
    ['أكمل', { description: 'الانتقال للتكرار التالي', category: 'keyword' }],
    ['اكمل', { description: 'الانتقال للتكرار التالي', category: 'keyword' }],
    ['متغير', { description: 'تعريف متغير جديد', category: 'keyword' }],
    ['م', { description: 'تعريف متغير (مختصر)', category: 'keyword' }],
    ['ثابت', { description: 'تعريف ثابت', category: 'keyword' }],
    ['ث', { description: 'تعريف ثابت (مختصر)', category: 'keyword' }],
    ['صح', { description: 'قيمة منطقية صحيحة', category: 'constant' }],
    ['خطأ', { description: 'قيمة منطقية خاطئة', category: 'constant' }],
    ['خطا', { description: 'قيمة منطقية خاطئة', category: 'constant' }],
    ['لا_شيء', { description: 'قيمة فارغة', category: 'constant' }],
    ['لاشيء', { description: 'قيمة فارغة', category: 'constant' }],

    // الفئات والكائنات
    ['صنف', { description: 'تعريف صنف (فئة) جديد', category: 'keyword' }],
    ['هذا', { description: 'الإشارة للكائن الحالي', category: 'keyword' }],
    ['جديد', { description: 'إنشاء كائن جديد', category: 'keyword' }],
    ['أصل', { description: 'الإشارة للفئة الأب', category: 'keyword' }],
    ['اصل', { description: 'الإشارة للفئة الأب', category: 'keyword' }],

    // الاستيراد والتصدير
    ['استورد', { description: 'استيراد وحدة', category: 'keyword' }],
    ['صدر', { description: 'تصدير عنصر', category: 'keyword' }],
    ['من', { description: 'تحديد مصدر الاستيراد', category: 'keyword' }],
    ['استخدم', { description: 'استخدام وحدة', category: 'keyword' }],
    ['بوصف', { description: 'تسمية مستعارة', category: 'keyword' }],
    ['كـ', { description: 'تسمية مستعارة', category: 'keyword' }],
    ['ك', { description: 'تسمية مستعارة', category: 'keyword' }],

    // معالجة الأخطاء
    ['حاول', { description: 'بدء كتلة معالجة الأخطاء', category: 'keyword' }],
    ['امسك', { description: 'التقاط الخطأ', category: 'keyword' }],
    ['أخيراً', { description: 'كتلة تنفذ دائماً', category: 'keyword' }],
    ['اخيراً', { description: 'كتلة تنفذ دائماً', category: 'keyword' }],
    ['أخيرا', { description: 'كتلة تنفذ دائماً', category: 'keyword' }],
    ['اخيرا', { description: 'كتلة تنفذ دائماً', category: 'keyword' }],
    ['ألقِ', { description: 'رمي خطأ', category: 'keyword' }],
    ['الق', { description: 'رمي خطأ', category: 'keyword' }],

    // البرمجة غير المتزامنة
    ['غير_متزامن', { description: 'دالة غير متزامنة', category: 'keyword' }],
    ['انتظر', { description: 'انتظار نتيجة غير متزامنة', category: 'keyword' }],
    ['أعطِ', { description: 'إرجاع قيمة في المولد', category: 'keyword' }],
    ['اعط', { description: 'إرجاع قيمة في المولد', category: 'keyword' }],

    // المطابقة النمطية
    ['طابق', { description: 'جملة المطابقة', category: 'keyword' }],
    ['حالة', { description: 'حالة في المطابقة', category: 'keyword' }],
    ['افتراضي', { description: 'الحالة الافتراضية', category: 'keyword' }],

    // الدوال المضمنة
    ['اطبع', { description: 'طباعة نص في الطرفية', category: 'function' }],
    ['طبع', { description: 'طباعة نص في الطرفية', category: 'function' }],
    ['أطبع', { description: 'طباعة نص في الطرفية', category: 'function' }],
    ['ادخل', { description: 'قراءة إدخال من المستخدم', category: 'function' }],
    ['دخل', { description: 'قراءة إدخال من المستخدم', category: 'function' }],
    ['نوع', { description: 'الحصول على نوع القيمة', category: 'function' }],
    ['طول', { description: 'الحصول على طول القيمة', category: 'function' }],
    ['احذف', { description: 'حذف عنصر', category: 'function' }],
    ['تأكد', { description: 'التحقق من شرط', category: 'function' }],
    ['تاكد', { description: 'التحقق من شرط', category: 'function' }],

    // ميزات متقدمة
    ['لامدا', { description: 'دالة مجهولة', category: 'keyword' }],
    ['دالة_صغيرة', { description: 'دالة مجهولة', category: 'keyword' }],
    ['وحدة', { description: 'تعريف وحدة', category: 'keyword' }],
    ['مدى', { description: 'نطاق أرقام', category: 'function' }],
    ['خطوة', { description: 'خطوة التكرار', category: 'keyword' }],
    ['مع', { description: 'مدير سياق', category: 'keyword' }],
    ['تعداد', { description: 'تعريف تعداد', category: 'keyword' }],
    ['بيانات', { description: 'صنف بيانات', category: 'keyword' }],
    ['واجهة', { description: 'تعريف واجهة', category: 'keyword' }],
    ['نوع_من', { description: 'التحقق من النوع', category: 'keyword' }],
    ['نوع_مثل', { description: 'التحقق من النوع', category: 'keyword' }],

    // التكرار
    ['كرر', { description: 'تكرار عدد محدد من المرات', category: 'keyword' }],
    ['مرة', { description: 'عدد مرات التكرار', category: 'keyword' }],
    ['مرات', { description: 'عدد مرات التكرار', category: 'keyword' }],

    // الذكاء الاصطناعي
    ['أونكس', { description: 'دعم ONNX للذكاء الاصطناعي', category: 'ai' }],
    ['نموذج', { description: 'نموذج ذكاء اصطناعي', category: 'ai' }],
    ['حمّل', { description: 'تحميل نموذج', category: 'ai' }],
    ['حمل', { description: 'تحميل نموذج', category: 'ai' }],
    ['احفظ', { description: 'حفظ نموذج', category: 'ai' }],
    ['استدل', { description: 'استدلال بالنموذج', category: 'ai' }],
    ['موتر', { description: 'موتر (Tensor)', category: 'ai' }],
    ['درّب', { description: 'تدريب النموذج', category: 'ai' }],
    ['درب', { description: 'تدريب النموذج', category: 'ai' }],
    ['توقع', { description: 'توقع باستخدام النموذج', category: 'ai' }],
    ['تنبؤ', { description: 'توقع باستخدام النموذج', category: 'ai' }]
]);

// الدوال المضمنة مع توقيعاتها
const builtInFunctions: Map<string, { signatures: string[]; description: string }> = new Map([
    ['اطبع', {
        signatures: ['اطبع(النص: نص)', 'اطبع(القيمة: أي)'],
        description: 'طباعة قيمة في الطرفية'
    }],
    ['ادخل', {
        signatures: ['ادخل(الرسالة: نص): نص'],
        description: 'قراءة إدخال من المستخدم'
    }],
    ['نوع', {
        signatures: ['نوع(القيمة: أي): نص'],
        description: 'الحصول على نوع القيمة'
    }],
    ['طول', {
        signatures: ['طول(النص: نص): رقم', 'طول(القائمة: قائمة): رقم'],
        description: 'الحصول على طول القيمة'
    }],
    ['مدى', {
        signatures: ['مدى(النهاية: رقم)', 'مدى(البداية: رقم، النهاية: رقم)', 'مدى(البداية: رقم، النهاية: رقم، الخطوة: رقم)'],
        description: 'إنشاء نطاق من الأرقام'
    }],
    ['تأكد', {
        signatures: ['تأكد(الشرط: منطقي)', 'تأكد(الشرط: منطقي، الرسالة: نص)'],
        description: 'التحقق من صحة شرط'
    }],
    ['قائمة', {
        signatures: ['قائمة()', 'قائمة(العناصر: ...)'],
        description: 'إنشاء قائمة جديدة'
    }],
    ['قاموس', {
        signatures: ['قاموس()', 'قاموس(الأزواج: ...)'],
        description: 'إنشاء قاموس جديد'
    }]
]);

connection.onInitialize((params: InitializeParams) => {
    const capabilities = params.capabilities;

    hasConfigurationCapability = !!capabilities.workspace?.configuration;
    hasWorkspaceFolderCapability = !!capabilities.workspace?.workspaceFolders;

    const result: InitializeResult = {
        capabilities: {
            textDocumentSync: TextDocumentSyncKind.Incremental,
            completionProvider: {
                resolveProvider: true,
                triggerCharacters: ['.', 'أ', 'ا', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'ه', 'و', 'ي']
            },
            hoverProvider: true,
            signatureHelpProvider: {
                triggerCharacters: ['(', '،', ',']
            },
            definitionProvider: true,
            referencesProvider: true,
            documentSymbolProvider: true,
            documentFormattingProvider: true,
            renameProvider: true
        }
    };

    return result;
});

connection.onInitialized(() => {
    if (hasConfigurationCapability) {
        connection.client.register(DidChangeConfigurationNotification.type, undefined);
    }
});

// التشخيصات
connection.onDidChangeWatchedFiles(() => {
    connection.console.log('تغيرت الملفات');
});

documents.onDidChangeContent(change => {
    validateDocument(change.document);
});

function validateDocument(textDocument: TextDocument): void {
    const diagnostics: Diagnostic[] = [];
    const text = textDocument.getText();
    const lines = text.split('\n');

    // فحص الأقواس غير المتوازنة
    let braces = 0;
    let brackets = 0;
    let parens = 0;

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        for (let j = 0; j < line.length; j++) {
            const char = line[j];
            if (char === '{') braces++;
            if (char === '}') braces--;
            if (char === '[') brackets++;
            if (char === ']') brackets--;
            if (char === '(') parens++;
            if (char === ')') parens--;

            // التحقق من عدم وجود فائض
            if (braces < 0 || brackets < 0 || parens < 0) {
                diagnostics.push({
                    severity: DiagnosticSeverity.Error,
                    range: {
                        start: { line: i, character: j },
                        end: { line: i, character: j + 1 }
                    },
                    message: 'قوس إغلاق غير متوافق',
                    source: 'المرجع'
                });
            }
        }
    }

    // التحقق من التوازن النهائي
    if (braces !== 0) {
        diagnostics.push({
            severity: DiagnosticSeverity.Error,
            range: {
                start: { line: lines.length - 1, character: 0 },
                end: { line: lines.length - 1, character: 1 }
            },
            message: braces > 0 ? 'أقواس فتح غير مغلقة: {' : 'أقواس إغلاق غير متوافقة: }',
            source: 'المرجع'
        });
    }

    // فحص النصوص غير المغلقة
    const stringPattern = /["'`](?:(?!\1)[^\\]|\\.)*\1?/g;
    let match;
    while ((match = stringPattern.exec(text)) !== null) {
        const matched = match[0];
        if (!matched.endsWith(matched[0])) {
            const startPos = textDocument.positionAt(match.index);
            diagnostics.push({
                severity: DiagnosticSeverity.Error,
                range: {
                    start: startPos,
                    end: { line: startPos.line, character: startPos.character + matched.length }
                },
                message: 'نص غير مغلق',
                source: 'المرجع'
            });
        }
    }

    // فحص الفواصل المنقوطة العربية
    const arabicSemicolonPattern = /؛\s*؛/g;
    while ((match = arabicSemicolonPattern.exec(text)) !== null) {
        const startPos = textDocument.positionAt(match.index);
        diagnostics.push({
            severity: DiagnosticSeverity.Warning,
            range: {
                start: startPos,
                end: { line: startPos.line, character: startPos.character + match[0].length }
            },
            message: 'فاصلة منقوطة مكررة',
            source: 'المرجع'
        });
    }

    connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}

// الإكمال التلقائي
connection.onCompletion((textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
    const document = documents.get(textDocumentPosition.textDocument.uri);
    if (!document) {
        return [];
    }

    const items: CompletionItem[] = [];

    // إضافة الكلمات المفتاحية
    arabicKeywords.forEach((info, keyword) => {
        items.push({
            label: keyword,
            kind: info.category === 'function' ? CompletionItemKind.Function :
                  info.category === 'constant' ? CompletionItemKind.Constant :
                  CompletionItemKind.Keyword,
            detail: info.description,
            documentation: info.description
        });
    });

    // إضافة الدوال المضمنة
    builtInFunctions.forEach((info, funcName) => {
        items.push({
            label: funcName,
            kind: CompletionItemKind.Function,
            detail: info.description,
            documentation: {
                kind: MarkupKind.Markdown,
                value: `**${funcName}**\n\n${info.description}\n\nالتوقيعات:\n${info.signatures.map(s => `- \`${s}\``).join('\n')}`
            }
        });
    });

    return items;
});

connection.onCompletionResolve((item: CompletionItem): CompletionItem => {
    return item;
});

// التحويم
connection.onHover((textDocumentPosition: TextDocumentPositionParams): Hover | undefined => {
    const document = documents.get(textDocumentPosition.textDocument.uri);
    if (!document) {
        return undefined;
    }

    const position = textDocumentPosition.position;
    const text = document.getText();
    const lines = text.split('\n');

    if (position.line >= lines.length) {
        return undefined;
    }

    const line = lines[position.line];

    // البحث عن الكلمة عند المؤشر
    const wordPattern = /[a-zA-Z_\u0600-\u06FF][a-zA-Z0-9_\u0600-\u06FF]*/g;
    let match;
    while ((match = wordPattern.exec(line)) !== null) {
        const start = match.index;
        const end = start + match[0].length;
        if (position.character >= start && position.character <= end) {
            const word = match[0];

            // البحث في الكلمات المفتاحية
            const keywordInfo = arabicKeywords.get(word);
            if (keywordInfo) {
                return {
                    contents: {
                        kind: MarkupKind.Markdown,
                        value: `**${word}**\n\n${keywordInfo.description}`
                    }
                };
            }

            // البحث في الدوال المضمنة
            const funcInfo = builtInFunctions.get(word);
            if (funcInfo) {
                return {
                    contents: {
                        kind: MarkupKind.Markdown,
                        value: `**${word}** (${funcInfo.description})\n\nالتوقيعات:\n${funcInfo.signatures.map(s => `- \`${s}\``).join('\n')}`
                    }
                };
            }
        }
    }

    return undefined;
});

// مساعدة التوقيع
connection.onSignatureHelp((textDocumentPosition: TextDocumentPositionParams): SignatureHelp | undefined => {
    const document = documents.get(textDocumentPosition.textDocument.uri);
    if (!document) {
        return undefined;
    }

    const text = document.getText();
    const position = textDocumentPosition.position;

    // البحث عن اسم الدالة قبل القوس
    const lineStart = text.lastIndexOf('\n', document.offsetAt(position)) + 1;
    const textBeforeCursor = text.substring(lineStart, document.offsetAt(position));

    const funcMatch = textBeforeCursor.match(/([a-zA-Z_\u0600-\u06FF][a-zA-Z0-9_\u0600-\u06FF]*)\s*\([^)]*$/);
    if (funcMatch) {
        const funcName = funcMatch[1];
        const funcInfo = builtInFunctions.get(funcName);
        if (funcInfo) {
            const signatures: SignatureInformation[] = funcInfo.signatures.map(sig => {
                const paramMatch = sig.match(/\(([^)]*)\)/);
                const params = paramMatch ? paramMatch[1].split('،').map(p => p.trim()).filter(p => p) : [];

                return {
                    label: sig,
                    documentation: funcInfo.description,
                    parameters: params.map(p => ({
                        label: p
                    }))
                };
            });

            return {
                signatures,
                activeSignature: 0,
                activeParameter: 0
            };
        }
    }

    return undefined;
});

// تنسيق المستند
connection.onDocumentFormatting((params: DocumentFormattingParams): TextEdit[] => {
    const document = documents.get(params.textDocument.uri);
    if (!document) {
        return [];
    }

    const text = document.getText();
    const lines = text.split('\n');
    const edits: TextEdit[] = [];

    // تنسيق بسيط: إزالة المسافات الزائدة
    lines.forEach((line: string, index: number) => {
        const trimmed = line.trimEnd();
        if (trimmed !== line) {
            edits.push({
                range: {
                    start: { line: index, character: 0 },
                    end: { line: index, character: line.length }
                },
                newText: trimmed
            });
        }
    });

    return edits;
});

// تعريف الرمز
connection.onDefinition((textDocumentPosition: TextDocumentPositionParams): Definition | undefined => {
    // TODO: تنفيذ البحث عن التعريفات
    return undefined;
});

// المراجع
connection.onReferences((params: ReferenceParams): Location[] | undefined => {
    // TODO: تنفيذ البحث عن المراجع
    return undefined;
});

// رموز المستند
connection.onDocumentSymbol((params: DocumentSymbolParams): SymbolInformation[] => {
    const document = documents.get(params.textDocument.uri);
    if (!document) {
        return [];
    }

    const text = document.getText();
    const symbols: SymbolInformation[] = [];

    // البحث عن تعريفات الدوال
    const funcPattern = /دالة\s+([a-zA-Z_\u0600-\u06FF][a-zA-Z0-9_\u0600-\u06FF]*)/g;
    let match;
    while ((match = funcPattern.exec(text)) !== null) {
        const pos = document.positionAt(match.index);
        symbols.push({
            name: match[1],
            kind: SymbolKind.Function,
            location: {
                uri: params.textDocument.uri,
                range: {
                    start: pos,
                    end: pos
                }
            }
        });
    }

    // البحث عن تعريفات المتغيرات
    const varPattern = /(متغير|م)\s+([a-zA-Z_\u0600-\u06FF][a-zA-Z0-9_\u0600-\u06FF]*)/g;
    while ((match = varPattern.exec(text)) !== null) {
        const pos = document.positionAt(match.index);
        symbols.push({
            name: match[2],
            kind: SymbolKind.Variable,
            location: {
                uri: params.textDocument.uri,
                range: {
                    start: pos,
                    end: pos
                }
            }
        });
    }

    // البحث عن تعريفات الأصناف
    const classPattern = /صنف\s+([a-zA-Z_\u0600-\u06FF][a-zA-Z0-9_\u0600-\u06FF]*)/g;
    while ((match = classPattern.exec(text)) !== null) {
        const pos = document.positionAt(match.index);
        symbols.push({
            name: match[1],
            kind: SymbolKind.Class,
            location: {
                uri: params.textDocument.uri,
                range: {
                    start: pos,
                    end: pos
                }
            }
        });
    }

    return symbols;
});

// إعادة التسمية
connection.onRenameRequest((params: RenameParams): WorkspaceEdit | undefined => {
    // TODO: تنفيذ إعادة التسمية
    return undefined;
});

// بدء الخادم
documents.listen(connection);
connection.listen();
