// Libraries
import { invoke } from '@tauri-apps/api'
import { appUserStore } from '../stores/appUser'

// Types
import type { AppUser } from 'src/types'

const load = async () => {
  const userSettings = await invoke('get_app_user')

  appUserStore.set(userSettings as AppUser)
}

export default {
  load,
}
