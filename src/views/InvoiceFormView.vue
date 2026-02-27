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
        <div class="card-header">
          <h2>{{ $t('invoiceForm.invoiceData') }}</h2>
        </div>
        <div class="card-body">
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ $t('invoiceForm.seller') }} *</label>
              <select class="form-select" v-model.number="invoice.seller_id" @change="onSellerChange">
                <option :value="0" disabled>{{ $t('common.choose') }}</option>
                <option :value="-1" style="font-weight: 600; color: var(--primary-color)">{{
                  $t('invoiceForm.createNewSeller') }}</option>
                <option v-for="s in sellers" :key="s.id" :value="s.id">{{ s.name }}</option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('invoiceForm.customer') }} *</label>
              <select class="form-select" v-model.number="invoice.customer_id">
                <option :value="0" disabled>{{ $t('common.choose') }}</option>
                <option :value="-1" style="font-weight: 600; color: var(--primary-color)">{{
                  $t('invoiceForm.createNewCustomer') }}</option>
                <option v-for="c in customers" :key="c.id" :value="c.id">{{ c.name }}</option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('invoiceForm.invoiceNumber') }} *</label>
              <div class="flex gap-2">
                <input class="form-input" v-model="invoice.invoice_number" :placeholder="suggestedNumber" />
                <button class="btn btn-secondary btn-sm" @click="autoNumber" title="Auto"
                  :disabled="!invoice.seller_id">🔄</button>
              </div>
              <div class="form-hint">{{ $t('invoiceForm.autoOrManual') }}</div>
            </div>
          </div>
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ $t('common.date') }} *</label>
              <input class="form-input" type="date" v-model="invoice.date" @change="blurInput" />
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
              <input class="form-input" type="date" v-model="invoice.due_date" @change="blurInput" />
            </div>
          </div>
          <div class="form-group" style="display: flex; align-items: center; gap: 8px; margin-bottom: 12px;">
            <input type="checkbox" id="alreadyPaidCheck" v-model="invoice.already_paid" :true-value="1"
              :false-value="0" />
            <label for="alreadyPaidCheck" style="font-size: 14px; cursor: pointer; color: var(--text-primary);">{{
              $t('invoiceForm.markAsPaidCheckbox') }}</label>
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('common.notes') }}</label>
            <textarea class="form-textarea" v-model="invoice.notes" rows="2"
              :placeholder="$t('invoiceForm.notesPlaceholder')"></textarea>
          </div>
        </div>
      </div>

      <div class="card mb-4">
        <div class="card-header">
          <h2>{{ $t('invoiceForm.positions') }}</h2>
          <div class="flex gap-2">
            <select class="form-select" style="width: auto; font-size: 13px" v-model.number="selectedProduct"
              v-if="availableProducts.length">
              <option :value="0">{{ $t('invoiceForm.fromCatalog') }}</option>
              <option v-for="p in availableProducts" :key="p.id" :value="p.id">{{ p.name }} ({{
                formatCurrency(p.price_net) }})</option>
            </select>
            <button class="btn btn-secondary btn-sm" v-if="selectedProduct" @click="addFromCatalog">{{ $t('common.add')
            }}</button>
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
              <tr v-for="(item, i) in items" :key="i" draggable="true" @dragstart="onDragStart(i, $event)"
                @dragover.prevent="onDragOver(i)" @dragend="onDragEnd"
                :class="{ 'drag-over': dragOverIndex === i, 'dragging': dragIndex === i }" style="cursor: grab">
                <td class="text-center drag-handle" style="cursor: grab; user-select: none">☰</td>
                <td><input class="form-input" v-model="item.description" :placeholder="$t('common.description')"
                    @input="recalc" @focus="selectAll" /></td>
                <td><input class="form-input text-right" v-model.number="item.quantity" type="number" step="0.01"
                    min="0" @input="recalc" @focus="selectAll" /></td>
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
                <td><input class="form-input text-right" v-model.number="item.price_net" type="number" step="0.01"
                    min="0" @input="recalc" @focus="selectAll" /></td>
                <td>
                  <div class="flex items-center gap-1">
                    <input class="form-input text-right" v-model.number="item.tax_rate" type="number" step="0.5" min="0"
                      max="100" style="width: 65px" @change="recalc" @focus="selectAll" />
                    <span style="font-size: 12px; color: var(--text-secondary)">%</span>
                  </div>
                </td>
                <td><input class="form-input text-right" v-model.number="item.total_gross" type="number" step="0.01"
                    min="0" @input="recalcFromGross(i)" @focus="selectAll" /></td>
                <td>
                  <div class="flex gap-1" style="justify-content: flex-end">
                    <button class="btn btn-ghost btn-sm" v-if="!item.product_id" @click="openProductModal(i)"
                      :title="$t('invoiceForm.saveToCatalog')">💾</button>
                    <button class="btn btn-ghost btn-sm" @click="removeItem(i)">✕</button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
          <div
            style="padding: 12px 16px; border-top: 1px solid var(--border-color); display: flex; justify-content: flex-end; gap: 16px; align-items: center">
            <span style="font-size: 13px; font-weight: 500; color: var(--text-primary)">{{
              $t('invoiceForm.shippingCosts') }}:</span>
            <div class="flex items-center gap-1">
              <input class="form-input text-right" type="number" step="0.01" min="0"
                v-model.number="invoice.shipping_net" style="width: 80px; font-size: 13px" placeholder="0.00"
                @focus="selectAll" />
              <span style="font-size: 12px; color: var(--text-secondary)">€</span>
            </div>
            <div class="flex items-center gap-1">
              <input class="form-input text-right" type="number" step="0.5" min="0" max="100"
                v-model.number="invoice.shipping_tax_rate" style="width: 60px; font-size: 13px" @focus="selectAll" />
              <span style="font-size: 12px; color: var(--text-secondary)">% {{ $t('invoiceForm.vat') }}</span>
            </div>
          </div>
          <div class="empty-state" v-if="!items.length" style="padding: 32px">
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
                <td class="text-right" style="color: var(--success); font-weight: 600">{{ formatCurrency(p.amount) }}
                </td>
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
                <input class="form-input" type="date" v-model="newPayment.date" style="font-size: 13px"
                  @change="blurInput" />
              </div>
              <div class="form-group" style="margin-bottom: 0">
                <label class="form-label" style="font-size: 11px">{{ $t('common.amount') }} (€)</label>
                <input class="form-input text-right" type="number" step="0.01" min="0"
                  v-model.number="newPayment.amount" style="font-size: 13px" @focus="selectAll" />
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
          <div v-if="remainingAmount > 0"
            style="padding: 8px 16px; font-size: 13px; color: var(--danger); font-weight: 600; border-top: 1px solid var(--border-color)">
            {{ $t('invoiceForm.remaining') }}: {{ formatCurrency(remainingAmount) }}
          </div>
        </div>
      </div>
    </div>

    <!-- Quick Add Customer Modal -->
    <div v-if="showCustomerModal" class="modal-overlay" @click.self="confirmCloseCustomer">
      <div class="modal modal-wide" @click.stop>
        <div class="modal-header">
          <h2>{{ $t('invoiceForm.quickCustomerTitle') }}</h2>
          <button class="btn btn-ghost btn-icon" @click="confirmCloseCustomer">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label class="form-label">{{ $t('common.name') }} *</label>
            <input class="form-input" v-model="newCustomerForm.name" :placeholder="$t('customers.namePlaceholder')" />
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('common.street') }}</label>
            <input class="form-input" v-model="newCustomerForm.street"
              :placeholder="$t('customers.streetPlaceholder')" />
          </div>
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ $t('common.zip') }}</label>
              <input class="form-input" v-model="newCustomerForm.zip" :placeholder="$t('customers.zipPlaceholder')" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('common.city') }}</label>
              <input class="form-input" v-model="newCustomerForm.city" :placeholder="$t('customers.cityPlaceholder')" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('common.country') }}</label>
              <input class="form-input" v-model="newCustomerForm.country" />
            </div>
          </div>
          <div class="form-row">
            <div class="form-group">
              <label class="form-label">{{ $t('common.phone') }}</label>
              <input class="form-input" v-model="newCustomerForm.phone" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('common.email') }}</label>
              <input class="form-input" v-model="newCustomerForm.email" type="email" />
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('common.notes') }}</label>
            <textarea class="form-textarea" v-model="newCustomerForm.notes"
              :placeholder="$t('customers.notesPlaceholder')"></textarea>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="confirmCloseCustomer">{{ $t('common.cancel') }}</button>
          <button class="btn btn-primary" @click="saveQuickCustomer" :disabled="!newCustomerForm.name">{{
            $t('common.save') }}</button>
        </div>
      </div>
    </div>

    <!-- Quick Add Seller Modal (Full Form) -->
    <div v-if="showSellerModal" class="modal-overlay" @click.self="confirmCloseSeller">
      <div class="modal modal-wide" @click.stop style="max-height: 90vh; overflow-y: auto;">
        <div class="modal-header">
          <h2>{{ $t('invoiceForm.quickSellerTitle') }}</h2>
          <button class="btn btn-ghost btn-icon" @click="confirmCloseSeller">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label class="form-label">{{ $t('common.name') }} (Firma) *</label>
            <input class="form-input" v-model="newSellerForm.name" :placeholder="$t('sellers.namePlaceholder')" />
          </div>
          <div class="form-row">
            <div class="form-group">
              <label class="form-label">{{ $t('common.firstName') }}</label>
              <input class="form-input" v-model="newSellerForm.first_name" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('common.lastName') }}</label>
              <input class="form-input" v-model="newSellerForm.last_name" />
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('sellers.logo') }}</label>
            <div class="flex items-center gap-2">
              <div class="logo-preview" @click="selectSellerLogo">
                <img v-if="newSellerForm.logo_data" :src="newSellerForm.logo_data" alt="Logo" />
                <div v-else class="logo-placeholder">{{ $t('sellers.chooseLogo') }}</div>
              </div>
              <button v-if="newSellerForm.logo_data" class="btn btn-ghost btn-sm"
                @click="newSellerForm.logo_data = ''">{{ $t('sellers.removeLogo') }}</button>
            </div>
          </div>
          <hr style="border-color: var(--border-color); margin: 8px 0 16px" />
          <div class="form-group">
            <label class="form-label">{{ $t('common.street') }}</label>
            <input class="form-input" v-model="newSellerForm.street" :placeholder="$t('customers.streetPlaceholder')" />
          </div>
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ $t('common.zip') }}</label>
              <input class="form-input" v-model="newSellerForm.zip" :placeholder="$t('customers.zipPlaceholder')" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('common.city') }}</label>
              <input class="form-input" v-model="newSellerForm.city" :placeholder="$t('customers.cityPlaceholder')" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('common.country') }}</label>
              <input class="form-input" v-model="newSellerForm.country" />
            </div>
          </div>
          <div class="form-row">
            <div class="form-group">
              <label class="form-label">{{ $t('common.phone') }}</label>
              <input class="form-input" v-model="newSellerForm.phone" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('common.email') }}</label>
              <input class="form-input" v-model="newSellerForm.email" type="email" />
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('sellers.website') }}</label>
            <input class="form-input" v-model="newSellerForm.website" placeholder="www.example.de" />
          </div>
          <hr style="border-color: var(--border-color); margin: 8px 0 16px" />
          <div class="form-row">
            <div class="form-group">
              <label class="form-label">{{ $t('sellers.taxId') }}</label>
              <input class="form-input" v-model="newSellerForm.tax_id" placeholder="12/345/67890" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('sellers.vatId') }}</label>
              <input class="form-input" v-model="newSellerForm.vat_id" placeholder="DE123456789" />
            </div>
          </div>
          <hr style="border-color: var(--border-color); margin: 8px 0 16px" />
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ $t('sellers.bank') }}</label>
              <input class="form-input" v-model="newSellerForm.bank_name" placeholder="Sparkasse" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('sellers.iban') }}</label>
              <input class="form-input" v-model="newSellerForm.bank_iban" placeholder="DE89 3704 0044 0532 0130 00" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('sellers.bic') }}</label>
              <input class="form-input" v-model="newSellerForm.bank_bic" placeholder="COBADEFFXXX" />
            </div>
          </div>
          <hr style="border-color: var(--border-color); margin: 8px 0 16px" />
          <div class="form-row">
            <div class="form-group">
              <label class="form-label">{{ $t('sellers.invoicePrefix') }}</label>
              <input class="form-input" v-model="newSellerForm.invoice_prefix" placeholder="RE" @focus="guessPrefix" />
              <div class="form-hint">{{ $t('sellers.prefixExample', { prefix: newSellerForm.invoice_prefix || 'RE' }) }}
              </div>
            </div>
            <div class="form-group" style="width: 140px;">
              <label class="form-label">{{ $t('sellers.color') }}</label>
              <input class="form-input" type="color" v-model="newSellerForm.color"
                style="height: 38px; padding: 2px 4px; cursor: pointer;" />
              <div class="form-hint" style="line-height: 1.2">{{ $t('sellers.colorHint') }}</div>
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('sellers.nextInvoiceNr') }}</label>
              <input class="form-input" v-model.number="newSellerForm.next_invoice_number" type="number" min="1" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('sellers.pdfTemplate') }}</label>
              <select class="form-select" v-model="newSellerForm.pdf_template">
                <option value="classic">Classic</option>
                <option value="modern">Modern</option>
                <option value="minimal">Minimal</option>
              </select>
            </div>
          </div>
          <hr style="border-color: var(--border-color); margin: 8px 0 16px" />
          <h3 style="margin-top: 0; margin-bottom: 12px; font-size: 14px; color: var(--text-primary)">{{
            $t('settings.invoiceDefaults') }}</h3>
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ $t('settings.defaultPaymentTerms') }}</label>
              <select class="form-select" v-model="newSellerForm.default_payment_terms">
                <option value="">{{ $t('common.choose') || '—' }}</option>
                <option value="Sofort fällig">{{ $t('invoiceForm.paymentImmediate') }}</option>
                <option value="7 Tage netto">{{ $t('invoiceForm.payment7') }}</option>
                <option value="14 Tage netto">{{ $t('invoiceForm.payment14') }}</option>
                <option value="30 Tage netto">{{ $t('invoiceForm.payment30') }}</option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('settings.defaultVat') }} (%)</label>
              <input class="form-input" v-model.number="newSellerForm.default_tax_rate" type="number" step="0.5" min="0"
                max="100" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('settings.currency') }}</label>
              <select class="form-select" v-model="newSellerForm.currency">
                <option value="">—</option>
                <option value="EUR">EUR (€)</option>
                <option value="USD">USD ($)</option>
                <option value="GBP">GBP (£)</option>
                <option value="CHF">CHF</option>
              </select>
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('settings.defaultNote') }}</label>
            <textarea class="form-textarea" v-model="newSellerForm.default_note" rows="2"
              :placeholder="$t('settings.defaultNotePlaceholder')"></textarea>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="confirmCloseSeller">{{ $t('common.cancel') }}</button>
          <button class="btn btn-primary" @click="saveQuickSeller" :disabled="!newSellerForm.name">{{ $t('common.save')
          }}</button>
        </div>
      </div>
    </div>

    <!-- Quick Add Product Modal -->
    <div v-if="showProductModal" class="modal-overlay" @click.self="confirmCloseProduct">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h2>{{ $t('products.newProductTitle') }}</h2>
          <button class="btn btn-ghost btn-icon" @click="confirmCloseProduct">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-row">
            <div class="form-group">
              <label class="form-label">{{ $t('products.type') }} *</label>
              <select class="form-select" v-model="newProductForm.type">
                <option value="product">{{ $t('products.typeProduct') }}</option>
                <option value="service">{{ $t('products.typeService') }}</option>
              </select>
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('common.name') }} *</label>
            <input class="form-input" v-model="newProductForm.name" :placeholder="$t('products.namePlaceholder')" />
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('common.description') }}</label>
            <textarea class="form-textarea" v-model="newProductForm.description" rows="2"
              :placeholder="$t('products.descriptionPlaceholder')"></textarea>
          </div>
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ $t('products.unit') }}</label>
              <select class="form-select" v-model="newProductForm.unit">
                <option value="Stk">{{ $t('products.unitPc') }}</option>
                <option value="Std">{{ $t('products.unitHour') }}</option>
                <option value="Pausch.">{{ $t('products.unitFlat') }}</option>
                <option value="kg">{{ $t('products.unitKg') }}</option>
                <option value="m">{{ $t('products.unitM') }}</option>
                <option value="Lizenz">{{ $t('products.unitLicense') }}</option>
              </select>
            </div>
            <div class="form-group" style="display: flex; flex-direction: column; justify-content: center;">
              <label class="form-label" style="opacity: 0; margin-bottom: 4px;">isGross</label>
              <div style="display: flex; align-items: center; gap: 8px;">
                <input type="checkbox" id="quickIsGrossCheck" v-model="quickIsGross" />
                <label for="quickIsGrossCheck"
                  style="font-size: 13px; cursor: pointer; color: var(--text-primary);">Brutto eingeben</label>
              </div>
            </div>
          </div>
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ quickIsGross ? 'Bruttopreis' : $t('products.netPrice') }} (€) *</label>
              <input class="form-input" v-model.number="quickDisplayPrice" type="number" step="0.01" min="0"
                @focus="selectAll" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('products.taxRate') }} (%)</label>
              <input class="form-input" v-model.number="newProductForm.tax_rate" type="number" step="0.5" min="0"
                max="100" @focus="selectAll" />
            </div>
          </div>
          <div class="form-group" v-if="newProductForm.type === 'product'">
            <label class="form-label">{{ $t('products.stock') }}</label>
            <input class="form-input" v-model.number="newProductForm.stock" type="number" min="0"
              style="max-width: 150px" />
          </div>
        </div>
        <div class="modal-footer">
          <div v-if="quickIsGross" style="font-size: 12px; color: var(--text-secondary); margin-right: auto;">
            {{ $t('products.calculatedNet') }}: {{ formatCurrency(quickCalculatedNetPrice) }}
          </div>
          <button class="btn btn-secondary" @click="confirmCloseProduct">{{ $t('common.cancel') }}</button>
          <button class="btn btn-primary" @click="confirmSaveProduct"
            :disabled="!newProductForm.name || quickDisplayPrice <= 0">{{
              $t('common.save') }}</button>
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
  createInvoice, updateInvoice, generateInvoiceNumber,
  getPaymentsForInvoice, addPayment, deletePayment,
  createCustomer, createSeller, createProduct,
  type Seller, type Customer, type Product, type Invoice, type InvoiceItem, type Payment
} from '../services/database';
import { generateInvoicePdf } from '../utils/pdfGenerator';
import { useToast } from '../composables/useToast';
import { confirm } from '@tauri-apps/plugin-dialog';

