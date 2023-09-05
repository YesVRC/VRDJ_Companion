<script lang="ts">
import {Downloader} from "$lib/downloader";
import {SlideToggle} from "@skeletonlabs/skeleton";

let dl = new Downloader();

let search;
let searchType: 'youtube'|'spotify';

$: map = dl.queue;
</script>
<form on:submit={async () => {await dl.getYoutubeFromSpotify(search);}}>
    <input type="text" bind:value={search} class="text-surface-900 w-[60%]" placeholder="Enter a URL"/>
    <button type="submit">
        Click Me To Test!
    </button>
</form>
<div class="flex-col">
    {#each map as [key, val]}
        <div class="card grid-rows-[9-1]">
            <p>{key}</p>
            <SlideToggle bind:value={val}/>
        </div>

    {/each}
</div>




