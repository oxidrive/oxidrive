<script lang="ts">
import "highlight.js/styles/atom-one-light.css";
import highlight from "highlight.js/lib/core";

interface Props {
	class?: string;
	filter: string;
}

const { filter, class: clazz }: Props = $props();

const hljs = highlight.newInstance();

hljs.registerLanguage("oxiql", () => ({
	keywords: ["*", "AND", "OR"],
	contains: [],
}));

const highlighted = $derived(
	hljs.highlight(filter, { language: "oxiql" }).value,
);
</script>

<pre class="filter-container {clazz}"><code class="filter">{@html highlighted}</code></pre>

<style lang="scss">
 @use "$lib/styles/oxi";

 .filter {
     background-color: oxi.primary(100);
     padding: oxi.size(2xs);
     border-radius: oxi.rounded(lg);
     display: block;
 }
</style>
