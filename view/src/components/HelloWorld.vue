<script setup lang="ts">
import { ref } from 'vue'
import { greet } from '../bindings'

defineProps<{
  msg: string
}>()

const msg1 = ref('Hello World111!')
const count = ref(0)
const name = ref('name')

async function increment() {
  msg1.value = await greet('name')
}
</script>

<template>
  <div class="greetings">
    <h1 class="green">{{ msg1 }}</h1>
    <button @click="increment">commit</button>
    <input type="text" v-model="name" />
    <h3>
      You’ve successfully created a project with
      <a href="https://vitejs.dev/" target="_blank" rel="noopener">Vite</a> +
      <a href="https://vuejs.org/" target="_blank" rel="noopener">Vue 3</a>. What's next?
    </h3>
  </div>
</template>

<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
  position: relative;
  top: -10px;
}

h3 {
  font-size: 1.2rem;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

@media (min-width: 1024px) {
  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
}
</style>
