<script setup>
import { nextTick, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

import { registerAll, unregisterAll, unregister, isRegistered } from '@tauri-apps/plugin-global-shortcut';
const keys = [
  'W','O','F','U','L','E','Enter'
]

const replay_keys = ref([])

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

// window.addEventListener('keydown', (e) => {
//   console.log(e)
// })

// window.addEventListener('keyup', (e) => {
//   console.log(e)
// })

const start = async () => {
  replay_keys.value = []
  for (let key of keys) {
    if (await isRegistered(key)) {
      await unregister(key)
    }
  }
  await registerAll(keys, (shortcut, ...arg) => {
    replay_keys.value.push(shortcut)
    console.log(`Shortcut ${shortcut} triggered`);
    console.log(...arg)
  })
}
const input = ref(null)
const replay = async () => {
  setTimeout(() => {
    nextTick(async () => {
      // input.value.foucs()
      const el = document.querySelector('#greet-input')
      el.focus()
      for (let key of keys) {
        await unregister(key)
      }
      // await unregisterAll(keys);
      // console.log(replay_keys.value)
      for (let key of replay_keys.value) {
        await invoke('keydown', { key });
      }
    })
  }, 3 * 1000);
}
</script>

<template>
  <!-- <form class="row" @submit.prevent="greet">
    <button type="submit">Greet</button>
  </form> -->
  <input ref="input" id="greet-input" v-model="name" placeholder="Enter a name..." />

  <button @click="start">开始录制</button>
  <button @click="replay">回放</button>

  <p>{{ greetMsg }}</p>
</template>
