import { createI18n } from 'vue-i18n';
import de from './de';
import en from './en';

const i18n = createI18n({
    legacy: false,
    locale: 'de',
    fallbackLocale: 'de',
    messages: { de, en },
});

export default i18n;
