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

        <Control v-if="isRegister" />
        <Register v-else />
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Control from './components/Control.vue'
import Register from './components/Register.vue'
import { invoke } from '@tauri-apps/api/tauri'

interface User {
    name: string
    api_pass: string
    workspace: string
}

const isRegister = ref(false)
const user = ref({
    name: '',
    api_pass: '',
    workspace: '',
} as User)

onMounted(async() => {
    user.value = JSON.parse(await invoke('get_user'))

    console.log(user.value)

    if (user.value.name === '' || user.value.api_pass === '' || user.value.workspace === '') {
        isRegister.value = false
    } else {
        isRegister.value = true

        // run_timer(counter)
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
