<script setup lang="ts">
import { ref, onMounted } from "vue";
import VueMarkdown from "vue-markdown-render"; 

let props = defineProps<{
  src: string; 
}>();

let url = "../public/posts/" + props.src;
console.log(url);

const markdownContent = ref<string>(""); 

onMounted(async () => {
  try {
    const response = await fetch(url); 
    console.log(response);
    if (response.ok) {
      markdownContent.value = await response.text(); 
    } else {
      console.error(`Failed to load markdown file at ${url}`);
    }
  } catch (error) {
    console.error("Error loading markdown file:", error);
  }
});
</script>

<template>
  <div>
    <VueMarkdown :source="markdownContent" />
  </div>
</template>


<style>
code {
    width: 100%;
    height:100%;
}
</style>