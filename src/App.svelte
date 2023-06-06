<style global lang="postcss">
@tailwind utilities;
@tailwind components;
@tailwind base;

:root {
  @apply font-mono antialiased;
  @apply text-secondary-200;
}
</style>

<script lang="ts">
import PairedDevices from './pages/PairedDevices/index.svelte'
</script>

<main class="container mx-auto">
  <PairedDevices />
</main>
