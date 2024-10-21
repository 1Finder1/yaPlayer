<script lang="ts">
    import ArtistList from "$lib/artistList.svelte"
    import Cover from "$lib/cover.svelte"
    import VolumeSelector from "$lib/volumeSelector.svelte"
    import {Button} from "$lib/components/ui/button";
    import {faBackward, faForward} from "@fortawesome/free-solid-svg-icons";
    import {playerStatus, queue} from "../stores/player";
    import type {IQueueStatus, ITrack} from "@type";
    import {onDestroy, onMount} from "svelte";
    import {getPlayerInfo, getQueueStatus, playerPlayPause, playerSeek, playerSetVolume} from "@backend";
    import {getPlayIcon} from "@helpers";
    import Fa from "svelte-fa"; // @ts-ignore

    let trackProgress: number,
        isPlaying: boolean,
        volume: number,
        queueStatus: IQueueStatus;


    const unsubscribe = queue.subscribe(v => queueStatus = v)

    const playerUnsubscribe = playerStatus.subscribe((v) => {
        console.log(v)
        if (!v) {
            trackProgress = 0
            isPlaying = false
            volume = 0
            return
        }
        trackProgress = v.position / v.total * 100
        isPlaying = v.is_playing
        volume = v.volume
    })

    const onSeek = (e: MouseEvent) => {
        if (!queueStatus.current_track?.durationMs) return

        const target = e.target as Element
        const rect = target.getBoundingClientRect();

        const x = e.screenX - rect.x
        const percent = x / rect.width
        const seconds = Math.round(percent * (queueStatus.current_track.durationMs / 1000))

        trackProgress = percent * 100

        playerSeek(seconds)
    }

    $: currentTrack = queueStatus?.current_track || null

    onDestroy(() => {
        unsubscribe()
        playerUnsubscribe()
    })

    // onMount(() => {
    //     getPlayerInfo().then(() => playerStatus.set(old => ({...old, total: old.total / 1000})))
    //     getQueueStatus().then(queue.set)
    // })
</script>

<div class="miniPlayer">
    <div class="progress" style="--progress: {trackProgress}%" on:click={onSeek}></div>
    <div class="miniPlayer-control">
        <div class="track-control">
            <Button size="icon" variant="ghost" disabled={!queueStatus?.prev_track}>
                <Fa icon={faBackward}/>
            </Button>
            <Button size="icon" on:click={() =>playerPlayPause(). then(data => console.log(data))}>
                <Fa icon={getPlayIcon(isPlaying)}/>
            </Button>
            <Button size="icon" variant="ghost" disabled={!queueStatus?.next_track}>
                <Fa icon={faForward}/>
            </Button>
        </div>
        <div class="track-info">
            {#if currentTrack}
                <Cover cover={{type: "pic", uri: currentTrack.coverUri}} size={50}/>
                <div class="track-name">
                    <p>{currentTrack.title}</p>
                    <ArtistList artists={currentTrack.artists }/>
                </div>

            {/if}
        </div>
<!--        <VolumeSelector volume={volume} onChange={playerSetVolume}/>-->
        <div class="player-settings">
        </div>
    </div>


</div>

<style>
    .progress {
        --pr: var(--progress);
        position: relative;

        height: 3px;

        transition: linear .3s, height .1s ease-in-out;
    }

    .progress::before {
        content: "";
        display: block;
        position: absolute;

        width: var(--pr);
        height: 100%;

        transition: width .3s linear, height .1s;

        @apply bg-yellow-500
    }

    .miniPlayer:hover > .progress {
        height: 6px;
    }

    .miniPlayer {
        display: flex;
        flex-direction: column;

        position: fixed;

        bottom: 1rem;
        left: 1rem;
        right: 1rem;

        overflow: hidden;

        @apply bg-gray-900 rounded-lg drop-shadow-lg
    }

    .miniPlayer-control {
        display: grid;
        grid-template-columns: fit-content(200px) 1fr 250px;
        align-items: center;

        @apply px-3 py-2 gap-2
    }

    .miniPlayer :global(.cover) {
        width: 50px;
        height: 50px;

        @apply rounded-sm
    }

    .track-info {
        display: flex;

        @apply gap-2
    }

</style>