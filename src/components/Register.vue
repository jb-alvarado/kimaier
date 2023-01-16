<template>
    <div class="card">
        <h2>Settings</h2>
        <input v-model="user.name" placeholder="Name" />
        <input v-model="user.api_pass" type="password" placeholder="API Password" />
        <input v-model="user.api_url" placeholder="API URL" />
        <input v-model="user.workspace" placeholder="Workspace" />
        <button type="button" @click="save()">Save</button>
    </div>

    <p>{{ saveMsg }}</p>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

import { storeToRefs } from 'pinia'
import { useMainStore } from '../stores/main'

const { user } = storeToRefs(useMainStore())

const emit = defineEmits(["reg-event"]);
const sendRegEvent = (val: boolean) => emit("reg-event", val);
const saveMsg = ref('')

async function save() {
    saveMsg.value = await invoke('save_user', { user: user.value })

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
h2 {
    margin-top: 0;
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
    width: 160px;
}
</style>
