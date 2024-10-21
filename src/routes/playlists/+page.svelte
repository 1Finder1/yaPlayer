<script lang="ts">
    import type { PageData } from './$types';

    import Playlist from "$lib/playlist.svelte"
    import type {IPlaylist} from "@type";

    export let data: PageData;
    const playlists = data.playlists as IPlaylist[]

    console.log(playlists)
</script>

<div class="flex flex-wrap gap-4">
    {#each playlists as playlist}
        <Playlist playlist={playlist} />
    {/each}
</div>