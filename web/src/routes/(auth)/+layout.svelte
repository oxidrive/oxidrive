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

<style>
    :global(body) {
        background-color: var(--oxi-primary-500);
    }

    .main {
        display: flex;
        flex-direction: column;
        width: 100%;
        min-height: 100vh;
        justify-content: center;
        align-items: center;
        padding: var(--oxi-size-2xl);
    }

    .panel {
        display: flex;
        background-color: var(--oxi-primary-100);
        width: 100%;
        height: 100%;
        padding: var(--oxi-size-5xl) var(--oxi-size-lg);
        border-radius: var(--oxi-size-3xl);
        flex-direction: column;
        justify-content: space-evenly;
        align-items: center;
        gap: var(--oxi-size-xl);

        @media screen and (min-width: 768px) {
            width: 50%;
        }

        @media screen and (min-width: 1024px) {
            width: 33%;
        }
    }
</style>
