<template>
    <div>
        <Control v-if="isRegister" />
        <Register v-else />
    </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import Control from './components/Control.vue'
import Register from './components/Register.vue'

import { storeToRefs } from 'pinia'
import { useMainStore } from './stores/main'

const { isRegister, user } = storeToRefs(useMainStore())
const mainStore = useMainStore()

onMounted(async () => {
    await mainStore.getStore()

    if (user.value.name === '' || user.value.activity_id === 0) {
        isRegister.value = false
    } else {
        isRegister.value = true
    }
})
</script>

