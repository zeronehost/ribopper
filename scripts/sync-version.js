
import fs from "node:fs";
import path from "node:path";

// 读取版本号
const version = fs.readFileSync(path.join(import.meta.dirname, '.version'), 'utf8').trim();
console.log(`Setting version to ${version}`);

// 更新 package.json
const packageJsonPath = path.join(import.meta.dirname, '..', 'package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
packageJson.version = version;
fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));

// 更新 Cargo.toml
const cargoTomlPath = path.join(import.meta.dirname, '..', 'Cargo.toml');
if (fs.existsSync(cargoTomlPath)) {
  let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
  cargoToml = cargoToml.replace(/^version = ".*"/m, `version = "${version}"`);
  fs.writeFileSync(cargoTomlPath, cargoToml);
}

// 更新 tauri.conf.json
const tauriConfPath = path.join(import.meta.dirname, '..', 'src-tauri', 'tauri.conf.json');
if (fs.existsSync(tauriConfPath)) {
  const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
  tauriConf.package.version = version;
  fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2));
}