<script lang="ts">
    import type {ICover} from "@type";
    import {getImgUrl} from "@helpers";

    export let cover: ICover | undefined
    export let size: number = 100
</script>

<div class="cover overflow-hidden">
    {#if !cover}
        <img src="https://music.yandex.ru/blocks/entity-cover/img/track_no-cover.svg" alt="">
    {:else if cover.type === 'mosaic'}
        <div class="grid grid-cols-2 h-full w-full">
            {#each cover.itemsUri as src}
                <img src={getImgUrl(src, size)} alt="">
            {/each}
        </div>
    {:else }
        <img src={getImgUrl(cover.uri, 200)} alt="">
    {/if}
</div>