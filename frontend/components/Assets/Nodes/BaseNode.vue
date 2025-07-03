<template>
    <div class="base-node">
        <!-- Severity bubble that displays the severity percentage only if applicable in analysis component -->
        <span
            v-if="data.severity !== null && data.severity !== undefined"
            class="severity-bubble"
            :style="{ backgroundColor: getSeverityColor(data.severity) }"
        >
            {{ data.severity.toFixed(1) }}%
        </span>
        <!-- Handle for connections -->
        <Handle
            type="target"
            :position="Position.Left"
            :style="{ 
                backgroundColor: '#555', 
                opacity: '0%', 
                left: '38px', 
                top: '50%',
                transform: 'translateY(-52%)',
                height: '90%',
                width: '30%',
                zIndex: data.showHandles ? 10 : 0
            }"
        />
        <img class="node-background node-icon" :src="icon" alt="Node Icon" @click="openProperties"/>
        <div class="node-label">{{ data.label }}</div>
        <!-- Handle for connections -->
        <Handle
            type="source"
            :position="Position.Right"
            :style="{
                backgroundColor: '#555', 
                opacity: '0%', 
                right: '37px', 
                top: '50%', 
                transform: 'translateY(-52%)',
                height: '90%',
                width: '30%',
                zIndex: data.showHandles ? 10 : 0
            }"
        />
        <!-- Properties dialog which stores common asset attributes and can store additional property information via slot component -->
        <Dialog 
            v-model:visible="showPopup" 
            :modal="true" 
            :closable="false" 
            :dismissable-mask="true"
            :style="{ width: '400px' }"
        >
            <div class="dialog-content">
                <div class="form-group">
                    <label for="name">Name:</label>
                    <InputText id="name" v-model="data.label" placeholder="Enter node name" class="input-field" />
                </div>

                <div class="form-group">
                    <label for="ipAddress">IP Address:</label>
                    <InputText id="ipAddress" v-model="data.ip_address" placeholder="Enter IP address" class="input-field" />
                </div>
            </div>
            <br>
            <slot></slot>
        </Dialog>
    </div>
</template>

<script setup>
    import { ref } from 'vue';
    import { Handle, Position } from '@vue-flow/core';
    import { Dialog, InputText } from 'primevue';

    defineProps(['data', 'icon', 'showHandles']);

    const showPopup = ref(false);

    const openProperties = () => {
        showPopup.value = true;
    };

    function getSeverityColor(severity) {
        if (severity >= 75) return '#e53935';
        if (severity >= 40) return '#ffb300';
        return '#43a047';
    }
</script>

<style scoped>
    .base-node {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-between;
        width: 75px;
        height: 50px;
        text-align: center;
        position: relative;
    }

    .severity-bubble {
        position: absolute;
        top: -16px;
        right: -4px;
        width: 32px;
        height: 32px;
        border-radius: 50%;
        color: #fff;
        font-weight: 600;
        font-size: 0.55em;
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 20;
        letter-spacing: 0.01em;
        background: var(--secondary-color);
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

    .dialog-content {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .input-field {
        width: 100%;
    }
</style>
