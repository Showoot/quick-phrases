<script setup lang="ts">
import { ref, watch } from 'vue'
import { useClipboard } from '../composables/useClipboard'

const props = defineProps<{
  filename: string | null
}>()

const emit = defineEmits<{
  close: []
}>()

const { getImageBase64 } = useClipboard()
const imageSrc = ref('')
const loading = ref(false)

watch(
  () => props.filename,
  async (name) => {
    if (name) {
      loading.value = true
      try {
        imageSrc.value = await getImageBase64(name)
      } catch {
        imageSrc.value = ''
      } finally {
        loading.value = false
      }
    }
  },
  { immediate: true }
)
</script>

<template>
  <n-modal :show="!!filename" preset="card" title="图片预览" style="width: 600px" @close="emit('close')">
    <div class="viewer-body">
      <n-spin :show="loading">
        <img v-if="imageSrc" :src="imageSrc" class="preview-image" />
      </n-spin>
      <n-empty v-if="!imageSrc && !loading" description="无法加载图片" />
    </div>
  </n-modal>
</template>

<style scoped>
.viewer-body {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 200px;
}

.preview-image {
  max-width: 100%;
  max-height: 60vh;
  border-radius: 8px;
  object-fit: contain;
}
</style>
