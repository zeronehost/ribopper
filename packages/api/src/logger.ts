// import { error, warn, info, debug, trace } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";

type Level = "error" | "warn" | "info" | "debug" | "trace";

class Logger {
  _print(level: Level, ...msg: unknown[]) {
    const str = msg.map(i => `${i}`).join(" ");
    invoke("web_log", {
      level,
      message: str
    });
  }
  error(error: Error) {
    console.error(error);
    if (typeof error === "string") {
      this._print("error", error);
    } else {
      this._print("error", error.message, error.cause, error.stack);
    }
  }
  
  warn(...message: any[]) {
    console.warn(...message);
    this._print("warn", ...message);
  }

  info(...message: any[]) {
    console.info(...message);
    this._print("info", ...message);
  }

  debug(...message: any[]) {
    console.debug(...message);
    this._print("debug", ...message);
  }

  trace(...message: any[]) {
    console.trace(...message);
    this._print("trace", ...message);
  }
}

export const logger = new Logger();