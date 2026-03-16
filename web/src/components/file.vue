<script setup lang="ts">

defineProps<{
  fileName: string,
  date?: string,
  gh?: string,
  back?: string
}>()

</script>

<template>
  <div class="file-container">
    <div class="file-header">
      <p class="file-meta">
        {{ fileName }} 
        <span v-if="date">- {{ date }}</span> 
        <span v-if="gh" class="github-link">
          <a target="_blank" :href="gh" class="github-anchor">
            <img class="github-image" src="/gh.png" alt="GitHub"/>
          </a>
        </span>
        <span v-if="back" class="github-link">
          <RouterLink :to="back" class="github-anchor">
            <img class="github-image" src="/back.png" alt="Back"/>
          </RouterLink>
        </span>
      </p>
    </div>
    <div class="file-content">
      <slot></slot>
    </div>
    <div v-if="back" class="file-header">
      <p class="file-meta">
        <RouterLink :to="back" class="github-anchor">
        <span class="github-link" id="bottom-link">
            <img class="github-image" src="/back.png" alt="Back"/>
        </span>
        <span class="footer-text">return to blogs</span>
        </RouterLink>
      </p>
    </div>
  </div>
</template>

<style scoped>

.footer-text {
  font-style: italic;
  margin-left: 2rem;
  color: var(--text-3);
}

a {
  text-decoration: none;
}

.github-image {
  width: 18px; 
  height: 18px;
  opacity: 0.7; 
  transition: opacity 0.2s ease;
  vertical-align: middle; 
  margin-left: 8px; 
  filter: var(--icon-filter);
}

.github-link {
  float: right;
  display: inline-flex;
  align-items: center;
}

.github-anchor {
  display: inline-flex;
  align-items: center;
  padding: 4px;
  border-radius: 4px;
  transition: background-color 0.2s ease;
}

.github-anchor:hover .github-image {
  opacity: 1;
}

#bottom-link {
  float:left;
}

.file-container {
  background: var(--surface-1);
  border: 1px solid var(--border-1);
  border-radius: 8px;
  overflow: hidden;
  transition: box-shadow 0.2s ease, border-color 0.2s ease;
  position: relative;
}

.file-header {
  background: var(--surface-3);
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border-1);
  display: flex;
  align-items: center;
  gap: 1rem;
}

.file-header {
  border-radius: 0;
}

.window-controls {
  display: flex;
  gap: 6px;
}

.window-controls span {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #ff5f56;
}

.window-controls span:nth-child(2) {
  background: #ffbd2e;
}

.window-controls span:nth-child(3) {
  background: #27c93f;
}

.file-meta {
  font-family: 'Fira Code', monospace;
  font-size: 0.9rem;
  color: var(--text-3);
  margin: 0;
  flex-grow: 1;
}

.file-content {
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  line-height: 1.6;
  padding: 1.25rem;
  font-size: 0.9rem;
  background-color: var(--surface-1);
  position: relative;
}

.file-content p {
  margin: 0.75rem 0;
  color: var(--text-2);
}

.file-content a {
  color: var(--accent-1);
  text-decoration: none;
  border-bottom: 1px dashed color-mix(in srgb, var(--accent-1) 40%, transparent);
  transition: all 0.2s ease;
  padding-bottom: 1px;
}

.file-content a:hover {
  color: var(--accent-2);
  border-bottom-style: solid;
  border-bottom-color: color-mix(in srgb, var(--accent-2) 60%, transparent);
}

@media (max-width: 768px) {
  .file-container {
    margin-bottom: 1rem;
  }

  .file-header {
    padding: 0.5rem 0.75rem;
  }

  .file-content {
    padding: 1rem;
  }
}

@keyframes typing {
  from { opacity: 0; transform: translateY(4px); }
  to { opacity: 1; transform: translateY(0); }
}

.file-content p {
  animation: typing 0.3s ease-out forwards;
  animation-delay: calc(var(--index, 0) * 0.1s);
}

.file-content p:nth-child(1) { --index: 1; }
.file-content p:nth-child(2) { --index: 2; }
.file-content p:nth-child(3) { --index: 3; }
</style>
