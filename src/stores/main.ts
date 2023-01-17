import { defineStore } from 'pinia'

interface Header {
    'X-AUTH-USER': string
    'X-AUTH-TOKEN': string
}

export const useMainStore = defineStore('main', {
    state: () => ({
        user: {
            name: '',
            api_pass: '',
            api_url: '',
        },
        work: {
            project: '',
            activity: '',
            project_id: 0,
            activity_id: 0,
        },
        allActivities: [] as any[],
        authHeader: {} as Header,
        isRunning: false,
    }),
    getters: {},

    actions: {
        async setActivities() {
            await fetch(`${this.user.api_url}/api/activities`, {
                method: 'GET',
                headers: new Headers({ 'Content-Type': 'application/json', ...this.authHeader }),
            })
                .then((response) => response.json())
                .then((data) => {
                    this.allActivities = data
                })
                .catch((e) => {
                    console.log('--- error', e)
                })
        },
    },
})
