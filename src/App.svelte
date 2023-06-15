<script lang="ts">
import bootstrap from './bootstrap'
import PairedDevices from './pages/PairedDevices/index.svelte'

import './styles.css'

bootstrap.init()
</script>

<main class="container mx-auto">
  <PairedDevices />
</main>