const { locale, t } = useI18n({ useScope: 'global' });

function selectAll(e: Event) {
  const target = e.target as HTMLInputElement;
  if (target) {
    // Kurzes Timeout verhindert, dass der Browser die Markierung durch das 
    // Setzen des Cursors direkt nach dem Klick wieder aufhebt.
    setTimeout(() => target.select(), 10);
  }
}

function blurInput(e: Event) {
  (e.target as HTMLInputElement)?.blur();
}

function formatCurrency(val: number): string {
  return new Intl.NumberFormat(locale.value === 'de' ? 'de-DE' : 'en-US', { style: 'currency', currency: 'EUR' }).format(val);
}

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
const currency = ref('EUR');

const invoice = ref<Invoice>({
  seller_id: 0, customer_id: 0, invoice_number: '', date: new Date().toISOString().split('T')[0],
  due_date: '', status: 'draft', notes: '', payment_terms: '14 Tage netto',
  total_net: 0, total_tax: 0, total_gross: 0,
  shipping_net: 0, shipping_tax_rate: 19, already_paid: 0
});

const items = ref<InvoiceItem[]>([]);

// Modals
const showCustomerModal = ref(false);
const showSellerModal = ref(false);
const showProductModal = ref(false);
const productSaveIndex = ref(-1);

