import dayjs from '../node_modules/dayjs/dayjs.min'

const { invoke } = window.__TAURI__.tauri

// let greetInputEl
// let greetMsgEl

// async function greet() {
//     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//     // greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
//     let response = await invoke('greet', { name: 'Joe' })
//     console.log(JSON.parse(response))
//     greetMsgEl.textContent = JSON.parse(response).name
// }

async function loadConfig() {
    let response = await invoke('get_user')

    return JSON.parse(response)
}

async function saveConfig(user, pass, work) {
    const userObj = { user: { name: user, api_pass: pass, workspace: work } }

    await invoke('save_user', userObj)
}

async function run_timer(counter) {
    setInterval((counter) => {
        counter.textContent = dayjs().format('HH:mm:ss')
        console.log(dayjs().format('HH:mm:ss'))
    }, 1000);
}

window.addEventListener('DOMContentLoaded', async () => {
    const controllerDiv = document.querySelector('#controller')
    const credentialDiv = document.querySelector('#credentials')
    const counter = document.querySelector('#counter')

    greetInputEl = document.querySelector('#greet-input')
    greetMsgEl = document.querySelector('#greet-msg')

    const user = document.querySelector('#user-input')
    const pass = document.querySelector('#pass-input')
    const work = document.querySelector('#work-input')

    const config = await loadConfig()

    if (config.name === '' || config.api_pass === '' || config.workspace === '') {
        controllerDiv.classList.add('hidden')
        credentialDiv.classList.remove('hidden')
    } else {
        user.value = config.name
        pass.value = config.api_pass
        work.value = config.workspace

        run_timer(counter)
    }

    // document.querySelector('#greet-button').addEventListener('click', () => greet())
    document
        .querySelector('#save-button')
        .addEventListener('click', () => {
            saveConfig(user.value, pass.value, work.value)
            config.name = user.value
            config.api_pass = pass.value
            config.workspace = work.value
        })
})
