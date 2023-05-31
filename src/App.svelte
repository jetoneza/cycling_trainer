<script lang="ts">
import Splash from './components/Splash.svelte'
import Nav from './components/Nav.svelte'

// States
let appStarted = false

// Functions
function initApp() {
  // TODO: Add initializations here

  appStarted = true
}
</script>

<main class="container">
  {#if !appStarted}
    <Splash on:initApp="{initApp}" />
  {:else}
    <Nav />
  {/if}
</main>
