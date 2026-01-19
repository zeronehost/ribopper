import { platform } from "node:os";
import { posix, resolve } from "node:path";

/**
 * Normalize path to posix style
 * @param path
 * @returns
 */
export const normalizePath = (path: string): string => posix.normalize(platform() === "win32" ? path.replace(/\\/g, "/") : path);

/**
 * @constant PROJECT_ROOT path to project root
 */
export const PROJECT_ROOT = resolve(import.meta.dirname, "..", "..", "..");
export const SUB_APP_ROOT = resolve(PROJECT_ROOT, "packages", "app");
export const SUB_API_ROOT = resolve(PROJECT_ROOT, "packages", "api");

export const PROJECT_PACKAGE_FILE = resolve(PROJECT_ROOT, "package.json");
export const SUB_APP_PACKAGE_FILE = resolve(SUB_APP_ROOT, "package.json");
export const SUB_API_PACKAGE_FILE = resolve(SUB_API_ROOT, "package.json");
export const RUST_CARGE_FILE = resolve(PROJECT_ROOT, "Cargo.toml");
export const TAURI_CONFIG_FILE = resolve(PROJECT_ROOT, "crates", "app", "tauri.conf.json");

export const SCRIPTS_ROOT = resolve(PROJECT_ROOT, "scripts");
export const SCRIPTS_BUILD_ROOT = resolve(SCRIPTS_ROOT, "build");
export const SCRIPTS_VERSION_PATH = resolve(SCRIPTS_BUILD_ROOT, ".version");