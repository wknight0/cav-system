<template>
    <BaseNode :data="data" :icon="laptopIcon" :showHandles="showHandles">
        <!-- Operating system property -->
        <div class="form-group radio-group">
            <div class="radio-row">
                <label for="operatingSystem">Operating System:</label>
                <div class="radio-buttons">
                    <label v-for="option in osOptions" :key="option.value">
                        <RadioButton 
                            :value="option.value" 
                            v-model="data.properties.operating_system" 
                        />
                        {{ option.label }}
                    </label>
                </div>
            </div>
        </div><br>

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

        <!-- Antivirus status property -->
        <div class="form-group radio-group">
            <div class="radio-row">
                <label for="antivirusStatus">Antivirus Status:</label>
                <div class="radio-buttons">
                    <label v-for="option in antivirusOptions" :key="option.value">
                        <RadioButton 
                            :value="option.value" 
                            v-model="data.properties.antivirus_status" 
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
    import laptopIcon from '../../../assets/icons/wirelessclient.svg';
    import { RadioButton } from 'primevue';

    const osOptions = [
        { label: 'Windows', value: 'windows' },
        { label: 'Linux', value: 'linux' },
        { label: 'MacOS', value: 'macos' },
        { label: 'Unknown', value: 'unknown' },
    ];

    const firewallOptions = [
        { label: 'Active', value: 'active' },
        { label: 'Inactive', value: 'inactive' },
        { label: 'Unknown', value: 'unknown' },
    ];

    const antivirusOptions = [
        { label: 'Active', value: 'active' },
        { label: 'Inactive', value: 'inactive' },
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