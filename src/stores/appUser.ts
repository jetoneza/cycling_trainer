import type { AppUser } from 'src/types'
import { writable } from 'svelte/store'

export const appUserStore = writable<AppUser>(undefined)
