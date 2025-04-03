<template>
    <BaseNode :data="data" :icon="mobilePhoneIcon" :showHandles="showHandles" />
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import mobilePhoneIcon from '../../../assets/icons/wirelessphone.svg';

    defineProps(['data', 'showHandles']);
</script>