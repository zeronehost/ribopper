import { readFileSync } from "node:fs";
import { join } from "node:path";

const version = readFileSync(join(import.meta.dirname, '.version'), 'utf8').trim();
console.log(version);