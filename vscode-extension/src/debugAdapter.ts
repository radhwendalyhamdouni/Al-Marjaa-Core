/**
 * محول التصحيح للغة المرجع
 * Debug Adapter for Al-Marjaa Language
 */

import {
    LoggingDebugSession,
    InitializedEvent,
    StoppedEvent,
    ContinuedEvent,
    OutputEvent,
    TerminatedEvent,
    Thread,
    StackFrame,
    Scope,
    Source,
    Breakpoint,
    BreakpointEvent,
    Variable
} from 'vscode-debugadapter';
import { DebugProtocol } from 'vscode-debugprotocol';
import { FileAccessor, basename } from './fileAccessor';
import { spawn, ChildProcess } from 'child_process';

interface MarjaaBreakpoint {
    id: number;
    line: number;
    verified: boolean;
}

interface MarjaaFrame {
    id: number;
    name: string;
    file: string;
    line: number;
    column: number;
}

interface MarjaaVariable {
    name: string;
    value: string;
    type: string;
    children?: MarjaaVariable[];
}

interface LaunchRequestArguments extends DebugProtocol.LaunchRequestArguments {
    program: string;
    stopOnEntry?: boolean;
    trace?: boolean;
    args?: string[];
}

export class MarjaaDebugSession extends LoggingDebugSession {
    private static threadID = 1;

    private _fileAccessor: FileAccessor;
    private _breakpoints: Map<string, MarjaaBreakpoint[]> = new Map();
    private _variableHandles = new class {
        private handles = new Map<number, MarjaaVariable>();
        private nextId = 1;

        add(variable: MarjaaVariable): number {
            const id = this.nextId++;
            this.handles.set(id, variable);
            return id;
        }

        get(id: number): MarjaaVariable | undefined {
            return this.handles.get(id);
        }

        clear(): void {
            this.handles.clear();
            this.nextId = 1;
        }
    }();

    private _debugProcess: ChildProcess | undefined;
    private _currentLine = 1;
    private _currentFile = '';
    private _isRunning = false;
    private _stackFrames: MarjaaFrame[] = [];
    private _variables: MarjaaVariable[] = [];

    constructor(fileAccessor: FileAccessor) {
        super('marjaa-debug.txt');
        this._fileAccessor = fileAccessor;
        this.setDebuggerLinesStartAt1(false);
        this.setDebuggerColumnsStartAt1(false);
    }

    protected initializeRequest(
        response: DebugProtocol.InitializeResponse,
        args: DebugProtocol.InitializeRequestArguments
    ): void {
        response.body = response.body || {};

        response.body.supportsConfigurationDoneRequest = true;
        response.body.supportsStepBack = false;
        response.body.supportsRestartFrame = false;
        response.body.supportsGotoTargetsRequest = false;
        response.body.supportsStepInTargetsRequest = false;
        response.body.supportsCompletionsRequest = false;
        response.body.supportsModulesRequest = false;
        response.body.supportsExceptionOptions = false;
        response.body.supportsValueFormattingOptions = false;
        response.body.supportsExceptionInfoRequest = false;
        response.body.supportTerminateDebuggee = true;
        response.body.supportsDelayedStackTraceLoading = false;
        response.body.supportsLogPoints = true;
        response.body.supportsClipboardContext = true;

        response.body.supportsBreakpointLocationsRequest = false;
        response.body.supportsConditionalBreakpoints = true;
        response.body.supportsHitConditionalBreakpoints = true;
        response.body.supportsEvaluateForHovers = true;
        response.body.supportsSetVariable = true;
        response.body.supportsReadMemoryRequest = false;
        response.body.supportsDisassembleRequest = false;

        this.sendResponse(response);
        this.sendEvent(new InitializedEvent());
    }

