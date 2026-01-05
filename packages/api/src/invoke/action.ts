import { invoke } from "@tauri-apps/api/core";
import { CREATE_ACTION, CREATE_ACTION_OPTION, CREATE_OPTION, DELETE_ACTION, DELETE_OPTION, EXEC_ACTION, GET_ACTION_BY_ID, GET_ACTIONS, GET_EXEC_ACTION, GET_OPTIONS_BY_ACTION_ID, UPDATE_ACTION, UPDATE_OPTION } from "./constants";
import type { Option, Action, NewAction, NewOption, UpdateAction, UpdateOption } from "@/models";

export const getActions = async () => await invoke<Action[]>(GET_ACTIONS);

export const getActionById = async (id: number) => await invoke<Action>(GET_ACTION_BY_ID, { id });

export const getOptionsByActionId = async (actionId: number) => await invoke<Option[]>(GET_OPTIONS_BY_ACTION_ID, { id: actionId });

export const createAction = async (action: NewAction) => await invoke<Action>(CREATE_ACTION, { action });
export const createActionOption = async (action: NewAction) => await invoke<Action>(CREATE_ACTION_OPTION, { action });

export const createOption = async (option: NewOption) => await invoke<Option>(CREATE_OPTION, { option });

export const updateAction = async (action: UpdateAction) => await invoke<boolean>(UPDATE_ACTION, { action });
export const updateOption = async (option: UpdateOption) => await invoke<boolean>(UPDATE_OPTION, { option });

export const deleteAction = async (actionId: number) => await invoke<boolean>(DELETE_ACTION, { id: actionId });
export const deleteOption = async (optionId: number) => await invoke<boolean>(DELETE_OPTION, { id: optionId });

export const getExecAction = async (recordId: number) => await invoke<Action[]>(GET_EXEC_ACTION, { id: recordId });
export const execAction = async (actionId: number) => await invoke<boolean>(EXEC_ACTION, { id: actionId });