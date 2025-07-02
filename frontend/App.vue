<template>
  <div id="app">
    <router-view></router-view>
  </div>
</template>

<script>
  import { invoke } from '@tauri-apps/api/tauri';

  const initializeStorage = async () => {
    try {
      await invoke('initialize_storage');
    } catch (error) {
      console.error('Failed to initialize storage:', error);
    }
  };

  initializeStorage();
</script>

<style>
  :root {
    --primary-color: #18181b;
    --primary-text-color: white;
    --secondary-color: #6ee7b7;
    --secondary-text-color: black;
    --tertiary-color: #939393;
    --warning-color: rgb(177, 51, 51);
  }

  body {
    margin: 0;
    height: 100vh;
    font-family: 'Roboto', sans-serif;
  }
</style>