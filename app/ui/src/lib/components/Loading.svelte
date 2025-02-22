<script>
import LoadingSpinner from "./LoadingSpinner.svelte";
</script>

<div class="loading">
    <LoadingSpinner />
</div>

<style lang="scss">
 @use "$lib/styles/oxi";

 .loading {
     position: absolute;
     z-index: 100;
     height: 100%;
     width: 100%;
     display: flex;
     flex-direction: column;
     justify-content: center;
     align-items: center;
     background-color: oxi.primary(500);
     top: 0;
 }
</style>
