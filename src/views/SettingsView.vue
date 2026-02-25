<template>
  <div>
    <div class="topbar">
      <div class="topbar-title">{{ $t('settings.title') }}</div>
    </div>
    <div class="page-content">

      <!-- Language -->
      <div class="card mb-4">
        <div class="card-header"><h2>{{ $t('settings.language') }}</h2></div>
        <div class="card-body">
          <div class="form-group">
            <label class="form-label">{{ $t('settings.languageHint') }}</label>
            <select class="form-select" style="max-width: 200px" v-model="language" @change="changeLanguage">
              <option value="de">Deutsch</option>
              <option value="en">English (UK)</option>
            </select>
          </div>
        </div>
      </div>

      <!-- PDF / Download -->
      <div class="card mb-4">
        <div class="card-header"><h2>{{ $t('settings.pdfExport') }}</h2></div>
        <div class="card-body">
          <div class="form-group">
            <label class="form-label">{{ $t('settings.downloadFolder') }}</label>
            <div class="flex gap-2 items-center">
              <input class="form-input" v-model="downloadFolder" readonly :placeholder="$t('settings.folderPlaceholder')" style="flex: 1" />
              <button class="btn btn-secondary btn-sm" @click="pickFolder">{{ $t('settings.chooseFolder') }}</button>
              <button class="btn btn-ghost btn-sm" v-if="downloadFolder" @click="downloadFolder = ''; saveSetting('download_folder', '')">✕</button>
            </div>
            <div class="form-hint">{{ $t('settings.folderHint') }}</div>
          </div>
        </div>
      </div>

      <!-- Standard-Werte für Rechnungen -->
      <div class="card mb-4">
        <div class="card-header"><h2>{{ $t('settings.invoiceDefaults') }}</h2></div>
        <div class="card-body">
          <div class="form-row-3">
            <div class="form-group">
              <label class="form-label">{{ $t('settings.defaultPaymentTerms') }}</label>
              <select class="form-select" v-model="defaultPaymentTerms" @change="saveSetting('default_payment_terms', defaultPaymentTerms)">
                <option value="Sofort fällig">{{ $t('invoiceForm.paymentImmediate') }}</option>
                <option value="7 Tage netto">{{ $t('invoiceForm.payment7') }}</option>
                <option value="14 Tage netto">{{ $t('invoiceForm.payment14') }}</option>
                <option value="30 Tage netto">{{ $t('invoiceForm.payment30') }}</option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('settings.defaultVat') }} (%)</label>
              <input class="form-input" v-model.number="defaultTaxRate" type="number" step="0.5" min="0" max="100" style="max-width: 120px"
                @change="saveSetting('default_tax_rate', String(defaultTaxRate))" />
            </div>
            <div class="form-group">
              <label class="form-label">{{ $t('settings.currency') }}</label>
              <select class="form-select" v-model="currency" @change="saveSetting('currency', currency)">
                <option value="EUR">EUR (€)</option>
                <option value="USD">USD ($)</option>
                <option value="GBP">GBP (£)</option>
                <option value="CHF">CHF</option>
              </select>
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">{{ $t('settings.defaultNote') }}</label>
            <textarea class="form-textarea" v-model="defaultNote" rows="2"
              :placeholder="$t('settings.defaultNotePlaceholder')"
              @blur="saveSetting('default_note', defaultNote)"></textarea>
            <div class="form-hint">{{ $t('settings.defaultNoteHint') }}</div>
          </div>
        </div>
      </div>

      <!-- Datenbank -->
      <div class="card mb-4">
        <div class="card-header"><h2>{{ $t('settings.database') }}</h2></div>
        <div class="card-body">
          <div class="flex gap-2">
            <button class="btn btn-secondary" @click="exportDb">{{ $t('settings.exportBackup') }}</button>
            <button class="btn btn-secondary" @click="importDb">{{ $t('settings.importBackup') }}</button>
          </div>
          <div class="form-hint" style="margin-top: 8px">{{ $t('settings.backupHint') }}</div>
          <div class="form-hint" style="margin-top: 4px">{{ $t('settings.importHint') }}</div>
          <div v-if="backupMessage" class="form-hint" style="margin-top: 8px; color: var(--success)">{{ backupMessage }}</div>
        </div>
      </div>

      <!-- App Info -->
      <div class="card mb-4">
        <div class="card-header"><h2>{{ $t('settings.about') }}</h2></div>
        <div class="card-body">
          <div style="display: flex; flex-direction: column; gap: 8px; font-size: var(--font-size-sm); color: var(--text-secondary)">
            <div><strong style="color: var(--text-primary)">{{ $t('app.title') }}</strong> – {{ $t('app.subtitle') }}</div>
            <div>
              {{ $t('settings.aboutDescription1') }}<br>
              {{ $t('settings.aboutDescription2') }}
            </div>
            <div style="font-style: italic;">{{ $t('settings.vibecoded') }}</div>
            <div>
              {{ $t('settings.copyright') }}<br>
              {{ $t('settings.version') }}: {{ appVersion }}
            </div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { getSetting, setSetting } from '../services/database';
