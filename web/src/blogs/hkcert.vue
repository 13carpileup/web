<script setup lang="ts">
import blog from '../components/blog.vue'
</script>

<template>
  <main class="blog-page">
    <div class="blog-shell">
      <header class="blog-header">
        <RouterLink class="back-link" to="/blog">← back to blog</RouterLink>
        <h1 class="blog-title">HKCERT CTF</h1>
        <p class="blog-meta">2024/11/17</p>
      </header>
      <article class="blog-article">
        <blog src="test.markdown"></blog>
      </article>
      <footer class="blog-footer">
        <RouterLink class="back-link" to="/blog">← back to blog</RouterLink>
      </footer>
    </div>
  </main>
</template>

<style scoped>
.blog-page {
  width: 100%;
}

.blog-shell {
  max-width: 920px;
  margin: 0 auto;
  padding: 1.5rem 0 3rem;
  animation: pageIn 0.25s ease-out;
}

.blog-header {
  border-bottom: 1px solid var(--border-1);
  padding-bottom: 1rem;
  margin-bottom: 1.75rem;
}

.back-link {
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
  color: var(--text-3);
  text-decoration: none;
  font-size: 0.9rem;
  font-family: 'JetBrains Mono', monospace;
}

.back-link:hover {
  color: var(--text-1);
}

.blog-title {
  margin: 0.6rem 0 0.2rem;
  font-size: 2rem;
  color: var(--text-1);
  letter-spacing: 0.01em;
}

.blog-meta {
  margin: 0;
  color: var(--text-3);
  font-size: 0.9rem;
  font-family: 'JetBrains Mono', monospace;
}

.blog-article {
  color: var(--text-2);
}

.blog-footer {
  border-top: 1px solid var(--border-1);
  margin-top: 2rem;
  padding-top: 1.5rem;
}

@keyframes pageIn {
  from { opacity: 0; transform: translateY(6px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
