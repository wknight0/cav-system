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
                @edges-delete="onEdgesDelete"
                @nodes-delete="onNodesDelete"
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
                        <Button class="icon-button" @click="addNode('Switch')">
                            <img src="../assets/icons/switch.svg" alt="Switch" />
                            <label>Switch</label>
                        </Button>
                        <Button class="icon-button" @click="addNode('Router')">
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
    import { invoke } from '@tauri-apps/api/tauri';
    import { VueFlow, useVueFlow } from '@vue-flow/core';
    import '@vue-flow/core/dist/style.css';
    import { Button } from 'primevue';
    import SwitchNode from './Assets/Nodes/SwitchNode.vue';
    import RouterNode from './Assets/Nodes/RouterNode.vue';
    import { createEthernetEdge } from './Assets/Edges/EthernetEdge.vue';
    import { createWirelessEdge } from './Assets/Edges/WirelessEdge.vue';

    const { fitView } = useVueFlow();

    const nodeTypes = {
        Switch: markRaw(SwitchNode),
        Router: markRaw(RouterNode),
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

    const addNode = (nodeType) => {
        const id = `node-${nodes.value.length + 1}`;
        const position = { x: 300, y: 200 };
        const label = `Node ${nodes.value.length + 1}`;

        const newNode = {
            id,
            type: nodeType,
            position,
            data: {
                label,
                showHandles: !!selectedConnectionType.value,
            },
        };

        nodes.value.push(newNode);
    };

    const onConnect = (params) => {
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

    const onEdgesDelete = (deletedEdges) => {
        edges.value = edges.value.filter(edge => !deletedEdges.some(deletedEdge => deletedEdge.id === edge.id));
    };

    const onNodesDelete = (deletedNodes) => {
        nodes.value = nodes.value.filter(node => !deletedNodes.some(deletedNode => deletedNode.id === node.id));
        edges.value = edges.value.filter(edge => 
            !deletedNodes.some(deletedNode => edge.source === deletedNode.id || edge.target === deletedNode.id)
        );
    }

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

    const loadNetwork = async () => {
        let assets;
        let connections; 

        try {
            assets = await invoke('retrieve_assets');
            connections = await invoke('retrieve_connections');
        } catch (error) {
            console.error(error);
            return;
        }

        nodes.value = assets.map(asset => ({
            id: asset._id,
            type: asset.asset_type,
            position: asset.position,
            data: {
                label: asset.label,
                properties: asset.properties,
                showHandles: false,
            },
        }));

        edges.value = connections.map(connection => {
            const edgeId = connection._id;

            switch (connection.connection_type) {
                case ('ethernet'):
                    return createEthernetEdge(
                        { source: connection.source, target: connection.destination },
                        edgeId
                    );
                case ('wireless'):
                    return createWirelessEdge(
                        { source: connection.source, target:connection.destination },
                        edgeId
                    );
                default:
                    console.warn("Unknown connection type...");
                    return null;
            }
        }).filter(edge => edge !== null);

        console.log('Network loaded...');
    }

    const updateNetwork = async () => {
        const assetData = nodes.value.map(node => ({
            _id: node.id,
            label: node.data.label,
            asset_type: node.type,
            position: node.position,
            ip_address: '192.168.1.1',
            properties: node.data.properties || {
                firewall_status: 'unknown',
                internet_facing: 'unknown',
                relations: [],
            },
        }));

        const connectionData = edges.value.map(edge => ({
            _id: edge.id,
            source: edge.source,
            destination: edge.target,
            type: edge.type,
            connection_type: edge.connection_type,
        }));

        try {
            await invoke('update_assets', {
                assets: assetData,
            });
            await invoke('update_connections', {
                connections: connectionData,
            })

            console.log('Network data successfully stored...');
        } catch (error) {
            console.error(error);
        }
    }    
    
    loadNetwork();
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
