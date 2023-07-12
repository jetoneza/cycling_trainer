import type { Activity } from 'src/types'
import { writable } from 'svelte/store'

export const activityStore = writable<Activity>(null)
