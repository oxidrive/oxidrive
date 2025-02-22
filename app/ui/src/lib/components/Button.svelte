<script lang="ts" module>
export type Variant = "filled" | "outline" | "ghost";
</script>

<script lang="ts">
import type { Snippet } from "svelte";
import type { HTMLButtonAttributes } from "svelte/elements";

interface Props extends HTMLButtonAttributes {
	loading?: boolean;
     variant: Variant,
	children: Snippet;
}

const { loading = false, variant, children, ...props }: Props = $props();
</script>

<button class="button {variant}" {...props}>
    {#if loading}
        <span class="loader"></span>
    {:else}
        {@render children()}
    {/if}
</button>

<style lang="scss">
  @use "$lib/styles/oxi";

.loader {
  width: 24px;
  height: 24px;
  border: 2px solid var(--oxi-white);
  border-radius: 50%;
  display: inline-block;
  position: relative;
  box-sizing: border-box;
  animation: rotation 1s linear infinite;
}

.loader::after {
  content: '';
  box-sizing: border-box;
  position: absolute;
  left: 0;
  top: 0;
  background: var(--oxi-white);
  width: 10px;
  height: 10px;
  transform: translate(-50%, 50%);
  border-radius: 50%;
}

@keyframes rotation {
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}
</style>
