<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Icon } from '@iconify/vue'

const appWindow = getCurrentWindow()

function minimize() { appWindow.minimize() }
function close() { appWindow.close() }
</script>

<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="titlebar-title">
      <Icon icon="ri:chat-quote-line" width="16" class="titlebar-icon" />
      <span>快捷话术</span>
    </div>
    <div class="titlebar-controls">
      <button class="ctrl-btn" @click="minimize" title="最小化">
        <svg width="12" height="12" viewBox="0 0 12 12"><rect y="5" width="12" height="1.5" fill="currentColor"/></svg>
      </button>
      <button class="ctrl-btn close" @click="close" title="关闭">
        <svg width="12" height="12" viewBox="0 0 12 12"><path d="M1 1l10 10M11 1L1 11" stroke="currentColor" stroke-width="1.3"/></svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 38px;
  background: var(--surface);
  border-bottom: 1px solid var(--border);
  user-select: none;
  flex-shrink: 0;
}
.titlebar-title {
  display: flex;
  align-items: center;
  gap: 7px;
  padding-left: 14px;
  font-size: 12.5px;
  font-weight: 550;
  color: var(--text-secondary);
  letter-spacing: -0.1px;
  pointer-events: none;
}
.titlebar-icon {
  color: var(--brand);
}
.titlebar-controls {
  display: flex;
  height: 100%;
}
.ctrl-btn {
  width: 46px;
  height: 100%;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background var(--transition-fast), color var(--transition-fast);
}
.ctrl-btn:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}
.ctrl-btn.close:hover {
  background: #e53e3e;
  color: #fff;
}
</style>
