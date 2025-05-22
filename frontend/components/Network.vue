<template>
    <div class="network">
        <div class="network-topology-wizard">
            <!-- Interactive topology section -->
            <VueFlow
                v-model:nodes="nodes"
                v-model:edges="edges"
                :node-types="nodeTypes"
                :fit-view="true"
                :pan-on-drag="true"
                :zoom-on-double-click="false"
                class="vue-flow-network-container"
                @connect="onConnect"
                @connect-start="onConnectStart"
                @connect-stop="onConnectStop"
                @edges-delete="onEdgesDelete"
                @nodes-delete="onNodesDelete"
            />
            <!-- Network topology palette section -->
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
                            <img src="../assets/icons/switch.svg" alt="Switch Icon" />
                            <label>Switch</label>
                        </Button>
                        <Button class="icon-button" @click="addNode('Router')">
                            <img src="../assets/icons/router.svg" alt="Router Icon" />
                            <label>Router</label>
                        </Button>
                        <Button class="icon-button" @click="addNode('Firewall')">
                            <img src="../assets/icons/firewall.svg" alt="Firewall Icon" />
                            <label>Firewall</label>
                        </Button>
                        <Button class="icon-button" @click="addNode('LoadBalancer')">
                            <img src="../assets/icons/loadbalancer.svg" alt="LoadBalancer Icon" />
                            <label>LoadBalancer</label>
                        </Button>
                        <Button class="icon-button" @click="addNode('Server')">
                            <img src="../assets/icons/server.svg" alt="Server Icon" />
                            <label>Server</label>
                        </Button>
                        <Button class="icon-button" @click="addNode('Storage')">
                            <img src="../assets/icons/storage.svg" alt="Storage Icon" />
                            <label>Storage</label>
                        </Button>
                        <Button class="icon-button" @click="addNode('Desktop')">
                            <img src="../assets/icons/wiredclient.svg" alt="Desktop Icon" />
                            <label>Desktop</label>
                        </Button>
                        <Button class="icon-button" @click="addNode('Laptop')">
                            <img src="../assets/icons/wirelessclient.svg" alt="Laptop Icon" />
                            <label>Laptop</label>
                        </Button>
                        <Button class="icon-button" @click="addNode('MobileDevice')">
                            <img src="../assets/icons/wirelessphone.svg" alt="MobileDevice Icon" />
                            <label>MobileDevice</label>
                        </Button>
                        <Button class="icon-button" @click="addNode('Printer')">
                            <img src="../assets/icons/printer.svg" alt="Printer Icon" />
                            <label>Printer</label>
                        </Button>
                        <Button class="icon-button" @click="addNode('IoTDevice')">
                            <img src="../assets/icons/iotdevice.svg" alt="IoTDevice Icon" />
                            <label>IoTDevice</label>
                        </Button>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="button-container">
            <Button @click="clearNetwork" class="clear-button">Clear Network</Button>
            <Button @click="updateNetwork" class="update-button">Update Network</Button>
        </div>
    </div>
</template>

