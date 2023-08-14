<template>
    <main>
      <router-view />
    </main>
</template>

<script lang="ts">

export default {
  name: "App",
  created() {
    document.title = "SmiteAnalyzer";
  },
};
</script>



<style>
</style>