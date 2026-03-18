<script setup lang="ts">
type Book = {
  title: string;
  shortTitle: string;
  author: string;
  progress: number;
  note?: string;
  color: string;
  height: number; // px
};

const books: Book[] = [
  {
    title: 'Axiomatic',
    shortTitle: 'Axiomatic',
    author: 'Greg Egan',
    progress: 70,
    note: 'LOVE GREG EGAN',
    color: '#9b79f7',
    height: 150,
  },
  {
    title: 'Introduction to the Theory of Computation',
    shortTitle: 'ToC',
    author: 'Michael Sipser',
    progress: 20,
    note: 'lowkey overdue on this loan',
    color: '#22c55e',
    height: 185,
  }
];

</script>

<template>
  <div class="bookshelf">
    <div class="shelf-inner">
        <div
        v-for="book in books"
        :key="book.title"
        class="book-item"
        :style="{
            '--accent': book.color,
            height: book.height + 'px'
        }"
        >
        <div class="spine">
          <div class="spine-text-container">
             <span class="spine-text">{{ book.shortTitle }}</span>
          </div>
          <div class="progress-glow" :style="{ height: book.progress + '%' }"></div>
        </div>

        <div class="expansion-content">
          <div class="details-inner">
            <div class="meta">
              <span class="full-title">{{ book.title }}</span>
              <span class="author">by {{ book.author }}</span>
            </div>
            
            <div class="progress-row">
              <div class="track"><div class="bar" :style="{ width: book.progress + '%' }"></div></div>
              <span class="percent">{{ book.progress }}%</span>
            </div>

            <p v-if="book.note" class="note">// {{ book.note }}</p>
          </div>
        </div>
      </div>
    </div>
    <div class="shelf-line"></div>
  </div>
</template>

<style scoped>
.bookshelf {
  margin-top: 1rem;
  width: 100%;
}

.shelf-inner {
  display: flex;
  align-items: flex-end;
  gap: 10px;
}

.book-item {
  display: flex;
  width: 52px; /* Fixed collapsed width */
  min-height: 160px;
  background: color-mix(in srgb, var(--surface-1) 85%, transparent);
  border: 1px solid var(--border-1);
  border-bottom: none;
  border-radius: 6px 6px 0 0;
  overflow: hidden;
  transition: width 0.4s cubic-bezier(0.23, 1, 0.32, 1), background 0.3s;
}

.book-item:hover {
  width: 340px; /* Room for long wrapped titles */
  background: color-mix(in srgb, var(--accent) 16%, var(--surface-1));
  border-color: var(--accent);
}

.spine {
  min-width: 52px;
  height: 100%;
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center; /* <-- REAL CENTERING */
  border-right: 1px solid var(--border-1);
}

.spine-text-container {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}


.spine-text {
  writing-mode: vertical-rl;
  transform: rotate(180deg);
  white-space: nowrap;
  color: var(--text-1);
  font-size: 0.85rem;
  font-family: 'JetBrains Mono', monospace;
  opacity: 0.8;
}

.book-item:hover .spine-text {
  opacity: 1;
  color: var(--accent);
}

.progress-glow {
  position: absolute;
  left: 0;
  bottom: 0;
  width: 3px;
  background: var(--accent);
  box-shadow: none;
}

.expansion-content {
  flex: 1;
  opacity: 0;
  transition: opacity 0.3s ease-in-out;
}

.book-item:hover .expansion-content {
  opacity: 1;
}

.details-inner {
  padding: 1.25rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  /* Allow the container to define the width */
  width: 280px; 
}

.full-title {
  color: var(--text-1);
  font-size: 1.05rem;
  font-weight: 700;
  line-height: 1.25;
  display: block;
  white-space: normal; /* THIS ENABLES WRAPPING */
  word-wrap: break-word;
}

.author {
  color: var(--accent);
  font-size: 0.8rem;
  font-family: 'JetBrains Mono', monospace;
}

.progress-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 4px;
}

.track {
  flex: 1;
  height: 6px;
  background: color-mix(in srgb, var(--surface-2) 80%, transparent);
  border-radius: 999px;
}

.bar {
  height: 100%;
  background: var(--accent);
  border-radius: 999px;
}

.percent {
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.75rem;
  color: var(--text-3);
}

.note {
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.75rem;
  color: var(--text-3);
  margin: 0;
  white-space: normal;
  line-height: 1.4;
}

.shelf-line {
  width: 100%;
  height: 1px;
  background: var(--border-1);
}
</style>
