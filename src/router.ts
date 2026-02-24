import { createRouter, createWebHashHistory } from 'vue-router';
import DashboardView from './views/DashboardView.vue';
import SellersView from './views/SellersView.vue';
import CustomersView from './views/CustomersView.vue';
import ProductsView from './views/ProductsView.vue';
import InvoicesView from './views/InvoicesView.vue';
import InvoiceFormView from './views/InvoiceFormView.vue';
import SettingsView from './views/SettingsView.vue';

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        { path: '/', name: 'dashboard', component: DashboardView },
        { path: '/sellers', name: 'sellers', component: SellersView },
        { path: '/customers', name: 'customers', component: CustomersView },
        { path: '/products', name: 'products', component: ProductsView },
        { path: '/invoices', name: 'invoices', component: InvoicesView },
        { path: '/invoices/new', name: 'invoice-new', component: InvoiceFormView },
        { path: '/invoices/:id/edit', name: 'invoice-edit', component: InvoiceFormView, props: true },
        { path: '/settings', name: 'settings', component: SettingsView },
    ],
});

export default router;
