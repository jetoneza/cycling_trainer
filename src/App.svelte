<script lang="ts">
import { onMount } from 'svelte'

// Bootstrap
import bootstrap from './bootstrap'

// Components
import Devices from './pages/Devices/index.svelte'

// Enums
import { Page } from './types'

import './styles.css'

let page = Page.Main

onMount(() => {
  bootstrap.init()
})

const getActivePage = () => {
  const pages = {
    [Page.Main]: null,
    [Page.Devices]: Devices,
  }

  return pages[page]
}
</script>

<main class="container mx-auto">
  <svelte:component this="{getActivePage()}" />
</main>
