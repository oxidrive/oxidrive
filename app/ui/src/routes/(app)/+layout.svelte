<script lang="ts">
import Navbar from "$lib/components/Navbar.svelte";
import type { LayoutData } from "./$types";

interface Props {
	data: LayoutData;
	children?: import("svelte").Snippet;
}

let { data, children }: Props = $props();

const year = new Date().getFullYear();
</script>

<div class="app">
    <header class="header">
        <Navbar user={data.currentUser!} />
    </header>

    <main class="main">
        {@render children?.()}
    </main>

    <footer class="footer">
        <p>&copy; {year} Oxidrive Project</p>
    </footer>
</div>

<style lang="scss">
 @use "$lib/styles/oxi";

 :global(body) {
     background-color: oxi.primary(500);
 }

 .app {
     display: flex;
     flex-direction: column;
     align-items: center;
     min-height: 100vh;
 }

 .header {
     width: 100%;
 }

 .main {
     border-radius: oxi.rounded(2xl) oxi.rounded(2xl) 0 0;
     background-color: oxi.white();
     width: 100%;
     flex: 1;
 }

 .footer {
     display: flex;
     flex-direction: column;
     justify-content: space-between;
     align-items: center;
     background-color: oxi.white();
     width: 100%;
     padding: oxi.size(2xs) oxi.size(xs);
 }
</style>
