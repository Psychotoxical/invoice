<template>
  <div>
    <div class="topbar">
      <div class="topbar-title">{{ $t('products.title') }}</div>
      <div class="topbar-actions">
        <button class="btn btn-primary" @click="openForm()">{{ $t('products.newProduct') }}</button>
      </div>
    </div>
    <div class="page-content">
      <div class="toolbar">
        <div class="toolbar-left">
          <div class="search-bar" style="max-width: 260px; width: 100%">
            <span class="search-icon">🔍</span>
            <input class="form-input" v-model="search" :placeholder="$t('products.searchPlaceholder')" />
          </div>
          <select class="form-select" style="width: auto" v-model="filterSeller">
            <option :value="0">{{ $t('products.allSellers') }}</option>
            <option v-for="s in sellers" :key="s.id" :value="s.id">{{ s.name }}</option>
          </select>
        </div>
      </div>
      <div class="tabs">
        <button class="tab" :class="{ active: activeTab === 'all' }" @click="activeTab = 'all'">{{ $t('products.tabAll')
        }}</button>
        <button class="tab" :class="{ active: activeTab === 'product' }" @click="activeTab = 'product'">{{
          $t('products.tabProducts') }}</button>
        <button class="tab" :class="{ active: activeTab === 'service' }" @click="activeTab = 'service'">{{
          $t('products.tabServices') }}</button>
      </div>
      <div class="card">
        <div class="card-body" style="padding: 0">
          <table class="data-table" v-if="filtered.length">
            <thead>
              <tr>
                <th>{{ $t('common.name') }}</th>
                <th>{{ $t('products.type') }}</th>
                <th>{{ $t('products.seller') }}</th>
                <th>{{ $t('products.unit') }}</th>
                <th class="text-right">{{ $t('products.netPrice') }}</th>
                <th class="text-right">{{ $t('products.vat') }}</th>
                <th class="text-right" v-if="activeTab !== 'service'">{{ $t('products.stock') }}</th>
                <th class="actions-cell">{{ $t('common.actions') }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="p in filtered" :key="p.id" @dblclick="openForm(p)" style="cursor: pointer">
                <td>
                  <strong>{{ p.name }}</strong>
                  <div class="text-xs text-secondary" v-if="p.description">{{ p.description }}</div>
                </td>
                <td><span class="badge" :class="p.type === 'product' ? 'badge-sent' : 'badge-paid'">{{ p.type ===
                  'product' ? $t('products.typeProduct') : $t('products.typeService') }}</span></td>
                <td>{{ sellerName(p.seller_id) }}</td>
                <td>{{ p.unit }}</td>
                <td class="text-right">{{ formatCurrency(p.price_net) }}</td>
                <td class="text-right">{{ p.tax_rate }}%</td>
                <td class="text-right" v-if="activeTab !== 'service'">{{ p.type === 'product' ? p.stock : '—' }}</td>
                <td class="actions-cell">
                  <button class="btn btn-ghost btn-sm" @click="openForm(p)">✏️</button>
                  <button class="btn btn-ghost btn-sm" @click="confirmDelete(p)">🗑️</button>
                </td>
              </tr>
            </tbody>
          </table>
          <div class="empty-state" v-else-if="!search && !filterSeller">
            <div class="empty-icon">📦</div>
            <div class="empty-title">{{ $t('products.noProducts') }}</div>
            <div class="empty-desc">{{ $t('products.createFirst') }}</div>
            <button class="btn btn-primary" @click="openForm()">{{ $t('products.newProduct') }}</button>
          </div>
          <div class="empty-state" v-else>
            <div class="empty-desc">{{ $t('common.noResults') }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Modal -->
    <div class="modal-overlay" v-if="showModal" @click.self="confirmClose">
      <div class="modal">
        <div class="modal-header">
          <h2>{{ editing ? $t('products.editProduct') : $t('products.newProductTitle') }}</h2>
          <button class="btn btn-ghost btn-icon" @click="confirmClose">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-row">
            <div class="form-group">
              <label class="form-label">{{ $t('products.seller') }} *</label>
              <select class="form-select" v-model.number="form.seller_id">
                <option :value="0" disabled>{{ $t('common.choose') }}</option>
                <option v-for="s in sellers" :key="s.id" :value="s.id">{{ s.name }}</option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('products.type') }} *</label>
              <select class="form-select" v-model="form.type">
                <option value="product">{{ $t('products.typeProduct') }}</option>
                <option value="service">{{ $t('products.typeService') }}</option>
              </select>
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('common.name') }} *</label>
            <input class="form-input" v-model="form.name" :placeholder="$t('products.namePlaceholder')" />
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('common.description') }}</label>
            <textarea class="form-textarea" v-model="form.description" rows="2"
              :placeholder="$t('products.descriptionPlaceholder')"></textarea>
          </div>
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ $t('products.unit') }}</label>
              <select class="form-select" v-model="form.unit">
                <option value="Stk">{{ $t('products.unitPc') }}</option>
                <option value="Std">{{ $t('products.unitHour') }}</option>
                <option value="Pausch.">{{ $t('products.unitFlat') }}</option>
                <option value="kg">{{ $t('products.unitKg') }}</option>
                <option value="m">{{ $t('products.unitM') }}</option>
                <option value="Lizenz">{{ $t('products.unitLicense') }}</option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('products.netPrice') }} (€) *</label>
              <input class="form-input" v-model.number="form.price_net" type="number" step="0.01" min="0" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('products.taxRate') }} (%)</label>
              <input class="form-input" v-model.number="form.tax_rate" type="number" step="0.5" min="0" max="100" />
            </div>
          </div>
          <div class="form-group" v-if="form.type === 'product'">
            <label class="form-label">{{ $t('products.stock') }}</label>
            <input class="form-input" v-model.number="form.stock" type="number" min="0" style="max-width: 150px" />
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="confirmClose">{{ $t('common.cancel') }}</button>
          <button class="btn btn-primary" @click="save" :disabled="!form.name || !form.seller_id">{{ $t('common.save')
          }}</button>
        </div>
      </div>
    </div>

    <!-- Delete confirm -->
    <div class="modal-overlay" v-if="deleteTarget" @click.self="deleteTarget = null">
      <div class="modal" style="max-width: 400px">
        <div class="modal-header">
          <h2>{{ $t('products.deleteConfirm') }}</h2>
        </div>
        <div class="modal-body">
          <p>{{ $t('products.deleteMessage', { name: deleteTarget.name }) }}</p>
          <div class="confirm-actions">
            <button class="btn btn-secondary" @click="deleteTarget = null">{{ $t('common.cancel') }}</button>
            <button class="btn btn-danger" @click="doDelete">{{ $t('common.delete') }}</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { getProducts, createProduct, updateProduct, deleteProduct, getSellers, type Product, type Seller } from '../services/database';
