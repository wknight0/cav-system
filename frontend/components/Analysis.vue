<template>
    <div class="analysis">
        <!-- Shows ranked cves list and network view if exists/info card, otherwise if error has been caught, shows error card, otherwise show loading wheel -->
        <div v-if="loadingRankedCves" class="loading-container">
            <Loading />
        </div>

        <div v-else-if="rankedCveError" class="card-container">
            <Card class="error-card">
                <template #header>
                    <h5>Failed to rank CVEs. Please ensure property fields for network assets have been filled out, and reload CVEs...</h5>
                </template>
            </Card>
        </div>

        <div v-else class="analysis-container">
            <div class="ranked-cves-list">
                <Card v-for="(ranked_cve, index) in ranked_cves" @click="toggleDetails(index)" class="ranked-cve-card">
                    <template #header>
                        <div class="card-header">
                            <h5 class="card-title">{{ ranked_cve.cve.name }}</h5>
                            <h5>{{ ranked_cve.score.toFixed(1) }}%</h5>
                        </div>
                        <div v-if="expandedDetails[index]" class="card-details">
                            <p class="card-text"><strong>CVE_ID:</strong> {{ ranked_cve.cve.cve_id }}</p>
                            <p class="card-text"><strong>Affected Asset Addresses:</strong> {{ ranked_cve.cve.host_address }}</p>
                        </div>
                    </template>
                </Card>
            </div>
            <div class="network-view">
                <VueFlow
                    v-model:nodes="nodes"
                    v-model:edges="edges"
                    :node-types="nodeTypes"
                    :fit-view="true"
                    :pan-on-drag="true"
                    :zoom-on-double-click="false"
                    class="vue-flow-analysis-container"
                />
            </div>
        </div>
    </div>
</template>

<script setup>
    import { ref, markRaw, watch, onMounted } from 'vue';
    import { invoke } from '@tauri-apps/api/tauri';
    import { VueFlow, useVueFlow } from '@vue-flow/core';
    import { Card } from 'primevue';
    import 'primeicons/primeicons.css';
    import '@vue-flow/core/dist/style.css';
    import Loading from './Loading.vue';
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

    /*
        Component to display ranked CVEs and their details. It fetches the ranked CVEs from the backend and displays them in a list format.
    */
    const ranked_cves = ref([]);
    const nodes = ref([]);
    const edges = ref([]);
    const showAnalysis = ref(false);
    const loadingRankedCves = ref(true);
    const rankedCveError = ref(false);
    const expandedDetails = ref([]);

    // Toggles the details of a ranked cve card
    function toggleDetails(index) {
        expandedDetails.value[index] = !expandedDetails.value[index];
    }
    
    // Fetches CVEs and Assets, ranks the CVEs, and returns them to display to frontend
    async function createRankedCves() {
        loadingRankedCves.value = true;
        showAnalysis.value = false;

        try {
            nodes.value = await invoke('retrieve_assets');
            edges.value = await invoke('retrieve_connections');
            ranked_cves.value = await invoke('create_ranked_cves');

            expandedDetails.value = Array(ranked_cves.value.length).fill(false);
        } catch (error) {
            loadingRankedCves.value = false;
            rankedCveError.value = true;
            console.error("Error fetching CVEs:", error);
        }
        loadingRankedCves.value = false;
        showAnalysis.value = true;
    }

    /*
        Component to display network view of ranked CVEs, using VueFlow to create a network graph of the ranked CVEs and their relationships.
    */
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
    const selectedConnectionType = ref(null);

    watch(selectedConnectionType, (newType) => {
        nodes.value.forEach((node) => {
            node.data.showHandles = !!newType;
        });
    });

    // Load the network topology from the database
    const loadNetwork = async () => {
        let assets = nodes.value;
        let connections = edges.value; 

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

    onMounted(async () => {
        await createRankedCves();
        loadNetwork();
    });
</script>

<style>
    .analysis {
        height: 100%;
        width: 100%;
        padding: 1rem;
        display: flex;
        overflow: hidden;
    }

    .analysis-container {
        height: 100%;
        width: 100%;
        display: flex;
        overflow: hidden;
    }

    .ranked-cves-list {
        height: 100%;
        width: 40%;
        padding: 16px 16px 0px 16px;
        border: var(--secondary-color) 2px dotted;
        overflow-y: auto;
    }

    .network-view {
        height: 100%;
        width: 60%;
        border: var(--secondary-color) 2px dotted;
        border-left: none;
        overflow-y: auto;
    }

    .vue-flow-analysis-container {
        height: 100%;
        width: 100%;
        overflow: hidden;
    }

    .ranked-cve-card {
        margin-bottom: 1rem;
        border: 1px solid #ddd;
        border-radius: 8px;
        overflow: hidden;
        padding: 1rem;
        cursor: pointer;
    }

    .ranked-cve-card:hover {
        border-color: var(--secondary-color);
    }

    .card-header {
        display: flex;
        justify-content: space-between;
        gap: 20px;
        align-items: center;
    }

    h5.card-title {
        font-size: 1.0rem;
        font-weight: bold;
        margin: 0;
    }

    .card-text {
        margin: 0.5rem 0;
        font-size: 0.9rem;
    }

    .card-details {
        margin-top: 1rem;
    }

    .loading-container, .card-container {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
    }

    .error-card {
       text-align: center;
       width: 75%;
       border: red dashed 1px; 
    }

    .toggle-icon {
        font-size: 1rem;
        cursor: pointer;
    }
</style>
