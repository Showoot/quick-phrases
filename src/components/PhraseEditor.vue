<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useMessage } from 'naive-ui'
import { usePhraseStore } from '../stores/phraseStore'
import { useCategoryStore } from '../stores/categoryStore'
import { useClipboard } from '../composables/useClipboard'
import { Icon } from '@iconify/vue'
import type { Phrase } from '../types'

const { t } = useI18n()

const props = defineProps<{ visible: boolean; phrase?: Phrase | null }>()
const emit = defineEmits<{ close: [] }>()

const phraseStore = usePhraseStore()
const categoryStore = useCategoryStore()
const { importImage, deleteImage } = useClipboard()
const message = useMessage()

const isEdit = computed(() => !!props.phrase)
const saving = ref(false)

const title = ref('')
const textContent = ref('')
const categoryId = ref<string | null>(null)
const tagsStr = ref('')
const imagePaths = ref<string[]>([])

watch(() => props.visible, (v) => {
  if (v) {
    if (props.phrase) {
      title.value = props.phrase.title
      textContent.value = props.phrase.text_content
      categoryId.value = props.phrase.category_id
      tagsStr.value = props.phrase.tags.join(', ')
      imagePaths.value = [...props.phrase.image_paths]
    } else {
      title.value = ''; textContent.value = ''; categoryId.value = categoryStore.selectedId
      tagsStr.value = ''; imagePaths.value = []
    }
  }
})

async function handleSave() {
  if (!title.value.trim()) { message.warning('请输入标题'); return }
  if (!categoryId.value) { message.warning('请选择分类'); return }
  saving.value = true
  try {
    const tags = tagsStr.value.split(/[,，]/).map(t => t.trim()).filter(Boolean)
    if (isEdit.value && props.phrase) {
      await phraseStore.updatePhrase(props.phrase.id, { title: title.value.trim(), text_content: textContent.value, category_id: categoryId.value, tags, image_paths: imagePaths.value })
      message.success(t('phrase.save'))
    } else {
      await phraseStore.createPhrase({ title: title.value.trim(), text_content: textContent.value, category_id: categoryId.value, tags, image_paths: imagePaths.value })
      message.success(t('phrase.create_btn'))
    }
    emit('close')
  } catch (e) { message.error(t('settings.saveError') + ': ' + e) } finally { saving.value = false }
}

async function handleUploadImage() {
  try {
    const { open } = await import('@tauri-apps/plugin-dialog')
    const sel = await open({ multiple: false, filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'webp'] }] })
    if (sel) { const fn = await importImage(sel); imagePaths.value.push(fn) }
  } catch (e) { message.error((e as Error).message) }
}

async function handleRemoveImage(idx: number) {
  const fn = imagePaths.value[idx]; imagePaths.value.splice(idx, 1)
  if (isEdit.value) { try { await deleteImage(fn) } catch { /* ignore */ } }
}
</script>

<template>
  <n-modal :show="visible" preset="card" :title="isEdit ? t('phrase.editPhrase') : t('phrase.newPhrase')"
    style="width: 640px; max-width: 90vw" :mask-closable="false" @close="emit('close')">
    <n-form label-placement="top">
      <n-form-item :label="t('phrase.title')" required>
        <n-input v-model:value="title" placeholder="输入标题" maxlength="50" show-count />
      </n-form-item>
      <n-form-item :label="t('phrase.category')" required>
        <n-select v-model:value="categoryId" :options="categoryStore.categories.map(c => ({ label: c.name, value: c.id }))" placeholder="选择分类" />
      </n-form-item>
      <n-form-item :label="t('phrase.content')">
        <n-input v-model:value="textContent" type="textarea" placeholder="输入话术文本..." :autosize="{ minRows: 4, maxRows: 10 }" show-count />
      </n-form-item>
      <n-form-item :label="t('phrase.image')">
        <div class="img-area">
          <n-tag v-for="(img, idx) in imagePaths" :key="img" closable @close="handleRemoveImage(idx)">{{ img }}</n-tag>
          <n-button size="small" dashed @click="handleUploadImage"><template #icon><Icon icon="ri:image-add-line" width="16" /></template>{{ t('phrase.addImage') }}</n-button>
        </div>
      </n-form-item>
      <n-form-item :label="t('phrase.tags')">
        <n-input v-model:value="tagsStr" :placeholder="t('phrase.tagsHint')" />
      </n-form-item>
    </n-form>
    <template #footer>
      <div class="footer"><n-button @click="emit('close')">{{ t('phrase.cancel') }}</n-button><n-button type="primary" :loading="saving" @click="handleSave">{{ isEdit ? t('phrase.save') : t('phrase.create_btn') }}</n-button></div>
    </template>
  </n-modal>
</template>

<style scoped>
.img-area { display: flex; flex-wrap: wrap; gap: 8px; align-items: center; }
.footer { display: flex; justify-content: flex-end; gap: 12px; }
</style>
