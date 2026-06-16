<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  visible: boolean
  message?: string
}>()

const show = ref(false)
let timer: ReturnType<typeof setTimeout>

watch(
  () => props.visible,
  (v) => {
    if (v) {
      show.value = true
      clearTimeout(timer)
      timer = setTimeout(() => {
        show.value = false
      }, 2200)
    }
  }
)
</script>

<template>
  <Transition name="toast">
    <div v-if="show" class="toast">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
        <polyline points="20 6 9 17 4 12"/>
      </svg>
      <span>{{ message || '已复制到剪贴板' }}</span>
    </div>
  </Transition>
</template>

<style scoped>
.toast {
  position: fixed;
  bottom: 28px;
  right: 28px;
  z-index: 100;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 18px;
  background: var(--text-primary);
  color: var(--bg);
  border-radius: var(--radius-md);
  font-size: 13px;
  font-family: var(--font-stack);
  font-weight: 500;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  pointer-events: none;
}

.toast-enter-active {
  transition: all 0.25s cubic-bezier(0.16, 1, 0.3, 1);
}

.toast-leave-active {
  transition: all 0.2s ease-in;
}

.toast-enter-from {
  opacity: 0;
  transform: translateY(12px) scale(0.95);
}

.toast-leave-to {
  opacity: 0;
  transform: translateY(4px);
}
</style>
