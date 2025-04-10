<template>
    <BaseNode :data="data" :icon="routerIcon" :showHandles="showHandles">
        <!-- Firewall status property -->
        <div class="form-group radio-group">
            <div class="radio-row">
                <label for="firewallStatus">Firewall Status:</label>
                <div class="radio-buttons">
                    <label v-for="option in firewallOptions" :key="option.value">
                        <RadioButton 
                            :value="option.value" 
                            v-model="data.properties.firewall_status" 
                        />
                        {{ option.label }}
                    </label>
                </div>
            </div>
        </div><br>

        <!-- VPN support property -->
        <div class="form-group radio-group">
            <div class="radio-row">
                <label for="vpnSupportOptions">VPN Support:</label>
                <div class="radio-buttons">
                    <label v-for="option in vpnSupportOptions" :key="option.value" >
                        <RadioButton 
                            :value="option.value" 
                            v-model="data.properties.vpn_support" 
                        />
                        {{ option.label }}
                    </label>
                </div>
            </div>
        </div><br>
    </BaseNode>
</template>

<script setup>
    import BaseNode from './BaseNode.vue';
    import routerIcon from '../../../assets/icons/router.svg';
    import { RadioButton } from 'primevue';

    const firewallOptions = [
        { label: 'Active', value: 'active' },
        { label: 'Inactive', value: 'inactive' },
        { label: 'Unknown', value: 'unknown' },
    ];

    const vpnSupportOptions = [
        { label: 'Yes', value: 'yes' },
        { label: 'No', value: 'no' },
        { label: 'Unknown', value: 'unknown' },
    ];

    defineProps(['data', 'showHandles']);
</script>

<style>
    .radio-group {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .radio-row {
        display: flex;
        align-items: center;
        width: 100%;
    }

    .radio-row label {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: flex-start;
        padding-right: 0.5rem;
        box-sizing: border-box;
        width: 20%;
    }

    .radio-buttons {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 10px;
        border: 1px dashed white;
        padding: 10px;
        width: 60%;
    }

    .radio-buttons label {
        display: flex;
        align-items: center;
        gap: 0.3rem;
        font-size: 0.9rem;
        color: var(--primary-text-color);
        cursor: pointer;
    }

    .radio-buttons input[type="radio"] {
        margin: 0;
    }
</style>