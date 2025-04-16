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

            </div>
        </div>
    </div>
</template>

<script setup>
    import { ref, onMounted } from 'vue';
    import { invoke } from '@tauri-apps/api/tauri';
    import { Button, Card } from 'primevue';
    import 'primeicons/primeicons.css';
    import Loading from './Loading.vue';

    const ranked_cves = ref([]);
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
            const fetchedRankedCves = await invoke('create_ranked_cves');
            ranked_cves.value = fetchedRankedCves;
            expandedDetails.value = Array(fetchedRankedCves.length).fill(false);
        } catch (error) {
            loadingRankedCves.value = false;
            rankedCveError.value = true;
            console.error("Error fetching CVEs:", error);
        }
        loadingRankedCves.value = false;
        showAnalysis.value = true;
    }

    onMounted(async () => {
        await createRankedCves();
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
        padding: 16px 16px 0px 16px;
        border: var(--secondary-color) 2px dotted;
        border-left: none;
        overflow-y: auto;
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
