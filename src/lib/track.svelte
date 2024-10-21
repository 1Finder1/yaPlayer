<script lang="ts">
    import Cover from "$lib/cover.svelte"
    import ArtistList from "$lib/artistList.svelte"
    import type {ITrack} from "@type";
    import {msToString} from "@helpers";
    import classNames from "classnames";

    export let track: ITrack
    export let onPlay: (id: number) => void;

    const onClick = () => {
        onPlay(track.id)
    }

    const blockClass = classNames('track', {"unavailable": !track.available})
</script>

<div class={blockClass} on:click={onClick}>
    <Cover cover={{ type: 'pic', uri: track.coverUri, itemsUri:[]}} size={50}/>
    <div class="flex flex-col">
        <p class="title text-lg">{track.title} {track.id}</p>
        <ArtistList artists={track.artists} />
    </div>
    <div>
        <p>{msToString(track.durationMs)}</p>
    </div>
</div>

<style>
    .track {
        position: relative;
        display: grid;
        grid-template-columns: 50px 1fr fit-content(200px);
        align-items: center;

        cursor: pointer;

        @apply gap-2 bg-gray-900 rounded-lg p-2
    }

    .track :global(.cover) {
        @apply rounded-lg
    }

    .title {
        transition: .3s;
    }

    .track.unavailable {
        cursor: not-allowed;
    }

    .track.unavailable::before {
        content: "";
        display: block;
        position: absolute;

        top: 0;
        left: 0;

        width: 100%;
        height: 100%;

        @apply bg-gray-100 opacity-50 rounded-lg
    }

    .track:not(.unavailable):hover .title {
        @apply text-yellow-500
    }

</style>