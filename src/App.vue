<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useCategoryStore } from './stores/categoryStore'
import { usePhraseStore } from './stores/phraseStore'
import { useTheme } from './composables/useTheme'
import CategorySidebar from './components/CategorySidebar.vue'
import TitleBar from './components/TitleBar.vue'
import HomeView from './views/HomeView.vue'
import SettingsView from './views/SettingsView.vue'
import CommandPalette from './components/CommandPalette.vue'
import { darkTheme, zhCN, dateZhCN, enUS, dateEnUS, type GlobalThemeOverrides } from 'naive-ui'

const categoryStore = useCategoryStore()
const phraseStore = usePhraseStore()
const { isDark } = useTheme()
const { locale } = useI18n()

const currentView = ref<'home' | 'settings'>('home')
const showCommandPalette = ref(false)

function navigateTo(view: 'home' | 'settings') {
  currentView.value = view
}

// Naive UI locale syncs with i18n locale
const naiveLocale = computed(() => {
  if (locale.value === 'zh-TW') return zhCN
  if (locale.value === 'en') return enUS
  return zhCN
})
const naiveDateLocale = computed(() => {
  if (locale.value === 'en') return dateEnUS
  return dateZhCN
})

const themeOverrides = computed<GlobalThemeOverrides>(() => ({
  common: {
    primaryColor: isDark.value ? '#7c6cfb' : '#6d5dfc',
    primaryColorHover: isDark.value ? '#9088ff' : '#5b4de0',
    primaryColorPressed: isDark.value ? '#6358d0' : '#4a3fd4',
    borderRadius: '8px',
    fontFamily: "Segoe UI, PingFang SC, Microsoft YaHei, sans-serif",
    fontSize: '14px',
  },
}))

function handleGlobalKeydown(e: KeyboardEvent) {
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
    e.preventDefault()
    showCommandPalette.value = true
  }
}

onMounted(async () => {
  await categoryStore.fetchCategories()
  await phraseStore.fetchPhrases()
  window.addEventListener('keydown', handleGlobalKeydown)
})
</script>

<template>
  <n-config-provider
    :locale="naiveLocale"
    :date-locale="naiveDateLocale"
    :theme="isDark ? darkTheme : undefined"
    :theme-overrides="themeOverrides"
  >
    <n-message-provider>
      <div class="app-layout" :class="{ dark: isDark }" @contextmenu.prevent>
        <TitleBar />
        <div class="app-body">
          <CategorySidebar
            :current-view="currentView"
            @navigate="navigateTo"
          />
          <main class="main-wrapper">
            <div class="main-content">
              <HomeView v-if="currentView === 'home'" />
              <SettingsView v-else-if="currentView === 'settings'" />
            </div>
          </main>
        </div>
      </div>
      <CommandPalette
        :visible="showCommandPalette"
        @close="showCommandPalette = false"
        @navigate="(v: 'home' | 'settings') => { currentView = v; showCommandPalette = false }"
      />
    </n-message-provider>
  </n-config-provider>
</template>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--bg);
  color: var(--text-primary);
}
.app-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}
.main-wrapper {
  flex: 1;
  overflow-y: auto;
  display: flex;
  justify-content: center;
  background: #f7f7f8;
}
.dark .main-wrapper {
  background: #171717;
}
.main-content {
  width: 100%;
  max-width: 1200px;
  display: flex;
  flex-direction: column;
}
</style>
