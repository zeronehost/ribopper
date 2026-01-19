import chalk from "chalk";
import stripAnsi from "strip-ansi";

const format = (tag: string, msg: string) =>
  msg
    .split("\n")
    .map((line, i) =>
      i === 0 ? `${tag} ${line}` : line.padStart(stripAnsi(tag).length),
    )
    .join("\n");

export const color = chalk;
export const logger = {
  info(msg = "", tag?: string) {
    this.log(format(chalk.blue(`[INFO] ${tag ? `${tag} ` : ""}`), msg));
  },
  success(msg = "", tag?: string) {
    this.log(format(chalk.green(`[SUCCESS] ${tag ? `${tag} ` : ""}`), msg));
  },
  warn(msg = "", tag?: string) {
    this.log(format(chalk.yellow(`[WARNING] ${tag ? `${tag} ` : ""}`), msg));
  },
  error(msg = "", tag?: string) {
    this.log(format(chalk.red(`[ERROR] ${tag ? `${tag} ` : ""}`), msg));
  },
  start(msg = "") {
    this.log(chalk.gray(msg));
  },
  end(msg = "") {
    this.log(chalk.green(msg));
  },
  log(msg = "") {
    console.log(msg);
  },
};