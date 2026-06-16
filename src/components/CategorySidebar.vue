<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useCategoryStore } from '../stores/categoryStore'
import { usePhraseStore } from '../stores/phraseStore'
import { useMessage } from 'naive-ui'
import { Icon } from '@iconify/vue'

const { t } = useI18n()

defineProps<{ currentView: 'home' | 'settings' }>()
const emit = defineEmits<{ navigate: [view: 'home' | 'settings'] }>()

const categoryStore = useCategoryStore()
const phraseStore = usePhraseStore()
const message = useMessage()

const showAddDialog = ref(false)
const newCategoryName = ref('')
const newCategoryIcon = ref('')
const editDialogVisible = ref(false)
const editingCategory = ref<{ id: string; name: string; icon: string | null } | null>(null)

const favoriteCount = computed(() => phraseStore.phrases.filter(p => p.is_favorite).length)

function selectCategory(id: string | null) {
  categoryStore.selectCategory(id)
  phraseStore.setCategory(id)
}

function selectFavorites() {
  categoryStore.selectCategory(null)
  phraseStore.setFavoritesFilter(true)
}

async function handleAdd() {
  if (!newCategoryName.value.trim()) return
  try {
    await categoryStore.createCategory(newCategoryName.value.trim(), newCategoryIcon.value.trim() || undefined)
    newCategoryName.value = ''; newCategoryIcon.value = ''
    showAddDialog.value = false
    message.success(t('category.created'))
  } catch (e) { message.error(t('category.createError') + ': ' + e) }
}

async function handleEdit() {
  if (!editingCategory.value || !newCategoryName.value.trim()) return
  try {
    await categoryStore.updateCategory(editingCategory.value.id, newCategoryName.value.trim())
    newCategoryName.value = ''; newCategoryIcon.value = ''
    editingCategory.value = null; editDialogVisible.value = false
    message.success(t('category.updated'))
  } catch (e) { message.error(t('category.updateError') + ': ' + e) }
}

async function handleDelete(id: string) {
  try { await categoryStore.deleteCategory(id); message.success(t('category.deleted')) }
  catch (e) { message.error(t('category.deleteError') + ': ' + e) }
}

function openEditDialog(id: string, name: string, icon: string | null) {
  editingCategory.value = { id, name, icon }
  newCategoryName.value = name; newCategoryIcon.value = icon || ''
  editDialogVisible.value = true
}

function categoryIcon(iconName: string | null): string {
  return iconName || 'ri:folder-line'
}
</script>

<template>
  <aside class="sidebar">
    <!-- Pinned: 全部 + 收藏 (fixed, no scroll) -->
    <div class="sidebar-pinned">
      <div class="nav-item" :class="{ active: categoryStore.selectedId === null && !phraseStore.favoritesOnly && currentView === 'home' }"
        @click="selectCategory(null); emit('navigate', 'home')">
        <Icon icon="ri:list-check-3" width="17" class="nav-icon" />
        <span class="nav-label">{{ t('sidebar.all') }}</span>
        <span class="nav-count">{{ phraseStore.phrases.length }}</span>
      </div>

      <div class="nav-item nav-fav" :class="{ active: phraseStore.favoritesOnly && currentView === 'home' }"
        @click="selectFavorites(); emit('navigate', 'home')">
        <Icon :icon="phraseStore.favoritesOnly ? 'ri:star-fill' : 'ri:star-line'" width="17" class="nav-icon"
          :style="{ color: phraseStore.favoritesOnly ? '#f0a020' : 'inherit' }" />
        <span class="nav-label">{{ t('sidebar.favorites') }}</span>
        <span class="nav-count">{{ favoriteCount }}</span>
      </div>
    </div>

    <div class="nav-divider" />

    <!-- Scrollable: categories only -->
    <nav class="sidebar-scroll">
      <div v-for="cat in categoryStore.categories" :key="cat.id" class="nav-item nav-category"
        :class="{ active: categoryStore.selectedId === cat.id && currentView === 'home' }"
        @click="selectCategory(cat.id); emit('navigate', 'home')">
        <Icon :icon="categoryIcon(cat.icon)" width="17" class="nav-icon cat-icon" />
        <span class="nav-label">{{ cat.name }}</span>
        <div class="nav-actions">
          <button class="action-dot" @click.stop="openEditDialog(cat.id, cat.name, cat.icon)" :title="t('sidebar.editCategory')">
            <Icon icon="ri:edit-line" width="12" />
          </button>
          <button class="action-dot" @click.stop="handleDelete(cat.id)" :title="t('phrase.delete')">
            <Icon icon="ri:delete-bin-line" width="12" />
          </button>
        </div>
      </div>
    </nav>

    <!-- Fixed footer -->
    <div class="sidebar-footer">
      <button class="footer-btn" @click="showAddDialog = true">
        <Icon icon="ri:add-line" width="16" />
        <span>{{ t('sidebar.newCategory') }}</span>
      </button>
      <button class="footer-btn settings-btn" :class="{ active: currentView === 'settings' }"
        @click="emit('navigate', currentView === 'settings' ? 'home' : 'settings')">
        <Icon icon="ri:settings-4-line" width="16" />
        <span>{{ currentView === 'settings' ? t('sidebar.backHome') : t('sidebar.settings') }}</span>
      </button>
    </div>

    <!-- Add Dialog -->
    <n-modal v-model:show="showAddDialog" preset="dialog" :title="t('sidebar.newCategory')"
      :positive-text="t('phrase.create_btn')" :negative-text="t('phrase.cancel')" @positive-click="handleAdd">
      <div class="dialog-form">
        <n-input v-model:value="newCategoryName" :placeholder="t('category.namePlaceholder')" maxlength="20" show-count />
        <n-input v-model:value="newCategoryIcon" :placeholder="t('category.iconPlaceholder')" style="margin-top:10px" />
        <div v-if="newCategoryIcon" class="icon-preview">
          <Icon :icon="newCategoryIcon || 'ri:folder-line'" width="28" />
          <span class="preview-hint">预览</span>
        </div>
      </div>
    </n-modal>

    <!-- Edit Dialog -->
    <n-modal v-model:show="editDialogVisible" preset="dialog" :title="t('sidebar.editCategory')"
      :positive-text="t('phrase.save')" :negative-text="t('phrase.cancel')" @positive-click="handleEdit">
      <div class="dialog-form">
        <n-input v-model:value="newCategoryName" :placeholder="t('category.namePlaceholder')" maxlength="20" show-count />
        <n-input v-model:value="newCategoryIcon" :placeholder="t('category.iconPlaceholder')" style="margin-top:10px" />
        <div v-if="newCategoryIcon" class="icon-preview">
          <Icon :icon="newCategoryIcon || 'ri:folder-line'" width="28" />
          <span class="preview-hint">预览</span>
        </div>
      </div>
    </n-modal>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 220px; min-width: 220px; height: 100%;
  background: var(--surface); border-right: 1px solid var(--border);
  display: flex; flex-direction: column; user-select: none;
}

