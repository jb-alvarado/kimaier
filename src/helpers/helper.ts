export default {
    secToHMS(sec: number) {
        let hours = Math.floor(sec / 3600)
        sec %= 3600
        let minutes = Math.floor(sec / 60)
        let seconds = Math.round(sec % 60)

        const m = String(minutes).padStart(2, '0')
        const h = String(hours).padStart(2, '0')
        const s = String(seconds).padStart(2, '0')

        return `${h}:${m}:${s}`
    },

    secToHM(sec: number) {
        const sign = Math.sign(sec)
        sec = Math.abs(sec)

        const hours = Math.floor(sec / 3600)
        sec %= 3600
        const minutes = Math.floor(sec / 60)

        const m = String(minutes).padStart(2, '0')
        const h = String(hours).padStart(2, '0')

        const hString = (sign === -1 ? '-' : '') + h

        return `${hString}:${m}`
    },
}
