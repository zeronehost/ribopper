import type { Action } from "@ribo/api";
import type { InjectionKey } from "vue";

export const optionContainerKey: InjectionKey<OptionContainer> = Symbol('OptionContainer');

export interface OptionContainer {
  selected: (action: Action) => void;
}