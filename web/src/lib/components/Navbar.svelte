<script lang="ts">
import logo from "$lib/assets/images/logo-white-with-name.svg";
import type { SchemaAccountInfo } from "$lib/openapi";
import NavMenu from "./NavMenu.svelte";

interface Props {
	user: SchemaAccountInfo;
}

const { user }: Props = $props();
</script>

<nav class="navbar">
    <div class="nav-group">
        <a href="/ui"><img src={logo} alt="Oxidrive" class="logo" /></a>
    </div>

    <div class="nav-group">
        <NavMenu {user}/>
    </div>
</nav>

<style>
    .navbar {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        padding: var(--oxi-size-2xs) var(--oxi-size-m);
    }

    .logo {
        height: 50px;
        width: 50px;
    }

    .nav-group {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        gap: var(--oxi-size-sm);
    }
</style>
