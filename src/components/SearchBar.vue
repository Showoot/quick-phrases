<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { usePhraseStore } from '../stores/phraseStore'
import { debounce } from '../utils'
import { Icon } from '@iconify/vue'

const { t } = useI18n()
const phraseStore = usePhraseStore()
const inputValue = ref('')

const debouncedSearch = debounce((val: unknown) => {
  phraseStore.setSearch(val as string)
}, 200)

watch(inputValue, (val) => { debouncedSearch(val) })

const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0
const shortcutHint = isMac ? '⌘K' : 'Ctrl+K'
</script>

<template>
  <div class="search-bar">
    <div class="search-wrapper">
      <Icon icon="ri:search-line" width="18" class="search-icon" />
      <input v-model="inputValue" class="search-input" :placeholder="shortcutHint + ' ' + t('search.placeholder')" />
      <kbd class="search-kbd">{{ shortcutHint }}</kbd>
    </div>
  </div>
</template>

<style scoped>
.search-bar { padding: 16px 24px 12px; }
.search-wrapper {
  position: relative; display: flex; align-items: center; height: 48px;
  background: #fff; border: 1px solid var(--border);
  border-radius: var(--radius-pill); transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}
.dark .search-wrapper { background: #212121; }
.search-wrapper:focus-within { border-color: var(--brand); box-shadow: 0 0 0 3px var(--brand-light); }
.search-icon { position: absolute; left: 16px; color: var(--text-tertiary); pointer-events: none; }
.search-input {
  flex: 1; height: 100%; border: none; outline: none; background: transparent;
  padding: 0 80px 0 44px; font-size: 14px; font-family: var(--font-stack); color: var(--text-primary);
}
.search-input::placeholder { color: var(--text-tertiary); }
.search-kbd {
  position: absolute; right: 8px; display: flex; align-items: center;
  padding: 3px 10px; font-size: 11px; font-family: var(--font-stack); font-weight: 500;
  color: var(--text-tertiary); background: var(--hover-bg); border: 1px solid var(--border); border-radius: 6px; letter-spacing: 0.3px;
}
</style>
