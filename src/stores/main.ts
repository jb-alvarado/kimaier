import { defineStore } from 'pinia'

interface Header {
    "X-AUTH-USER": string,
    "X-AUTH-TOKEN": string,
}

export const useMainStore = defineStore('main', {
    state: () => ({
        user: {
            name: '',
            api_pass: '',
            api_url: '',
            workspace: '',
        },
        authHeader: {} as Header,
    }),
    getters: {},

    actions: {
        // reset() {
        //     // `this` is the store instance
        //     this.counter = 0
        // },
    },
})
