import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Phrase, CreatePhraseRequest, UpdatePhraseRequest } from '../types'

export const usePhraseStore = defineStore('phrases', () => {
  const phrases = ref<Phrase[]>([])
  const loading = ref(false)
  const selectedCategoryId = ref<string | null>(null)
  const searchQuery = ref('')
  const favoritesOnly = ref(false)

  const filteredPhrases = computed(() => {
    let result = phrases.value
    if (favoritesOnly.value) {
      result = result.filter(p => p.is_favorite)
    }
    return result
  })

  async function fetchPhrases() {
    loading.value = true
    try {
      phrases.value = await invoke<Phrase[]>('list_phrases', {
        categoryId: selectedCategoryId.value,
        search: searchQuery.value || null,
      })
    } catch (e) {
      console.error('Failed to fetch phrases:', e)
    } finally {
      loading.value = false
    }
  }

  async function createPhrase(request: CreatePhraseRequest): Promise<Phrase> {
    const phrase = await invoke<Phrase>('create_phrase', { request })
    await fetchPhrases()
    return phrase
  }

  async function updatePhrase(id: string, request: UpdatePhraseRequest): Promise<Phrase> {
    const phrase = await invoke<Phrase>('update_phrase', { id, request })
    await fetchPhrases()
    return phrase
  }

  async function deletePhrase(id: string): Promise<void> {
    await invoke('delete_phrase', { id })
    await fetchPhrases()
  }

  async function incrementUsage(id: string): Promise<Phrase> {
    const phrase = await invoke<Phrase>('increment_usage', { id })
    const idx = phrases.value.findIndex(p => p.id === id)
    if (idx >= 0) phrases.value[idx] = phrase
    return phrase
  }

  async function toggleFavorite(id: string): Promise<Phrase> {
    const phrase = await invoke<Phrase>('toggle_favorite', { id })
    const idx = phrases.value.findIndex(p => p.id === id)
    if (idx >= 0) phrases.value[idx] = phrase
    return phrase
  }

  function setCategory(categoryId: string | null) {
    selectedCategoryId.value = categoryId
    favoritesOnly.value = false
    fetchPhrases()
  }

  function setSearch(query: string) {
    searchQuery.value = query
    fetchPhrases()
  }

  function setFavoritesFilter(show: boolean) {
    favoritesOnly.value = show
    if (show) {
      selectedCategoryId.value = null
      searchQuery.value = ''
      // Fetch all then filter client-side
      fetchPhrases()
    }
  }

  return {
    phrases,
    loading,
    selectedCategoryId,
    searchQuery,
    favoritesOnly,
    filteredPhrases,
    fetchPhrases,
    createPhrase,
    updatePhrase,
    deletePhrase,
    incrementUsage,
    toggleFavorite,
    setCategory,
    setSearch,
    setFavoritesFilter,
  }
})
