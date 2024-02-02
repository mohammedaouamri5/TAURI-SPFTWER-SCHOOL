import { group } from "console";

export enum LOGlevel {
    Trace,
    Info,
    Warn,
    Error,
    Fatal,
}

interface TagDetails {
    enabled: boolean;
    levelFilter: LOGlevel;
}

export class LOG {
    private static logger: LOG;
    private static enabledTags: Map<string, TagDetails> = new Map();

    private constructor(private name: string, private level: LOGlevel) { }

    private static getTimestamp(): string {
        return new Date().toISOString();
    }

    private static getLogStyle(level: LOGlevel): string {
        switch (level) {
            case LOGlevel.Trace:
                return 'color: gray;';
            case LOGlevel.Info:
                return 'color: #4dff00;';
            case LOGlevel.Warn:
                return 'color: orange; font-weight: bold;';
            case LOGlevel.Error:
                return 'color: red; font-weight: bold;';
            case LOGlevel.Fatal:
                return 'color: white; background-color: red; font-weight: bold;';
            default:
                return '';
        }
    }

    private static log(level: LOGlevel, tag: string, ...args: any[]): void {
        const logString = tag ? `[${tag}]` : '';
        const logStyle = this.getLogStyle(level);
        console.log(`%c${this.getTimestamp()} [${LOGlevel[level]}] ${logString}\n`, logStyle, ...args);
    }

    public static init(): void {
        this.logger = new LOG('APP', LOGlevel.Trace);
    }

    public static shutdown(): void {
        console.log('Logging stopped');
    }

    public static setTagDetails(tag: string, enabled: boolean, levelFilter: LOGlevel): void {
        this.enabledTags.set(tag, { enabled, levelFilter });
    }

    public static message(level: LOGlevel, tag: string, ...args: any[]): void {
        const tagDetails = this.enabledTags.get(tag) || { enabled: true, levelFilter: LOGlevel.Trace };

        if (tagDetails.enabled && tagDetails.levelFilter <= level) {
            this.log(level, tag, ...args);
        }
    }

    public static startgroup(level: LOGlevel, tag: string , ...args: any[]): void {
 
        const tagDetails = this.enabledTags.get(tag) || { enabled: true, levelFilter: LOGlevel.Trace };
        if (tagDetails.enabled && tagDetails.levelFilter <= level) {
            const logString = tag ? `[${tag}]` : '';
            const logStyle = this.getLogStyle(level);
            console.groupCollapsed(`%c${this.getTimestamp()} [${LOGlevel[level]}] ${logString}`, logStyle  , ...args);
        }
    }

    public static trace(tag: string, ...args: any[]): void {
        this.message(LOGlevel.Trace, tag, ...args);
    }

    public static info(tag: string, ...args: any[]): void {
        this.message(LOGlevel.Info, tag, ...args);
    }

    public static startInfo(tag: string, ...args: any[]): void {
        this.startgroup(LOGlevel.Info, tag, ...args);
    }

    public static startTrace(tag: string, ...args: any[]): void {
        this.startgroup(LOGlevel.Trace, tag, ...args);
    }

    public static startError(tag: string, ...args: any[]): void {
        this.startgroup(LOGlevel.Error, tag, ...args);
    }

    public static startFatal(tag: string, ...args: any[]): void {
        this.startgroup(LOGlevel.Fatal, tag, ...args); 
    }

    public static startWarn(tag: string, ...args: any[]): void {
        this.startgroup(LOGlevel.Warn, tag, ...args);
    }

    public static end(): void {
        console.groupEnd();
    }

    public static warn(tag: string, ...args: any[]): void {
        this.message(LOGlevel.Warn, tag, ...args);
    }

    public static error(tag: string, ...args: any[]): void {
        this.message(LOGlevel.Error, tag, ...args);
    }

    public static fatal(tag: string, ...args: any[]): void {
        this.message(LOGlevel.Fatal, tag, ...args);
    }

    // Conditional log functions
    public static ifTrace(Fboolean:Fboolean, tag: string, ...args: any[]): void {
        if (Fboolean()) {
            this.trace(tag, ...args);
        }
    }

    public static ifInfo(Fboolean: Fboolean, tag: string, ...args: any[]): void {
        if (Fboolean()) {
            this.info(tag, ...args);
        }
    }

    public static ifWarn(Fboolean: Fboolean, tag: string, ...args: any[]): void {
        if (Fboolean()) {
            this.warn(tag, ...args);
        }
    }

    public static ifError(Fboolean:Fboolean, tag: string, ...args: any[]): void {
        if (Fboolean()) {
            this.error(tag, ...args);
        }
    }

    public static ifFatal(Fboolean:Fboolean , tag: string, ...args: any[]): void {
        if (Fboolean()) {
            this.fatal(tag, ...args);
        }
    }


}
type Fboolean = () => boolean;







