<template>
    <div class="container">
        <div class="card">
            <p>
                Kimaier v{{ appVersion }}
            </p>
            <h4>Settings</h4>
            <input v-model="user.name" placeholder="Name" />
            <input v-model="user.api_pass" type="password" placeholder="API Password" />
            <input v-model="user.api_url" placeholder="API URL" />

            <div class="work-group">
                <input v-model="user.project" placeholder="Project" />
                <input v-model="user.activity" placeholder="Activity" />
            </div>

            <div class="btn-group">
                <button type="button" @click="sendRegEvent(true)">Cancel</button>
                <button type="button" @click="saveSettings()">Save</button>
            </div>
        </div>

        <p>{{ saveMsg }}</p>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { getVersion } from '@tauri-apps/api/app';

import { storeToRefs } from 'pinia'
import { useMainStore } from '../stores/main'

const { authHeader, user, allActivities } = storeToRefs(useMainStore())
const mainStore = useMainStore()

const emit = defineEmits(['reg-event'])
const sendRegEvent = (val: boolean) => emit('reg-event', val)

const appVersion = ref();
const saveMsg = ref('')

async function saveSettings() {
    authHeader.value = {
        'X-AUTH-USER': user.value.name,
        'X-AUTH-TOKEN': user.value.api_pass,
    }

    user.value.api_url = user.value.api_url.replace(/\/+$/, '')

    await mainStore.setActivities()

    for (const activity of allActivities.value) {
        if (activity.parentTitle && activity.parentTitle.toLowerCase() === user.value.project.toLocaleLowerCase()) {
            user.value.project_id = activity.project
        }

        if (activity.name.toLowerCase() === user.value.activity.toLocaleLowerCase()) {
            user.value.activity_id = activity.id
        }
    }

    saveMsg.value = await invoke('save_settings', { user: user.value })

    setTimeout(() => {
        saveMsg.value = ''
        if (user.value.activity !== '' && user.value.project !== '') {
            sendRegEvent(true)
        }
    }, 1000)
}

onMounted(async () => {
    appVersion.value = await getVersion()
})
</script>

<style scoped>
.container {
    padding-top: 1vh;
}

.card p {
    font-size: 12px;
}

h4 {
    margin-top: 0;
    margin-bottom: 0.3em;
}

.work-group h4 {
    margin-top: 1em;
}

.card {
    display: table;
    flex-wrap: wrap;
    align-content: center;
}

input {
    display: block;
    margin: 0.5em auto 0 auto;
}

input {
    width: 70vw;
}

button {
    width: 80px;
    margin: 0.5em 0.25em 0 0.25em;
}
</style>
