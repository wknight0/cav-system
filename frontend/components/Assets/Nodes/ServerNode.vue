<template>
    <BaseNode :data="data" :icon="serverIcon" :showHandles="showHandles" />
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import serverIcon from '../../../assets/icons/server.svg';

    defineProps(['data', 'showHandles']);
</script>