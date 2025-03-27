<template>
    <div class="main-panel">
        <div class="main-content">
            <ButtonGroup class="main-header">
                <Button @click="switchView(Analysis, 'Analysis')" :class="getButtonClass('Analysis')">Analysis</button>
                <Button @click="switchView(CVE, 'CVE')" :class="getButtonClass('CVE')">CVE</button>
                <Button @click="switchView(Network, 'Network')" :class="getButtonClass('Network')">Network</button>
            </ButtonGroup>

            <div class="component">
                <component :is="currentView"/>
            </div>
        </div>
    </div>
</template>
  
<script setup>
    import { shallowRef } from 'vue';
    import Analysis from './Analysis.vue';
    import CVE from './CVE.vue';
    import Network from './Network.vue';
    import { Button, ButtonGroup } from "primevue";
  
    const currentView = shallowRef(Analysis);
    const currentTextView = shallowRef("Analysis");

    // Function to switch views
    const switchView = (view, viewText) => {
        currentView.value = view;
        currentTextView.value = viewText;
    };

    // Function to get current view to update active button
    const getButtonClass = (view) => {
        return view === currentTextView.value ? 'view-button active-button' : 'view-button';
    };
</script>
  
<style>
    .main-panel, .main-content {
        width: 100vw;
        height: 100vh;
    }

    .main-header {
        display: flex;
        justify-content: space-around;
        height: 50px;
        width: 100%;
    }

    .component {
        height: calc(100% - 50px);
        width: 100%;
    }

    Button {
        width: 33.33%;
    }

    .p-button {
        border-radius: 0 !important;
        border: 0 !important;
        color: var(--primary-text-color) !important;
        background-color: var(--primary-color) !important;
        border-bottom: var(--primary-color) 2px solid !important;
        font-weight: bold;
    }

    .view-button:hover {
        border-bottom: var(--secondary-color) 2px solid !important;
    }

    .active-button {
        background-color: var(--secondary-color) !important;
        border-bottom: var(--secondary-color) 2px solid !important;
        color: var(--secondary-text-color) !important;
    }
</style>
