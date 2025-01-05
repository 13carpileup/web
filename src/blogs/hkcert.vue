<script setup lang="ts">
import { ref, onMounted } from 'vue'
import file from '../components/file.vue'
import blog from '../components/blog.vue'

const isLoading = ref(true)

onMounted(() => {
  setTimeout(() => {
    isLoading.value = false
  }, 200)
})
</script>

<template>
    <file fileName="hkcert.md" date="2024/11/17">
        <div v-if="isLoading" class="loading-container">
            <div class="spinner"></div>
        </div>
        <blog v-else src="test.markdown"></blog>
    </file>
</template>

<style scoped>
.loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 200px;
}

.spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #f3f3f3;
    border-top: 4px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 1rem;
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}
</style>