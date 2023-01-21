<template>
    <div class="container">
        <div class="card">
            <button id="control-button" type="button" @click="setState()">
                <div v-if="isRunning" class="circle-pause">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                        <!--! Font Awesome Free 6.2.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc. -->
                        <path
                            d="M200 160C186.8 160 176 170.8 176 184v144C176 341.3 186.8 352 200 352S224 341.3 224 328v-144C224 170.8 213.3 160 200 160zM312 160C298.8 160 288 170.8 288 184v144c0 13.25 10.75 24 24 24s24-10.75 24-24v-144C336 170.8 325.3 160 312 160zM256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208S370.7 464 256 464z"
                        />
                    </svg>
                </div>
                <div v-else class="circle-play">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                        <!--! Font Awesome Free 6.2.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc. -->
                        <path
                            d="M188.3 147.1C195.8 142.8 205.1 142.1 212.5 147.5L356.5 235.5C363.6 239.9 368 247.6 368 256C368 264.4 363.6 272.1 356.5 276.5L212.5 364.5C205.1 369 195.8 369.2 188.3 364.9C180.7 360.7 176 352.7 176 344V167.1C176 159.3 180.7 151.3 188.3 147.1V147.1zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z"
                        />
                    </svg>
                </div>
            </button>
        </div>

        <p class="timer">{{ secToHMS(timer) }}</p>

        <div class="footer">
            <a href="#" class="settings-button" @click="isRegister = false">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                    <!--! Font Awesome Free 6.2.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc. -->
                    <path
                        d="M495.9 166.6c3.2 8.7 .5 18.4-6.4 24.6l-43.3 39.4c1.1 8.3 1.7 16.8 1.7 25.4s-.6 17.1-1.7 25.4l43.3 39.4c6.9 6.2 9.6 15.9 6.4 24.6c-4.4 11.9-9.7 23.3-15.8 34.3l-4.7 8.1c-6.6 11-14 21.4-22.1 31.2c-5.9 7.2-15.7 9.6-24.5 6.8l-55.7-17.7c-13.4 10.3-28.2 18.9-44 25.4l-12.5 57.1c-2 9.1-9 16.3-18.2 17.8c-13.8 2.3-28 3.5-42.5 3.5s-28.7-1.2-42.5-3.5c-9.2-1.5-16.2-8.7-18.2-17.8l-12.5-57.1c-15.8-6.5-30.6-15.1-44-25.4L83.1 425.9c-8.8 2.8-18.6 .3-24.5-6.8c-8.1-9.8-15.5-20.2-22.1-31.2l-4.7-8.1c-6.1-11-11.4-22.4-15.8-34.3c-3.2-8.7-.5-18.4 6.4-24.6l43.3-39.4C64.6 273.1 64 264.6 64 256s.6-17.1 1.7-25.4L22.4 191.2c-6.9-6.2-9.6-15.9-6.4-24.6c4.4-11.9 9.7-23.3 15.8-34.3l4.7-8.1c6.6-11 14-21.4 22.1-31.2c5.9-7.2 15.7-9.6 24.5-6.8l55.7 17.7c13.4-10.3 28.2-18.9 44-25.4l12.5-57.1c2-9.1 9-16.3 18.2-17.8C227.3 1.2 241.5 0 256 0s28.7 1.2 42.5 3.5c9.2 1.5 16.2 8.7 18.2 17.8l12.5 57.1c15.8 6.5 30.6 15.1 44 25.4l55.7-17.7c8.8-2.8 18.6-.3 24.5 6.8c8.1 9.8 15.5 20.2 22.1 31.2l4.7 8.1c6.1 11 11.4 22.4 15.8 34.3zM256 336c44.2 0 80-35.8 80-80s-35.8-80-80-80s-80 35.8-80 80s35.8 80 80 80z"
                    />
                </svg>
            </a>
            <div class="spacer"></div>
            <a :href="user.api_url" class="settings-button" target="_blank">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512">
                    <!--! Font Awesome Free 6.2.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc. -->
                    <path
                        d="M365.3 93.38l-74.63-74.64C278.6 6.742 262.3 0 245.4 0L64-.0001c-35.35 0-64 28.65-64 64l.0065 384c0 35.34 28.65 64 64 64H320c35.2 0 64-28.8 64-64V138.6C384 121.7 377.3 105.4 365.3 93.38zM336 448c0 8.836-7.164 16-16 16H64.02c-8.838 0-16-7.164-16-16L48 64.13c0-8.836 7.164-16 16-16h160L224 128c0 17.67 14.33 32 32 32h79.1V448zM96 280C96 293.3 106.8 304 120 304h144C277.3 304 288 293.3 288 280S277.3 256 264 256h-144C106.8 256 96 266.8 96 280zM264 352h-144C106.8 352 96 362.8 96 376s10.75 24 24 24h144c13.25 0 24-10.75 24-24S277.3 352 264 352z"
                    />
                </svg>
            </a>
        </div>
    </div>
