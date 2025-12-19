import { invoke } from "@tauri-apps/api/core";
import type { RiboRecordWithTargets, RiboRecord, RecordQuery, UpdateRecord } from "@/models";
import { CLEAR_RECORD, DELETE_RECORD, GET_RECORD, GET_RECORDS, COPY_RECORD, UPDATE_RECORD } from "./constants";

export const getRecords = async (query: RecordQuery = {}) => await invoke<RiboRecordWithTargets[]>(GET_RECORDS, { query });
export const getRecord = async (id: number) => await invoke<RiboRecord>(GET_RECORD, { id });

export const updateRecord = async (record: UpdateRecord) => await invoke<boolean>(UPDATE_RECORD, { record });

export const deleteRecord = async (id: number) => await invoke<boolean>(DELETE_RECORD, { id });
export const clearRecord = async () => await invoke<boolean>(CLEAR_RECORD);

export const copyRecord = async (id: number) => await invoke<boolean>(COPY_RECORD, { id });

export const qrcodeRecord = async (id: number, schema: string) => {
  
  // const channel = new Channel((res) => {
  //   console.log(res);
  // });

  // await invoke<string>(QRCODE_RECORD, { id, channel });
  // return []
  const res = await fetch(`${schema}localhost/qrcode?${id}`);
  if (res.status !== 200) {
    throw res.text()
  } else {
    return await res.blob()
  }
};