const emptyCustomer = (): Customer => ({
  name: '', street: '', city: '', zip: '', country: 'Deutschland',
  phone: '', email: '', notes: ''
});

const emptySeller = (): Seller => ({
  name: '', first_name: '', last_name: '', street: '', city: '', zip: '', country: 'Deutschland',
  phone: '', email: '', website: '', tax_id: '', vat_id: '',
  bank_name: '', bank_iban: '', bank_bic: '', logo_data: '',
  invoice_prefix: 'RE', next_invoice_number: 1, pdf_template: 'classic'
});

const emptyProduct = (): Product => ({
  seller_id: 0, name: '', description: '', type: 'product',
  unit: 'Stk', price_net: 0, tax_rate: 19, stock: 0, active: 1
});

const newCustomerForm = ref<Customer>(emptyCustomer());
const originalCustomerForm = ref<Customer>(emptyCustomer());

const newSellerForm = ref<Seller>(emptySeller());
const originalSellerForm = ref<Seller>(emptySeller());

const newProductForm = ref<Product>(emptyProduct());
const originalProductForm = ref<Product>(emptyProduct());

const quickIsGross = ref(false);
const quickDisplayPrice = ref(0);

const quickCalculatedNetPrice = computed(() => {
  if (!quickIsGross.value) return quickDisplayPrice.value;
  const taxMultiplier = 1 + (newProductForm.value.tax_rate / 100);
  return Math.round((quickDisplayPrice.value / taxMultiplier) * 100) / 100;
});

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
  let net = typeof invoice.value.shipping_net === 'number' ? invoice.value.shipping_net : 0;
  let tax = Math.round(net * (typeof invoice.value.shipping_tax_rate === 'number' ? invoice.value.shipping_tax_rate : 19)) / 100;
  let gross = net + tax;
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
  const sNet = typeof invoice.value.shipping_net === 'number' ? invoice.value.shipping_net : 0;
  if (sNet > 0) {
    const sRate = typeof invoice.value.shipping_tax_rate === 'number' ? invoice.value.shipping_tax_rate : 19;
    const sTax = Math.round(sNet * sRate) / 100;
    if (!map[sRate]) map[sRate] = 0;
    map[sRate] += sTax;
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
      // Default fallback
      calcDueDate();
    }
  } catch (e) { console.error(e); }
});

