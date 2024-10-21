<script lang="ts">
    import type {IArtist} from "@type";

    export let artists: IArtist[]

    const len = artists.length
</script>

<div class="flex">
    {#each artists as artist, index}
        <p class="text-gray-400 whitespace-pre-wrap">{artist.name}{index + 1 === artists.length ? '' : ', '}</p>
    {/each}
</div>