<template>
    <div class="container">
        <div class="card">
            <button v-if="isRunning" id="control-button" type="button" @click="setState()">
                <i class="fa-regular fa-circle-pause"></i>
            </button>
            <button v-else id="control-button" type="button" @click="setState()">
                <i class="fa-regular fa-circle-play"></i>
            </button>
        </div>

        <p class="timer">{{ secToHMS(timer) }}</p>

        <div class="footer">
            <button class="settings-button" type="button" @click="sendRegEvent(false)">
                <i class="fa-solid fa-gear"></i>
            </button>
            <div class="spacer"></div>
            <a :href="user.api_url" target="_blank">
                <button class="settings-button" type="button">
                    <i class="fa-regular fa-file-lines"></i>
                </button>
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

const { user, work, authHeader, isRunning } = storeToRefs(useMainStore())
const emit = defineEmits(['reg-event'])
const sendRegEvent = (val: boolean) => emit('reg-event', val)

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
            await fetch(`${user.value.api_url}/api/timesheets/${activity.id}`, {
                method: 'DELETE',
                headers: new Headers({ 'Content-Type': 'application/json', ...authHeader.value }),
            }).then((response) => {
                if (response.status === 204) {
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
                project: work.value.project_id,
                activity: work.value.activity_id,
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
    font-size: 40vw;
    padding: 15% 0;
}

#control-button .fa-circle-play {
    color: green;
}
#control-button .fa-circle-pause {
    color: orange;
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
    border-radius: 50%;
    width: 38px;
    height: 38px;
    padding: 6px 0 0 0;
    font-size: 24px;
    color: gray;
    background: none !important;
    box-shadow: none;
}

.timer {
    font-size: 24px;
    margin: 1.5em 0 0.5em 0;
}

</style>
