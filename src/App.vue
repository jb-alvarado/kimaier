<template>
    <div>
        <Control v-if="currentPage === page.Control" />
        <Statistics v-else-if="currentPage === page.Statistics" />
        <Register v-else />
    </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import Control from './components/Control.vue'
import Register from './components/Register.vue'
import Statistics from './components/Statistics.vue'

import { storeToRefs } from 'pinia'
import { useMainStore } from './stores/main'

const { currentPage, user, page } = storeToRefs(useMainStore())
const mainStore = useMainStore()

onMounted(async () => {
    await mainStore.getStore()

    if (user.value.name === '' || user.value.activity_id === 0 || user.value.week_hours === 0) {
        currentPage.value = page.value.Settings
    } else {
        currentPage.value = page.value.Control
    }
})
</script>
