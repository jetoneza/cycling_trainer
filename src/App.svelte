<style>
.logo.rust {
  filter: drop-shadow(0 0 2em #ff3e00);
}
</style>

<script lang="ts">
import Bluetooth from './lib/Bluetooth.svelte'
</script>

<main class="container">
  <h1>Cycling Trainer!</h1>

  <div class="row">
    <a href="https://www.rust-lang.org/" target="_blank">
      <img src="/rust.svg" class="logo rust" alt="Rust Logo" />
    </a>
  </div>

  <div class="row">
    <Bluetooth />
  </div>
</main>
