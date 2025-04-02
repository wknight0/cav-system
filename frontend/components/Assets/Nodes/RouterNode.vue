<template>
    <BaseNode :data="data" :icon="routerIcon" :showHandles="showHandles" />
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import routerIcon from '../../../assets/icons/router.svg';

    defineProps(['data', 'showHandles']);
</script>