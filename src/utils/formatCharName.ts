import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { Language } from '../stores/settings'
import { Character } from '../stores/characters'

export function useLang() {
    const { locale } = useI18n()
    return computed((): Language => {
        const raw = locale.value?.split('_')[1]?.toLowerCase()
        return (raw as Language) ?? 'en'
    })
}

export function formatCharName(costume: Character, lang: Language): string {
    return `${costume.character_name[lang] ?? costume.character_name.en} - ${costume.costume_name[lang] ?? costume.costume_name.en}`
}

export function getCharName(costume: Character, lang: Language): { character: string; costume: string } {
    return {
        character: costume.character_name[lang] ?? costume.character_name.en,
        costume: costume.costume_name[lang] ?? costume.costume_name.en
    }
}