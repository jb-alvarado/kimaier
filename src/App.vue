<template>
    <div class="container">
        <!-- <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>

    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <p>
      Recommended IDE setup:
      <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
      +
      <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
      +
      <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"
        >Tauri</a
      >
      +
      <a href="https://github.com/rust-lang/rust-analyzer" target="_blank"
        >rust-analyzer</a
      >
    </p> -->

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

const { user, authHeader } = storeToRefs(useMainStore())
// const { setUser } = useMainStore()

const isRegister = ref(false)

const setRegister = (val: boolean) => (isRegister.value = val)

onMounted(async () => {
    user.value = JSON.parse(await invoke('get_user'))

    if (
        user.value.name === '' ||
        user.value.api_pass === '' ||
        user.value.api_url === '' ||
        user.value.workspace === ''
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
