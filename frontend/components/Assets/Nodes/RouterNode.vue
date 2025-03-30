<template>
    <div class="router-node">
        <Handle
            type="target"
            :position="Position.Left"
            :style="{ 
                backgroundColor: '#555', 
                opacity: '0%', 
                left: '38px', 
                top: '50%', 
                transform: 'translateY(-52%)',
                zIndex: data.showHandles ? 10 : 0
            }"
        />
        <img class="node-background node-icon" src="../../../assets/icons/router.svg" alt="Router Icon"/>
        <div class="node-label">{{ data.label }}</div>
        <Handle
            type="source"
            :position="Position.Right"
            :style="{
                backgroundColor: '#555', 
                opacity: '0%', 
                right: '37px', 
                top: '50%', 
                transform: 'translateY(-52%)',
                zIndex: data.showHandles ? 10 : 0
            }"
        />
    </div>
</template>

<script setup>
    import { Handle, Position } from '@vue-flow/core';
    defineProps(['data', 'showHandles']);
</script>

<style scoped>
    .router-node {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-between;
        width: 75px;
        height: 50px;
        text-align: center;
        position: relative;
    }

    .node-background {
        background-color: var(--primary-color);
        border-radius: 8px;
        z-index: 1;
    }

    .node-icon {
        width: 48px;
        height: 48px;
    }

    .node-label {
        text-align: center;
    }

    .vue-flow__handle {
        width: 24px;
        height: 48px;
        position: absolute;
        z-index: 10;
    }
</style>