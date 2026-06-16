<script setup lang="ts">
import { ref, watch, computed, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { usePhraseStore } from '../stores/phraseStore'
import { useClipboard } from '../composables/useClipboard'
import { useMessage } from 'naive-ui'
import { Icon } from '@iconify/vue'

const { t } = useI18n()

const props = defineProps<{ visible: boolean }>()
const emit = defineEmits<{ close: []; navigate: [view: 'home' | 'settings'] }>()

const phraseStore = usePhraseStore()
const { copyText } = useClipboard()
const message = useMessage()
const query = ref('')
const selectedIndex = ref(0)
const inputRef = ref<HTMLInputElement | null>(null)

const filteredPhrases = computed(() => {
  if (!query.value.trim()) return [...phraseStore.phrases].sort((a, b) => b.usage_count - a.usage_count).slice(0, 5)
  const q = query.value.toLowerCase()
  return phraseStore.phrases.filter(p => p.title.toLowerCase().includes(q) || p.text_content.toLowerCase().includes(q) || p.tags.some(t => t.toLowerCase().includes(q))).slice(0, 8)
})

const actions = computed(() => [
  { id: 'new', label: query.value ? `新建 "${query.value}"` : t('phrase.newPhrase'), icon: 'ri:add-line', action: () => { emit('close'); emit('navigate', 'home') } },
  { id: 'settings', label: t('settings.title'), icon: 'ri:settings-4-line', action: () => { emit('close'); emit('navigate', 'settings') } },
])

const totalItems = computed(() => filteredPhrases.value.length + actions.value.length)

watch(() => props.visible, async (v) => { if (v) { query.value = ''; selectedIndex.value = 0; await nextTick(); inputRef.value?.focus() } })

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') emit('close')
  else if (e.key === 'ArrowDown') { e.preventDefault(); selectedIndex.value = Math.min(selectedIndex.value + 1, totalItems.value - 1) }
  else if (e.key === 'ArrowUp') { e.preventDefault(); selectedIndex.value = Math.max(selectedIndex.value - 1, 0) }
  else if (e.key === 'Enter') { e.preventDefault(); activateSelected() }
}

function activateSelected() {
  const pc = filteredPhrases.value.length
  if (selectedIndex.value < pc) { const ph = filteredPhrases.value[selectedIndex.value]; copyText(ph.text_content); phraseStore.incrementUsage(ph.id); message.success(t('phrase.copySuccess')); emit('close') }
  else { const ai = selectedIndex.value - pc; if (ai >= 0 && ai < actions.value.length) actions.value[ai].action() }
}
function selectItem(idx: number) { selectedIndex.value = idx; activateSelected() }
</script>

<template>
  <Transition name="p">
    <div v-if="visible" class="overlay" @click.self="emit('close')">
      <div class="palette">
        <div class="p-input">
          <Icon icon="ri:search-line" width="18" class="p-search-icon" />
          <input ref="inputRef" v-model="query" class="p-input-el" :placeholder="t('search.placeholder')" @keydown="handleKeydown" />
        </div>
        <div class="p-results">
          <div v-if="filteredPhrases.length > 0" class="p-group">
            <div class="p-group-label">{{ t('app.title') }}</div>
            <div v-for="(ph, i) in filteredPhrases" :key="ph.id" class="p-item" :class="{ sel: selectedIndex === i }" @click="selectItem(i)" @mouseenter="selectedIndex = i">
              <Icon icon="ri:chat-quote-line" width="16" class="p-icon" />
              <div class="p-content"><span class="p-title">{{ ph.title }}</span><span class="p-desc">{{ ph.text_content.slice(0, 60) }}{{ ph.text_content.length > 60 ? '...' : '' }}</span></div>
              <span class="p-hint">{{ t('phrase.copy') }}</span>
            </div>
          </div>
          <div class="p-group">
            <div class="p-group-label">{{ t('settings.title') }}</div>
            <div v-for="(act, i) in actions" :key="act.id" class="p-item" :class="{ sel: selectedIndex === filteredPhrases.length + i }" @click="selectItem(filteredPhrases.length + i)" @mouseenter="selectedIndex = filteredPhrases.length + i">
              <Icon :icon="act.icon" width="16" class="p-icon" />
              <div class="p-content"><span class="p-title">{{ act.label }}</span></div>
            </div>
          </div>
          <div v-if="filteredPhrases.length === 0 && query" class="p-empty">{{ t('settings.noPhraseData') }}</div>
        </div>
        <div class="p-footer"><div class="p-hints"><span><kbd>↑↓</kbd> 导航</span><span><kbd>⏎</kbd> 选择</span><span><kbd>Esc</kbd> 关闭</span></div></div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.overlay { position: fixed; inset: 0; z-index: 200; display: flex; justify-content: center; padding-top: 15vh; background: rgba(0,0,0,.35); backdrop-filter: blur(4px); }
.palette { width: 560px; max-height: 480px; background: var(--surface); border: 1px solid var(--border); border-radius: 16px; box-shadow: 0 16px 48px rgba(0,0,0,.18); display: flex; flex-direction: column; overflow: hidden; }
.p-input { display: flex; align-items: center; padding: 14px 18px; border-bottom: 1px solid var(--border); }
.p-search-icon { color: var(--text-tertiary); margin-right: 12px; flex-shrink: 0; }
.p-input-el { flex: 1; border: none; outline: none; background: transparent; font-size: 16px; font-family: var(--font-stack); color: var(--text-primary); }
.p-input-el::placeholder { color: var(--text-tertiary); }
.p-results { flex: 1; overflow-y: auto; padding: 8px; }
.p-group { margin-bottom: 2px; }
.p-group-label { padding: 6px 12px 4px; font-size: 11px; font-weight: 600; color: var(--text-tertiary); text-transform: uppercase; letter-spacing: .5px; }
.p-item { display: flex; align-items: center; gap: 10px; padding: 10px 12px; border-radius: 8px; cursor: pointer; transition: background var(--transition-fast); }
.p-item:hover,.p-item.sel { background: var(--hover-bg); }
.p-icon { flex-shrink: 0; color: var(--text-tertiary); }
.p-content { flex: 1; display: flex; flex-direction: column; min-width: 0; }
.p-title { font-size: 14px; font-weight: 500; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.p-desc { font-size: 12px; color: var(--text-tertiary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; margin-top: 1px; }
.p-hint { font-size: 12px; color: var(--text-tertiary); }
.p-empty { padding: 32px; text-align: center; color: var(--text-tertiary); font-size: 14px; }
.p-footer { padding: 8px 16px; border-top: 1px solid var(--border); background: var(--hover-bg); }
.p-hints { display: flex; gap: 16px; }
.p-hints span { font-size: 11px; color: var(--text-tertiary); }
.p-hints kbd { font-family: var(--font-stack); background: var(--surface); border: 1px solid var(--border); border-radius: 4px; padding: 1px 5px; font-size: 10px; }
.p-enter-active { transition: all .2s cubic-bezier(.16,1,.3,1); }
.p-leave-active { transition: all .15s ease; }
.p-enter-from,.p-leave-to { opacity: 0; }
</style>
