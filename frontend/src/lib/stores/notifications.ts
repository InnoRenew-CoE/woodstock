import { get, writable, type Writable } from "svelte/store";

export const notificationsStore: Writable<Notification[]> = writable([]);

export function pushNotification(notification: Notification) {
  notificationsStore.update((data) => [...data, notification]);

  const index = get(notificationsStore).indexOf(notification);
  setTimeout(() => {
    notificationsStore.update((data) => data.filter((_, i) => i !== index));
  }, 3000);
}

export type Notification = {
  class?: string;
  title: string;
  body: string;
};
