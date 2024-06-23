<script lang="ts">
import Navbar from "$lib/components/Navbar.svelte";
</script>

<div class="app">
    <header>
        <Navbar />
    </header>
    <main class="main">
        <slot />
    </main>
</div>

<style>
    .app {
        background-color: var(--oxi-color-primary-500);
        width: 100%;
        min-height: 100dvh;
    }

 .main {
     background-color: var(--oxi-color-white);
     border-top-left-radius: var(--oxi-size-4);
     border-top-right-radius: var(--oxi-size-4);
     min-height: calc(100dvh - var(--oxi-navbar-height));
 }
</style>
