<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <header>    
    <div class="wrapper">
      <h2 class = "location">~/alex_climie/{{ $route.name }}</h2>
      <nav>
        <RouterLink to="/">/home</RouterLink>
        <RouterLink to="/projects">/projects</RouterLink>
        <a target = "_blank" href = "https://13carpileup.github.io/">/blog</a>
      </nav>
    </div>
  </header>
  <div class = "content">
    <RouterView />
  </div>
</template>

<style scoped>
.content {
  align-items: center;
  width: 100%;
  padding: 2rem;
  margin: auto;
  background-color: #1a1a1a;
  background-image: radial-gradient(#2a2a2a 1px, transparent 1px);
  background-size: 32px 32px;
  border-radius: 10px;

}


.location {
  color: #acdeae;
  font-size: 1.1rem;
}

.wrapper {
  max-width: 1400px;
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