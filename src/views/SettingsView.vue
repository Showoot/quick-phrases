<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useMessage } from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { openUrl } from '@tauri-apps/plugin-opener'
import { useSettings, type AppSettings } from '../composables/useSettings'
import { useTheme } from '../composables/useTheme'
import { usePhraseStore } from '../stores/phraseStore'
import { setLanguage } from '../i18n'
import { Icon } from '@iconify/vue'

const { t, locale } = useI18n()
const message = useMessage()
const { settings, fetchSettings, saveSettings, getStoragePath, setStoragePath, openStorageDir } = useSettings()
const { isDark, toggleDark } = useTheme()
const phraseStore = usePhraseStore()

const storagePath = ref('')
const pathLoading = ref(false)
const currentLang = ref(locale.value)

onMounted(async () => {
  await fetchSettings()
  storagePath.value = await getStoragePath()
  currentLang.value = locale.value
})

const langOptions = [
  { value: 'zh-CN', label: '简体中文' },
  { value: 'zh-TW', label: '繁體中文' },
  { value: 'en', label: 'English' },
]

function handleLanguageChange(v: string) {
  currentLang.value = v
  setLanguage(v)
  // Also update naive-ui locale via parent
  window.location.reload()
}

async function handleAutoStartChange(val: boolean) {
  try { settings.value.auto_start = val; await saveSettings(settings.value); message.success(val ? t('settings.autoStartOn') : t('settings.autoStartOff')) }
  catch (e) { message.error(t('settings.saveError') + ': ' + e); settings.value.auto_start = !val }
}

async function handleCloseBehaviorChange(val: string) {
  try { settings.value.close_behavior = val as AppSettings['close_behavior']; await saveSettings(settings.value) }
  catch (e) { message.error(t('settings.saveError') + ': ' + e) }
}

async function handleChangeStoragePath() {
  try {
    const sel = await open({ directory: true, multiple: false, title: '选择存储目录' })
    if (sel) { pathLoading.value = true; await setStoragePath(typeof sel === 'string' ? sel : sel); storagePath.value = await getStoragePath(); message.success(t('settings.pathChanged')) }
  } catch (e) { message.error(t('settings.saveError') + ': ' + e) } finally { pathLoading.value = false }
}

async function handleOpenStorageDir() { try { await openStorageDir() } catch (e) { message.error((e as Error).message) } }

async function handleExport() {
  try {
    const [categories, phrases] = await Promise.all([invoke('list_categories'), invoke('list_phrases')])
    const json = JSON.stringify({ categories, phrases }, null, 2)
    const a = document.createElement('a'); a.href = URL.createObjectURL(new Blob([json], { type: 'application/json' }))
    a.download = `quick-phrases-${new Date().toISOString().slice(0, 10)}.json`; a.click()
    message.success(t('settings.exportSuccess'))
  } catch (e) { message.error(t('settings.exportError') + ': ' + e) }
}

async function handleImport() {
  try {
    const sel = await open({ multiple: false, filters: [{ name: 'JSON', extensions: ['json'] }] })
    if (!sel) return
    const fp = typeof sel === 'string' ? sel : sel
    const text = await invoke<string>('read_text_file', { path: fp })
    const data = JSON.parse(text)
    if (data.phrases && Array.isArray(data.phrases)) {
      const items = (data.phrases as Record<string, unknown>[]).map(p => ({ title: String(p.title || ''), text_content: String(p.text_content || ''), category_name: p.category_name ? String(p.category_name) : null, tags: Array.isArray(p.tags) ? p.tags.map(String) : [] }))
      const result = await invoke<unknown[]>('import_phrases', { items })
      await phraseStore.fetchPhrases()
      message.success(t('settings.importSuccess', { count: result.length }))
    } else { message.error(t('settings.noPhraseData')) }
  } catch (e) { message.error(t('settings.importError') + ': ' + e) }
}
</script>

