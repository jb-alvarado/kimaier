<template>
    <div class="container">
        <div class="card">
            <h4>General</h4>
            <input v-model="user.name" placeholder="Name" />
            <input v-model="user.api_pass" type="password" placeholder="API Password" />
            <input v-model="user.api_url" placeholder="API URL" />

            <button type="button" @click="saveUser()">Save</button>

            <div class="work-group">
                <h4>Work</h4>
                <input v-model="work.project" placeholder="Project" />
                <input v-model="work.activity" placeholder="Activity" />
                <button type="button" @click="saveWork()">Save</button>
            </div>
        </div>

        <p>{{ saveMsg }}</p>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

import { storeToRefs } from 'pinia'
import { useMainStore } from '../stores/main'

const { authHeader, user, work, allActivities } = storeToRefs(useMainStore())
const mainStore = useMainStore()
const emit = defineEmits(['reg-event'])
const sendRegEvent = (val: boolean) => emit('reg-event', val)

const saveMsg = ref('')

async function saveUser() {
    saveMsg.value = await invoke('save_user', { user: user.value })
    authHeader.value = {
        'X-AUTH-USER': user.value.name,
        'X-AUTH-TOKEN': user.value.api_pass,
    }

    mainStore.setActivities()

    setTimeout(() => {
        saveMsg.value = ''
        if (work.value.activity !== '' && work.value.project !== '') {
            sendRegEvent(true)
        }
    }, 1000)
}

async function saveWork() {
    for (const activity of allActivities.value) {
        if (activity.parentTitle && activity.parentTitle.toLowerCase() === work.value.project.toLocaleLowerCase()) {
            work.value.project_id = activity.project
        }

        if (activity.name.toLowerCase() === work.value.activity.toLocaleLowerCase()) {
            work.value.activity_id = activity.id
        }
    }

    saveMsg.value = await invoke('save_work', { work: work.value })

    setTimeout(() => {
        saveMsg.value = ''
        sendRegEvent(true)
    }, 1000)
}

onMounted(async () => {
    user.value = JSON.parse(await invoke('get_user'))
})
</script>

<style scoped>
.container {
    padding-top: 3vh !important;
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

button,
input {
    display: block;
    margin: 0.5em auto 0 auto;
}

input {
    width: 70vw;
}

button {
    width: 80px;
}
</style>
