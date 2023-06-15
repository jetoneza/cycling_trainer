<script lang="ts">
import { createEventDispatcher } from 'svelte'

// Types
import { DispatchMessage, Page } from '../../types'

const dispatch = createEventDispatcher()

const handleDevicesClick = () => {
  dispatch(DispatchMessage.PageChange, {
    page: Page.Devices,
  })
}
</script>

<div class="main-page p-10">
  Main

  <button on:click="{handleDevicesClick}">Devices</button>
</div>
