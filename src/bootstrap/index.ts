import listeners from './listeners'
import settings from './settings'

const init = async () => {
  await settings.load()
  listeners.setup()
}

export default {
  init,
}
