<template>
    <div class="card">
        <button v-if="isRunning" id="control-button" type="button" @click="setState()">
            <i class="fa-regular fa-circle-pause"></i>
        </button>
        <button v-else id="control-button" type="button" @click="setState()">
            <i class="fa-regular fa-circle-play"></i>
        </button>
    </div>

    <p class="timer">{{ timer }}</p>

    <div class="footer">
        <button class="settings-button" type="button" @click="sendRegEvent(false)">
            <i class="fa-solid fa-gear"></i>
        </button>
        <a href="https://github.com/rust-lang/rust-analyzer" target="_blank" class="settings-button" type="button">
            <i class="fa-solid fa-pen-to-square"></i>
        </a>
    </div>
</template>

<script setup lang="ts">
import dayjs from 'dayjs'
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

import { storeToRefs } from 'pinia'
import { useMainStore } from '../stores/main'

const { user, authHeader } = storeToRefs(useMainStore())

const emit = defineEmits(['reg-event'])
const sendRegEvent = (val: boolean) => emit('reg-event', val)

const timer = ref(dayjs().format('HH:mm:ss'))
const activities = ref([] as any[])
const isRunning = ref(false)

async function getActivities() {
    await fetch(`${user.value.api_url}/api/timesheets/active`, {
        method: 'GET',
        headers: new Headers({ 'Content-Type': 'application/json', ...authHeader.value }),
    })
        .then(response => response.json())
        .then(data => {
            if (data && data.length > 0) {
                activities.value = data
                isRunning.value = true
            } else {
                isRunning.value = false
            }
        })
        .catch(() => {
            isRunning.value = false
            activities.value = []
        })
}

function setTimer() {
    console.log(activities.value)
    for (const act of activities.value) {
        console.log(act.begin)
    }
}

async function status() {
    await getActivities()
    setTimer()

    async function setStatus(resolve: any) {
        /*
            recursive function as a endless loop
        */
        const time = dayjs()
        timer.value = time.format('HH:mm:ss')

        // console.log(timer.value)

        if (time.unix() % 20 === 0) {
            await getActivities()
            setTimer()
        }

        setTimeout(() => setStatus(resolve), 1000)
    }
    return new Promise((resolve) => setStatus(resolve))
}

async function setState() {
    if (isRunning.value) {
        console.log('--- stop')
    } else {
        // const acts = JSON.parse((await invoke('start_activity')) as any)
        // console.log(acts)
    }
}

onMounted(async () => {
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
    text-align: left;
    margin-left: 15vw;
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
