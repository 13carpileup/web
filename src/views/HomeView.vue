<script setup lang="ts">
import file from '../components/file.vue'
</script>

<template>
  <main>
    <file fileName="me.md" date="2024/12" class = "content">
      <h1>hi! I'm alex, a high school student.</h1>
      <p>I love</p>
    </file>
    <file fileName="tech.md" date="2024/11" class = "content">
      <h1>hi! I'm alex, a high school student.</h1>
      <p>I love</p>
    </file>
    <file fileName="socials.md" date="1970/1" class = "content">
      <h1>hi! I'm alex, a high school student.</h1>
      <p>I love</p>
    </file>
  </main>
</template>

<style scoped>

.content {
  max-width: 85%;
  margin: auto auto 20px;
}

</style>
