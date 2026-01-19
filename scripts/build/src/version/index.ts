import { logger } from "../utils"
import { read } from "./read"
import { update as updateFn } from "./update";
import { sync as syncFn } from "./sync";

export const versionAction = async ({
  update,
  sync
}: { update: boolean, sync: boolean }) => {
  if (update && !sync) {
    // update
    await updateFn()
  } else if (!update && sync) {
    // sync
    await syncFn();
  } else if (!update && !sync) {
    // query
    const version = await read();
    logger.log(version);
  } else {
    throw new Error("Invalid arguments")
  }
}

