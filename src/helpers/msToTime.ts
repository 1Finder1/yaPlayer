export function msToString(ms: number | null | undefined) {
    if (!ms) return "00:00"
    const secs = Math.floor(ms / 1000 % 60);
    const minutes = Math.floor(ms / 1000 / 60);

    const prettySecs = secs < 10 ? '0' + secs: secs

    return `${minutes}:${prettySecs}`
}