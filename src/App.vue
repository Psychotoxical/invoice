<template>
  <div class="app-layout" :data-theme="theme">
    <aside class="sidebar">
      <div class="sidebar-header">
        <div>
          <h1>📄 {{ $t('app.title') }}</h1>
          <div class="subtitle">{{ $t('app.subtitle') }}</div>
        </div>
        <div class="sidebar-header-actions">
          <button class="sidebar-icon-btn" @click="toggleTheme" :title="theme === 'dark' ? $t('app.lightTheme') : $t('app.darkTheme')">
            {{ theme === 'dark' ? '☀️' : '🌙' }}
          </button>
          <router-link to="/settings" class="sidebar-icon-btn" :title="$t('app.settings')">
            ⚙️
          </router-link>
        </div>
      </div>
      <nav class="sidebar-nav">
        <router-link to="/" class="nav-item" :class="{ active: $route.path === '/' }">
          <span class="nav-icon">📊</span> {{ $t('nav.dashboard') }}
        </router-link>
        <router-link to="/invoices" class="nav-item" :class="{ active: $route.path.startsWith('/invoices') }">
          <span class="nav-icon">📋</span> {{ $t('nav.invoices') }}
        </router-link>
        <router-link to="/customers" class="nav-item" :class="{ active: $route.path === '/customers' }">
          <span class="nav-icon">👥</span> {{ $t('nav.customers') }}
        </router-link>
        <router-link to="/sellers" class="nav-item" :class="{ active: $route.path === '/sellers' }">
          <span class="nav-icon">🏢</span> {{ $t('nav.sellers') }}
        </router-link>
        <router-link to="/products" class="nav-item" :class="{ active: $route.path === '/products' }">
          <span class="nav-icon">📦</span> {{ $t('nav.products') }}
        </router-link>
      </nav>
    </aside>
    <main class="main-content">
      <router-view />
    </main>
    <ToastContainer />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { getSetting, setSetting } from './services/database';
import ToastContainer from './components/ToastContainer.vue';

const { locale } = useI18n();
const theme = ref('light');

onMounted(async () => {
  try {
    const saved = await getSetting('theme');
    if (saved) theme.value = saved;
    const lang = await getSetting('language');
    if (lang) locale.value = lang;
  } catch {
    // DB not ready in browser dev mode
  }
});

async function toggleTheme() {
  theme.value = theme.value === 'dark' ? 'light' : 'dark';
  try {
    await setSetting('theme', theme.value);
  } catch {
    // ignore in browser dev
  }
}
</script>