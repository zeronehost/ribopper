import { existsSync } from "node:fs";
import { read } from "./read";
import { RUST_CARGE_FILE, PROJECT_PACKAGE_FILE, TAURI_CONFIG_FILE, logger } from "../utils";
import { readFile, writeFile } from "node:fs/promises";
import chalk from "chalk";

export const sync = async () => {
  const version = await read();
  logger.log(`当前版本号：${chalk.yellow.bold(version)}`);
  if (existsSync(PROJECT_PACKAGE_FILE)) {
    const content = await readFile(PROJECT_PACKAGE_FILE, "utf-8");
    const pkg = JSON.parse(content.trim());
    pkg.version = version;
    console.log(version);
    await writeFile(PROJECT_PACKAGE_FILE, JSON.stringify(pkg, null, 2));
  }
  if (existsSync(RUST_CARGE_FILE)) {
    const content = await readFile(RUST_CARGE_FILE, "utf-8");
    const config = content.replace(/^version = ".*"/m, `version = "${version}"`);
    await writeFile(RUST_CARGE_FILE, config);
  }

  if (existsSync(TAURI_CONFIG_FILE)) {
    const content = await readFile(TAURI_CONFIG_FILE, "utf-8");
    const conf = JSON.parse(content.trim());
    conf.version = version;
    if (conf.bundle?.macOS) {
      conf.bundle.macOS.bundleVersion = version;
    }
    await writeFile(TAURI_CONFIG_FILE, JSON.stringify(conf, null, 2));
  }
}