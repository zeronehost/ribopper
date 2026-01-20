import chalk from "chalk";
import inquirer from "inquirer";
import { read, write } from "./read";
import { logger } from "../utils";
import semver from "semver";

// { value: "patch", name: `更新补丁版本号(0.0.${chalk.yellow("x")})` },
// { value: "minor", name: `更新次版本号(0.${chalk.yellow("x")}.0)` },
// { value: "major", name: `更新主版本号(${chalk.yellow("x")}.0.0)` },

export const update = async () => {
  const version = await read();
  logger.log(`当前版本号：${chalk.yellow.bold(version)}`);
  const { type, isOk } = await inquirer.prompt([
    {
      type: "rawlist",
      name: "type",
      message: "请选择需要修改的版本类型(x表示待修改的内容)",
      default: "patch",
      choices: [
        { value: "patch", name: `更新补丁版本号(0.0.${chalk.yellow("x")})` },
        { value: "minor", name: `更新次版本号(0.${chalk.yellow("x")}.0)` },
        { value: "major", name: `更新主版本号(${chalk.yellow("x")}.0.0)` },
      ]
    },
    {
      type: "confirm",
      name: "isOk",
      message: ({ type }) =>
        `确认将版本号 ${chalk.yellow(version)} -> ${chalk.yellow(semver.inc(version, type))}`,
      default: true,
    }
  ]);
  
  if (!isOk) return logger.warn("取消修改版本号");
  const newVersion = semver.inc(version, type);
  if (!newVersion) return logger.error("更新版本号失败");
  await write(newVersion);
  logger.success(`更新版本成功：${version} -> ${newVersion}`);
}
