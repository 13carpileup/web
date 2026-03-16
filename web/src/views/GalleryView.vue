<script setup lang="ts">
import file from '../components/file.vue'
import { onMounted, ref } from 'vue'

const images = [
    {url: 'gallery/meowzers.jpg', title: "mr meow", location:"hong kong", date: "2025/03/26"},
]



onMounted(() => {
 
})
</script>

<template>
  <main class="gallery-container" ref="containerRef">

 
    

  </main>
</template>

<style scoped>
.gallery-container {
  position: relative;
  width: 100%;
  height: 100vh;
  overflow: hidden;
}

.image-card {
  position: absolute;
  width: 250px;
  background: color-mix(in srgb, var(--surface-1) 92%, transparent);
  padding: 1rem;
  border: 1px solid var(--border-1);
  box-shadow: 0 2px 6px var(--shadow-1);
  transition: transform 0.3s ease, z-index 0.1s;
  z-index: 1;
  border-radius: 8px;
}

.image-card:hover {
  transform: scale(1.05) rotate(0deg) !important;
  z-index: 10;
}

.img {
  width: 100%;
  height: auto;
  object-fit: cover;
  border: 1px solid var(--border-1);
  border-radius: 6px;
}

.image-info {
  margin-top: 0.5rem;
}

.image-info h3 {
  margin: 0;
  font-size: 1rem;
  text-transform: capitalize;
  color: var(--text-1);
}

.image-info p {
  margin: 0.25rem 0 0;
  font-size: 0.8rem;
  color: var(--text-2);
}

.pin {
  position: absolute;
  top: 10px;
  left: 50%;
  transform: translateX(-50%);
  width: 12px;
  height: 12px;
  background-color: #ff3333;
  border-radius: 50%;
  border: 1px solid #990000;
  box-shadow: none;
}

.connections {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 0;
  pointer-events: none;
}

.string-line {
  stroke: var(--text-3);
  stroke-width: 1;
  stroke-dasharray: 5, 5;
  opacity: 0.6;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .image-card {
    width: 200px;
  }
}

@media (max-width: 480px) {
  .image-card {
    width: 150px;
  }
}
</style>
