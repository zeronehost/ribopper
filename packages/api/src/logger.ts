import { error, warn, info, debug, trace } from "@tauri-apps/plugin-log";


class Logger {
  _print(...msg: unknown[]) {
    const str = msg.map(i => `${i}`).join(" ");
    return `[web] ${str}`
  }
  error(...message: any[]) {
    console.error(message);
    error(this._print(...message));
  }
  
  warn(...message: any[]) {
    console.warn(message);
    warn(this._print(...message));
  }

  info(...message: any[]) {
    console.info(message);
    info(this._print(...message));
  }

  debug(...message: any[]) {
    console.debug(message);
    debug(this._print(...message));
  }

  trace(...message: any[]) {
    console.trace(message);
    trace(this._print(...message));
  }
}

export const logger = new Logger();