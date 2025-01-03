<script setup lang="ts">
defineProps<{
  fileName: string,
  date: string
}>()
</script>

<template>
  <div class="file-container">
    <p class="file-meta">{{ fileName }} - {{ date }}</p>
    <div class="file-content">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped>
.file-container {
  padding: 1rem;
  border: 1px solid var(--vt-c-terminal-gray);
  border-radius: 8px;

}

.file-content:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.file-meta {
  font-family: 'Fira Code', monospace;
  font-size: 0.9rem;
  font-style: italic;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--vt-c-terminal-gray);
}

.file-content {
  font-family: 'Courier New', Courier, monospace;
  line-height: 1.6;
  color: var(--vt-c-text-dark-1);
  padding: 1rem;
  font-size: 13px;
  background-color: rgb(34, 34, 34);
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.file-content h1 {
  font-weight: 600;
  font-size: 2rem;
  color: var(--vt-c-terminal-green);
  margin: 1rem 0;
}

.file-content h3 {
  font-size: 1.2rem;
  color: var(--vt-c-text-dark-2);
  margin-bottom: 0.75rem;
}

.file-content p {
  font-size: 1rem;
  margin: 0.5rem 0;
  color: var(--vt-c-text-dark-1);
}

@media (max-width: 768px) {
  .file-container {
    padding: 0.75rem;
  }

  .file-content h1 {
    font-size: 1.8rem;
  }

  .file-content p {
    font-size: 0.95rem;
  }
}
</style>