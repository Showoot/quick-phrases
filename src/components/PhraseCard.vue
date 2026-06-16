<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useMessage } from 'naive-ui'
import type { Phrase } from '../types'
import { usePhraseStore } from '../stores/phraseStore'
import { useClipboard } from '../composables/useClipboard'
import { formatDate } from '../utils'
import { Icon } from '@iconify/vue'

const { t } = useI18n()

const props = defineProps<{ phrase: Phrase }>()
const emit = defineEmits<{ edit: [phrase: Phrase] }>()

const phraseStore = usePhraseStore()
const { copyText } = useClipboard()
const message = useMessage()

const copying = ref(false)
const copied = ref(false)
const menuOpen = ref(false)
const showDeleteConfirm = ref(false)

function toggleMenu() { menuOpen.value = !menuOpen.value }
function closeMenu() { menuOpen.value = false }

async function handleCopy() {
  if (copying.value) return
  copying.value = true
  try {
    await copyText(props.phrase.text_content)
    await phraseStore.incrementUsage(props.phrase.id)
    copied.value = true
    message.success(t('phrase.copySuccess'))
    setTimeout(() => (copied.value = false), 2000)
  } catch (e) {
    message.error(t('phrase.copyError') + ': ' + e)
  } finally {
    copying.value = false
  }
}

async function handleToggleFavorite() {
  await phraseStore.toggleFavorite(props.phrase.id)
}

function handleEdit() {
  closeMenu()
  emit('edit', props.phrase)
}

function handleDeleteClick() {
  showDeleteConfirm.value = true
}

async function confirmDelete() {
  await phraseStore.deletePhrase(props.phrase.id)
  showDeleteConfirm.value = false
  message.success(t('phrase.deleted'))
}

function handleClickOutside() {
  menuOpen.value = false
}
</script>

