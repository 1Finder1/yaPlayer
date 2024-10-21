import {writable} from "svelte/store";
import type {IPlayerStatus, IQueueStatus} from "@type";

export const queue = writable<IQueueStatus>()

export const playerStatus = writable<IPlayerStatus>();