import { useToast } from '../composables/useToast';
import { confirm } from '@tauri-apps/plugin-dialog';

const { locale, t } = useI18n({ useScope: 'global' });
const toast = useToast();
const products = ref<Product[]>([]);
const sellers = ref<Seller[]>([]);
const search = ref('');
const filterSeller = ref(0);
const activeTab = ref('all');
const showModal = ref(false);
const editing = ref(false);
const deleteTarget = ref<Product | null>(null);

const emptyForm = (): Product => ({
  seller_id: 0, name: '', description: '', type: 'product',
  unit: 'Stk', price_net: 0, tax_rate: 19, stock: 0, active: 1
});

const form = ref<Product>(emptyForm());
const originalForm = ref<Product>(emptyForm());

const filtered = computed(() => {
  let items = products.value;
  if (filterSeller.value) items = items.filter(p => p.seller_id === filterSeller.value);
  if (activeTab.value !== 'all') items = items.filter(p => p.type === activeTab.value);
  if (search.value) {
    const q = search.value.toLowerCase();
    items = items.filter(p => p.name.toLowerCase().includes(q) || p.description.toLowerCase().includes(q));
  }
  return items;
});

onMounted(load);

async function load() {
  try {
    sellers.value = await getSellers();
    products.value = await getProducts();
  } catch (e) { console.error(e); }
}

function sellerName(id: number): string {
  return sellers.value.find(s => s.id === id)?.name || '—';
}

function formatCurrency(val: number): string {
  return new Intl.NumberFormat(locale.value === 'de' ? 'de-DE' : 'en-US', { style: 'currency', currency: 'EUR' }).format(val);
}

function openForm(p?: Product) {
  if (p) {
    editing.value = true;
    form.value = { ...p };
    originalForm.value = { ...p };
  } else {
    editing.value = false;
    const f = emptyForm();
    if (filterSeller.value) f.seller_id = filterSeller.value;
    if (sellers.value.length === 1 && sellers.value[0].id) f.seller_id = sellers.value[0].id;
    form.value = { ...f };
    originalForm.value = { ...f };
  }
  showModal.value = true;
}

async function confirmClose() {
  const hasChanges = JSON.stringify(form.value) !== JSON.stringify(originalForm.value);
  if (hasChanges) {
    const agreed = await confirm(t('common.unsavedChanges'), { title: 'VibeBill', kind: 'warning' });
    if (!agreed) return;
  }
  showModal.value = false;
}

async function save() {
  try {
    if (editing.value) await updateProduct(form.value);
    else await createProduct(form.value);
    showModal.value = false;
    await load();
    toast.success(t('toast.productSaved'));
  } catch (e) { console.error(e); toast.error(t('toast.error')); }
}

function confirmDelete(p: Product) { deleteTarget.value = p; }

async function doDelete() {
  if (!deleteTarget.value?.id) return;
  try { await deleteProduct(deleteTarget.value.id); deleteTarget.value = null; await load(); toast.success(t('toast.productDeleted')); }
  catch (e) { console.error(e); toast.error(t('toast.error')); }
}
</script>
