<template>
    <BaseNode :data="data" :icon="firewallIcon" :showHandles="showHandles" />
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import firewallIcon from '../../../assets/icons/firewall.svg';

    defineProps(['data', 'showHandles']);
</script>