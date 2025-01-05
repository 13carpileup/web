<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRoute, RouterLink, RouterView } from 'vue-router';

const route = useRoute();

const typedLocation = ref('');

function typeText(target: string, time: number, prefix = '', speed = 65) {
  typedLocation.value = prefix; 
  const n = prefix.length;
  let i = 0;

  const typeInterval = setInterval(() => {
    if (i < (target.length - n) && time == lastUpdate) {
      typedLocation.value += target[i + n];
      i++;
    } else {
      clearInterval(typeInterval); 
    }
  }, speed);
}

function deleteText(n: number, time: number, speed = 65) {
  return new Promise<void>((resolve) => {
    let i = 0;

    const typeInterval = setInterval(() => {
      if (i < n && time === lastUpdate) {
        typedLocation.value = typedLocation.value.slice(0, -1);
        i++;
      } else {
        clearInterval(typeInterval);
        resolve();
      }
    }, speed);
  });
}

let lastUpdate = 0;
let lastRoute = ''

watch(() => route.name, async (newRouteName) => {
  lastUpdate = Date.now();
  let newLocation = `${String(newRouteName)}`;

  if (newLocation != "undefined" && newLocation != "null") {
    // you see.. by recalling Date.now(), we avoid a race condition.... very important stuff ....
    if (newLocation.startsWith(lastRoute)) {
      typeText(newLocation, Date.now(), lastRoute);
    }
    else if (lastRoute.startsWith(newLocation)) {
      deleteText(lastRoute.length - newLocation.length, Date.now());
    }
    else {
      await deleteText(lastRoute.length, lastUpdate, 45);
      typeText(newLocation, lastUpdate);
    }
    lastRoute = newLocation;
  }
  
});

</script>

<template>
  <header>    
    <div class="wrapper">
      <h2 class = "location">~/alex_climie/{{ typedLocation }}</h2>
      <nav>
        <p class = "joke">[alex@web]$ cd </p>
        <RouterLink class="link" to="/"><span class = "phone-hide">../</span>home</RouterLink>
        <RouterLink class="link" to="/projects"><span class = "phone-hide">../</span>projects</RouterLink>
        <RouterLink class="link" to="/blog"><span class = "phone-hide">../</span>blog</RouterLink>
      </nav>
    </div>
</header>
  <div class = "content">
    <RouterView />
  </div>

  <div class ="preloading">
    <img class="github-image" src="/gh.png" alt="GitHub"/>
    <img class="github-image" src="/back.png" alt="Back"/>
  </div>
</template>

<style scoped>
.preloading {
  display: none;
}

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
    justify-content: center;
  }

  .location {
    display: none;
  }

  .joke {
    display: none;
  }

  .phone-hide {
    display: none;
  }


}
</style>