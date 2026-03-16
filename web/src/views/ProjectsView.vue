<script setup lang="ts">
import file from '../components/file.vue'
</script>

<template>
  <main class="projects-container">
    <div class="timeline">
      
      <div class="timeline-item active">
        <div class="timeline-content">
          <file fileName="avionics.c" date="2025/9 - now" gh="https://github.com/UBC-Rocket/ulysses/tree/feature/sensor_fusion">
            <div class="project-details">
              <div class="image-wrapper">
                <img src="/rocket.jpg" alt="UBC Rocket" class="project-image" />
                <div class="image-overlay"></div>
              </div>
              <h2 class="project-title">UBC Rocket</h2>
              <p class="project-description">Engineering design team. Working on kalman filters for the thrust vectoring project (self landing rocket drone)!</p>
              <p class="project-tech">tech: c, embedded systems, pcb design</p>
            </div>
          </file>
        </div>
      </div>

      <div class="timeline-item">
        <div class="timeline-content">
          <file fileName="moodle.md" date="2025/4">
            <div class="project-details">
              <h2 class="project-title">Graduation Tracker</h2>
              <p class="project-description">Graduation countdown, exam countdown, and friend analyzer. Has since been anonymized for privacy.</p>
              <p class="project-link">
                Access: 
                <a target="_blank" href="https://graduation.alexclimie.com/">graduation.alexclimie.com</a>
              </p>
              <p class="project-tech">tech: rust, vue, digitalocean</p>
            </div>
          </file>
        </div>
      </div>

      <div class="timeline-item">
        <div class="timeline-content">
          <file fileName="topdownuniverse.md" date="2024/12" gh="https://github.com/13carpileup/topdownuniverse">
            <div class="project-details">
              <h2 class="project-title">Top-Down Universe</h2>
              <p class="project-description">Imagine the universe, but we removed a dimension.</p>
              <p class="project-link">
                Access: 
                <a target="_blank" href="https://topdownuniverse.vercel.app/">topdownuniverse.vercel.app</a>
              </p>
              <p class="project-tech">tech: js, pixijs</p>
            </div>
          </file>
        </div>
      </div>

      <div class="timeline-item">
        <div class="timeline-content">
          <file fileName="nomissmayhem.md" date="2024/11" gh="https://github.com/13carpileup/nomissmayhem">
            <div class="project-details">
              <h2 class="project-title">No Miss Mayhem</h2>
              <p class="project-description">counterspell hong kong! top 10 global finalist!</p>
              <p class="project-link">
                Play: 
                <a target="_blank" href="https://nomissmayhem.vercel.app/">nomissmayhem.vercel.app</a>
              </p>
              <p class="project-tech">tech: js (canvas api 💀), rust (backend)</p>
            </div>
          </file>
        </div>
      </div>

      <div class="timeline-item">
        <div class="timeline-content">
          <file fileName="wiki.md" date="2024/12" gh="https://github.com/13carpileup/wikiscraper">
            <div class="project-details">
              <h2 class="project-title">WikiScraper</h2>
              <p class="project-description">a partially functioning wikipedia scraper</p>
              <p class="project-link">
                Access: 
                <a target="_blank" href="http://wiki.alexclimie.com:3000/">wiki.alexclimie.com</a>
              </p>
              <p class="project-tech">tech: expressjs, postgres, py</p>
            </div>
          </file>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.projects-container {
  min-height: 50vh;
  padding: 2rem 0;
}

.timeline {
  position: relative;
  max-width: 1000px;
  margin: 0 auto;
}

/* Central Line */
.timeline::after {
  content: '';
  position: absolute;
  width: 2px;
  background: linear-gradient(
    to bottom,
    color-mix(in srgb, var(--success) 80%, transparent) 0%,
    var(--border-1) 30%
  );
  top: 0;
  bottom: 0;
  left: 50%;
  margin-left: -1px;
}

.timeline-item {
  padding: 10px 40px;
  position: relative;
  width: 50%;
  box-sizing: border-box;
}

.timeline-item:nth-child(odd) { left: 0; }
.timeline-item:nth-child(even) { left: 50%; }

/* Timeline Dots */
.timeline-item::after {
  content: '';
  position: absolute;
  width: 12px;
  height: 12px;
  right: -6px;
  background-color: var(--surface-1);
  border: 1px solid var(--accent-1);
  top: 25px;
  border-radius: 50%;
  z-index: 1;
  transition: all 0.3s ease;
}

.timeline-item:nth-child(even)::after { left: -6px; }
.timeline-item:hover::after {
  background-color: var(--accent-1);
  box-shadow: none;
}

/* --- Active Item Styling (Rocket) --- */
.timeline-item.active::after {
  background-color: var(--surface-1);
  border-color: var(--success);
  box-shadow: none;
}

.image-wrapper {
  width: 100%;
  height: 120px;
  border-radius: 8px;
  overflow: hidden;
  position: relative;
  margin-bottom: 0.5rem;
  border: 1px solid var(--border-1);
}

.project-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.5s ease;
}

.image-wrapper:hover .project-image {
  transform: scale(1.05);
}

/* --- General Content Styling --- */
.project-details {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.project-title {
  font-size: 1.4rem;
  color: var(--text-1);
  margin: 0;
}

.project-description {
  font-size: 1rem;
  color: var(--text-2);
  margin: 0;
}

.project-link a {
  font-size: 1rem;
  color: var(--accent-1);
  text-decoration: none;
  border-bottom: 1px dashed color-mix(in srgb, var(--accent-1) 40%, transparent);
  transition: all 0.2s ease;
  padding-bottom: 1px;
}

.project-link a:hover {
  color: var(--accent-2);
  border-bottom-style: solid;
  border-bottom-color: color-mix(in srgb, var(--accent-2) 60%, transparent);
}

.project-tech {
  font-size: 0.9rem;
  color: var(--text-3);
  margin: 0;
  font-style: italic;
}

/* Mobile Responsiveness */
@media (max-width: 768px) {
  .timeline::after { left: 20px; }
  .timeline-item { width: 100%; padding-left: 50px; padding-right: 10px; }
  .timeline-item:nth-child(odd), .timeline-item:nth-child(even) { left: 0; }
  .timeline-item:nth-child(odd)::after, .timeline-item:nth-child(even)::after { left: 14px; }
}
</style>
