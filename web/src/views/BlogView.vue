<script setup lang="ts">
import file from '../components/file.vue'
import blog from '../components/blog.vue'

</script>

<template>
  <main class="projects-container">
    <div class="projects-grid">
    <RouterLink to="/blog/hkcert.md">
      <file fileName="hkcert.md" date="2024/11/17">
        <div class="project-content">
          <h2 class="project-title">HKCERT CTF</h2>
          <p class="project-description">A writeup of the electronic cookbook cryptography challenge @ HKCERT 2024 qualifiers. Click to read!</p>
        </div>
      </file>
    </RouterLink>
    </div>
  </main>
</template>

<style scoped>
.projects-container {
  min-height: 50vh;
}

.projects-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
  max-width: 1200px;
  margin: 0 auto;
}

.project-content {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.project-title {
  font-size: 1.4rem;
  color: #ffffff;
  margin: 0;
}

.project-description {
  font-size: 1rem;
  color: #d4d4d4;
  margin: 0;
}g

.project-link a {
  font-size: 1rem;
  color: #8257e5;
  text-decoration: none;
  border-bottom: 1px dashed rgba(130, 87, 229, 0.4);
  transition: all 0.2s ease;
  padding-bottom: 1px;
}

.project-link a:hover {
  color: #9b79f7;
  border-bottom-style: solid;
  border-bottom-color: rgba(130, 87, 229, 0.8);
}

.project-tech {
  font-size: 0.9rem;
  color: #a8a8a8;
  margin: 0;
  font-style: italic;
}

a {
  text-decoration: none;
}

@media (max-width: 768px) {

  .projects-grid {
    grid-template-columns: 1fr;
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>