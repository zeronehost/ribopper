import { exit } from "node:process";
import { logger } from "./logger";

export const createAction = (
  fn: (option: any) => void,
) => async (option: any) => {
  try {
    // oxlint-disable-next-line await-thenable
    await fn(option);
  } catch (error) {
    if ((error as Error).name === 'ExitPromptError') {
      exit(1);
    }
    logger.error((error as Error).message);
    // oxlint-disable-next-line no-unused-expressions
    (error as Error).stack && logger.log((error as Error).stack as string);
    exit(1);
  }
}