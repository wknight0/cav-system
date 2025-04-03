<template>
    <BaseNode :data="data" :icon="desktopIcon" :showHandles="showHandles" />
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import desktopIcon from '../../../assets/icons/wiredclient.svg';

    defineProps(['data', 'showHandles']);
</script>