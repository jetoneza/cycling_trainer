<style global lang="postcss">
@tailwind utilities;
@tailwind components;
@tailwind base;

:root {
  @apply font-mono antialiased;
  @apply text-secondary;
}
</style>

<script lang="ts">
import PairedDevices from './pages/PairedDevices.svelte'
</script>

<main class="container mx-auto">
  <PairedDevices />
</main>
