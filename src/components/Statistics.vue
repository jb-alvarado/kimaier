<template>
    <div class="container">
        <div class="card">
            <div class="table">
                <div class="cell name-col"><strong>Today:</strong></div>
                <div class="cell">{{ helper.secToHMS(timeToday) }}</div>
                <div class="cell name-col"><strong>Week:</strong></div>
                <div class="cell">{{ helper.secToHM(timeWeek) }}</div>
                <div class="cell name-col"><strong>Month:</strong></div>
                <div class="cell">{{ helper.secToHM(timeMonth) }}</div>
                <div class="cell name-col"><strong>Target:</strong></div>
                <div class="cell">{{ targetHours }}</div>
                <div class="cell name-col"><strong>Left:</strong></div>
                <div class="cell">{{ helper.secToHM(timeLeft) }}</div>
            </div>
        </div>

        <Footer />
    </div>
</template>
<script setup lang="ts">
import dayjs from 'dayjs'
import utc from 'dayjs/plugin/utc.js'
import timezone from 'dayjs/plugin/timezone.js'
import customParseFormat from 'dayjs/plugin/customParseFormat'
import { onMounted, onBeforeUnmount, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useMainStore } from '../stores/main'
import Footer from './Footer.vue'
import helper from '../helpers/helper'

dayjs.Ls.en.weekStart = 1;
dayjs.extend(utc)
dayjs.extend(timezone)
dayjs.extend(customParseFormat)

const { user, authHeader, timeToday, timeMonth, timeWeek, timeLeft, targetHours, todaysActivities, monthActivities, weekActivities } =
    storeToRefs(useMainStore())
const statisticsTimeout = ref()

async function getActivities(date: string): Promise<any[]> {
    let list = [] as any[]

    await fetch(`${user.value.api_url}/api/timesheets?begin=${date}&size=200`, {
        method: 'GET',
        headers: new Headers({ 'Content-Type': 'application/json', ...authHeader.value }),
    })
        .then((response) => response.json())
        .then((data) => {
            if (data && data.length > 0) {
                list = data
            } else {
                list = []
            }
        })
        .catch(() => {
            list = []
        })

    return list
}

async function getTotalHours(date: any): Promise<number> {
    const daysInMonth = date.daysInMonth()
    const dayHours = user.value.week_hours / user.value.work_days.length
    let holidays = [] as string[]
    let currentDay = 1
    let weekHours = 0
    let totalHours = 0

    if (user.value.state !== '' && user.value.state !== 'none') {
        await fetch(`https://feiertage-api.de/api/?jahr=${date.format('YYYY')}&nur_land=${user.value.state}`, {
            method: 'GET',
        })
            .then((response) => response.json())
            .then((data) => {
                if (data) {
                    Object.entries(data).forEach(([_, holiday]: any) => {
                        if (holiday.hinweis === '' && date.isSame(holiday.datum, 'month')) {
                            holidays.push(holiday.datum)
                        }
                    })
                }
            })
    }

    while (daysInMonth >= currentDay) {
        const current = date.date(currentDay)
        const currentDate = current.format('YYYY-MM-DD')

        if (current.format('dd') === 'Mo') {
            totalHours += weekHours
            weekHours = 0
        }

        if (
            user.value.work_days.includes(current.format('dd')) &&
            !holidays.includes(currentDate) &&
            weekHours < user.value.week_hours
        ) {
            weekHours += dayHours
        }

        currentDay++
    }

    totalHours += weekHours

    return totalHours
}

function getWeekActivities(activities: any[]) {
    let activitiesList = []

    for (const act of activities.reverse()) {
        const begin = dayjs(act.begin, 'YYYY-MM-DD')

        if (dayjs().isSame(begin, 'week')) {
            activitiesList.push(act)
        }
    }

    return activitiesList
}

function setTimer(time: any, activities: any[]): number {
    if (activities.length === 0) {
        return 0
    }

    let timeDiff = 0

    for (const act of activities.reverse()) {
        const begin = dayjs(act.begin, 'YYYY-MM-DDTHH:mm:ss+000ZZ')

        if (act.end) {
            const end = dayjs(act.end, 'YYYY-MM-DDTHH:mm:ss+000ZZ')
            timeDiff += end.diff(begin, 'second')
        } else {
            timeDiff += time.diff(begin, 'second')
        }
    }

    return timeDiff
}

async function status() {
    let time = dayjs()
    let today = time.utc().format('YYYY-MM-DDT00:00:00')
    let month = time.utc().format('YYYY-MM-01T00:00:00')
    todaysActivities.value = await getActivities(today)
    monthActivities.value = await getActivities(month)
    weekActivities.value = getWeekActivities(monthActivities.value)
    targetHours.value = await getTotalHours(time)

    async function setStatus(resolve: any) {
        /*
            recursive function as a endless loop
        */
        time = dayjs()
        today = time.utc().format('YYYY-MM-DDT00:00:00')
        month = time.utc().format('YYYY-MM-01T00:00:00')
        timeToday.value = setTimer(time, todaysActivities.value)
        timeMonth.value = setTimer(time, monthActivities.value)
        timeWeek.value = setTimer(time, weekActivities.value)
        timeLeft.value = targetHours.value * 3600 - timeMonth.value

        if (time.unix() % 60 === 0) {
            todaysActivities.value = await getActivities(today)
            monthActivities.value = await getActivities(month)
            weekActivities.value = getWeekActivities(monthActivities.value)
        }

        statisticsTimeout.value = setTimeout(() => setStatus(resolve), 1000)
    }
    return new Promise((resolve) => setStatus(resolve))
}

onMounted(() => {
    status()
})

onBeforeUnmount(() => {
    clearTimeout(statisticsTimeout.value)
})
</script>
<style scoped>
* {
    box-sizing: border-box;
}

.table {
    width: 50vw;
    max-width: 160px;
    margin: 0 auto 20px auto;
}
.table:after {
    content: '';
    display: table;
    clear: both;
}
.cell {
    float: left;
    width: 50%;
    text-align: left;
    padding: 0px 2px 0px 2px;
}

@media screen and (max-width: 200px) {
    .container {
        padding-top: 1vh;
    }

    .cell {
        width: 100%;
    }

    .name-col {
        margin-top: 10px;
    }
}
</style>
