<template>
    <div class="card">
        <button id="control-button" type="button">
            <i class="fa-regular fa-circle-play"></i>
        </button>
    </div>

    <p class="timer">{{ timer }}</p>
</template>

<script setup lang="ts">
import dayjs from 'dayjs'
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri'

const greetMsg = ref('')
const name = ref('')
const timer = ref(dayjs().format('HH:mm:ss'))

async function status() {
    async function setStatus(resolve: any) {
        /*
            recursive function as a endless loop
        */
        timer.value = dayjs().format('HH:mm:ss')

        setTimeout(() => setStatus(resolve), 1000)
    }
    return new Promise((resolve) => setStatus(resolve))
}

onMounted(() => {
    status()
})

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg.value = await invoke('greet', { name: name.value })
}
</script>

<style scoped>
#control-button {
    width: 70vw;
    height: 70vw;
    font-size:40vw;
    padding: 15% 0;
}

.timer {
    font-size: 24px;
    margin-top: 2em;
}
</style>
