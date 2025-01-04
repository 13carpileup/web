<script setup lang="ts">
import file from '../components/file.vue'
</script>

<template>
  <main class="projects-container">
    <div class="projects-grid">
      <file fileName="topdownuniverse.md" date="2024/12">
        <div class="project-content">
          <h2 class="project-title">Top-Down Universe</h2>
          <p class="project-description">Imagine the universe, but we removed a dimension.</p>
          <p class="project-link">
            Access: 
            <a target="_blank" href="https://topdownuniverse.vercel.app/">topdownuniverse.vercel.app</a>
          </p>
          <p class="project-tech">tech: js, pixijs</p>
        </div>
      </file>
      <file fileName="nomissmayhem.md" date="2024/11">
        <div class="project-content">
          <h2 class="project-title">No Miss Mayhem</h2>
          <p class="project-description">counterspell hong kong! top 10 global finalist! etc</p>
          <p class="project-link">
            Play: 
            <a target="_blank" href="https://nomissmayhem.vercel.app/">nomissmayhem.vercel.app</a>
          </p>
          <p class="project-tech">tech: js (canvas api :skull:), rust (backend)</p>
        </div>
      </file>
      <file fileName="wiki.md" date="2024/12">
        <div class="project-content">
          <h2 class="project-title">WikiScraper</h2>
          <p class="project-description">a partially functioning wikipedia scraper</p>
          <p class="project-link">
            Access: 
            <a target="_blank" href="https://alexclimie.com/">wiki.alexclimie.com</a>
          </p>
          <p class="project-tech">tech: expressjs, postgres, py</p>
        </div>
      </file>
      <file fileName="lionel3.md" date="2024/6">
        <div class="project-content">
          <h2 class="project-title">Lionel 3</h2>
          <p class="project-description">i don't like my school's website, so I made my own version</p>
          <p class="project-tech">tech: rust</p>
        </div>
      </file>
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
  font-size: 1.5rem;
  font-weight: bold;
  color: #ffffff;
  margin: 0;
}

.project-description {
  font-size: 1rem;
  color: #d4d4d4;
  margin: 0;
}

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

@media (max-width: 768px) {
  .projects-container {
    padding: 1rem;
  }
  
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