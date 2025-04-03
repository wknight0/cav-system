<template>
    <BaseNode :data="data" :icon="IoTDeviceIcon" :showHandles="showHandles" />
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import IoTDeviceIcon from '../../../assets/icons/iotdevice.svg';

    defineProps(['data', 'showHandles']);
</script>