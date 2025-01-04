<script setup lang="ts">
import file from '../components/file.vue'
</script>

<template>
  <main>
    <file fileName="me.md" class = "content" id = "me">
      <h1 class = "intro">hi! I'm alex, a high school student.</h1>
      <p>i am an ib victim (i.e. i spend all of my time complaining and none of it studying)</p>
    </file>
    <file fileName="tech.md" class = "content" id = "tech">
      <p class = "body">i like playing ctfs, coding barely functional programs, and hardware. i'm currently working on a legally distinct version of block blast, except with co-op!</p>
      <div class = "tech-stack">
        <span class = "tech-item">rust</span>
        <span class = "tech-item">c++</span>
        <span class = "tech-item">js</span>
        <span class = "tech-item">py</span>
        <span class = "tech-item">i use arch btw</span>
      </div>
    </file>
    <file fileName="socials.md" class = "content" id = "socials">
      <p class = "body">find me on discord (13carpileup)</p>
    </file>
  </main>
</template>

<style scoped>

.content {
  max-width: 85%;
  margin: auto auto 20px;
}

.intro {
  color: white;
}

.body {
  color: rgb(219, 219, 219);
}

.tech-stack {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
  margin-top: .8rem;
}

.tech-item {
  background: rgba(130, 87, 229, 0.1);
  color: #9b79f7;
  padding: 0.2rem 1rem;
  border-radius: 20px;
  font-size: 0.8rem;
  font-family: 'JetBrains Mono', monospace;
  border: 1px solid rgba(130, 87, 229, 0.2);
  transition: all 0.2s ease;
}
</style>
