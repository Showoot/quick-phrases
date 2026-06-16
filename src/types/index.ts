// === Category ===
export interface Category {
  id: string
  name: string
  icon: string | null
  sort_order: number
  created_at: number
}

// === Phrase ===
export interface Phrase {
  id: string
  category_id: string
  title: string
  text_content: string
  image_paths: string[]
  tags: string[]
  is_favorite: boolean
  usage_count: number
  created_at: number
  updated_at: number
}

// === Request types ===
export interface CreatePhraseRequest {
  category_id: string
  title: string
  text_content: string
  image_paths: string[]
  tags: string[]
}

export interface UpdatePhraseRequest {
  category_id?: string
  title?: string
  text_content?: string
  image_paths?: string[]
  tags?: string[]
  is_favorite?: boolean
}
