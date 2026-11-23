export type Locale = 'en' | 'zh';

const STORAGE_KEY = 'codecbox.locale';

function loadLocale(): Locale {
	if (typeof localStorage === 'undefined') return 'en';
	return localStorage.getItem(STORAGE_KEY) === 'zh' ? 'zh' : 'en';
}

function applyDocument(locale: Locale) {
	if (typeof document === 'undefined') return;
	document.documentElement.lang = locale === 'zh' ? 'zh-CN' : 'en';
}

class LocaleStore {
	current = $state<Locale>(loadLocale());

	constructor() {
		applyDocument(this.current);
	}

	set(next: Locale) {
		this.current = next;
		if (typeof localStorage !== 'undefined') {
			localStorage.setItem(STORAGE_KEY, next);
		}
		applyDocument(next);
	}
}

export const locale = new LocaleStore();
