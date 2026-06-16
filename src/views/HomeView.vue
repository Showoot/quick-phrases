<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { usePhraseStore } from '../stores/phraseStore'
import SearchBar from '../components/SearchBar.vue'
import PhraseCard from '../components/PhraseCard.vue'
import PhraseEditor from '../components/PhraseEditor.vue'
import CopyToast from '../components/CopyToast.vue'
import { Icon } from '@iconify/vue'
import type { Phrase } from '../types'

const { t } = useI18n()
const phraseStore = usePhraseStore()
const editorVisible = ref(false)
const editingPhrase = ref<Phrase | null>(null)
const fabHovered = ref(false)
const showCopyToast = ref(false)

function openNewPhrase() { editingPhrase.value = null; editorVisible.value = true }
function openEditPhrase(phrase: Phrase) { editingPhrase.value = phrase; editorVisible.value = true }
function closeEditor() { editorVisible.value = false; editingPhrase.value = null }
</script>

<template>
  <div class="home-view">
    <SearchBar />
    <div class="content-area">
      <div v-if="phraseStore.phrases.length === 0 && !phraseStore.loading" class="empty-state">
        <div class="empty-icon">
          <Icon icon="ri:chat-quote-line" width="48" />
        </div>
        <h3 class="empty-title">{{ t('phrase.noPhrases') }}</h3>
        <p class="empty-desc">{{ t('phrase.noPhrasesDesc') }}</p>
        <button class="empty-btn" @click="openNewPhrase">{{ t('phrase.create') }}</button>
      </div>
      <n-spin v-if="phraseStore.loading" class="loading-spinner" />
      <div v-if="phraseStore.phrases.length > 0" class="phrase-grid">
        <PhraseCard v-for="phrase in phraseStore.phrases" :key="phrase.id" :phrase="phrase" @edit="openEditPhrase" />
      </div>
    </div>

    <!-- FAB -->
    <div class="fab-container" @mouseenter="fabHovered = true" @mouseleave="fabHovered = false">
      <button class="fab-btn" :class="{ expanded: fabHovered }" @click="openNewPhrase">
        <Icon icon="ri:add-line" width="20" />
        <Transition name="fab-label">
          <span v-if="fabHovered" class="fab-label">{{ t('phrase.newPhrase') }}</span>
        </Transition>
      </button>
    </div>

    <PhraseEditor :visible="editorVisible" :phrase="editingPhrase" @close="closeEditor" />
    <CopyToast :visible="showCopyToast" />
  </div>
</template>

<style scoped>
.home-view { height: 100%; display: flex; flex-direction: column; position: relative; }
.content-area { flex: 1; overflow-y: auto; padding: 8px 24px 24px; }

.empty-state { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 80px 20px; text-align: center; }
.empty-icon { color: var(--text-tertiary); margin-bottom: 16px; }
.empty-title { font-size: 17px; font-weight: 600; color: var(--text-primary); margin-bottom: 6px; }
.empty-desc { font-size: 14px; color: var(--text-secondary); margin-bottom: 20px; }
.empty-btn { padding: 8px 20px; background: var(--brand); color: #fff; border: none; border-radius: var(--radius-sm); font-size: 14px; font-family: var(--font-stack); cursor: pointer; transition: background var(--transition-fast); }
.empty-btn:hover { background: var(--brand-hover); }
.loading-spinner { display: flex; justify-content: center; margin-top: 80px; }

.phrase-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 14px; align-content: start; }

.fab-container { position: absolute; bottom: 24px; right: 24px; z-index: 10; }
.fab-btn {
  display: flex; align-items: center; gap: 8px; height: 44px; min-width: 44px; padding: 0 12px;
  background: var(--brand); color: #fff; border: none; border-radius: var(--radius-pill);
  cursor: pointer; font-family: var(--font-stack); font-size: 14px; font-weight: 550;
  transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  box-shadow: 0 2px 10px rgba(109, 93, 252, 0.3); overflow: hidden; white-space: nowrap;
}
.fab-btn:hover { box-shadow: 0 4px 16px rgba(109, 93, 252, 0.4); padding-right: 16px; }
.fab-btn.expanded { padding-right: 18px; }
.fab-label { font-size: 13.5px; letter-spacing: 0.1px; }
.fab-label-enter-active { transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1); }
.fab-label-leave-active { transition: all 0.15s ease; }
.fab-label-enter-from { opacity: 0; transform: translateX(-6px); }
.fab-label-leave-to { opacity: 0; transform: translateX(-4px); }
</style>
