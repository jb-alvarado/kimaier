<template>
    <div>
        <Control v-if="isRegister" @reg-event="setRegister" />
        <Register v-else @reg-event="setRegister" />
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import Control from './components/Control.vue'
import Register from './components/Register.vue'
import { invoke } from '@tauri-apps/api/tauri'

import { storeToRefs } from 'pinia'
import { useMainStore } from './stores/main'

const { authHeader, user, work } = storeToRefs(useMainStore())
const setRegister = (val: boolean) => (isRegister.value = val)

const isRegister = ref(false)

onMounted(async () => {
    user.value = JSON.parse(await invoke('get_user'))
    work.value = JSON.parse(await invoke('get_work'))

    if (
        user.value.name === '' ||
        user.value.api_pass === '' ||
        user.value.api_url === '' ||
        work.value.activity_id === 0
    ) {
        isRegister.value = false
    } else {
        isRegister.value = true

        authHeader.value = {
            "X-AUTH-USER": user.value.name,
            "X-AUTH-TOKEN": user.value.api_pass
        }
    }
})
</script>

<style scoped>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}
</style>
