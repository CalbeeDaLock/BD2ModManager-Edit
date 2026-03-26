import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import Aura from '@primeuix/themes/aura';

import PrimeVue from 'primevue/config';

import "./App.css"
import { createPinia } from "pinia";
import ToastService from 'primevue/toastservice';

import {createI18n} from 'vue-i18n'

import en_US from './locales/en-US.json'
import pt_BR from './locales/pt-BR.json'
import zh_CN from './locales/zh-CN.json'
import zh_TW from './locales/zh-TW.json'
import ja_JP from './locales/ja-JP.json'
import ko_KR from './locales/ko-KR.json'
import ConfirmPlugin from "./plugins/ConfirmPlugin";

const pinia = createPinia()

const i18n = createI18n({
    legacy: false,
    locale: 'en_US',
    fallbackLocale: 'en_US',
    messages: {
        en_US,
        zh_CN,
        zh_TW,
        ja_JP,
        ko_KR,
        pt_BR
    }
})

const app = createApp(App)
app.use(router)
.use(pinia)
.use(PrimeVue, {
    theme: {
        preset: Aura
    }
})
.use(i18n)
.use(ToastService)
.use(ConfirmPlugin)

app.mount("#app");