import { open, save } from '@tauri-apps/plugin-dialog';
import { copyFile } from '@tauri-apps/plugin-fs';
import { appConfigDir, join } from '@tauri-apps/api/path';
import { getVersion } from '@tauri-apps/api/app';

const { locale, t } = useI18n();

const language = ref('en');
const downloadFolder = ref('');
const defaultPaymentTerms = ref('14 Tage netto');
const defaultTaxRate = ref(19);
const currency = ref('EUR');
const defaultNote = ref('');
const backupMessage = ref('');
const appVersion = ref('');

onMounted(async () => {
  try {
    const lang = await getSetting('language');
    if (lang) language.value = lang;
    downloadFolder.value = await getSetting('download_folder');
    const pt = await getSetting('default_payment_terms');
    if (pt) defaultPaymentTerms.value = pt;
    const tr = await getSetting('default_tax_rate');
    if (tr) defaultTaxRate.value = Number(tr);
    const cur = await getSetting('currency');
    if (cur) currency.value = cur;
    defaultNote.value = await getSetting('default_note');
    
    try {
      appVersion.value = await getVersion();
    } catch {
      appVersion.value = '1.0.18';
    }
  } catch (e) { console.error(e); }
});

async function changeLanguage() {
  locale.value = language.value;
  await saveSetting('language', language.value);
}

async function saveSetting(key: string, value: string) {
  try { await setSetting(key, value); } catch (e) { console.error(e); }
}

async function pickFolder() {
  try {
    const selected = await open({ directory: true, multiple: false, title: 'Download-Ordner wählen' });
    if (selected && typeof selected === 'string') {
      downloadFolder.value = selected;
      await saveSetting('download_folder', selected);
    }
  } catch (e) { console.error(e); }
}

async function exportDb() {
  try {
    const dataDir = await appConfigDir();
    const dbPath = await join(dataDir, 'vibebill.db');
    const now = new Date();
    const ts = `${now.getFullYear()}${String(now.getMonth() + 1).padStart(2, '0')}${String(now.getDate()).padStart(2, '0')}_${String(now.getHours()).padStart(2, '0')}${String(now.getMinutes()).padStart(2, '0')}`;
    const defaultName = `vibebill_backup_${ts}.db`;

    const dest = await save({
      defaultPath: defaultName,
      filters: [{ name: 'SQLite Database', extensions: ['db'] }],
    });
    if (dest) {
      await copyFile(dbPath, dest);
      backupMessage.value = `✓ Backup gespeichert: ${dest}`;
      setTimeout(() => backupMessage.value = '', 5000);
    }
  } catch (e) {
    console.error('Backup error:', e);
    backupMessage.value = t('settings.backupError') + String(e);
  }
}

async function importDb() {
  try {
    const selected = await open({
      multiple: false,
      title: t('settings.importBackup'),
      filters: [{ name: 'SQLite Database', extensions: ['db'] }],
    });
    if (selected && typeof selected === 'string') {
      const dataDir = await appConfigDir();
      const dbPath = await join(dataDir, 'vibebill.db');
      await copyFile(selected, dbPath);
      backupMessage.value = t('settings.importSuccess');
    }
  } catch (e) {
    console.error('Import error:', e);
    backupMessage.value = String(e);
  }
}
</script>
