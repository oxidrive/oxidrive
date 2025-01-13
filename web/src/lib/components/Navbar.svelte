<script lang="ts">
import logo from "$lib/assets/images/logo-white-with-name.svg";
import type { SchemaAccountInfo } from "$lib/openapi";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import Popover from "./Popover.svelte";

interface Props {
	user: SchemaAccountInfo;
}

let { user }: Props = $props();
</script>

<nav class="navbar">
    <div class="nav-start">
        <a href="/ui"><img src={logo} alt="Oxidrive" class="logo" /></a>
    </div>

    <div class="nav-end">
        <div>
            <Popover label={user.username}>
                {#snippet button()}
                    <i class="fa-regular fa-circle-user text-white text-2xl"></i>
                {/snippet}

                <p class="username">{user.username}</p>

                <form  action="/auth/session/delete" method="post">
                    <button class="menu-entry" type="submit">
                        <i class="fa-solid fa-right-from-bracket text-primary-500"></i>
                        <Localized id="navbar-logout" />
                    </button>
                </form>
            </Popover>
        </div>
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

   .username {
       text-transform: capitalize;
       margin-bottom: var(--oxi-size-sm);
   }

 .menu-entry {
     display: flex;
     flex-direction: row;
     justify-content: space-between;
     align-items: center;
     gap: var(--oxi-size-sm);
 }

    @media screen and (min-width: 768px) {
        .navbar {
            padding: var(--oxi-size-m) var(--oxi-size-3xl);
        }

        .logo {
            height: 98px;
            width: 98px;
        }
    }
</style>
