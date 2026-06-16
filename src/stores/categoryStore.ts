import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Category } from '../types'

export const useCategoryStore = defineStore('categories', () => {
  const categories = ref<Category[]>([])
  const loading = ref(false)
  const selectedId = ref<string | null>(null)

  async function fetchCategories() {
    loading.value = true
    try {
      categories.value = await invoke<Category[]>('list_categories')
    } catch (e) {
      console.error('Failed to fetch categories:', e)
    } finally {
      loading.value = false
    }
  }

  async function createCategory(name: string, icon?: string): Promise<Category> {
    const cat = await invoke<Category>('create_category', { name, icon })
    await fetchCategories()
    return cat
  }

  async function updateCategory(id: string, name: string): Promise<Category> {
    const cat = await invoke<Category>('update_category', { id, name })
    await fetchCategories()
    return cat
  }

  async function deleteCategory(id: string): Promise<void> {
    await invoke('delete_category', { id })
    // Clear selection if deleted
    if (selectedId.value === id) {
      selectedId.value = null
    }
    await fetchCategories()
  }

  async function reorderCategories(ids: string[]): Promise<void> {
    await invoke('reorder_categories', { ids })
    await fetchCategories()
  }

  function selectCategory(id: string | null) {
    selectedId.value = id
  }

  return {
    categories,
    loading,
    selectedId,
    fetchCategories,
    createCategory,
    updateCategory,
    deleteCategory,
    reorderCategories,
    selectCategory,
  }
})
