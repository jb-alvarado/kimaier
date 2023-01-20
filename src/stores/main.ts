import { defineStore } from 'pinia'
import { Store } from 'tauri-plugin-store-api'

interface Header {
    'X-AUTH-USER': string
    'X-AUTH-TOKEN': string
}

interface User {
    name: string
    api_pass: string
    api_url: string
    project: string
    activity: string
    project_id: number
    activity_id: number
}

export const useMainStore = defineStore('main', {
    state: () => ({
        store: new Store('kimaier.dat'),
        user: {
            name: '',
            api_pass: '',
            api_url: '',
            project: '',
            activity: '',
            project_id: 0,
            activity_id: 0,
        } as User,
        allActivities: [] as any[],
        authHeader: {} as Header,
        isRegister: false,
        isRunning: false,
    }),
    getters: {},

    actions: {
        async getStore() {
            await this.store
                .get('user')
                .then((data: any) => {
                    console.log(data.name)
                    this.user = data
                    this.isRegister = true
                    this.authHeader = {
                        'X-AUTH-USER': data.name,
                        'X-AUTH-TOKEN': data.api_pass,
                    }
                })
                .catch(() => (this.isRegister = false))
        },

        async setStore() {
            await this.store.set('user', this.user).then(() => {
                setTimeout(() => {
                    this.store.save()
                }, 100)
            })
        },

        async setActivities() {
            await fetch(`${this.user?.api_url}/api/activities`, {
                method: 'GET',
                headers: new Headers({ 'Content-Type': 'application/json', ...this.authHeader }),
            })
                .then((response) => response.json())
                .then((data) => {
                    this.allActivities = data
                })
                .catch((e) => {
                    alert(e)
                })
        },
    },
})
