<script lang="ts">
import { onMount } from 'svelte'

// Bootstrap
import bootstrap from './bootstrap'

// Components
import Devices from './pages/Devices/index.svelte'
import Main from './pages/Main/index.svelte'
import Activities from './pages/Activities/index.svelte'
import Workout from './pages/Workout/index.svelte'
import Navigation from './components/Navigation/index.svelte'

// Enums
import { Page, type BasicObject } from './types'

import './styles.css'

const pages = {
  [Page.Main]: Main,
  [Page.Activities]: Activities,
  [Page.Devices]: Devices,
  [Page.Workout]: Workout,
}

let page = Page.Main

onMount(async () => {
  await bootstrap.init()
})

$: getActivePage = () => pages[page]

const handlePageChange = (event: BasicObject) => (page = event.detail.page)
</script>

<main class="container mx-auto">
  {#if page !== Page.Workout}
    <Navigation page="{page}" on:pagechange="{handlePageChange}" />
  {/if}

  <svelte:component
    this="{getActivePage()}"
    on:pagechange="{handlePageChange}"
  />
</main>
