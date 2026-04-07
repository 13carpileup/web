<script setup lang="ts">
import file from '../components/file.vue'

type Photo = {
  id: number;
  src: string;
  date: string;
  desc: string;
};

const sources = [
  { src: '/gallery/20260407.jpg', date: '2026/04/07', desc: "i don't think that you're allowed to park there"},
  { src: '/gallery/20260326.jpg', date: '2026/03/26', desc: 'downtown vancouver' },
  { src: '/gallery/20260307.jpg', date: '2026/03/07', desc: 'geese on the roof' },
  { src: '/gallery/20260224.jpg', date: '2026/02/24', desc: 'stuck in a lab' },
  { src: '/gallery/20260217.jpg', date: '2026/02/17', desc: 'CONCRETE JUNGLE WHERE DREAMS ARE MADE OF' },
  { src: '/gallery/20260217_1.jpg', date: '2026/02/17', desc: 'chill guys' },
  { src: '/gallery/20260119.jpg', date: '2026/01/19', desc: 'foggy day' },
  { src: '/gallery/20251231.jpg', date: '2025/12/31', desc: 'salt spring' },
  { src: '/gallery/20251231_1.jpg', date: '2025/12/31', desc: 'beautiful little guy' },
  { src: '/gallery/20251223.jpg', date: '2025/12/23', desc: 'passionate montreal streets' },
  { src: '/gallery/20251031.jpg', date: '2025/10/31', desc: 'fall guy spotted' },
  { src: '/gallery/20251018.jpg', date: '2025/10/18', desc: 'grindin on grouse' },
  { src: '/gallery/20250628.jpg', date: '2025/06/28', desc: 'swiss cat' },
  { src: '/gallery/20250617.jpg', date: '2025/06/17', desc: 'italian cat' },
  { src: '/gallery/meowzers.jpg', date: '2025/03/26', desc: 'hong kong cat' },
];

const photos: Photo[] = Array.from({ length: sources.length }, (_, i) => {
  const base = sources[i];
  return {
    id: sources.length - i,
    src: base.src,
    date: base.date,
    desc: base.desc,
  };
});
</script>

<template>
  <main class="gallery-page">
    <div class="gallery-grid">
      <file
        v-for="photo in photos"
        :key="photo.id"
        class="photo-file"
        :fileName="`photo_${String(photo.id).padStart(2, '0')}.jpg`"
        :date="photo.date"
      >
        <div class="photo-body">
          <img :src="photo.src" :alt="`Photo ${photo.id}`" loading="lazy" />
          <p class="photo-desc">{{ photo.desc }}</p>
        </div>
      </file>
    </div>
  </main>
</template>

<style scoped>
.gallery-page {
  width: 100%;
}

.gallery-grid {
  columns: 3 260px;
  column-gap: 16px;
}

.photo-file :deep(.file-content) {
  padding: 0.75rem;
}

.photo-file {
  display: inline-block;
  width: 100%;
  margin: 0 0 16px;
  break-inside: avoid;
}

.photo-body {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.photo-body img {
  width: 100%;
  height: auto;
  display: block;
  border: 1px solid var(--border-1);
  border-radius: 6px;
}

.photo-desc {
  margin: 0;
  font-size: 0.8rem;
  color: var(--text-3);
  font-family: 'JetBrains Mono', monospace;
}

@media (max-width: 768px) {
  .gallery-grid {
    columns: 2 200px;
    column-gap: 12px;
  }
}

@media (max-width: 520px) {
  .gallery-grid {
    columns: 1 100%;
  }
}
</style>
