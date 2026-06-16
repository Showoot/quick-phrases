import { createApp } from 'vue'
import { createPinia } from 'pinia'
import naive from 'naive-ui'
import App from './App.vue'
import { i18n } from './i18n'
import './assets/styles/global.css'

const app = createApp(App)
app.use(createPinia())
app.use(naive)
app.use(i18n)
app.mount('#app')
