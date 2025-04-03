<template>
    <BaseNode :data="data" :icon="laptopIcon" :showHandles="showHandles" />
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import laptopIcon from '../../../assets/icons/wirelessclient.svg';

    defineProps(['data', 'showHandles']);
</script>