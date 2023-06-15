<script lang="ts">
import { onMount } from 'svelte'

// Bootstrap
import bootstrap from './bootstrap'

// Components
import Devices from './pages/Devices/index.svelte'
import Main from './pages/Main/index.svelte'

// Enums
import { Page, type BasicObject } from './types'

import './styles.css'

const pages = {
  [Page.Main]: Main,
  [Page.Devices]: Devices,
}

let page = Page.Main

onMount(() => {
  bootstrap.init()
})

$: getActivePage = () => pages[page]

const handlePageChange = (event: BasicObject) => (page = event.detail.page)
</script>

<main class="container mx-auto">
  <svelte:component
    this="{getActivePage()}"
    on:pagechange="{handlePageChange}"
  />
</main>
