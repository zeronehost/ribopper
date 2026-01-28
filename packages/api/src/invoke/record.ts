import { invoke } from "@tauri-apps/api/core";
import { Record, RecordQuery, UpdateRecord } from "@/models";
import { CLEAR_RECORD, DELETE_RECORD, GET_RECORD, GET_RECORDS, COPY_RECORD, UPDATE_RECORD, QRCODE_RECORD, SHOW_RECORD_ACTIONS } from "./constants";

export const getRecords = async (query: RecordQuery = {}) => await invoke<Record[]>(GET_RECORDS, { query });
export const getRecord = async (id: number) => await invoke<Record>(GET_RECORD, { id });

export const updateRecord = async (record: UpdateRecord) => await invoke<boolean>(UPDATE_RECORD, { record });

export const deleteRecord = async (id: number) => await invoke<boolean>(DELETE_RECORD, { id });
export const clearRecord = async () => await invoke<void>(CLEAR_RECORD);

export const copyRecord = async (id: number) => await invoke<boolean>(COPY_RECORD, { id });

export const qrcodeRecord = async (id: number) => {
  const data = await invoke<string>(QRCODE_RECORD, { id });
  const arr = Uint8Array.from(data);
  return new Blob([arr], { type: "image/png" });
};

export const showRecordActions = async (recordId: number, label: string) => await invoke<void>(SHOW_RECORD_ACTIONS, {id: recordId, label});