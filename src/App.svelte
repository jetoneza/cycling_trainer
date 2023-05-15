<style>
.logo.rust:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}
</style>

<script lang="ts">
import Greet from './lib/Greet.svelte'
</script>

<main class="container">
  <h1>Cycling Trainer!</h1>

  <div class="row">
    <a href="https://www.rust-lang.org/" target="_blank">
      <img src="/rust.svg" class="logo rust" alt="Rust Logo" />
    </a>
  </div>

  <div class="row">
    <Greet />
  </div>
</main>
