<template>
    <div class="loading-div">
        <ProgressSpinner style="width: 50px; height: 50px" strokeWidth="8" fill="transparent" animationDuration=".5s" />
    </div>
</template>

<script setup>
    import { ProgressSpinner } from "primevue";
</script>

<style>
    .loading-div {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
    }
</style>