async function onSellerChange() {
  if (invoice.value.seller_id === -1) {
    invoice.value.seller_id = 0;
    newSellerForm.value = emptySeller();
    originalSellerForm.value = emptySeller();
    showSellerModal.value = true;
    return;
  }

  const seller = sellers.value.find(s => s.id === invoice.value.seller_id);
  if (seller) {
    if (seller.default_tax_rate !== null && seller.default_tax_rate !== undefined) {
      defaultTaxRate.value = seller.default_tax_rate;
    }
    if (seller.currency) {
      currency.value = seller.currency;
    }
  }

  if (!isEdit.value && invoice.value.seller_id) {
    try {
      suggestedNumber.value = await peekInvoiceNumber(invoice.value.seller_id);

      // Apply seller defaults to the new invoice
      if (seller) {
        if (seller.default_payment_terms) {
          invoice.value.payment_terms = seller.default_payment_terms;
        }
        if (seller.default_note) {
          invoice.value.notes = seller.default_note;
        }
        calcDueDate();
      }
    } catch (e) { console.error(e); }
  }
}

watch(() => invoice.value.customer_id, (newVal) => {
  if (newVal === -1) {
    invoice.value.customer_id = 0;
    newCustomerForm.value = emptyCustomer();
    originalCustomerForm.value = emptyCustomer();
    showCustomerModal.value = true;
  }
});

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

  // set status to paid if checked, but don't undo paid if already paid and unchecked (handled manually by user if needed)
  if (invoice.value.already_paid && invoice.value.status === 'draft') {
    invoice.value.status = 'paid';
  }

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

