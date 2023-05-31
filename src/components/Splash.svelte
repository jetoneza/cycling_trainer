<style>
.logo.rust {
  filter: drop-shadow(0 0 2em #ff3e00);
}

.splash {
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}
</style>

<script lang="ts">
import { createEventDispatcher } from 'svelte'

const dispatch = createEventDispatcher()

// Functions
function initApp() {
  dispatch('initApp')
}
</script>

<div class="splash">
  <h1>Cycling Trainer!</h1>

  <div class="row">
    <a href="https://www.rust-lang.org/" target="_blank">
      <img src="/rust.svg" class="logo rust" alt="Rust Logo" />
    </a>
  </div>

  <div class="row">
    <button on:click="{initApp}">Let's Go!</button>
  </div>
</div>