    protected launchRequest(
        response: DebugProtocol.LaunchResponse,
        args: LaunchRequestArguments
    ): void {
        this._currentFile = args.program;
        this._currentLine = 1;
        this._isRunning = false;

        // بدء عملية التصحيح
        this.startDebugging(args);

        if (args.stopOnEntry) {
            this.sendResponse(response);
            this.sendEvent(new StoppedEvent('entry', MarjaaDebugSession.threadID));
        } else {
            this.sendResponse(response);
            this._isRunning = true;
        }
    }

    private startDebugging(args: LaunchRequestArguments): void {
        const programPath = args.program;

        this.sendEvent(new OutputEvent(`بدء تصحيح: ${programPath}\n`, 'console'));

        // هنا يمكن تشغيل المفسر في وضع التصحيح
        // هذا مثال مبسط - في التطبيق الحقيقي، يجب التواصل مع المفسر

        this._isRunning = true;
        this._stackFrames = [{
            id: 0,
            name: 'الدالة الرئيسية',
            file: programPath,
            line: 1,
            column: 1
        }];

        // محاكاة المتغيرات
        this._variables = [
            { name: 'المتغيرات المحلية', value: '', type: 'scope' }
        ];
    }

    protected disconnectRequest(
        response: DebugProtocol.DisconnectResponse,
        args: DebugProtocol.DisconnectArguments
    ): void {
        this._isRunning = false;
        if (this._debugProcess) {
            this._debugProcess.kill();
            this._debugProcess = undefined;
        }
        this.sendResponse(response);
    }

    protected setBreakPointsRequest(
        response: DebugProtocol.SetBreakpointsResponse,
        args: DebugProtocol.SetBreakpointsArguments
    ): void {
        const filePath = args.source.path || '';
        const clientBreakpoints = args.breakpoints || [];

        const bps: MarjaaBreakpoint[] = [];
        const existing = this._breakpoints.get(filePath) || [];

        clientBreakpoints.forEach((bp, index) => {
            const id = existing.length + index;
            const verified = true; // يمكن التحقق من السطر موجود
            bps.push({
                id,
                line: bp.line,
                verified
            });
        });

        this._breakpoints.set(filePath, bps);

        response.body = {
            breakpoints: bps.map(bp => ({
                id: bp.id,
                line: bp.line,
                verified: bp.verified
            }))
        };

        this.sendResponse(response);
    }

    protected configurationDoneRequest(
        response: DebugProtocol.ConfigurationDoneResponse,
        args: DebugProtocol.ConfigurationDoneArguments
    ): void {
        this.sendResponse(response);
    }

    protected threadsRequest(response: DebugProtocol.ThreadsResponse): void {
        response.body = {
            threads: [new Thread(MarjaaDebugSession.threadID, 'الخيط الرئيسي')]
        };
        this.sendResponse(response);
    }

    protected stackTraceRequest(
        response: DebugProtocol.StackTraceResponse,
        args: DebugProtocol.StackTraceArguments
    ): void {
        const frames: StackFrame[] = this._stackFrames.map((frame, index) => {
            return new StackFrame(
                index,
                frame.name,
                new Source(basename(frame.file), frame.file),
                frame.line,
                frame.column
            );
        });

        response.body = {
            stackFrames: frames,
            totalFrames: frames.length
        };

        this.sendResponse(response);
    }

    protected scopesRequest(
        response: DebugProtocol.ScopesResponse,
        args: DebugProtocol.ScopesArguments
    ): void {
        const scopes: Scope[] = [
            new Scope('محلي', this._variableHandles.add({ name: 'local', value: '', type: 'scope' }), false),
            new Scope('عام', this._variableHandles.add({ name: 'global', value: '', type: 'scope' }), true)
        ];

        response.body = {
            scopes
        };

        this.sendResponse(response);
    }