async function saveQuickCustomer() {
  if (!newCustomerForm.value.name) return;
  try {
    const id = await createCustomer(newCustomerForm.value);
    customers.value = await getCustomers();
    invoice.value.customer_id = id;
    showCustomerModal.value = false;
    toast.success(t('toast.customerSaved'));
  } catch (e) { console.error(e); toast.error(t('toast.error')); }
}

function guessPrefix() {
  if (newSellerForm.value.name && newSellerForm.value.invoice_prefix === 'RE') {
    newSellerForm.value.invoice_prefix = newSellerForm.value.name.substring(0, 3).toUpperCase();
  }
}

async function selectSellerLogo() {
  try {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = 'image/*';
    input.onchange = (e: Event) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;
      const reader = new FileReader();
      reader.onload = () => {
        newSellerForm.value.logo_data = reader.result as string;
      };
      reader.readAsDataURL(file);
    };
    input.click();
  } catch (e) {
    console.error('Logo select error:', e);
  }
}

async function saveQuickSeller() {
  if (!newSellerForm.value.name) return;
  try {
    const id = await createSeller(newSellerForm.value);
    sellers.value = await getSellers();
    invoice.value.seller_id = id;
    showSellerModal.value = false;
    toast.success(t('toast.sellerSaved'));
    onSellerChange();
  } catch (e) { console.error(e); toast.error(t('toast.error')); }
}

