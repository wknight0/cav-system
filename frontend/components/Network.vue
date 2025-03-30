<template>
    <div class="network">
        <div class="network-topology-wizard">
            <VueFlow
                v-model:nodes="nodes"
                v-model:edges="edges"
                :node-types="nodeTypes"
                :fit-view="true"
                :pan-on-drag="false"
                :zoom-on-double-click="false"
                class="vue-flow-container"
                @connect="onConnect"
                @connect-start="onConnectStart"
                @connect-stop="onConnectStop"
            />
            <div class="network-palette">
                <div class="button-section top-section">
                    <h3 class="section-title">Connections</h3>
                    <div class="icon-button-grid top-buttons">
                        <Button 
                            class="icon-button"
                            :class="{ active: selectedConnectionType === 'ethernet' }"
                            @click="selectConnectionType('ethernet')"
                        >
                            <img src="../assets/icons/ethernet.svg" alt="Router" />
                            <label>Ethernet</label>
                        </Button>
                        <Button 
                            class="icon-button"
                            :class="{ active: selectedConnectionType === 'wireless' }"
                            @click="selectConnectionType('wireless')"
                        >
                            <img src="../assets/icons/wifi.svg" alt="Router" />
                            <label>Wireless</label>
                        </Button>
                    </div>
                </div>
                <div class="button-section bottom-section">
                    <h3 class="section-title">Nodes</h3>
                    <div class="icon-button-grid bottom-buttons">
                        <Button class="icon-button" @click="addRouterNode">
                            <img src="../assets/icons/router.svg" alt="Router" />
                            <label>Router</label>
                        </Button>
                        <Button class="icon-button">
                            <img src="../assets/icons/router.svg" alt="Router" />
                            <label>Router</label>
                        </Button>
                        <Button class="icon-button">
                            <img src="../assets/icons/router.svg" alt="Router" />
                            <label>Router</label>
                        </Button>
                        <Button class="icon-button">
                            <img src="../assets/icons/router.svg" alt="Router" />
                            <label>Router</label>
                        </Button>
                        <Button class="icon-button">
                            <img src="../assets/icons/router.svg" alt="Router" />
                            <label>Router</label>
                        </Button>
                        <Button class="icon-button">
                            <img src="../assets/icons/router.svg" alt="Router" />
                            <label>Router</label>
                        </Button>
                        <Button class="icon-button">
                            <img src="../assets/icons/router.svg" alt="Router" />
                            <label>Router</label>
                        </Button>
                        <Button class="icon-button">
                            <img src="../assets/icons/router.svg" alt="Router" />
                            <label>Router</label>
                        </Button>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="button-container">
            <Button @click="updateNetwork" class="update-button">Update Network</Button>
        </div>
    </div>
</template>

<script setup>
    import { ref, markRaw, watch } from 'vue';
    import { VueFlow, useVueFlow } from '@vue-flow/core';
    import '@vue-flow/core/dist/style.css';
    import { Button } from 'primevue';
    import RouterNode from './Assets/Nodes/RouterNode.vue';
    import { createEthernetEdge } from './Assets/Edges/EthernetEdge.vue';
    import { createWirelessEdge } from './Assets/Edges/WirelessEdge.vue';

    const { fitView } = useVueFlow();
    
    const nodeTypes = {
        router: markRaw(RouterNode),
    }

    const nodes = ref([]);
    const edges = ref([]);
    const selectedConnectionType = ref(null);

    watch(selectedConnectionType, (newType) => {
        nodes.value.forEach((node) => {
            node.data.showHandles = !!newType;
        });
    });

    const selectConnectionType = (type) => {
        selectedConnectionType.value = selectedConnectionType.value === type ? null : type;
    };

    const addRouterNode = () => {
        const id = `node-${nodes.value.length + 1}`;
        const position = { x: 300, y: 200 };
        const label = `Router ${nodes.value.length + 1}`;

        const newRouterNode = {
            id,
            type: 'router',
            position,
            data: {
                label,
                showHandles: !!selectedConnectionType.value,
            },
        };

        nodes.value.push(newRouterNode);
    };

    const onConnect = (params) => {
        console.log('Connection Params:', params);

        const edgeId = `edge-${edges.value.length + 1}`;
        let newEdge;

        if (selectedConnectionType.value === 'ethernet') {
            newEdge = createEthernetEdge(params, edgeId);
        } else if (selectedConnectionType.value === 'wireless') {
            newEdge = createWirelessEdge(params, edgeId);
        }

        if (selectedConnectionType.value === 'ethernet') {
            newEdge = createEthernetEdge(params, edgeId);
        } else if (selectConnectionType.value === 'wireless') {
            newEdge = createWirelessEdge(params, edgeId);
        }

        if (newEdge) {
            edges.value.push(newEdge);
        }

        selectedConnectionType.value = null;
    };

    

    const onConnectStart = () => {
        console.log('Connection started');
    };

    const onConnectStop = () => {
        console.log('Connection stopped');
    };

    const clearTopology = () => {
        nodes.value = [];
        edges.value = [];
    };

    const updateNetwork = async () => {
        console.log("Temporary update network functionality...");
    }

    
</script>

<style>
    

    .network {
        height: 100%;
        width: 100%;
        padding: 1rem;
        overflow: hidden;
    }

    .network-topology-wizard {
        display: flex;
        flex-direction: row;
        height: calc(100% - 50px);
        border: white 2px solid;
    }

    .vue-flow-container {
        height: 100%;
        width: 100%;
        max-width: calc(100% - 150px);
        max-height: 100%;
        position: relative;
        overflow: hidden;
        border-right: 2px solid var(--primary-text-color);
    }

    .network-palette {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        width: 150px;
        overflow: hidden scroll;
    }

    .section-title {
        font-size: 1.2rem;
        font-weight: bold;
        margin-bottom: 0.5rem;
        text-align: center;
    }

    .icon-button-grid {
        display: grid;
        gap: 0.25rem;
        justify-items: center;
        align-items: center;
    }

    .top-buttons {
        grid-template-columns: repeat(2, 1fr);
    }

    .bottom-buttons {
        grid-template-columns: repeat(2, 1fr);
        margin-bottom: 15px;
    }

    .icon-button {
        width: 60px;
        height: 80px;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        text-align: center;
        border-radius: 8px;
        cursor: pointer;
    }
    
    .icon-button img {
        width: 48px;
        height: 48px;
    }
    
    .icon-button:hover label, .icon-button:hover img, .icon-button.active img, .icon-button.active label {
        filter: brightness(0%) saturate(100%) invert(81%) sepia(31%) saturate(527%) hue-rotate(100deg) brightness(91%) contrast(104%);;
    }

    .update-button:hover {
        color: var(--secondary-text-color) !important;
        background-color: var(--secondary-color) !important;
    }

    .icon-button label {
        font-size: 0.8rem;
        color: var(--primary-text-color);
        text-align: center;
    }
</style>
