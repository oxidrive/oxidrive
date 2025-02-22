<script lang="ts">
import { getUser } from "$lib/auth";
import type { SchemaAccountInfo } from "$lib/openapi";
import { createDialog, createDropdownMenu, melt } from "@melt-ui/svelte";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import { fade, fly } from "svelte/transition";

interface Props {
	user: SchemaAccountInfo;
}

const { user }: Props = $props();

const {
	elements: { trigger, portalled, overlay, content, close },
	states: { open },
} = createDialog({ forceVisible: true });
</script>

<Localized id="navmenu-trigger">
    {#snippet children({ text })}
        <button use:melt={$trigger} title={text} aria-label={text}>
            <i class="fa-solid fa-bars text-white text-2xl"></i>
        </button>
    {/snippet}
</Localized>

{#if $open}
    <div use:melt={$portalled} class="">
        <div
            use:melt={$overlay}
            class="overlay"
            transition:fade={{ duration: 150 }}
        >
        </div>

        <nav
            use:melt={$content}
            class="menu"
            transition:fly={{
                           x: 450,
                           duration: 300,
                           opacity: 1,
                           }}
        >
            <header class="header">
                <p class="username">{user.username}</p>
                <Localized id="navmenu-close">
                    {#snippet children({ text })}
                        <button use:melt={$close} aria-label={text} title={text}>
                            <i class="fa-solid fa-xmark text-2xl"></i>
                        </button>
                    {/snippet}
                </Localized>
            </header>

            <section class="section">
                <a class="menu-entry" href="/ui/account">
                    <i class="fa-solid fa-circle-user text-primary-500"></i>
                    <Localized id="navmenu-account-settings" />
                </a>
            </section>

            <hr>

            <section class="section">
                    <a class="menu-entry" href="/ui/collections">
                        <i class="fa-solid fa-tags text-primary-500"></i>
                        <Localized id="navmenu-collections" />
                    </a>
                    <a class="menu-entry" href="/ui/files">
                        <i class="fa-solid fa-folder text-primary-500"></i>
                        <Localized id="navmenu-files" />
                    </a>
            </section>

            <hr>

            <section class="section">
                <form  action="/auth/session/delete" method="post">
                    <button class="menu-entry" type="submit">
                        <i class="fa-solid fa-right-from-bracket text-primary-500"></i>
                        <Localized id="navmenu-logout" />
                    </button>
                </form>
            </section>
        </nav>
</div>

{/if}

<style lang="scss">
 @use "$lib/styles/oxi";

 .navmenu {
     background-color: oxi.primary(50);
     padding: oxi.size(2xs);
     border: 1px solid oxi.primary(300);
     border-radius: oxi.rounded(2xl);
 }

 .menu-entry {
     display: flex;
     flex-direction: row;
     justify-content: left;
     align-items: center;
     gap: oxi.size(sm);
 }

.overlay {
    position: fixed;
    inset: 0;
    z-index: 50;
    background-color: rgb(oxi.black() / 50%);
}

 .menu {
     background-color: oxi.white();
     display: flex;
     flex-direction: column;
     gap: oxi.size(sm);
     min-height: 100vh;
     padding: oxi.size(xl);
     position: fixed;
     right: 0;
     top: 0;
     width: 100%;
     z-index: 50;

     @include oxi.mq(s) {
         max-width: 50vw;
     }
 }

 .username {
     text-transform: capitalize;
 }

 .header {
     display: flex;
     flex-direction: row;
     justify-content: space-between;
     align-items: center;
     width: 100%;
 }

.section {
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
    align-items: center;
    width: 100%;
    gap: oxi.size(2xs);

    & > * {
        width: 100%;
    }
}
</style>