<script setup>
    import { ref, markRaw, watch } from 'vue';
    import { invoke } from '@tauri-apps/api/tauri';
    import { VueFlow, useVueFlow } from '@vue-flow/core';
    import '@vue-flow/core/dist/style.css';
    import { Button, Dialog } from 'primevue';
    import SwitchNode from './Assets/Nodes/SwitchNode.vue';
    import RouterNode from './Assets/Nodes/RouterNode.vue';
    import FirewallNode from './Assets/Nodes/FirewallNode.vue';
    import LoadBalancerNode from './Assets/Nodes/LoadBalancerNode.vue';
    import ServerNode from './Assets/Nodes/ServerNode.vue';
    import StorageNode from './Assets/Nodes/StorageNode.vue';
    import DesktopNode from './Assets/Nodes/DesktopNode.vue';
    import LaptopNode from './Assets/Nodes/LaptopNode.vue';
    import MobileDeviceNode from './Assets/Nodes/MobileDeviceNode.vue';
    import PrinterNode from './Assets/Nodes/PrinterNode.vue';
    import IoTDeviceNode from './Assets/Nodes/IoTDeviceNode.vue';
    import { createEthernetEdge } from './Assets/Edges/EthernetEdge.vue';
    import { createWirelessEdge } from './Assets/Edges/WirelessEdge.vue';

    const nodeTypes = {
        Switch: markRaw(SwitchNode),
        Router: markRaw(RouterNode),
        Firewall: markRaw(FirewallNode),
        LoadBalancer: markRaw(LoadBalancerNode),
        Server: markRaw(ServerNode),
        Storage: markRaw(StorageNode),
        Desktop: markRaw(DesktopNode),
        Laptop: markRaw(LaptopNode),
        MobileDevice: markRaw(MobileDeviceNode),
        Printer: markRaw(PrinterNode),
        IoTDevice: markRaw(IoTDeviceNode),
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

    // Adds a new node to the network topology
    const addNode = (nodeType) => {
        try {
            const typeCount = nodes.value.filter(n => n.type === nodeType).length;
            const id = `${nodeType.toLowerCase()}-${typeCount + 1}`;
            const position = { x: 300, y: 200 };
            const label = `${nodeType} ${typeCount + 1}`;

            const newNode = {
                id,
                type: nodeType,
                position,
                // Data contains default values for the node and is modified only where relevant depending on the node type
                data: {
                    label,
                    showHandles: !!selectedConnectionType.value,
                    ip_address: '',
                    properties: {
                        operating_system: 'unknown',
                        internet_facing: 'unknown',
                        antivirus_status: 'unknown',
                        firewall_status: 'unknown',
                        firmware_status: 'unknown',
                        tamper_detection_status: 'unknown',
                        vpn_support: 'unknown',
                        vlan_support: 'unknown',
                        poe_support: 'unknown',
                        encryption_enabled: 'unknown',
                        virtualization_enabled: 'unknown',
                        network_enabled: 'unknown',
                        health_check_enabled: 'unknown',
                    },
                },
            };

        nodes.value.push(newNode);
        } catch (error) {
            console.error('Error adding node:', error);
            return;
        }
    };

    // onConnect attribute for connecting nodes together for Vue Flow
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

    // Handles deletion of the Vue Flow edges
    const onEdgesDelete = (deletedEdges) => {
        edges.value = edges.value.filter(edge => !deletedEdges.some(deletedEdge => deletedEdge.id === edge.id));
    };

    // Handles deletion of the Vue Flow nodes
    const onNodesDelete = (deletedNodes) => {
        nodes.value = nodes.value.filter(node => !deletedNodes.some(deletedNode => deletedNode.id === node.id));
        edges.value = edges.value.filter(edge => 
            !deletedNodes.some(deletedNode => edge.source === deletedNode.id || edge.target === deletedNode.id)
        );
    }

    // Handles connection start and stop events
    const onConnectStart = () => {
        console.log('Connection started');
    };

    const onConnectStop = () => {
        console.log('Connection stopped');
    };

    // Load the network topology from the database
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
                showHandles: false,
                ip_address: asset.ip_address,
                properties: {
                    operating_system: asset.properties.operating_system,
                    internet_facing: asset.properties.internet_facing,
                    antivirus_status: asset.properties.antivirus_status,
                    firewall_status: asset.properties.firewall_status,
                    firmware_status: asset.properties.firmware_status,
                    tamper_detection_status: asset.properties.tamper_detection_status,
                    vpn_support: asset.properties.vpn_support,
                    vlan_support: asset.properties.vlan_support,
                    poe_support: asset.properties.poe_support,
                    encryption_enabled: asset.properties.encryption_enabled,
                    virtualization_enabled: asset.properties.virtualization_enabled,
                    network_enabled: asset.properties.network_enabled,
                    health_check_enabled: asset.properties.health_check_enabled,
                }
            },
        }));

        edges.value = connections.map(connection => {
            const edgeId = connection._id;

            switch (connection.connection_type) {
                case ('ethernet'):
                    return createEthernetEdge(
                        { source: connection.source, target: connection.destination, color: '#32a852' },
                        edgeId
                    );
                case ('wireless'):
                    return createWirelessEdge(
                        { source: connection.source, target:connection.destination, color: 'var(--secondary-color)' },
                        edgeId
                    );
                default:
                    console.warn("Unknown connection type...");
                    return null;
            }
        }).filter(edge => edge !== null);

        console.log('Network loaded...');
    }

    // Clears the network topology
    const clearNetwork = async () => {
        nodes.value = [];
        edges.value = [];
    };

    // Updates the network topology in the database
    const updateNetwork = async () => {
        const assetData = nodes.value.map(node => ({
            _id: node.id,
            label: node.data.label,
            asset_type: node.type,
            position: node.position,
            ip_address: node.data.ip_address,
            properties: {
                operating_system: node.data.properties.operating_system,
                internet_facing: node.data.properties.internet_facing,
                antivirus_status: node.data.properties.antivirus_status,
                firewall_status: node.data.properties.firewall_status,
                firmware_status: node.data.properties.firmware_status,
                tamper_detection_status: node.data.properties.tamper_detection_status,
                vpn_support: node.data.properties.vpn_support,
                vlan_support: node.data.properties.vlan_support,
                poe_support: node.data.properties.poe_support,
                encryption_enabled: node.data.properties.encryption_enabled,
                virtualization_enabled: node.data.properties.virtualization_enabled,
                network_enabled: node.data.properties.network_enabled,
                health_check_enabled: node.data.properties.health_check_enabled,
            },
        }));

        const connectionData = edges.value.map(edge => ({
            _id: edge.id,
            source: edge.source,
            destination: edge.target,
            type: edge.type,
            connection_type: edge.connection_type,
        }));

        console.log('Asset Data:', assetData);

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

    .vue-flow-network-container {
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
        background-color: var(--primary-color) !important;
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

    .button-container {
        justify-content: space-evenly;
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

    .clear-button:hover {
        color: var(--secondary-text-color) !important;
        background-color: var(--warning-color) !important;
    }

    .icon-button label {
        font-size: 0.6rem;
        color: var(--primary-text-color);
        text-align: center;
    }
</style>
