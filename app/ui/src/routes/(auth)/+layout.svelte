<script lang="ts">
interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
</script>

<main class="main">
    <div class="panel">
        {@render children?.()}
    </div>
</main>

<style lang="scss">
 @use "$lib/styles/oxi";

 :global(body) {
     background-color: oxi.primary(500);
 }

    .main {
        display: flex;
        flex-direction: column;
        width: 100%;
        min-height: 100vh;
        justify-content: center;
        align-items: center;
        padding: oxi.size(2xl);
    }

    .panel {
        display: flex;
        background-color: oxi.primary(100);
        max-width: 500px;
        width: 100%;
        height: 100%;
        padding: oxi.size(5xl) oxi.size(lg);
        border-radius: oxi.rounded(4xl);
        flex-direction: column;
        justify-content: space-evenly;
        align-items: center;
        gap: oxi.size(xl);

        @include oxi.mq(s) {
            width: 50%;
        }

        @include oxi.mq(l) {
            width: 33%;
        }
    }
</style>
