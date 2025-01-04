<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRoute, RouterLink, RouterView } from 'vue-router';

const route = useRoute();

const typedLocation = ref('');

function typeText(target: string, speed = 65) {
  typedLocation.value = ''; 
  let i = 0;

  const typeInterval = setInterval(() => {
    if (i < target.length) {
      typedLocation.value += target[i];
      i++;
    } else {
      clearInterval(typeInterval); 
    }
  }, speed);
}

watch(() => route.name, (newRouteName) => {
  const newLocation = `${String(newRouteName)}`;
  if (newLocation != "undefined" && newLocation != "null") {
    typeText(newLocation);
  }
  
});

</script>

<template>
  <header>    
    <div class="wrapper">
      <h2 class = "location">~/alex_climie/{{ typedLocation }}</h2>
      <nav>
        <p class = "joke">[alex@web]$ cd </p>
        <RouterLink to="/">../home</RouterLink>
        <RouterLink to="/projects">../projects</RouterLink>
        <a target = "_blank" href = "https://13carpileup.github.io/">../blog</a>
      </nav>
    </div>
  </header>
  <div class = "content">
    <RouterView />
  </div>
</template>

<style scoped>
.joke {
  color: rgb(89, 103, 85);
}

.content {
  align-items: center;
  width: 100%;
  padding: 2rem;
  margin: auto;
  background-color: #1a1a1a;
  background-image: radial-gradient(#434343 1px, transparent 2px);
  background-size: 32px 32px;
  border-radius: 10px;

}


.location {
  color: #b2ffb5;
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