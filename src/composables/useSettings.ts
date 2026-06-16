import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface AppSettings {
  auto_start: boolean
  close_behavior: 'minimize_to_tray' | 'quit'
  custom_storage_path: string | null
}

const defaultSettings: AppSettings = {
  auto_start: false,
  close_behavior: 'minimize_to_tray',
  custom_storage_path: null,
}

const settings = ref<AppSettings>({ ...defaultSettings })
const loading = ref(false)

export function useSettings() {
  async function fetchSettings() {
    loading.value = true
    try {
      settings.value = await invoke<AppSettings>('get_settings')
    } catch (e) {
      console.error('Failed to load settings:', e)
    } finally {
      loading.value = false
    }
  }

  async function saveSettings(s: AppSettings) {
    try {
      await invoke('update_settings', { settings: s })
      settings.value = { ...s }
    } catch (e) {
      console.error('Failed to save settings:', e)
      throw e
    }
  }

  async function getStoragePath(): Promise<string> {
    return invoke<string>('get_storage_path')
  }

  async function setStoragePath(path: string): Promise<void> {
    await invoke('set_storage_path', { newPath: path })
  }

  async function openStorageDir(): Promise<void> {
    await invoke('open_storage_dir')
  }

  return {
    settings,
    loading,
    fetchSettings,
    saveSettings,
    getStoragePath,
    setStoragePath,
    openStorageDir,
    defaultSettings,
  }
}
