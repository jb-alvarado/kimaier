<template>
    <div class="container">
        <div class="card">
            <form @submit.prevent="saveSettings()" @reset="currentPage = page.Control">
                <input v-model="user.name" placeholder="Name" required />
                <input v-model="user.api_pass" type="password" placeholder="API Password" required />
                <input v-model="user.api_url" placeholder="API URL" required />
                <input v-model="user.project" placeholder="Project" required />
                <input v-model="user.activity" placeholder="Activity" required />
                <input v-model="user.week_hours" placeholder="Hours per Week" type="number" tep="0.1" required />
                <input v-model="user.work_start" type="date" required />

                <div class="day-group">
                    <button
                        v-for="day in days"
                        type="button"
                        key="day.name"
                        class="day-btn"
                        :class="day.active ? 'btn-active' : ''"
                        @click="activateDay(day)"
                    >
                        {{ day.name }}
                    </button>
                </div>

                <div class="btn-group">
                    <button type="reset">Cancel</button>
                    <button type="submit">Save</button>
                </div>
            </form>

            <p>Kimaier v{{ appVersion }}</p>
        </div>

        <p>{{ saveMsg }}</p>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { getVersion } from '@tauri-apps/api/app'

import { storeToRefs } from 'pinia'
import { useMainStore } from '../stores/main'

const { allActivities, authHeader, currentPage, page, user } = storeToRefs(useMainStore())
const mainStore = useMainStore()

const appVersion = ref()
const saveMsg = ref('')

const days = ref([
    { name: 'Mo', active: false },
    { name: 'Tu', active: false },
    { name: 'We', active: false },
    { name: 'Th', active: false },
    { name: 'Fr', active: false },
    { name: 'Sa', active: false },
    { name: 'Su', active: false },
])

async function saveSettings() {
    user.value.api_url = user.value.api_url.replace(/\/+$/, '')
    authHeader.value = {
        'X-AUTH-USER': user.value.name,
        'X-AUTH-TOKEN': user.value.api_pass,
    }

    await mainStore.setActivities()

    for (const activity of allActivities.value) {
        if (activity.parentTitle && activity.parentTitle.toLowerCase() === user.value.project.toLocaleLowerCase()) {
            user.value.project_id = activity.project
        }

        if (activity.name.toLowerCase() === user.value.activity.toLocaleLowerCase()) {
            user.value.activity_id = activity.id
        }
    }

    await mainStore.setStore()

    setTimeout(() => {
        saveMsg.value = ''
        if (user.value.activity !== '' && user.value.project !== '') {
            currentPage.value = page.value.Control
        }
    }, 1000)
}

function activateDay(day: any) {
    day.active = !day.active

    if (day.active && !user.value.work_days.includes(day.name)) {
        user.value.work_days.push(day.name)
    } else if (!day.active && user.value.work_days.includes(day.name)) {
        const index = user.value.work_days.indexOf(day.name)

        user.value.work_days.splice(index, 1)
    }
}

onMounted(async () => {
    appVersion.value = await getVersion()

    for (const dayName of user.value.work_days) {
        for (const day of days.value) {
            if (dayName === day.name) {
                day.active = true
            }
        }
    }
})
</script>

<style scoped>
.card p {
    font-size: 12px;
}

.card {
    display: table;
    flex-wrap: wrap;
    align-content: center;
    margin-top: 10px;
    text-align: center;
}

h5 {
    margin-top: 0;
    margin-bottom: 0.2em;
}

input {
    display: block;
    margin: 0.5em auto 0 auto;
}

input {
    width: 70vw;
    height: 20px;
}

button {
    width: 80px;
    margin: 0.5em 0 0 0;
}

input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

.day-group > button {
    font-size: 12px;
    margin: 0.7em 0 0 0;
    padding: 1px;
    line-height: 27px;
    width: 25px;
    height: 25px;
    border-radius: 0;
}

.btn-active {
    background-color: rgba(137, 43, 226, 0.521);
}

.day-group > .day-btn:first-child {
    border-top-left-radius: 4px;
    border-bottom-left-radius: 4px;
}

.day-group > .day-btn:last-child {
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
}

.btn-group {
    display: flex;
    gap: 1em;
    justify-content: center;
}
</style>
