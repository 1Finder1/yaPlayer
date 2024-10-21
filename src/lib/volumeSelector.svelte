<script lang="ts">
    import * as Tooltip from "$lib/components/ui/tooltip";
    import Fa from "svelte-fa";
    import {faVolumeHigh} from "@fortawesome/free-solid-svg-icons/faVolumeHigh"
    import {Slider} from "$lib/components/ui/slider";


    export let onChange: (vol: number) => unknown
    export let volume: number

    let value: number[] = [0, volume]

</script>


<Tooltip.Root openDelay={0}>
    <Tooltip.Trigger>
        <div class="volumeSelector">
            <Fa icon={faVolumeHigh}/>
            <div class="volumeSelector-slider">
                <Slider class="w-[200px]" min={0} max={100} step={1} value={[volume]} onValueChange={e => onChange(e[0])}/>
            </div>
        </div>

    </Tooltip.Trigger>

    <Tooltip.Content class="bg-gray-700 text-xl">
        {volume}
    </Tooltip.Content>
</Tooltip.Root>


<style>
    .volumeSelector {
        display: flex;
        align-items: center;

        transition: .3s;

        @apply gap-4
    }

    .volumeSelector-slider {
        opacity: 0;

        transition: opacity .3s;
    }

    .volumeSelector:hover .volumeSelector-slider {
        opacity: 1;
    }
</style>
