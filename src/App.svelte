<script lang="ts">
import { onMount } from 'svelte'

// Bootstrap
import bootstrap from './bootstrap'

// Components
import Devices from './pages/Devices/index.svelte'
import Main from './pages/Main/index.svelte'
import Workout from './pages/Workout/index.svelte'
import Navigation from './components/Navigation/index.svelte'

// Enums
import { Page, type BasicObject } from './types'

import './styles.css'

const pages = {
  [Page.Main]: Main,
  [Page.Activities]: Workout,
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
  {#if page !== Page.Activities}
    <Navigation page="{page}" on:pagechange="{handlePageChange}" />
  {/if}

  <svelte:component this="{getActivePage()}" />
</main>
