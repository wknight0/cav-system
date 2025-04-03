<template>
    <BaseNode :data="data" :icon="storageIcon" :showHandles="showHandles" />
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import storageIcon from '../../../assets/icons/storage.svg';

    defineProps(['data', 'showHandles']);
</script>