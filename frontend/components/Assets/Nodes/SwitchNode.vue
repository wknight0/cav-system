<template>
    <BaseNode :data="data" :icon="switchIcon" :showHandles="showHandles">
        <!-- VLAN support property -->
        <div class="form-group radio-group">
            <div class="radio-row">
                <label for="vlanSupport">VLAN Support:</label>
                <div class="radio-buttons">
                    <label v-for="option in vlanOptions" :key="option.value">
                        <RadioButton 
                            :value="option.value" 
                            v-model="data.properties.vlan_support" 
                        />
                        {{ option.label }}
                    </label>
                </div>
            </div>
        </div><br>

        <div class="form-group radio-group">
            <!-- PoE support property -->
            <div class="radio-row">
                <label for="poeSupport">PoE Support:</label>
                <div class="radio-buttons">
                    <label v-for="option in poeOptions" :key="option.value">
                        <RadioButton 
                            :value="option.value" 
                            v-model="data.properties.poe_support"
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
    import switchIcon from '../../../assets/icons/switch.svg';
    import { RadioButton } from 'primevue';

    const vlanOptions = [
        { label: 'Yes', value: 'yes' },
        { label: 'No', value: 'no' },
        { label: 'Unknown', value: 'unknown' },
    ];

    const poeOptions = [
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