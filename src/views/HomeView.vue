<script setup lang="ts">
import { ref, onMounted } from 'vue';
import file from '../components/file.vue';

const searchTerm = ref('');

const searchResults = ref<{uri: string, id:string, name:string, artists:{name:string}[]}[]>([]);
const isSearching = ref(false);


const searchSongs = async () => {
  if (!searchTerm.value) {
    searchResults.value = [];
    return;
  }
  
  isSearching.value = true;
  try {
    const response = await fetch(`https://wiki.alexclimie.com/api/search?search_term=${encodeURIComponent(searchTerm.value)}`);
    const data = await response.json();
    searchResults.value = data;
  } catch (error) {
    console.error('Error searching songs:', error);
  } finally {
    isSearching.value = false;
  }
};

const addSongToQueue = async (uri: string) => {
  try {
    await fetch(`https://wiki.alexclimie.com/api/add_song?uri=${encodeURIComponent(uri)}`);
    searchTerm.value = '';
    searchResults.value = [];
  } catch (error) {
    console.error('Error adding song to queue:', error);
  }
};

onMounted(async () => {
  const response = await fetch('https://wiki.alexclimie.com/api/freshen_data');
})
</script>

<template>
<main class="all">
  <file fileName="me.md" class="content" id="me">
    <h1 class="intro">Hi! I'm Alex, an incoming UBC student.</h1>
    <p>I am studying computer science and mathematics.</p>
  </file>

  <file fileName="tech.md" class="content" id="tech">
    <p class="body">I have a lot of interests! Theoretical CS (algorithms, computation, etc) and abstract math form a significant subset of them, though.</p>
    <div class="tech-stack">
      <span class="tech-item">Rust</span>
      <span class="tech-item">C++</span>
      <span class="tech-item">JS</span>
      <span class="tech-item">py</span>
      <span class="tech-item">archbtw</span>
    </div>
  </file>

  <file fileName="music.md" class="content" id="socials">
    <p class="body">Reach out via email for anything formal (climiealex@gmail.com), or discord for anything at all (13carpileup).</p>
  </file>

  <file fileName="songs.md" class="content" id="songs">
    <h2 class="body">Send a song request</h2>
    <div class="search-container">
      <input 
        type="text" 
        v-model="searchTerm" 
        @input="searchSongs"
        placeholder="Search for a song..."
        class="search-input"
      />
      <div v-if="searchResults.length > 0" class="search-results">
        <div 
          v-for="song in searchResults" 
          :key="song.id"
          @click="addSongToQueue(song.uri)"
          class="search-result-item"
        >
          <span class="song-name">{{ song.name }}</span>
          <span class="artist-name">{{ song.artists[0].name }}</span>
        </div>
      </div>
    </div>
    <br>
    <p>Your suggested song will be automatically added to my spotify queue</p>
  </file>
</main>
</template>

<style scoped>
.all {
  padding-left: 8rem;
  padding-right: 8rem;
}

.content:not(:last-child) {
  margin-bottom: 1.5rem;
}

.content {
  margin: auto;
  overflow: visible;
}

.intro {
  color: white;
}

.body {
  color: rgb(219, 219, 219);
}

.tech-stack {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
  margin-top: .8rem;
}

.tech-item {
  background: rgba(130, 87, 229, 0.1);
  color: #9b79f7;
  padding: 0.2rem 1rem;
  border-radius: 20px;
  font-size: 0.8rem;
  font-family: 'JetBrains Mono', monospace;
  border: 1px solid rgba(130, 87, 229, 0.2);
  transition: all 0.2s ease;
}

.search-container {
  position: relative;
  margin-top: 1rem;
}

.search-input {
  width: 100%;
  padding: 0.5rem 1rem;
  background: rgba(130, 87, 229, 0.1);
  border: 1px solid rgba(130, 87, 229, 0.2);
  border-radius: 20px;
  color: white;
  font-family: 'JetBrains Mono', monospace;
  outline: none;
}

.search-results {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: rgba(18, 18, 18, 0.95);
  border: 1px solid rgba(130, 87, 229, 0.2);
  border-radius: 10px;
  margin-top: 0.5rem;
  max-height: 300px;
  overflow-y: scroll;
  z-index: 10;
}

.search-result-item {
  padding: 0.75rem 1rem;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  transition: background-color 0.2s ease;
}

.search-result-item:hover {
  background: rgba(130, 87, 229, 0.1);
}

.song-name {
  color: white;
  font-size: 0.9rem;
}

.artist-name {
  color: rgb(219, 219, 219);
  font-size: 0.8rem;
}

@media (max-width: 768px) {
  .all {
    padding-left: 0rem;
    padding-right: 0rem;
  }
}
</style>