<template>
  <div class="card">
    <!-- Top row: title + three-dot menu -->
    <div class="card-top">
      <h3 class="card-title" @click="handleCopy">{{ phrase.title }}</h3>
      <div class="top-actions" v-click-outside="handleClickOutside">
        <button class="dot-btn" @click.stop="toggleMenu" title="更多操作">
          <Icon icon="ri:more-fill" width="16" />
        </button>
        <div v-if="menuOpen" class="dropdown">
          <button class="dropdown-item" @click.stop="handleEdit">
            <Icon icon="ri:edit-line" width="14" /> {{ t('phrase.edit') }}
          </button>
          <button class="dropdown-item danger" @click.stop="handleDeleteClick">
            <Icon icon="ri:delete-bin-line" width="14" /> {{ t('phrase.delete') }}
          </button>
        </div>
      </div>
    </div>

    <!-- Body -->
    <div class="card-main" @click="handleCopy">
      <p class="card-text">{{ phrase.text_content }}</p>
      <div v-if="phrase.tags.length > 0" class="card-tags">
        <span v-for="tag in phrase.tags" :key="tag" class="tag-chip">#{{ tag }}</span>
      </div>
    </div>

    <!-- Footer: time left, buttons right -->
    <div class="card-footer">
      <div class="footer-left">
        <span class="time">{{ formatDate(phrase.updated_at) }}</span>
      </div>
      <div class="footer-right">
        <button
          class="icon-btn"
          :class="{ active: phrase.is_favorite }"
          @click.stop="handleToggleFavorite"
          :title="t('sidebar.favorites')"
        >
          <Icon :icon="phrase.is_favorite ? 'ri:star-fill' : 'ri:star-line'" width="15"
            :style="{ color: phrase.is_favorite ? '#f0a020' : 'inherit' }" />
        </button>
        <button class="copy-btn" :class="{ success: copied }" @click.stop="handleCopy" :disabled="copying">
          <Icon :icon="copied ? 'ri:check-line' : 'ri:file-copy-line'" width="14" />
          <span>{{ copied ? t('phrase.copied') : t('phrase.copy') }}</span>
        </button>
      </div>
    </div>

    <!-- Delete confirmation popover -->
    <div v-if="showDeleteConfirm" class="confirm-overlay" @click.self="showDeleteConfirm = false">
      <div class="confirm-box">
        <p class="confirm-text">{{ t('phrase.confirmDelete') }}</p>
        <div class="confirm-actions">
          <button class="confirm-btn cancel" @click="showDeleteConfirm = false">{{ t('phrase.cancel') }}</button>
          <button class="confirm-btn ok" @click="confirmDelete">{{ t('phrase.delete') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<!-- v-click-outside directive -->
<script lang="ts">
import { defineComponent, type Directive } from 'vue'

const vClickOutside: Directive = {
  mounted(el, binding) {
    el.__clickOutsideHandler = (e: MouseEvent) => {
      if (!(el === e.target || el.contains(e.target as Node))) {
        binding.value(e)
      }
    }
    document.addEventListener('click', el.__clickOutsideHandler)
  },
  unmounted(el) {
    document.removeEventListener('click', el.__clickOutsideHandler)
  },
}

export default defineComponent({
  directives: { clickOutside: vClickOutside },
})
</script>

<style scoped>
.card {
  background: #fff;
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 16px 20px 12px;
  transition: transform var(--transition-fast), border-color var(--transition-fast);
  display: flex;
  flex-direction: column;
  cursor: default;
  position: relative;
}
.dark .card { background: #212121; }
.card:hover { transform: translateY(-2px); border-color: var(--border-hover); }

/* Top row */
.card-top { display: flex; align-items: flex-start; justify-content: space-between; margin-bottom: 8px; }
.card-title {
  font-size: 15px; font-weight: 600; color: var(--text-primary);
  line-height: 1.4; letter-spacing: -0.1px; cursor: pointer; flex: 1; padding-right: 8px;
}

/* Three-dot menu */
.top-actions { position: relative; flex-shrink: 0; }
.dot-btn {
  width: 28px; height: 28px; border: none; border-radius: 6px;
  background: transparent; color: var(--text-tertiary); cursor: pointer;
  display: flex; align-items: center; justify-content: center; transition: all var(--transition-fast);
}
.dot-btn:hover { background: var(--hover-bg); color: var(--text-primary); }

.dropdown {
  position: absolute; right: 0; top: 32px; z-index: 20;
  background: var(--surface); border: 1px solid var(--border); border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0,0,0,0.1); min-width: 120px; overflow: hidden;
}
.dropdown-item {
  display: flex; align-items: center; gap: 8px; width: 100%; padding: 9px 14px;
  border: none; background: transparent; color: var(--text-secondary);
  font-size: 13px; font-family: var(--font-stack); cursor: pointer; transition: background var(--transition-fast);
}
.dropdown-item:hover { background: var(--hover-bg); color: var(--text-primary); }
.dropdown-item.danger:hover { color: #e53e3e; }

/* Body */
.card-main { flex: 1; cursor: pointer; margin-bottom: 4px; }
.card-text {
  font-size: 13.5px; color: var(--text-secondary); line-height: 1.65;
  display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden;
}

/* Tags in card body */
.card-tags { display: flex; gap: 6px; flex-wrap: wrap; margin-top: 10px; }
.tag-chip {
  font-size: 12px; color: var(--text-tertiary); font-weight: 450;
  background: var(--hover-bg); padding: 2px 8px; border-radius: 4px; white-space: nowrap;
}

/* Footer */
.card-footer { display: flex; align-items: center; justify-content: space-between; margin-top: 10px; padding-top: 8px; border-top: 1px solid var(--border); }
.footer-left { display: flex; align-items: center; flex: 1; min-width: 0; }
.time { font-size: 11.5px; color: var(--text-tertiary); white-space: nowrap; }

.footer-right { display: flex; align-items: center; gap: 6px; flex-shrink: 0; margin-left: auto; }

.icon-btn {
  display: flex; align-items: center; justify-content: center;
  width: 30px; height: 30px; border: 1px solid var(--border); border-radius: 6px;
  background: var(--surface); color: var(--text-secondary); cursor: pointer; transition: all var(--transition-fast);
}
.icon-btn:hover { background: var(--hover-bg); color: var(--text-primary); }
.icon-btn.active { border-color: #ffe8a0; background: #fffdf5; }
.dark .icon-btn.active { border-color: #5c4a1a; background: #2d2a1a; }

.copy-btn {
  display: flex; align-items: center; gap: 5px; padding: 5px 10px;
  border: 1px solid var(--border); border-radius: 6px;
  background: var(--surface); color: var(--text-secondary);
  font-size: 12.5px; font-family: var(--font-stack); cursor: pointer; transition: all var(--transition-fast); white-space: nowrap;
}
.copy-btn:hover { background: var(--hover-bg); color: var(--text-primary); }
.copy-btn.success { color: #10b981; border-color: #a7f3d0; background: #f0fdf4; }
.dark .copy-btn.success { background: #0a2e1d; border-color: #1a5c38; }

/* Delete confirmation overlay */
.confirm-overlay {
  position: absolute; inset: 0; z-index: 30;
  background: rgba(255,255,255,0.85); border-radius: var(--radius-lg);
  display: flex; align-items: center; justify-content: center;
}
.dark .confirm-overlay { background: rgba(33,33,33,0.9); }
.confirm-box {
  background: var(--surface); border: 1px solid var(--border); border-radius: 10px;
  padding: 20px 24px; box-shadow: 0 8px 24px rgba(0,0,0,0.12); text-align: center; max-width: 240px;
}
.confirm-text { font-size: 14px; color: var(--text-primary); margin-bottom: 16px; font-weight: 500; }
.confirm-actions { display: flex; gap: 8px; justify-content: center; }
.confirm-btn {
  padding: 7px 18px; border-radius: 6px; font-size: 13px; font-family: var(--font-stack); cursor: pointer; border: 1px solid var(--border); transition: all var(--transition-fast);
}
.confirm-btn.cancel { background: var(--surface); color: var(--text-secondary); }
.confirm-btn.cancel:hover { background: var(--hover-bg); }
.confirm-btn.ok { background: #e53e3e; color: #fff; border-color: #e53e3e; }
.confirm-btn.ok:hover { background: #c53030; }
</style>
