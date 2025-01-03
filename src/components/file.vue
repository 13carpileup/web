<script setup lang="ts">
defineProps<{
  fileName: string
}>()
</script>

<template>
  <p>{{ fileName }}</p>
  <slot></slot>
</template>

<style scoped>
p {
  font-family: 'Fira Code', monospace;
  color: var(--vt-c-terminal-gray);
  padding: 0.5rem 0;
  border-bottom: 1px solid var(--vt-c-terminal-gray);
  margin-bottom: 1rem;
}

h1 {
  font-weight: 500;
  font-size: 2.2rem;
  color: var(--vt-c-terminal-green);
  margin: 1rem 0;
}

h3 {
  font-size: 1.2rem;
  color: var(--vt-c-text-dark-1);
}

.greetings h1,
.greetings h3 {
  text-align: left;
}
</style>
