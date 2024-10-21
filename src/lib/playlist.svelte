<script lang="ts">
    import type {IPlaylist} from "@type";
    import Cover from "$lib/cover.svelte"
    import {goto} from "$app/navigation";
    import {page} from "$app/stores";

    export let playlist: IPlaylist;

    const onClick = () => {
        const url = new URL($page.url);



        url.searchParams.set("id", playlist.kind + "")
        url.searchParams.set("owner", playlist.uid + "")

        goto(`/playlist${url.search}`)
    }
    
</script>

<div class="playlist">
    <Cover cover={playlist.cover} />
    <div class="playlist-title" on:click={onClick}>
        <p tabindex="0">{playlist.title}</p>
    </div>
</div>

<style>
    .playlist {
        position: relative;
        width: 200px;
        height: 200px;

        overflow: hidden;

        @apply rounded-lg
    }

    .playlist :global(.cover) {
        width: 200px;
        height: 200px;
    }

    .playlist-title {
        display: flex;
        flex-direction: column-reverse;
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;

        background: linear-gradient(to top, rgba(0, 0, 0, .6) 15%, rgba(0, 0, 0, .1) 60%);

        transition: .3s;

        cursor: pointer;

        @apply p-2 text-xl hover:text-yellow-500;
    }
</style>

