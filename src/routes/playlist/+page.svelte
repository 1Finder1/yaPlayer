<script lang="ts">
    import Track from "$lib/track.svelte"
    import type { PageData } from './$types';
    import {invoke} from "@tauri-apps/api/tauri";
    import {playPlaylist} from "@backend";

    export let data: PageData;

    let playlist = data.playlist

    let playTrack = (track_id: number) => {
        playPlaylist(playlist.uid, playlist.kind, track_id)
    }
</script>

{#if !playlist}
    <p>Нет текущего плейлиста</p>
{:else}
    <div class="flex flex-col gap-2">
        {#each playlist.tracks as track}
            <Track track={track.track} onPlay={playTrack}  />
        {/each}
    </div>
{/if}