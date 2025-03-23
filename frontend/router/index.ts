import { createRouter, createWebHistory } from 'vue-router';
import MainPanel from '../components/MainPanel.vue';

const routes = [
    {
        path: '/',
        name: 'MainPanel',
        component: MainPanel,
        props: true
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
