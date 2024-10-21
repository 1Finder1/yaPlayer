import { faPause } from '@fortawesome/free-solid-svg-icons/faPause'
import { faPlay } from '@fortawesome/free-solid-svg-icons/faPlay'

export function getImgUrl(uri: string | undefined | null, size: number) {
	if (!uri)
		return 'https://music.yandex.ru/blocks/playlist-cover/playlist-cover_no_cover1.png'
	return `https://${uri.replace('%%', `${size}x${size}`)}`
}

export function getPlayIcon(isPlaying: boolean) {
	return isPlaying ? faPlay : faPause
}