    protected variablesRequest(
        response: DebugProtocol.VariablesResponse,
        args: DebugProtocol.VariablesArguments
    ): void {
        const variables: Variable[] = [];

        // محاكاة المتغيرات
        const exampleVars: MarjaaVariable[] = [
            { name: 'س', value: '10', type: 'رقم' },
            { name: 'النص', value: '"مرحباً"', type: 'نص' },
            { name: 'القائمة', value: '[1, 2, 3]', type: 'قائمة' },
            { name: 'القاموس', value: '{"أ": 1}', type: 'قاموس' }
        ];

        exampleVars.forEach(v => {
            variables.push({
                name: v.name,
                value: v.value + ' (' + v.type + ')',
                variablesReference: v.children ? this._variableHandles.add(v) : 0
            });
        });

        response.body = {
            variables
        };

        this.sendResponse(response);
    }

    protected continueRequest(
        response: DebugProtocol.ContinueResponse,
        args: DebugProtocol.ContinueArguments
    ): void {
        this._isRunning = true;
        this.sendEvent(new ContinuedEvent(MarjaaDebugSession.threadID, true));
        this.sendResponse(response);
    }

    protected nextRequest(
        response: DebugProtocol.NextResponse,
        args: DebugProtocol.NextArguments
    ): void {
        // التنقل للسطر التالي
        this._currentLine++;
        if (this._stackFrames.length > 0) {
            this._stackFrames[0].line = this._currentLine;
        }
        this.sendResponse(response);
        this.sendEvent(new StoppedEvent('step', MarjaaDebugSession.threadID));
    }

    protected stepInRequest(
        response: DebugProtocol.StepInResponse,
        args: DebugProtocol.StepInArguments
    ): void {
        // الدخول في الدالة
        this.sendResponse(response);
        this.sendEvent(new StoppedEvent('step', MarjaaDebugSession.threadID));
    }

    protected stepOutRequest(
        response: DebugProtocol.StepOutResponse,
        args: DebugProtocol.StepOutArguments
    ): void {
        // الخروج من الدالة
        if (this._stackFrames.length > 1) {
            this._stackFrames.shift();
        }
        this.sendResponse(response);
        this.sendEvent(new StoppedEvent('step', MarjaaDebugSession.threadID));
    }

    protected pauseRequest(
        response: DebugProtocol.PauseResponse,
        args: DebugProtocol.PauseArguments
    ): void {
        this._isRunning = false;
        this.sendResponse(response);
        this.sendEvent(new StoppedEvent('pause', MarjaaDebugSession.threadID));
    }

    protected evaluateRequest(
        response: DebugProtocol.EvaluateResponse,
        args: DebugProtocol.EvaluateArguments
    ): void {
        // تقييم تعبير
        const expression = args.expression;
        let result = '';

        // محاكاة بسيطة للتقييم
        try {
            if (expression.startsWith('اطبع')) {
                result = '[طباعة]';
            } else if (/^\d+$/.test(expression)) {
                result = expression;
            } else if (/^".*"$/.test(expression)) {
                result = expression;
            } else {
                result = `[قيمة ${expression}]`;
            }
        } catch (e) {
            result = `خطأ: ${(e as Error).message}`;
        }

        response.body = {
            result,
            variablesReference: 0
        };

        this.sendResponse(response);
    }

    protected setVariableRequest(
        response: DebugProtocol.SetVariableResponse,
        args: DebugProtocol.SetVariableArguments
    ): void {
        // تغيير قيمة متغير
        const name = args.name;
        const value = args.value;

        this.sendEvent(new OutputEvent(`تم تغيير ${name} = ${value}\n`, 'console'));

        response.body = {
            value,
            type: 'متغير'
        };

        this.sendResponse(response);
    }

    protected sourceRequest(
        response: DebugProtocol.SourceResponse,
        args: DebugProtocol.SourceArguments
    ): void {
        // إرجاع محتوى المصدر
        const sourceRef = args.sourceReference;

        response.body = {
            content: '// المصدر غير متوفر',
            mimeType: 'text/x-marjaa'
        };

        this.sendResponse(response);
    }
}
