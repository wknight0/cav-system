<template>
    <BaseNode :data="data" :icon="printerIcon" :showHandles="showHandles" />
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import printerIcon from '../../../assets/icons/printer.svg';

    defineProps(['data', 'showHandles']);
</script>