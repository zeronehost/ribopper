import {} from "semver";
import { existsSync } from "node:fs";
import { SCRIPTS_VERSION_PATH } from "../utils";
import { readFile, writeFile } from "node:fs/promises";
import { version } from "../../package.json"

export const read = async (): Promise<string> => {
  if (existsSync(SCRIPTS_VERSION_PATH)) {
    const version = await readFile(SCRIPTS_VERSION_PATH, "utf-8");
    
    return version.trim();
  }
  return version;
}

export const write = async (version: string) => {
  await writeFile(SCRIPTS_VERSION_PATH, version);
}