</template>

<script setup lang="ts">
import dayjs from 'dayjs'
import utc from 'dayjs/plugin/utc.js'
import timezone from 'dayjs/plugin/timezone.js'
import customParseFormat from 'dayjs/plugin/customParseFormat'
import { onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useMainStore } from '../stores/main'

dayjs.extend(utc)
dayjs.extend(timezone)
dayjs.extend(customParseFormat)

const { user, authHeader, isRegister, isRunning } = storeToRefs(useMainStore())

const timer = ref(0)
const runningActivities = ref([] as any[])

async function getActivities() {
    await fetch(`${user.value.api_url}/api/timesheets/active`, {
        method: 'GET',
        headers: new Headers({ 'Content-Type': 'application/json', ...authHeader.value }),
    })
        .then((response) => response.json())
        .then((data) => {
            if (data && data.length > 0) {
                runningActivities.value = data
                isRunning.value = true
            } else {
                isRunning.value = false
                runningActivities.value = []
            }
        })
        .catch(() => {
            isRunning.value = false
            runningActivities.value = []
        })
}

function secToHMS(sec: number) {
    let hours = Math.floor(sec / 3600)
    sec %= 3600
    let minutes = Math.floor(sec / 60)
    let seconds = Math.round(sec % 60)

    const m = String(minutes).padStart(2, '0')
    const h = String(hours).padStart(2, '0')
    const s = String(seconds).padStart(2, '0')

    return `${h}:${m}:${s}`
}

function setTimer(time: any) {
    if (runningActivities.value.length === 0 && timer.value !== 0) {
        timer.value = 0
    }

    for (const act of runningActivities.value) {
        const begin = dayjs(act.begin, 'YYYY-MM-DDTHH:mm:ss+000ZZ')
        const diff = time.diff(begin, 'second')
        timer.value = diff
    }
}

async function status() {
    await getActivities()

    async function setStatus(resolve: any) {
        /*
            recursive function as a endless loop
        */
        const time = dayjs()
        setTimer(time)

        if (time.unix() % 60 === 0) {
            await getActivities()
        }

        setTimeout(() => setStatus(resolve), 1000)
    }
    return new Promise((resolve) => setStatus(resolve))
}

async function setState() {
    if (isRunning.value) {
        for (const activity of runningActivities.value) {
            await fetch(`${user.value.api_url}/api/timesheets/${activity.id}/stop`, {
                method: 'PATCH',
                headers: new Headers({ 'Content-Type': 'application/json', ...authHeader.value }),
            }).then((response) => {
                if (response.status === 200) {
                    isRunning.value = false
                    runningActivities.value = []
                }
            })
        }
    } else {
        await fetch(`${user.value.api_url}/api/timesheets`, {
            method: 'POST',
            headers: new Headers({ 'Content-Type': 'application/json', ...authHeader.value }),
            body: JSON.stringify({
                begin: dayjs(),
                end: '',
                project: user.value.project_id,
                activity: user.value.activity_id,
                description: '',
                tags: '',
            }),
        }).then(async (response) => {
            if (response.status === 200) {
                isRunning.value = true
                await getActivities()
            }
        })
    }
}

onMounted(() => {
    status()
})
</script>

<style scoped>
#control-button {
    width: 70vw;
    height: 70vw;
    line-height: 70vw;
    padding: 0;
}

#control-button svg {
    width: 60vw;
    height: 60vw;
    vertical-align: middle;
}

.circle-play svg {
    fill: green;
}
.circle-pause svg {
    fill: orange;
}

.footer {
    display: table;
    text-align: center;
    width: 100%;
}

.spacer {
    display: table-cell;
}

.settings-button {
    display: inline-block;
    border-radius: 50%;
    width: 38px;
    height: 38px;
    line-height: 38px;
}

.settings-button:hover {
    border-color: #396cd8;
}

.settings-button svg {
    width: 25px;
    height: 25px;
    fill: gray;
    vertical-align: middle;
}

.timer {
    font-size: 24px;
    margin: 1.5em 0 0.5em 0;
}
</style>
