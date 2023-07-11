<script lang="ts">
import { createEventDispatcher } from 'svelte'
import { DispatchMessage, Page } from '../.././types'

import './styles.css'

const dispatch = createEventDispatcher()

export let page: Page

$: items = [
  {
    name: 'Home',
    isActive: page === Page.Main,
    page: Page.Main,
  },
  {
    name: 'Activities',
    isActive: page === Page.Activities,
    page: Page.Activities,
  },
  {
    name: 'Devices',
    isActive: page === Page.Devices,
    page: Page.Devices,
  },
]

const handlePageChange = (page: Page) => {
  dispatch(DispatchMessage.PageChange, {
    page,
  })
}
</script>

<div class="navigation">
  <ul>
    {#each items as item}
      <li class="{item.isActive ? 'active' : ''} relative">
        <button on:click="{() => handlePageChange(item.page)}">
          {item.name}
          <span class="indicator"></span>
        </button>
      </li>
    {/each}
  </ul>
</div>