/* Pinned section (全部 + 收藏) — fixed, no scroll */
.sidebar-pinned {
  padding: 14px 8px 6px;
  flex-shrink: 0;
}

/* Scrollable categories area */
.sidebar-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 4px 8px;
}

/* Nav items */
.nav-item {
  display: flex; align-items: center; gap: 10px;
  padding: 8px 10px; margin-bottom: 1px; border-radius: 6px;
  cursor: pointer; color: var(--text-secondary);
  font-size: 13.5px; font-weight: 450;
  position: relative; transition: all var(--transition-fast);
}
.nav-item:hover { background: var(--hover-bg); color: var(--text-primary); }
.nav-item.active { background: var(--active-bg); color: var(--text-primary); font-weight: 550; }
.nav-item.active::before {
  content: ''; position: absolute; left: 0; top: 6px; bottom: 6px;
  width: 3px; background: var(--brand); border-radius: 0 3px 3px 0;
}
.nav-icon { flex-shrink: 0; color: inherit; }
.cat-icon { color: var(--text-tertiary); }
.nav-item.active .cat-icon { color: var(--brand); }
.nav-label { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.nav-count { font-size: 11px; color: var(--text-tertiary); font-weight: 450; min-width: 18px; text-align: right; }
.nav-actions { display: none; gap: 2px; }
.nav-item:hover .nav-actions { display: flex; }
.action-dot { background: none; border: none; cursor: pointer; color: var(--text-tertiary); padding: 2px; border-radius: 3px; display: flex; }
.action-dot:hover { color: var(--text-primary); background: var(--border); }

.nav-divider { height: 1px; background: var(--border); margin: 4px 14px; flex-shrink: 0; }

/* Footer */
.sidebar-footer {
  padding: 8px; border-top: 1px solid var(--border);
  display: flex; flex-direction: column; gap: 2px; flex-shrink: 0;
}
.footer-btn {
  display: flex; align-items: center; gap: 8px; padding: 8px 10px; border-radius: 6px;
  background: none; border: none; cursor: pointer; color: var(--text-secondary);
  font-size: 13px; font-family: var(--font-stack); width: 100%; transition: all var(--transition-fast);
}
.footer-btn:hover { background: var(--hover-bg); color: var(--text-primary); }
.footer-btn.active { color: var(--brand); }
.dialog-form { display: flex; flex-direction: column; }
.icon-preview { display: flex; align-items: center; gap: 8px; margin-top: 10px; color: var(--text-secondary); }
.preview-hint { font-size: 12px; color: var(--text-tertiary); }
</style>
