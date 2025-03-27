<template>
    <div class="cve">
        <!-- Shows cve list in card view if exists/info card, otherwise if importing cves has caused an error, show error message, otherwise show loading wheel -->
        <div class="sliding-div">
            <div v-if="loadingCves" class="loading-container">
                <Loading />
            </div>
            
            <div v-else-if="cveError" class="card-container">
                <Card class="error-card">
                    <template #header>
                        <h5>Failed to import or fetch CVEs. Please ensure valid NIST API Key exists in config and if error persists, check logs.</h5>
                    </template>
                </Card>
            </div>
    
            <div v-else-if="cves.length === 0" class="card-container">
                <Card class="info-card">
                    <template #header>
                        <h5>No CVEs found in database. Update config and import CVEs to get started.</h5>
                    </template>
                </Card>
            </div>

            <div v-else>
                <Card v-for="(cve, index) in cves" :key="index" @click="toggleDetails(index)" class="cve-card">
                    <template #header>
                        <div class="card-header">
                            <h5 class="card-title">{{ cve.name }}</h5>
                            <Button
                                :icon="expandedDetails[index] ? 'pi pi-chevron-down' : 'pi pi-chevron-right'"
                                class="p-button-text toggle-icon"
                            />
                        </div>
                        <div v-if="expandedDetails[index]" class="card-details">
                            <p class="card-text"><strong>CVE_ID:</strong> {{ cve.cve_id }}</p>
                            <p class="card-text"><strong>Host Address:</strong> {{ cve.host_address }}</p>
                            <p class="card-text"><strong>Confidentiality:</strong> {{ cve.values.c }}</p>
                            <p class="card-text"><strong>Integrity:</strong> {{ cve.values.i }}</p>
                            <p class="card-text"><strong>Availability:</strong> {{ cve.values.a }}</p>
                        </div>
                    </template>
                </Card>
            </div>
        </div>

        <!-- Buttons handling updating of credentials and importing of CVE data -->
        <div class="button-container">
            <Button @click="updateCredentials" class="update-button">Update Credentials</Button>
            <Button @click="importCves" class="import-button">Import CVEs</Button>
        </div>

        <!-- Dialog Popup for Updating Configuration (NIST Key) -->
        <Dialog header="Update Configuration" v-model:visible="showConfigDialog" :modal="true" :closable="false" :style="{ width: '400px' }">
            <div class="dialog-content">
                <label for="nistKey">NIST API Key: </label><br>
                <InputText id="nistKey" v-model="newNistKey" placeholder="Enter NIST API key" class="input-field" />

                <div class="dialog-footer">
                    <Button label="Cancel" class="cancel-config-button" @click="cancelUpdate" />
                    <Button label="Save" class="save-config-button" @click="saveConfig" />
                </div>
            </div>
        </Dialog>
    </div>
</template>

<script setup>
    import { ref, onMounted } from 'vue';
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from '@tauri-apps/api/dialog';
    import { Card, Button, Dialog, InputText, } from 'primevue';
    import 'primeicons/primeicons.css';
    import Loading from './Loading.vue';

    const selectedFilePathCves = ref('');
    const cves = ref([]);
    const showCves = ref(false);
    const loadingCves = ref(true);
    const showConfigDialog = ref(false);
    const cveError = ref(false);
    const newNistKey = ref('');
    const expandedDetails = ref([]);

    // Handles opening the dialog to update the user's credentials
    const updateCredentials = async () => {
        showConfigDialog.value = true;
        try {
            const config = await invoke("retrieve_credentials");
            newNistKey.value = config.nist_key || '';
        } catch (error) {
            console.error('Error fetching credentials:', error);
        }
    }

    // Handles importing CVEs by selecting a CSV file (requires NIST api key to exist in config)
    const importCves = async () => {
        const selected = await open({
            multiple: false,
            directory: false,
            filters: [{ name: 'CSV Files', extensions: ['csv']}]
        });

        if (selected) {
            loadingCves.value = true;
            showCves.value = false;
            cveError.value = false;
            selectedFilePathCves.value = selected;

            // Import CVEs using path specified
            try {
                let result = await updateCves(selectedFilePathCves.value);
                if (result !== true) {
                    throw new Error('Failed to import CVEs');
                }
                await fetchCves();
            } catch (error) {
                cveError.value = true;
                console.error('Error during CVE import or fetch:', error);
            } finally {
                loadingCves.value = false;
                showCves.value = !cveError.value;
            }
        } else {
            console.log('No file selected');
        }

        loadingCves.value = false;
        showCves.value = true;
    };

    // Fetches the CVEs from the database
    async function fetchCves() {
        loadingCves.value = true;
        showCves.value = false;

        try {
            const fetchedCves = await invoke('retrieve_cves');
            cves.value = fetchedCves;
            expandedDetails.value = Array(fetchedCves.length).fill(false);
        } catch (error) {
            console.error("Error fetching CVEs:", error);
        }
        loadingCves.value = false;
        showCves.value = true;
    }

    // Updates the CVEs in the database using the specified file path
    async function updateCves(filePath) {
        try {
            let result = await invoke('update_cves', { filePath });
            if (result !== true) {
                throw new Error('Failed to update CVEs');
            }
            console.log('CVEs updated successfully.');
            return result;
        } catch (error) {
            console.error("Error updating CVEs:", error);
        }
    }

    // Toggles the details of a cve card
    function toggleDetails(index) {
        expandedDetails.value[index] = !expandedDetails.value[index];
    }

    // Saves the config input on the dialog box to the database
    async function saveConfig() {
        try {
            await invoke('update_credentials', { credentials: { nist_key: newNistKey.value } });
            console.log('Credentials updated successfully.');
            showConfigDialog.value = false;
        } catch (error) {
            console.error('Error updating credentials:', error);
            alert('Failed to update credentials. Please check the logs.');
        }
    }

    // Cancels the config update and close the dialog
    function cancelUpdate() {
        newNistKey.value = '';
        showConfigDialog.value = false;
    }

    // Fetches cves on component mount
    onMounted(async () => {
        await fetchCves();
    });
</script>

<style>
    .cve {
        height: 100%;
        width: 100%;
        padding: 1rem;
    }

    .sliding-div {
        height: calc(100% - 50px);
        padding: 16px 16px 0px 16px;
        border: var(--secondary-color) 2px dotted;
        overflow-y: auto;
    }

    .cve-card {
        margin-bottom: 1rem;
        border: 1px solid #ddd;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        overflow: hidden;
        padding: 1rem;
        cursor: pointer;
    }

    .cve-card:hover {
        border-color: var(--secondary-color);
    }

    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .card-title {
        font-size: 1.25rem;
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

    .loading-container {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
    }

    .card-container {
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

    .info-card {
        text-align: center;
        width: 75%;
        border: var(--primary-text-color) dashed 1px;
    }
    
    .toggle-icon {
        font-size: 1rem;
        cursor: pointer;
    }

    .button-container {
        display: flex;
        justify-content: space-around;
        margin-top: 1rem;
    }
    
    div.p-card-body {
        padding: 0;
    }

    .update-button:hover, .import-button:hover {
        color: var(--secondary-text-color) !important;
        background-color: var(--secondary-color) !important;
    }

    .input-field {
        width: 100%;
    }

    .dialog-footer {
        display: flex;
        justify-content: space-evenly;
        margin-top: 1rem;
    }

    .cancel-config-button:hover, .save-config-button:hover {
        color: var(--secondary-text-color) !important;
        background-color: var(--secondary-color) !important;
    }
</style>