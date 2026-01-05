import type { RiboEvent } from "@ribo/api";
import type { InjectionKey } from "vue";

export const isEqual = (a: any, b: any): boolean => {
  if (a === b) return true;
  if (a == null || b == null) return false;
  if (Array.isArray(a) && Array.isArray(b)) {
    if (a.length !== b.length) return false;
    return a.every((item, index) => isEqual(item, b[index]));
  }
  if (typeof a === "object" && typeof b === "object") {
    const keysA = Object.keys(a);
    const keysB = Object.keys(b);
    if (keysA.length !== keysB.length) return false;
    return keysA.every((key) => Reflect.has(b, key) && isEqual(a[key], b[key]));
  }
  return false;
};

export const rootContextKey: InjectionKey<RootContext> = Symbol("ribopper-root");
export type RootContext = {
  register: <T>(cb: (event: RiboEvent<T>) => void) => void;
  unregister: <T>(cb: (event: RiboEvent<T>) => void) => void;
}