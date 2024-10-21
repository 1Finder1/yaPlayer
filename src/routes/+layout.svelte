<script lang="ts">
    import "../app.css";
    import MiniPlayer from '$lib/miniPlayer.svelte'
    import {page} from "$app/stores";
    import {onDestroy} from "svelte";
    import {subscribeEvents} from "@helpers";

    // setInterval(() => {
    //     // invoke<IQueueStatus>("get_queue_status").then(res => queue.set(res))
    //     getPlayerInfo().then(res => {
    //         console.log(res)
    //         playerStatus.set(res)
    //     })
    // }, 1000)

    const unsubscribeEvents = subscribeEvents();

    let path: string;
    $: path = $page.url.pathname

    onDestroy(() => {
        unsubscribeEvents()
    })
</script>


<nav>
    <a href="/rec" class={path === '/rec' ? "selected": ""}>Рекомендации</a>
    <a href="/" class={path === '/' ? "selected": ""}>Плеер</a>
    <a href="/playlists" class={path === '/playlists' ? "selected": ""}>Мои плейлисты</a>
</nav>

<main>
    <slot></slot>
</main>
{#if path !== '/'}
    <MiniPlayer/>
{/if}

<style>
    nav {
        display: flex;

        justify-content: center;
        @apply mb-4 gap-2
    }

    nav a {
        transition: .3s;
        @apply rounded-lg px-3 py-2
    }

    nav a.selected {
        @apply bg-gray-900
    }

    nav a:hover {
        @apply bg-gray-800
    }
</style>
