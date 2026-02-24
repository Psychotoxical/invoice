<template>
  <div>
    <div class="topbar">
      <div class="topbar-title">{{ isEdit ? $t('invoices.editInvoice') : $t('invoices.newInvoiceTitle') }}</div>
      <div class="topbar-actions">
        <router-link to="/invoices" class="btn btn-secondary">{{ $t('common.back') }}</router-link>
        <button class="btn btn-primary" @click="saveInvoice" :disabled="!canSave">💾 {{ $t('common.save') }}</button>
        <button v-if="isEdit" class="btn btn-secondary" @click="exportPdf">📄 {{ $t('invoiceForm.pdf') }}</button>
      </div>
    </div>
    <div class="page-content">
      <div class="card mb-4">
        <div class="card-header"><h2>{{ $t('invoiceForm.invoiceData') }}</h2></div>
        <div class="card-body">
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ $t('invoiceForm.seller') }} *</label>
              <select class="form-select" v-model.number="invoice.seller_id" @change="onSellerChange">
                <option :value="0" disabled>{{ $t('common.choose') }}</option>
                <option v-for="s in sellers" :key="s.id" :value="s.id">{{ s.name }}</option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('invoiceForm.customer') }} *</label>
              <select class="form-select" v-model.number="invoice.customer_id">
                <option :value="0" disabled>{{ $t('common.choose') }}</option>
                <option v-for="c in customers" :key="c.id" :value="c.id">{{ c.name }}</option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('invoiceForm.invoiceNumber') }} *</label>
              <div class="flex gap-2">
                <input class="form-input" v-model="invoice.invoice_number" :placeholder="suggestedNumber" />
                <button class="btn btn-secondary btn-sm" @click="autoNumber" title="Auto" :disabled="!invoice.seller_id">🔄</button>
              </div>
              <div class="form-hint">{{ $t('invoiceForm.autoOrManual') }}</div>
            </div>
          </div>
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ $t('common.date') }} *</label>
              <input class="form-input" type="date" v-model="invoice.date" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('invoiceForm.paymentTerms') }}</label>
              <select class="form-select" v-model="invoice.payment_terms" @change="calcDueDate">
                <option value="Sofort fällig">{{ $t('invoiceForm.paymentImmediate') }}</option>
                <option value="7 Tage netto">{{ $t('invoiceForm.payment7') }}</option>
                <option value="14 Tage netto">{{ $t('invoiceForm.payment14') }}</option>
                <option value="30 Tage netto">{{ $t('invoiceForm.payment30') }}</option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('invoiceForm.dueDate') }}</label>
              <input class="form-input" type="date" v-model="invoice.due_date" />
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('common.notes') }}</label>
            <textarea class="form-textarea" v-model="invoice.notes" rows="2" :placeholder="$t('invoiceForm.notesPlaceholder')"></textarea>
          </div>
        </div>
      </div>

      <div class="card mb-4">
        <div class="card-header">
          <h2>{{ $t('invoiceForm.positions') }}</h2>
          <div class="flex gap-2">
            <select class="form-select" style="width: auto; font-size: 13px" v-model.number="selectedProduct" v-if="availableProducts.length">
              <option :value="0">{{ $t('invoiceForm.fromCatalog') }}</option>
              <option v-for="p in availableProducts" :key="p.id" :value="p.id">{{ p.name }} ({{ formatCurrency(p.price_net) }})</option>
            </select>
            <button class="btn btn-secondary btn-sm" v-if="selectedProduct" @click="addFromCatalog">{{ $t('common.add') }}</button>
            <button class="btn btn-primary btn-sm" @click="addItem">{{ $t('invoiceForm.freePosition') }}</button>
          </div>
        </div>
        <div class="card-body" style="padding: 0; overflow-x: auto">
          <table class="invoice-items-table" v-if="items.length">
            <thead>
              <tr>
                <th style="width:45px">{{ $t('invoiceForm.pos') }}</th>
                <th>{{ $t('common.description') }}</th>
                <th style="width:100px">{{ $t('invoiceForm.quantity') }}</th>
                <th style="width:110px">{{ $t('invoiceForm.unit') }}</th>
                <th style="width:140px">{{ $t('invoiceForm.unitPrice') }}</th>
                <th style="width:100px">{{ $t('invoiceForm.vat') }}</th>
                <th style="width:120px" class="text-right">{{ $t('invoiceForm.total') }}</th>
                <th style="width:45px"></th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(item, i) in items" :key="i"
                draggable="true"
                @dragstart="onDragStart(i, $event)"
                @dragover.prevent="onDragOver(i)"
                @dragend="onDragEnd"
                :class="{ 'drag-over': dragOverIndex === i, 'dragging': dragIndex === i }"
                style="cursor: grab">
                <td class="text-center drag-handle" style="cursor: grab; user-select: none">☰</td>
                <td><input class="form-input" v-model="item.description" :placeholder="$t('common.description')" @input="recalc" /></td>
                <td><input class="form-input text-right" v-model.number="item.quantity" type="number" step="0.01" min="0" @input="recalc" /></td>
                <td>
                  <select class="form-select" v-model="item.unit">
                    <option value="Stk">{{ $t('invoiceForm.unitPc') }}</option>
                    <option value="Std">{{ $t('invoiceForm.unitHour') }}</option>
                    <option value="Pausch.">{{ $t('invoiceForm.unitFlat') }}</option>
                    <option value="kg">{{ $t('invoiceForm.unitKg') }}</option>
                    <option value="m">{{ $t('invoiceForm.unitM') }}</option>
                    <option value="Lizenz">{{ $t('invoiceForm.unitLicense') }}</option>
                  </select>
                </td>
                <td><input class="form-input text-right" v-model.number="item.price_net" type="number" step="0.01" min="0" @input="recalc" /></td>
                <td>
                  <div class="flex items-center gap-1">
                    <input class="form-input text-right" v-model.number="item.tax_rate" type="number" step="0.5" min="0" max="100" style="width: 65px" @change="recalc" />
                    <span style="font-size: 12px; color: var(--text-secondary)">%</span>
                  </div>
                </td>
                <td><input class="form-input text-right" v-model.number="item.total_gross" type="number" step="0.01" min="0" @input="recalcFromGross(i)" /></td>
                <td><button class="btn btn-ghost btn-sm" @click="removeItem(i)">✕</button></td>
              </tr>
            </tbody>
          </table>
          <div class="empty-state" v-else style="padding: 32px">
            <div class="empty-desc">{{ $t('invoiceForm.noPositions') }}</div>
          </div>
        </div>
      </div>

      <!-- Totals -->
      <div class="invoice-totals" v-if="items.length">
        <table>
          <tbody>
            <tr>
              <td class="total-label">{{ $t('invoiceForm.netTotal') }}</td>
              <td class="total-value">{{ formatCurrency(totals.net) }}</td>
            </tr>
            <tr v-for="tax in taxBreakdown" :key="tax.rate">
              <td class="total-label">{{ $t('invoiceForm.vatAmount', { rate: tax.rate }) }}</td>
              <td class="total-value">{{ formatCurrency(tax.amount) }}</td>
            </tr>
            <tr class="grand-total">
              <td class="total-label">{{ $t('invoiceForm.grossTotal') }}</td>
              <td class="total-value">{{ formatCurrency(totals.gross) }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Payments (only on edit) -->
      <div class="card mb-4" v-if="isEdit">
        <div class="card-header">
          <h2>{{ $t('invoiceForm.payments') }}</h2>
          <div class="flex items-center gap-2">
            <span class="badge" :class="remainingAmount <= 0 ? 'badge-paid' : 'badge-overdue'" style="font-size: 13px">
              {{ formatCurrency(paidTotal) }} / {{ formatCurrency(totals.gross) }}
            </span>
          </div>
        </div>
        <div class="card-body" style="padding: 0">
          <table class="data-table" v-if="payments.length">
            <thead>
              <tr>
                <th>{{ $t('common.date') }}</th>
                <th class="text-right">{{ $t('common.amount') }}</th>
                <th>{{ $t('invoiceForm.paymentMethod') }}</th>
                <th>{{ $t('common.notes') }}</th>
                <th style="width: 45px"></th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="p in payments" :key="p.id">
                <td>{{ formatDate(p.date) }}</td>
                <td class="text-right" style="color: var(--success); font-weight: 600">{{ formatCurrency(p.amount) }}</td>
                <td>{{ p.method }}</td>
                <td>{{ p.notes }}</td>
                <td><button class="btn btn-ghost btn-sm" @click="removePayment(p.id!)">🗑️</button></td>
              </tr>
            </tbody>
          </table>
          <div style="padding: 12px 16px; border-top: 1px solid var(--border-color)">
            <div class="form-row" style="align-items: flex-end">
              <div class="form-group" style="margin-bottom: 0">
                <label class="form-label" style="font-size: 11px">{{ $t('common.date') }}</label>
                <input class="form-input" type="date" v-model="newPayment.date" style="font-size: 13px" />
              </div>
              <div class="form-group" style="margin-bottom: 0">
                <label class="form-label" style="font-size: 11px">{{ $t('common.amount') }} (€)</label>
                <input class="form-input text-right" type="number" step="0.01" min="0" v-model.number="newPayment.amount" style="font-size: 13px" />
              </div>
              <div class="form-group" style="margin-bottom: 0">
                <label class="form-label" style="font-size: 11px">{{ $t('invoiceForm.paymentMethod') }}</label>
                <select class="form-select" v-model="newPayment.method" style="font-size: 13px">
                  <option value="">—</option>
                  <option value="bank">{{ $t('invoiceForm.methodBank') }}</option>
                  <option value="cash">{{ $t('invoiceForm.methodCash') }}</option>
                  <option value="paypal">PayPal</option>
                  <option value="other">{{ $t('invoiceForm.methodOther') }}</option>
                </select>
              </div>
              <div class="form-group" style="margin-bottom: 0">
                <button class="btn btn-primary btn-sm" @click="addNewPayment" :disabled="!newPayment.amount">
                  + {{ $t('invoiceForm.addPayment') }}
                </button>
              </div>
            </div>
          </div>
          <div v-if="remainingAmount > 0" style="padding: 8px 16px; font-size: 13px; color: var(--danger); font-weight: 600; border-top: 1px solid var(--border-color)">
            {{ $t('invoiceForm.remaining') }}: {{ formatCurrency(remainingAmount) }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import {
  getSellers, getCustomers, getProducts, getInvoice,
  createInvoice, updateInvoice, generateInvoiceNumber, getSetting,
  getPaymentsForInvoice, addPayment, deletePayment,
  type Seller, type Customer, type Product, type Invoice, type InvoiceItem, type Payment
} from '../services/database';
import { generateInvoicePdf } from '../utils/pdfGenerator';
import { useToast } from '../composables/useToast';

const { locale, t } = useI18n();
const route = useRoute();
const router = useRouter();
const toast = useToast();

const dragIndex = ref<number | null>(null);
const dragOverIndex = ref<number | null>(null);

const isEdit = computed(() => !!route.params.id);
const sellers = ref<Seller[]>([]);
const customers = ref<Customer[]>([]);
const products = ref<Product[]>([]);
const selectedProduct = ref(0);
const suggestedNumber = ref('');
const defaultTaxRate = ref(19);

const invoice = ref<Invoice>({
  seller_id: 0, customer_id: 0, invoice_number: '', date: new Date().toISOString().split('T')[0],
  due_date: '', status: 'draft', notes: '', payment_terms: '14 Tage netto',
  total_net: 0, total_tax: 0, total_gross: 0
});

const items = ref<InvoiceItem[]>([]);

// Payments
const payments = ref<Payment[]>([]);
const newPayment = ref({ date: new Date().toISOString().split('T')[0], amount: 0, method: '', notes: '' });
const paidTotal = computed(() => payments.value.reduce((s, p) => s + p.amount, 0));
const remainingAmount = computed(() => Math.max(0, totals.value.gross - paidTotal.value));

const availableProducts = computed(() => {
  if (!invoice.value.seller_id) return [];
  return products.value.filter(p => p.seller_id === invoice.value.seller_id && p.active);
});

const canSave = computed(() =>
  invoice.value.seller_id && invoice.value.customer_id && invoice.value.invoice_number && items.value.length > 0
);

const totals = computed(() => {
  let net = 0, tax = 0, gross = 0;
  for (const item of items.value) {
    net += item.total_net;
    tax += item.total_tax;
    gross += item.total_gross;
  }
  return { net, tax, gross };
});

const taxBreakdown = computed(() => {
  const map: Record<number, number> = {};
  for (const item of items.value) {
    if (!map[item.tax_rate]) map[item.tax_rate] = 0;
    map[item.tax_rate] += item.total_tax;
  }
  return Object.entries(map)
    .map(([rate, amount]) => ({ rate: Number(rate), amount }))
    .filter(t => t.amount > 0)
    .sort((a, b) => a.rate - b.rate);
});

onMounted(async () => {
  try {
    sellers.value = await getSellers();
    customers.value = await getCustomers();
    products.value = await getProducts();

    if (route.params.id) {
      const inv = await getInvoice(Number(route.params.id));
      if (inv) {
        invoice.value = inv;
        items.value = inv.items || [];
        payments.value = await getPaymentsForInvoice(inv.id!);
      }
    } else {
      // Load defaults from settings
      const pt = await getSetting('default_payment_terms');
      if (pt) invoice.value.payment_terms = pt;
      const tr = await getSetting('default_tax_rate');
      if (tr) defaultTaxRate.value = Number(tr);
      const dn = await getSetting('default_note');
      if (dn) invoice.value.notes = dn;
      calcDueDate();
    }
  } catch (e) { console.error(e); }
});

async function onSellerChange() {
  if (!isEdit.value && invoice.value.seller_id) {
    try {
      suggestedNumber.value = await peekInvoiceNumber(invoice.value.seller_id);
    } catch (e) { console.error(e); }
  }
}

async function peekInvoiceNumber(sellerId: number): Promise<string> {
  const seller = sellers.value.find(s => s.id === sellerId);
  if (!seller) return '';
  const year = new Date().getFullYear();
  const num = String(seller.next_invoice_number).padStart(4, '0');
  return `${seller.invoice_prefix}-${year}-${num}`;
}

async function autoNumber() {
  if (!invoice.value.seller_id) return;
  try {
    const num = await generateInvoiceNumber(invoice.value.seller_id);
    invoice.value.invoice_number = num;
    // reload sellers to get updated next_invoice_number
    sellers.value = await getSellers();
  } catch (e) { console.error(e); }
}

function calcDueDate() {
  if (!invoice.value.date) return;
  const base = new Date(invoice.value.date);
  let days = 14;
  if (invoice.value.payment_terms === 'Sofort fällig') days = 0;
  else if (invoice.value.payment_terms === '7 Tage netto') days = 7;
  else if (invoice.value.payment_terms === '30 Tage netto') days = 30;
  base.setDate(base.getDate() + days);
  invoice.value.due_date = base.toISOString().split('T')[0];
}

function addItem() {
  items.value.push({
    product_id: null, position: items.value.length + 1,
    description: '', quantity: 1, unit: 'Stk', price_net: 0,
    tax_rate: defaultTaxRate.value, total_net: 0, total_tax: 0, total_gross: 0
  });
}

function addFromCatalog() {
  const p = products.value.find(prod => prod.id === selectedProduct.value);
  if (!p) return;
  items.value.push({
    product_id: p.id || null, position: items.value.length + 1,
    description: p.name + (p.description ? ' – ' + p.description : ''),
    quantity: 1, unit: p.unit, price_net: p.price_net,
    tax_rate: p.tax_rate, total_net: p.price_net,
    total_tax: Math.round(p.price_net * p.tax_rate) / 100,
    total_gross: p.price_net + Math.round(p.price_net * p.tax_rate) / 100
  });
  selectedProduct.value = 0;
  recalc();
}

function removeItem(i: number) {
  items.value.splice(i, 1);
  recalc();
}

function onDragStart(index: number, e: DragEvent) {
  dragIndex.value = index;
  if (e.dataTransfer) {
    e.dataTransfer.effectAllowed = 'move';
    e.dataTransfer.setData('text/plain', String(index));
  }
}

function onDragOver(index: number) {
  if (dragIndex.value === null || dragIndex.value === index) return;
  dragOverIndex.value = index;
  const draggedItem = items.value[dragIndex.value];
  items.value.splice(dragIndex.value, 1);
  items.value.splice(index, 0, draggedItem);
  dragIndex.value = index;
}

function onDragEnd() {
  dragIndex.value = null;
  dragOverIndex.value = null;
}

function recalc() {
  for (const item of items.value) {
    item.total_net = Math.round(item.quantity * item.price_net * 100) / 100;
    item.total_tax = Math.round(item.total_net * item.tax_rate) / 100;
    item.total_gross = Math.round((item.total_net + item.total_tax) * 100) / 100;
  }
}

function recalcFromGross(index: number) {
  const item = items.value[index];
  if (!item || item.total_gross <= 0) return;
  const gross = item.total_gross;
  const taxMultiplier = 1 + item.tax_rate / 100;
  item.total_net = Math.round((gross / taxMultiplier) * 100) / 100;
  item.total_tax = Math.round((gross - item.total_net) * 100) / 100;
  if (item.quantity > 0) {
    item.price_net = Math.round((item.total_net / item.quantity) * 100) / 100;
  }
}

async function saveInvoice() {
  recalc();
  invoice.value.total_net = totals.value.net;
  invoice.value.total_tax = totals.value.tax;
  invoice.value.total_gross = totals.value.gross;

  // update positions
  items.value.forEach((item, i) => item.position = i + 1);

  try {
    if (isEdit.value) {
      await updateInvoice(invoice.value, items.value);
    } else {
      await createInvoice(invoice.value, items.value);
    }
    toast.success(t('toast.invoiceSaved'));
    router.push('/invoices');
  } catch (e) { console.error('Save invoice error:', e); toast.error(t('toast.error')); }
}

async function exportPdf() {
  try {
    const full = await getInvoice(invoice.value.id!);
    if (full) { await generateInvoicePdf(full); toast.success(t('toast.pdfExported')); }
  } catch (e) { console.error(e); toast.error(t('toast.error')); }
}

function formatCurrency(val: number): string {
  return new Intl.NumberFormat(locale.value === 'de' ? 'de-DE' : 'en-US', { style: 'currency', currency: 'EUR' }).format(val);
}

function formatDate(d: string): string {
  if (!d) return '';
  return new Date(d).toLocaleDateString(locale.value === 'de' ? 'de-DE' : 'en-US');
}

async function addNewPayment() {
  if (!newPayment.value.amount || !invoice.value.id) return;
  try {
    await addPayment({
      invoice_id: invoice.value.id,
      amount: newPayment.value.amount,
      date: newPayment.value.date,
      method: newPayment.value.method,
      notes: newPayment.value.notes,
    });
    payments.value = await getPaymentsForInvoice(invoice.value.id);
    newPayment.value = { date: new Date().toISOString().split('T')[0], amount: 0, method: '', notes: '' };
    toast.success(t('toast.paymentAdded'));
  } catch (e) { console.error(e); toast.error(t('toast.error')); }
}

async function removePayment(id: number) {
  if (!invoice.value.id) return;
  try {
    await deletePayment(id);
    payments.value = await getPaymentsForInvoice(invoice.value.id);
    toast.success(t('toast.paymentDeleted'));
  } catch (e) { console.error(e); toast.error(t('toast.error')); }
}

watch(() => invoice.value.date, calcDueDate);
</script>
