<script setup lang="ts">
import { ref, onMounted, nextTick } from "vue";
import VueMarkdown from "vue-markdown-render";
import Prism from "prismjs";
import "prismjs/themes/prism-tomorrow.css";
import 'prismjs/components/prism-bash'
import 'prismjs/components/prism-javascript'
import 'prismjs/components/prism-json'
import 'prismjs/components/prism-liquid'
import 'prismjs/components/prism-markdown'
import 'prismjs/components/prism-markup-templating'
import 'prismjs/components/prism-php'
import 'prismjs/components/prism-scss'
import 'prismjs/components/prism-python'


let props = defineProps<{
  src: string;
}>();

let url = "../posts/" + props.src;
console.log(url);

const markdownContent = ref<string>("");

onMounted(async () => {
  try {
    const response = await fetch(url);
    console.log(response);
    if (response.ok) {
      markdownContent.value = await response.text();
      nextTick(() => {
        Prism.highlightAll();
      });
    } else {
      console.error(`Failed to load markdown file at ${url}`);
    }
  } catch (error) {
    console.error("Error loading markdown file:", error);
  }
});
</script>

<template>
  <div class="vue-markdown-render">
    <VueMarkdown :source="markdownContent" />
  </div>
</template>

<style>
.vue-markdown-render {
  line-height: 1.6;
  color: rgb(208, 208, 208);
}

.vue-markdown-render h1,
.vue-markdown-render h2,
.vue-markdown-render h3,
.vue-markdown-render h4,
.vue-markdown-render h5,
.vue-markdown-render h6 {
  font-weight: bold;
  margin-bottom: 0.5rem;
}

.vue-markdown-render p {
  margin-bottom: 1rem;
}

.vue-markdown-render pre {
  border-radius: 10px;
  padding: 1rem;
  overflow-x: auto;
}

.vue-markdown-render code {
  font-family: "Source Code Pro", monospace;
}

.vue-markdown-render img {
  max-width: 100%;
}
</style>