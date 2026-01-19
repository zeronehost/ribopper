import { Command } from "commander";
import {name, version} from "../package.json";
import { createAction } from "./utils";
import { versionAction } from "./version";

export default (args: string[]) => {
  const cmd = new Command(name);

  cmd
    .version(`${name} V${version}`, "-v, --version", "Print the version number")
    .helpOption("-h, --help", "Display help for command")
    .usage("<command> [options]");

  cmd
    .command("version")
    .description("Version management")
    .alias("v")
    .option("-u, --update", "Update the version")
    .option("-s, --sync", "Sync the version")
    .action(createAction(versionAction))
  
  cmd.parse(args);
}