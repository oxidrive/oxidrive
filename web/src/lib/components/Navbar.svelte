<script lang="ts">
import logo from "$lib/assets/logo-white-with-name.svg";
</script>

<nav class="navbar">
    <div class="navbar-left">
        <a href="/"><img src={logo} alt="Oxidrive" class="logo" /></a>
    </div>

    <div class="navbar-right">
        <i class="fa-solid fa-magnifying-glass text-white text-2xl"></i>
        <i class="fa-solid fa-bell text-white text-2xl"></i>
        <i class="fa-regular fa-circle-user text-white text-2xl"></i>
    </div>
</nav>

<style lang="scss">
    :root {
        --oxi-navbar-height: 72px;
    }

    .navbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0 var(--oxi-size-4);
        height: var(--oxi-navbar-height);

        .navbar-left {
            display: flex;
            justify-content: start;
            flex-grow: 1;
        }

        .navbar-right {
            display: flex;
            justify-content: space-around;
            flex-grow: 1;
        }

        .logo {
            height: 50px;
            width: 50px;
        }
    }
</style>
