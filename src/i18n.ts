import { createI18n } from 'vue-i18n'
import zhCN from './locales/zh-CN'
import zhTW from './locales/zh-TW'
import en from './locales/en'

const saved = localStorage.getItem('quick-phrases-lang') || 'zh-CN'

export const i18n = createI18n({
  legacy: false,
  locale: saved,
  fallbackLocale: 'zh-CN',
  messages: { 'zh-CN': zhCN, 'zh-TW': zhTW, en },
})

export function setLanguage(lang: string) {
  i18n.global.locale.value = lang as 'zh-CN' | 'zh-TW' | 'en'
  localStorage.setItem('quick-phrases-lang', lang)
}
