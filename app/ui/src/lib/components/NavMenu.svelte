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

<style>
 .navmenu {
     background-color: var(--oxi-primary-50);
     padding: var(--oxi-size-2xs);
     border: 1px solid var(--oxi-primary-300);
     border-radius: var(--oxi-rounded-2xl);
 }

 .menu-entry {
     display: flex;
     flex-direction: row;
     justify-content: left;
     align-items: center;
     gap: var(--oxi-size-sm);
 }

.overlay {
    position: fixed;
    inset: 0;
    z-index: 50;
    background-color: rgb(var(--oxi-black) / 0.5);
}

.menu {
    position: fixed;
    right: 0;
    top: 0;
    z-index: 50;
    min-height: 100vh;
    width: 100%;
    background-color: var(--oxi-white);
    padding: var(--oxi-size-xl);

    @media screen and (min-width: 768px) {
        max-width: 50vw;
    }

    display: flex;
    flex-direction: column;
    gap: var(--oxi-size-sm);
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
    gap: var(--oxi-size-2xs);

    & > * {
        width: 100%;
    }
}
</style>
