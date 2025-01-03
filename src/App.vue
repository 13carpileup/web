<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
</script>

<template>
  <header>    
    <div class="wrapper">
      <h2 class = "location">~/alex_climie/{{ $route.name }}</h2>
      <nav>
        <RouterLink to="/">/home</RouterLink>
        <RouterLink to="/projects">/projects</RouterLink>
        <RouterLink to="/blog">/blog</RouterLink>
      </nav>
    </div>
  </header>

  <RouterView />
</template>

<style scoped>
.location {
  color: #888;
  font-size: 1.1rem;
  margin-bottom: 0.5rem;
}

.wrapper {
  max-width: 1200px;
  margin: 0 auto;
  padding: 1rem 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #1a1a1a;
  border-radius: 4px;
}

header {
  width: 100%;
  padding: 1rem 0;
}

nav {
  display: flex;
  gap: 1rem;
  align-items: center;
}

nav a {
  color: #888;
  text-decoration: none;
  padding: 0.5rem 1rem;
  transition: color 0.3s ease;
}

nav a:hover {
  color: #fff;
}

nav a.router-link-exact-active {
  color: #fff;
}

@media (max-width: 768px) {
  .wrapper {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  
  nav {
    width: 100%;
    justify-content: flex-end;
  }
}
</style>