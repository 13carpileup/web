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
  color:rgb(128, 128, 128);
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
  background: #222222;
  border: 1px solid #333;
  border-radius: 10px;
  overflow: hidden;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
  position: relative;
}

.file-container:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2),
              0 0 0 1px rgba(255, 255, 255, 0.1),
              0 0 32px rgba(130, 87, 229, 0.1);
}

.file-header {
  background: #2a2a2a;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid #333;
  display: flex;
  align-items: center;
  gap: 1rem;
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
  color: #888;
  margin: 0;
  flex-grow: 1;
}

.file-content {
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  line-height: 1.6;
  padding: 1.25rem;
  font-size: 0.9rem;
  background-color: #222222;
  position: relative;
}

.file-content::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, 
    rgba(130, 87, 229, 0.2),
    rgba(214, 51, 132, 0.2)
  );
}

.file-content p {
  margin: 0.75rem 0;
  color: #d4d4d4;
}

.file-content a {
  color: #8257e5;
  text-decoration: none;
  border-bottom: 1px dashed rgba(130, 87, 229, 0.4);
  transition: all 0.2s ease;
  padding-bottom: 1px;
}

.file-content a:hover {
  color: #9b79f7;
  border-bottom-style: solid;
  border-bottom-color: rgba(130, 87, 229, 0.8);
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