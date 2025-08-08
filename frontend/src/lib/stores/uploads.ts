import { writable, type Writable } from "svelte/store";

export type FileUploadProgress = {
  file: string;
  progress: number;
};

export const uploadProgressStore: Writable<FileUploadProgress[]> = writable([]);
