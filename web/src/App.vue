<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { useRoute, RouterLink, RouterView } from 'vue-router';

const route = useRoute();

const typedLocation = ref('');
const theme = ref<'dark' | 'light'>('dark');

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

const newRouteName = ref<string | null>(null);

onMounted(() => {
  newRouteName.value = route.name as string | null;
  updateLocation();

  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'light' || savedTheme === 'dark') {
    applyTheme(savedTheme);
  } else {
    const prefersLight = window.matchMedia?.('(prefers-color-scheme: light)')?.matches;
    applyTheme(prefersLight ? 'light' : 'dark');
  }
});

watch(() => route.name, (newRoute) => {
  newRouteName.value = newRoute as string | null;
  updateLocation();
});

async function updateLocation() {
  if (!newRouteName.value) return;

  lastUpdate = Date.now();
  let newLocation = `${String(newRouteName.value)}`;

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
}

function applyTheme(value: 'dark' | 'light') {
  theme.value = value;
  document.documentElement.setAttribute('data-theme', value);
  localStorage.setItem('theme', value);
}

function toggleTheme() {
  applyTheme(theme.value === 'dark' ? 'light' : 'dark');
}
</script>

<template>
  <header>    
    <div class="wrapper">
      <div class="location-container">
        <h2 class="location">~/alex_climie/{{ typedLocation }}</h2>
      </div>
      <nav>
        <p class="joke">[alex@web]$ cd </p>
        <RouterLink class="link" to="/"><span class="phone-hide">../</span>home</RouterLink>
        <RouterLink class="link" to="/projects"><span class="phone-hide">../</span>projects</RouterLink>
        <RouterLink class="link" to="/blog"><span class="phone-hide">../</span>blog</RouterLink>
        <button
          type="button"
          class="theme-toggle-icon"
          :class="{ 'is-dark': theme === 'dark' }"
          @click="toggleTheme"
          :aria-label="theme === 'dark' ? 'Switch to light theme' : 'Switch to dark theme'"
          :title="theme === 'dark' ? 'Switch to light theme' : 'Switch to dark theme'"
        >
          <svg class="icon icon-sun" viewBox="0 0 24 24" aria-hidden="true">
            <circle cx="12" cy="12" r="4.5" />
            <line x1="12" y1="2" x2="12" y2="5" />
            <line x1="12" y1="19" x2="12" y2="22" />
            <line x1="2" y1="12" x2="5" y2="12" />
            <line x1="19" y1="12" x2="22" y2="12" />
            <line x1="4.2" y1="4.2" x2="6.7" y2="6.7" />
            <line x1="17.3" y1="17.3" x2="19.8" y2="19.8" />
            <line x1="4.2" y1="19.8" x2="6.7" y2="17.3" />
            <line x1="17.3" y1="6.7" x2="19.8" y2="4.2" />
          </svg>
          <svg class="icon icon-moon" viewBox="0 0 24 24" aria-hidden="true">
            <path
              d="M21 12.5c-1.4.6-3 .9-4.7.7-4.1-.4-7.3-3.8-7.3-7.9 0-1.1.2-2.1.6-3.1-3.6 1.2-6.2 4.6-6.2 8.6 0 5 4.1 9.1 9.1 9.1 3.9 0 7.2-2.4 8.5-5.9z"
            />
          </svg>
        </button>
      </nav>
    </div>
  </header>
  <div class="content">
    <RouterView v-slot="{ Component }">
      <Transition name="page-fade" mode="out-in">
        <component :is="Component" />
      </Transition>
    </RouterView>
  </div>

  <div class="preloading">
    <img class="github-image" src="/gh.png" alt="GitHub"/>
    <img class="github-image" src="/back.png" alt="Back"/>
  </div>
</template>

<style scoped>
.preloading {
  display: none;
}

.joke {
  color: var(--text-3);
}

.content {
  align-items: center;
  width: 100%;
  padding: 2rem 0;
  margin: 0 auto;
  background-color: transparent;
}

.location-container {
  display: flex;
  align-items: center;
}

.location {
  color: var(--accent-1);
  font-family: "Nunito", sans-serif;
  font-size: 1.1rem;
  font-weight: 300;
  letter-spacing: 0.8px;
  transition: all 0.3s ease;
}

.wrapper {
  max-width: 1400px;
  margin: 0 auto;
  padding: 1rem 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: transparent;
}

header {
  width: 100%;
  padding: 1rem 0;
  border-bottom: 1px solid var(--border-1);
}

nav {
  display: flex;
  gap: 1rem;
  align-items: center;
}

nav a {
  color: var(--text-3);
  text-decoration: none;
  padding: 0.5rem 1rem;
  border-bottom: 1px solid transparent;
  transition: color 0.3s ease;
}

nav a:hover {
  color: var(--text-1);
  border-bottom-color: var(--border-1);
}

nav a.router-link-exact-active {
  color: var(--text-1);
  border-bottom-color: var(--accent-1);
}

.theme-toggle-icon {
  width: 32px;
  height: 32px;
  padding: 0;
  border-radius: 6px;
  border: 1px solid var(--border-1);
  background: var(--surface-1);
  color: var(--text-2);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  position: relative;
  cursor: pointer;
  transition: background 0.2s ease, color 0.2s ease;
}

.theme-toggle-icon:hover {
  color: var(--text-1);
}

.theme-toggle-icon .icon {
  width: 18px;
  height: 18px;
  position: absolute;
  opacity: 0;
  transform: scale(0.9);
  transition: opacity 0.2s ease, transform 0.2s ease;
  fill: none;
  stroke: currentColor;
  stroke-width: 1.6;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.theme-toggle-icon .icon-moon {
  fill: currentColor;
  stroke: none;
}

.theme-toggle-icon.is-dark .icon-sun {
  opacity: 1;
  transform: scale(1);
}

.theme-toggle-icon:not(.is-dark) .icon-moon {
  opacity: 1;
  transform: scale(1);
}

@media (max-width: 768px) {
  .wrapper {
    display:block;
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  
  nav {
    width: 100%;
    justify-content: center;
    flex-wrap: wrap;
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

:global(.page-fade-enter-active),
:global(.page-fade-leave-active) {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

:global(.page-fade-enter-from),
:global(.page-fade-leave-to) {
  opacity: 0;
  transform: translateY(6px);
}
</style>
