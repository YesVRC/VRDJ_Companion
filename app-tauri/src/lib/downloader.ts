import {invoke} from "@tauri-apps/api";
import dayjs from 'dayjs';
import {writable, get} from "svelte/store";
import type {Writable} from "svelte/store";

export class Downloader {
    spotifyData: Writable<{accessToken: string, time: number}>;
    queue: Map<string, boolean>;
    INSTANCE: Downloader;
    constructor() {
        this.spotifyData = writable<{accessToken: string, time: number}>();
        this.queue = new Map<string, boolean>();
        this.INSTANCE = this;
    }

    async getTokenSpotify(): Promise<string>{
        if(get(this.spotifyData)?.time != undefined || get(this.spotifyData)?.time != null){
            if(dayjs().millisecond() >  get(this.spotifyData).time)
            {
                return get(this.spotifyData).accessToken;
            }
        }
            const res: string = await invoke('spotify_auth');
            const parse = JSON.parse(res);
            this.spotifyData.set({accessToken: parse['access_token'],time: dayjs().add(3600, 'second').second()});
            return get(this.spotifyData).accessToken;
    }

    async searchYoutube(search: string): Promise<string> {
        const result: string = await invoke('youtube_search', {search: search});
        const parse = JSON.parse(result);
        return parse['items'][0]['id']['videoId'];
    }
    async searchSpotify(search: string): Promise<{ artist_name: string, song_name: string } | null>{
        const fix = search.split('/').pop();
        if (fix != null) {
            const fix2 = fix.split('?')[0];
            const res = await invoke('spotify_get_track', {id: fix2, token: await this.getTokenSpotify()}) + '\n';
            const parse = JSON.parse(res);
            const song_name = parse['name'];
            const artist_name = parse['artists'][0]['name'];
            this.queue.set(`${song_name}-${artist_name}`.replace(' ', '-'), false);
            return {
                artist_name,
                song_name
            }
        }
        else return null;
    }

    async getYoutubeFromSpotify(search: string){
        const spotifyData = await this.searchSpotify(search);
        const ytSearch = `${spotifyData?.song_name} by ${spotifyData?.artist_name} audio HQ`;
        const youtubeData = await this.searchYoutube(ytSearch);
        const fileName = `${spotifyData?.song_name}-${spotifyData?.artist_name}.mp3`.replace(' ', '-');
        const final = await invoke('youtube_download', {url: `https://www.youtube.com/watch?v=${youtubeData}`, fileName: fileName});
        this.queue.set(fileName.split('.mp3')[0], true);
    }
}





