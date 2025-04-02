<template>
    <BaseNode :data="data" :icon="switchIcon" :showHandles="showHandles" />
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import switchIcon from '../../../assets/icons/switch.svg';

    defineProps(['data', 'showHandles']);
</script>
