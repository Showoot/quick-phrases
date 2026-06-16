import { ref, watchEffect } from 'vue'

const THEME_KEY = 'quick-phrases-theme'
const isDark = ref(false)

export function useTheme() {
  // Initialize from localStorage
  const stored = localStorage.getItem(THEME_KEY)
  if (stored !== null) {
    isDark.value = stored === 'dark'
  } else {
    // Respect system preference
    isDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches
  }

  // Apply dark class to document
  watchEffect(() => {
    document.documentElement.classList.toggle('dark', isDark.value)
    localStorage.setItem(THEME_KEY, isDark.value ? 'dark' : 'light')
  })

  function toggleDark() {
    isDark.value = !isDark.value
  }

  function setDark(v: boolean) {
    isDark.value = v
  }

  return { isDark, toggleDark, setDark }
}
