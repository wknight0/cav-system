<template>
    <BaseNode :data="data" :icon="loadBalancerIcon" :showHandles="showHandles" />
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import loadBalancerIcon from '../../../assets/icons/loadbalancer.svg';

    defineProps(['data', 'showHandles']);
</script>