function openProductModal(index: number) {
  const item = items.value[index];
  if (!item) return;

  if (!invoice.value.seller_id) {
    toast.error(t('invoiceForm.chooseSellerFirst'));
    return;
  }

  quickIsGross.value = false;
  quickDisplayPrice.value = item.price_net || 0;

  productSaveIndex.value = index;
  const f: Product = {
    seller_id: invoice.value.seller_id,
    name: item.description || t('invoiceForm.freePosition'),
    description: '',
    type: item.unit === 'Std' || item.unit === 'Pausch.' ? 'service' : 'product',
    unit: item.unit || 'Stk',
    price_net: item.price_net || 0,
    tax_rate: item.tax_rate || 19,
    stock: 0,
    active: 1
  };
  newProductForm.value = f;
  originalProductForm.value = { ...f };
  showProductModal.value = true;
}

async function confirmCloseCustomer() {
  const hasChanges = JSON.stringify(newCustomerForm.value) !== JSON.stringify(originalCustomerForm.value);
  if (hasChanges) {
    const agreed = await confirm(t('common.unsavedChanges'), { title: 'VibeBill', kind: 'warning' });
    if (!agreed) return;
  }
  showCustomerModal.value = false;
}

async function confirmCloseSeller() {
  const hasChanges = JSON.stringify(newSellerForm.value) !== JSON.stringify(originalSellerForm.value);
  if (hasChanges) {
    const agreed = await confirm(t('common.unsavedChanges'), { title: 'VibeBill', kind: 'warning' });
    if (!agreed) return;
  }
  showSellerModal.value = false;
}

async function confirmCloseProduct() {
  const hasChanges = JSON.stringify(newProductForm.value) !== JSON.stringify(originalProductForm.value);
  if (hasChanges) {
    const agreed = await confirm(t('common.unsavedChanges'), { title: 'VibeBill', kind: 'warning' });
    if (!agreed) return;
  }
  showProductModal.value = false;
}

async function confirmSaveProduct() {
  if (!newProductForm.value.name || !newProductForm.value.seller_id) return;
  try {
    newProductForm.value.price_net = quickCalculatedNetPrice.value;
    const id = await createProduct(newProductForm.value);

    if (productSaveIndex.value >= 0) {
      items.value[productSaveIndex.value].product_id = id;
    }

    products.value = await getProducts(); // refresh catalog
    showProductModal.value = false;
    toast.success(t('invoiceForm.productSavedToCatalog'));
  } catch (e) { console.error(e); toast.error(t('toast.error')); }
}

</script>

<style scoped></style>
