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
  background: rgba(255, 255, 255, 0.9);
  padding: 1rem;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
  transition: transform 0.3s ease, z-index 0.1s;
  z-index: 1;
}

.image-card:hover {
  transform: scale(1.05) rotate(0deg) !important;
  z-index: 10;
}

.img {
  width: 100%;
  height: auto;
  object-fit: cover;
  border: 1px solid #333;
}

.image-info {
  margin-top: 0.5rem;
}

.image-info h3 {
  margin: 0;
  font-size: 1rem;
  text-transform: capitalize;
  color: #111;
}

.image-info p {
  margin: 0.25rem 0 0;
  font-size: 0.8rem;
  color: #444;
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
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
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
  stroke: #fff;
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