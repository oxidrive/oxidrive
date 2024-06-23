<div class="fullscreen">
    <main class="box">
        <slot />
    </main>
</div>

<style>
    .fullscreen {
        min-height: 100vh;
        width: 100%;
        background-color: var(--oxi-color-primary-500);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        padding: var(--oxi-size-12);

        @media screen and (min-width: 768px) {
            padding: 0;
        }
    }

    .box {
        background-color: var(--oxi-color-primary-100);
        border-radius: var(--oxi-rounded-3xl);
        display: flex;
        flex-direction: column;
        justify-content: space-evenly;
        align-items: center;
        gap: var(--oxi-size-8);
        padding: var(--oxi-size-16) var(--oxi-size-4);
        width: 100%;

        @media screen and (min-width: 768px) {
            width: 50%;
        }

        @media screen and (min-width: 1024px) {
            width: 33%;
        }
    }
</style>