<template>
  <div class="settings-view">
    <h2 class="page-title">{{ t('settings.title') }}</h2>

    <!-- General -->
    <section class="section">
      <h3 class="section-title">{{ t('settings.general') }}</h3>
      <div class="section-body">
        <div class="row">
          <div class="row-info"><span class="row-label">{{ t('settings.autoStart') }}</span><span class="row-desc">{{ t('settings.autoStartDesc') }}</span></div>
          <n-switch :value="settings.auto_start" @update:value="handleAutoStartChange" />
        </div>
        <div class="row">
          <div class="row-info"><span class="row-label">{{ t('settings.closeBehavior') }}</span><span class="row-desc">{{ t('settings.closeBehaviorDesc') }}</span></div>
          <div class="segmented">
            <button :class="{ active: settings.close_behavior === 'minimize_to_tray' }" @click="handleCloseBehaviorChange('minimize_to_tray')">{{ t('settings.minimizeToTray') }}</button>
            <button :class="{ active: settings.close_behavior === 'quit' }" @click="handleCloseBehaviorChange('quit')">{{ t('settings.quitApp') }}</button>
          </div>
        </div>
      </div>
    </section>

    <!-- Data -->
    <section class="section">
      <h3 class="section-title">{{ t('settings.data') }}</h3>
      <div class="section-body">
        <div class="row">
          <div class="row-info"><span class="row-label">{{ t('settings.storagePath') }}</span><span class="row-desc path">{{ storagePath }}</span></div>
          <div class="btn-row"><button class="link-btn" @click="handleOpenStorageDir">{{ t('settings.openDir') }}</button><button class="link-btn" @click="handleChangeStoragePath" :disabled="pathLoading">{{ pathLoading ? t('settings.migrating') : t('settings.change') }}</button></div>
        </div>
        <div class="row">
          <div class="row-info"><span class="row-label">{{ t('settings.dataManage') }}</span><span class="row-desc">{{ t('settings.dataManageDesc') }}</span></div>
          <div class="btn-row"><button class="pri-btn" @click="handleExport"><Icon icon="ri:download-line" width="14" />{{ t('settings.export') }}</button><button class="link-btn" @click="handleImport"><Icon icon="ri:upload-line" width="14" />{{ t('settings.import') }}</button></div>
        </div>
      </div>
    </section>

    <!-- Appearance -->
    <section class="section">
      <h3 class="section-title">{{ t('settings.appearance') }}</h3>
      <div class="section-body">
        <div class="row">
          <div class="row-info"><span class="row-label">{{ t('settings.darkMode') }}</span><span class="row-desc">{{ t('settings.darkModeDesc') }}</span></div>
          <n-switch :value="isDark" @update:value="toggleDark" />
        </div>
        <div class="row">
          <div class="row-info"><span class="row-label">{{ t('settings.language') }}</span><span class="row-desc">{{ t('settings.languageDesc') }}</span></div>
          <select class="lang-select" :value="currentLang" @change="handleLanguageChange(($event.target as HTMLSelectElement).value)">
            <option v-for="l in langOptions" :key="l.value" :value="l.value">{{ l.label }}</option>
          </select>
        </div>
      </div>
    </section>

    <!-- About -->
    <section class="section">
      <h3 class="section-title">{{ t('settings.about') }}</h3>
      <div class="section-body">
        <p class="about-text">快捷话术 v0.2.0</p>
        <p class="about-text">{{ t('settings.aboutText') }}</p>
        <p class="about-text">
          {{ t('settings.openSource') }}:
          <a class="link" @click.prevent="openUrl('https://github.com/Showoot/quick-phrases')">github.com/Showoot/quick-phrases</a>
        </p>
      </div>
    </section>
  </div>
</template>

<style scoped>
.settings-view {
  padding: 32px 48px;
  max-width: 720px;
  margin: 0 auto;
  overflow-y: auto;
  height: 100%;
  width: 100%;
}

.page-title { font-size: 24px; font-weight: 700; color: var(--text-primary); margin-bottom: 32px; letter-spacing: -0.3px; }

.section { margin-bottom: 32px; }
.section-title { font-size: 13px; font-weight: 600; color: var(--text-primary); margin-bottom: 12px; padding-bottom: 0; }
.section-body { display: flex; flex-direction: column; }

.row { display: flex; align-items: center; justify-content: space-between; min-height: 44px; padding: 8px 0; border-bottom: 1px solid var(--border); }
.row:last-child { border-bottom: none; }

.row-info { display: flex; flex-direction: column; gap: 2px; flex: 1; min-width: 0; padding-right: 24px; }
.row-label { font-size: 14px; color: var(--text-primary); font-weight: 500; }
.row-desc { font-size: 12px; color: var(--text-tertiary); }
.row-desc.path { word-break: break-all; }

/* Segmented control */
.segmented { display: flex; background: var(--hover-bg); border-radius: 8px; padding: 3px; flex-shrink: 0; }
.segmented button {
  padding: 6px 14px; border: none; border-radius: 6px; background: transparent;
  font-size: 13px; color: var(--text-secondary); cursor: pointer; font-family: var(--font-stack); transition: all var(--transition-fast);
}
.segmented button.active { background: var(--surface); color: var(--text-primary); font-weight: 500; box-shadow: 0 1px 3px rgba(0,0,0,0.06); }

/* Buttons */
.btn-row { display: flex; gap: 8px; flex-shrink: 0; }
.link-btn { padding: 6px 14px; border: 1px solid var(--border); border-radius: 6px; background: var(--surface); color: var(--text-secondary); font-size: 13px; font-family: var(--font-stack); cursor: pointer; transition: all var(--transition-fast); display: flex; align-items: center; gap: 5px; }
.link-btn:hover { background: var(--hover-bg); color: var(--text-primary); }
.link-btn:disabled { opacity: 0.5; }
.pri-btn { padding: 6px 14px; border: 1px solid var(--brand); border-radius: 6px; background: var(--brand); color: #fff; font-size: 13px; font-family: var(--font-stack); cursor: pointer; transition: all var(--transition-fast); display: flex; align-items: center; gap: 5px; }
.pri-btn:hover { background: var(--brand-hover); }

/* Language select */
.lang-select { padding: 8px 12px; border: 1px solid var(--border); border-radius: 8px; background: var(--surface); color: var(--text-primary); font-size: 13px; font-family: var(--font-stack); cursor: pointer; outline: none; }
.lang-select:focus { border-color: var(--brand); }

.about-text { font-size: 13px; color: var(--text-secondary); margin: 3px 0; }
.link { color: var(--brand); cursor: pointer; text-decoration: none; }
.link:hover { text-decoration: underline; }
</style>
