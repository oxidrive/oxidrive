<script lang="ts">
import { faFileCircleQuestion } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";

// Go read FileIcon.svelte for the reason we need this torture instead of a nice object
const style = "height: 80px; width: 80px" as unknown as object;

export let text: string;
</script>

<div class="no-files">
    <FontAwesomeIcon
        icon={faFileCircleQuestion}
        {style}
        class="text-primary-200"
    />
    <p class="text-primary-400 text-lg">{text}</p>
</div>

<style>
    .no-files {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 100%;
        height: 100%;
        gap: var(--oxi-size-2);
    }
</style>
