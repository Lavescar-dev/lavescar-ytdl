/**
 * Translation dictionaries for the desktop app.
 *
 * EN is the source of truth — every component imports `t.X.Y` references
 * that exist on `Dict`. TR mirrors the structure; missing keys fall back
 * to EN (handled in `i18n.svelte.ts`).
 */

export interface Dict {
  topbar:    { pauseAll: string; settings: string; logs: string };
  sidebar:   {
    workspace: string;
    download: string;
    queue: string;
    history: string;
    presets: string;
    cookies: string;
    runtime: string;
    disk: string;
    target: string;
    free: string;
    deps: { ytdlp: string; ffmpeg: string; aria2c: string; cookies: string };
  };
  urlbar: {
    sourceUrl: string;
    clipboardActive: string;
    placeholder: string;
    fetch: string;
    fetching: string;
    queue: string;
    queueing: string;
  };
  metadata: {
    title: string;
    awaitingInput: string;
    fetching: string;
    fetchingFrom: string;
    pasteHint: string;
    ready: string;
    error: string;
    duration: string;
    bestV: string;
    bestA: string;
    sizeEst: string;
    subs: string;
    chapters: string;
    configureSubtitles: string;
    noSubtitlesSelected: string;
  };
  preset: {
    title: string;
    hint: string;
    video: string;
    audio: string;
    manage: string;
    empty: string;
    default: string;
  };
  download: {
    active: string;
    running: string;
    concurrent: string;
    throttle: string;
    throttleOn: string;
    throttleOff: string;
    queued: string;
    done: string;
    openInMpv: string;
    showInFolder: string;
    moveUp: string;
    removeBtn: string;
    pause: string;
    cancel: string;
    eta: string;
  };
  status: { ready: string; active: string; queued: string; preset: string; target: string };
  settings: {
    title: string;
    saveHint: string;
    outputDir: string;
    browse: string;
    concurrent: string;
    concurrentHint: string;
    throttle: string;
    throttleApplyHint: string;
    defaultPreset: string;
    cookieSource: string;
    cookieNone: string;
    watchClipboard: string;
    updateYtdlp: string;
    updateYtdlpUpdating: string;
    updateYtdlpUpdatedTo: string;
    updatePhase: { resolving: string; downloading: string; installing: string; done: string };
    save: string;
    saving: string;
    cancel: string;
    language: string;
    languageEn: string;
    languageTr: string;
  };
  presetsView: {
    title: string;
    newPreset: string;
    edit: string;
    delete: string;
    deleteConfirm: string;
    name: string;
    category: string;
    formatSpec: string;
    flagsLabel: string;
    flagsPlaceholder: string;
    hotkey: string;
    hotkeyPlaceholder: string;
    defaultPreset: string;
    save: string;
    saving: string;
    cancel: string;
    editTitle: string;
    newTitle: string;
  };
  playlist: {
    by: string;
    selected: string;
    selectedOf: (n: number, total: number) => string;
    all: string;
    none: string;
    rangePlaceholder: string;
    apply: string;
    preset: string;
    cancel: string;
    queueing: string;
    queueN: (n: number) => string;
    close: string;
  };
  subtitle: {
    title: string;
    manualMode: string;
    autoMode: string;
    embedFile: string;
    notAvailable: string;
    selectedOf: (n: number, total: number) => string;
    none: string;
    apply: string;
    cancel: string;
    badgeManual: string;
    badgeAuto: string;
    close: string;
  };
  error: {
    kinds: Record<
      'geo_blocked' | 'auth_required' | 'not_found' | 'network' | 'io' | 'parse' | 'shell' | 'unknown' | 'fetch_failed',
      { title: string; suggestion: string }
    >;
    openCookies: string;
    dismiss: string;
  };
  update: {
    available: (v: string) => string;
    install: string;
    later: string;
    downloading: string;
    installed: string;
    restarting: string;
    failed: string;
    dismiss: string;
  };
  history: {
    title: string;
    searchPlaceholder: string;
    empty: string;
    copyPath: string;
    statusDone: string;
    statusError: string;
    statusCancelled: string;
  };
  queueView: {
    title: string;
    empty: string;
    cancel: string;
    items: (n: number) => string;
  };
  cookies: {
    title: string;
    intro: string;
    sourceFirefox: string;
    sourceChromium: string;
    sourceBrave: string;
    sourceCustom: string;
    customPath: string;
    test: string;
    futureNote: string;
  };
  orphans: {
    summary: (n: number, mb: string, dir: string) => string;
    deleteAll: string;
    cleaning: string;
    dismiss: string;
  };
  clipboard: {
    fetch: string;
    dismiss: string;
  };
  shortcuts: {
    title: string;
    focusUrl: string;
    presetIndex: (n: number) => string;
    openSettings: string;
    showCheatsheet: string;
    closeOverlay: string;
    switchVideo: string;
    switchAudio: string;
    pressAnytime: string;
    toClose: string;
  };
  langSwitch: { en: string; tr: string };
}

export const en: Dict = {
  topbar: { pauseAll: '⏸ pause all', settings: '⚙ settings', logs: '≡ logs' },
  sidebar: {
    workspace: 'workspace',
    download: 'download',
    queue: 'queue',
    history: 'history',
    presets: 'presets',
    cookies: 'cookies',
    runtime: 'runtime',
    disk: 'disk',
    target: 'target',
    free: 'free',
    deps: { ytdlp: 'yt-dlp', ffmpeg: 'ffmpeg', aria2c: 'aria2c', cookies: 'cookies' }
  },
  urlbar: {
    sourceUrl: 'source url',
    clipboardActive: 'clipboard listener active',
    placeholder: 'https://… or paste from clipboard',
    fetch: 'fetch',
    fetching: 'fetching…',
    queue: '▸ queue',
    queueing: 'queueing…'
  },
  metadata: {
    title: 'metadata',
    awaitingInput: 'awaiting input',
    fetching: 'fetching…',
    fetchingFrom: 'fetching from yt-dlp…',
    pasteHint: 'paste a url above to inspect formats',
    ready: 'ready',
    error: 'error',
    duration: 'duration',
    bestV: 'best-v',
    bestA: 'best-a',
    sizeEst: 'size-est',
    subs: 'subs',
    chapters: 'chapters',
    configureSubtitles: 'configure subtitles…',
    noSubtitlesSelected: 'no subtitles selected'
  },
  preset: {
    title: 'preset',
    hint: '⌘1-3 within tab',
    video: 'video',
    audio: 'audio',
    manage: '+ manage presets',
    empty: 'no presets in this category yet.',
    default: 'default'
  },
  download: {
    active: 'active',
    running: 'running',
    concurrent: 'concurrent',
    throttle: 'throttle',
    throttleOn: 'on',
    throttleOff: 'off',
    queued: 'queued',
    done: 'done',
    openInMpv: '▸ open in mpv',
    showInFolder: 'Show in folder',
    moveUp: 'Move up',
    removeBtn: 'Remove',
    pause: 'Pause',
    cancel: 'Cancel',
    eta: 'eta'
  },
  status: { ready: 'ready', active: 'ACTIVE', queued: 'QUEUED', preset: 'PRESET', target: 'TARGET' },
  settings: {
    title: 'settings',
    saveHint: '⌘S save · ESC close',
    outputDir: 'output directory',
    browse: 'browse…',
    concurrent: 'concurrent downloads',
    concurrentHint: 'applies immediately to queued downloads',
    throttle: 'throttle download speed',
    throttleApplyHint: 'applies to new downloads · active tasks keep their original rate',
    defaultPreset: 'default preset',
    cookieSource: 'cookie source',
    cookieNone: 'none',
    watchClipboard: 'watch clipboard for URLs',
    updateYtdlp: '▸ update yt-dlp',
    updateYtdlpUpdating: 'updating…',
    updateYtdlpUpdatedTo: 'updated to',
    updatePhase: { resolving: 'resolving', downloading: 'downloading', installing: 'installing', done: 'done' },
    save: 'save',
    saving: 'saving…',
    cancel: 'cancel',
    language: 'language',
    languageEn: 'English',
    languageTr: 'Türkçe'
  },
  presetsView: {
    title: 'presets',
    newPreset: '+ new preset',
    edit: 'edit',
    delete: 'delete',
    deleteConfirm: 'delete this preset?',
    name: 'name',
    category: 'category',
    formatSpec: 'format spec (yt-dlp -f)',
    flagsLabel: 'extra flags (space-separated)',
    flagsPlaceholder: '--embed-metadata --sponsorblock-remove sponsor',
    hotkey: 'hotkey',
    hotkeyPlaceholder: '⌘1 · optional',
    defaultPreset: 'default preset',
    save: 'save',
    saving: 'saving…',
    cancel: 'cancel',
    editTitle: 'edit preset',
    newTitle: 'new preset'
  },
  playlist: {
    by: 'by',
    selected: 'selected',
    selectedOf: (n, total) => `${n} / ${total} selected`,
    all: 'all',
    none: 'none',
    rangePlaceholder: 'range e.g. 1-10, 15, 20-25',
    apply: 'apply',
    preset: 'preset',
    cancel: 'cancel',
    queueing: 'queueing…',
    queueN: (n) => `▸ queue ${n}`,
    close: 'Close'
  },
  subtitle: {
    title: 'subtitles',
    manualMode: 'manual captions (author-provided)',
    autoMode: 'auto-generated captions',
    embedFile: 'embed into video file (otherwise saved alongside as .vtt)',
    notAvailable: 'no subtitles available for this video.',
    selectedOf: (n, total) => `${n} of ${total} language${total === 1 ? '' : 's'} selected`,
    none: 'no subtitles',
    apply: 'apply',
    cancel: 'cancel',
    badgeManual: 'manual',
    badgeAuto: 'auto',
    close: 'Close'
  },
  error: {
    kinds: {
      geo_blocked:    { title: 'Geo-blocked',          suggestion: 'Try a VPN or load cookies via Settings → cookie source.' },
      auth_required:  { title: 'Sign-in required',     suggestion: 'Switch to the Cookies view and pick a browser with a logged-in session.' },
      not_found:      { title: 'Video unavailable',    suggestion: 'The video may have been removed, set to private, or region-locked.' },
      network:        { title: 'Network error',        suggestion: 'Check your connection. yt-dlp automatically retries; give it a moment.' },
      io:             { title: 'Disk error',           suggestion: 'Verify the output directory exists and has free space.' },
      parse:          { title: 'Response parse error', suggestion: 'yt-dlp output was unexpected. Try updating yt-dlp from Settings.' },
      shell:          { title: 'Binary launch error',  suggestion: 'The yt-dlp sidecar could not start. Reinstall or update it.' },
      unknown:        { title: 'yt-dlp error',         suggestion: 'Check the full message below, or try again.' },
      fetch_failed:   { title: 'Metadata fetch failed', suggestion: 'The URL could not be inspected. Double-check it is a supported site.' }
    },
    openCookies: '→ open cookies view',
    dismiss: 'Dismiss'
  },
  update: {
    available: (v) => `Update available · ${v}`,
    install: 'install & relaunch',
    later: 'later',
    downloading: 'Downloading update…',
    installed: 'Update installed.',
    restarting: 'restarting…',
    failed: 'Update failed:',
    dismiss: 'dismiss'
  },
  history: {
    title: 'history',
    searchPlaceholder: 'search title, url, codec…',
    empty: 'no completed downloads yet.',
    copyPath: 'copy path',
    statusDone: 'done',
    statusError: 'error',
    statusCancelled: 'cancelled'
  },
  queueView: {
    title: 'queue',
    empty: 'queue is empty — paste a URL in the download view.',
    cancel: 'cancel',
    items: (n) => `${n} item${n === 1 ? '' : 's'}`
  },
  cookies: {
    title: 'cookies',
    intro: "yt-dlp needs browser cookies for private or age-gated content. Pick a source below; it's forwarded as --cookies-from-browser automatically.",
    sourceFirefox: 'Firefox',
    sourceChromium: 'Chromium',
    sourceBrave: 'Brave',
    sourceCustom: 'Custom path…',
    customPath: '/path/to/cookies.txt',
    test: 'test · fetch private video',
    futureNote: 'Advanced (Faz 6): Firefox cookies.sqlite + Chromium Cookies (DPAPI / Keychain / libsecret decrypt) automatic import.'
  },
  orphans: {
    summary: (n, mb, dir) => `${n} leftover .part file${n === 1 ? '' : 's'} · ${mb} in ${dir}`,
    deleteAll: 'delete all',
    cleaning: 'cleaning…',
    dismiss: 'dismiss'
  },
  clipboard: {
    fetch: 'fetch',
    dismiss: 'dismiss'
  },
  shortcuts: {
    title: 'keyboard shortcuts',
    focusUrl: 'focus URL input',
    presetIndex: (n) => `active category preset #${n}`,
    openSettings: 'open settings',
    showCheatsheet: 'show shortcuts cheatsheet',
    closeOverlay: 'close overlay',
    switchVideo: 'switch to video preset tab',
    switchAudio: 'switch to audio preset tab',
    pressAnytime: 'press',
    toClose: 'to close'
  },
  langSwitch: { en: 'EN', tr: 'TR' }
};

export const tr: Dict = {
  topbar: { pauseAll: '⏸ tümünü duraklat', settings: '⚙ ayarlar', logs: '≡ kayıtlar' },
  sidebar: {
    workspace: 'çalışma alanı',
    download: 'indirme',
    queue: 'kuyruk',
    history: 'geçmiş',
    presets: 'preset’ler',
    cookies: 'cookies',
    runtime: 'çalışma ortamı',
    disk: 'disk',
    target: 'hedef',
    free: 'boş',
    deps: { ytdlp: 'yt-dlp', ffmpeg: 'ffmpeg', aria2c: 'aria2c', cookies: 'cookies' }
  },
  urlbar: {
    sourceUrl: 'kaynak url',
    clipboardActive: 'pano dinleyici aktif',
    placeholder: 'https://… ya da panodan yapıştır',
    fetch: 'getir',
    fetching: 'getiriliyor…',
    queue: '▸ kuyruk',
    queueing: 'kuyruğa eklendi…'
  },
  metadata: {
    title: 'metadata',
    awaitingInput: 'girdi bekleniyor',
    fetching: 'getiriliyor…',
    fetchingFrom: 'yt-dlp’den getiriliyor…',
    pasteHint: 'format’ları görmek için yukarıya bir url yapıştırın',
    ready: 'hazır',
    error: 'hata',
    duration: 'süre',
    bestV: 'en iyi-v',
    bestA: 'en iyi-a',
    sizeEst: 'boyut-tah',
    subs: 'altyazı',
    chapters: 'bölümler',
    configureSubtitles: 'altyazıları ayarla…',
    noSubtitlesSelected: 'altyazı seçilmedi'
  },
  preset: {
    title: 'preset',
    hint: '⌘1-3 sekme içinde',
    video: 'video',
    audio: 'ses',
    manage: '+ preset’leri yönet',
    empty: 'bu kategoride henüz preset yok.',
    default: 'varsayılan'
  },
  download: {
    active: 'aktif',
    running: 'çalışıyor',
    concurrent: 'eşzamanlı',
    throttle: 'hız sınırı',
    throttleOn: 'açık',
    throttleOff: 'kapalı',
    queued: 'kuyrukta',
    done: 'bitti',
    openInMpv: '▸ mpv ile aç',
    showInFolder: 'Klasörde göster',
    moveUp: 'Yukarı taşı',
    removeBtn: 'Kaldır',
    pause: 'Duraklat',
    cancel: 'İptal',
    eta: 'kalan'
  },
  status: { ready: 'hazır', active: 'AKTİF', queued: 'KUYRUKTA', preset: 'PRESET', target: 'HEDEF' },
  settings: {
    title: 'ayarlar',
    saveHint: '⌘S kaydet · ESC kapat',
    outputDir: 'çıktı klasörü',
    browse: 'gözat…',
    concurrent: 'eşzamanlı indirme',
    concurrentHint: 'kuyruktaki indirmelere anında uygulanır',
    throttle: 'indirme hızını sınırla',
    throttleApplyHint: 'yeni indirmelere uygulanır · aktif görevler eski hızlarında devam eder',
    defaultPreset: 'varsayılan preset',
    cookieSource: 'cookie kaynağı',
    cookieNone: 'yok',
    watchClipboard: 'panoyu URL için izle',
    updateYtdlp: '▸ yt-dlp güncelle',
    updateYtdlpUpdating: 'güncelleniyor…',
    updateYtdlpUpdatedTo: 'şuna güncellendi',
    updatePhase: { resolving: 'çözümleniyor', downloading: 'indiriliyor', installing: 'kuruluyor', done: 'tamam' },
    save: 'kaydet',
    saving: 'kaydediliyor…',
    cancel: 'iptal',
    language: 'dil',
    languageEn: 'English',
    languageTr: 'Türkçe'
  },
  presetsView: {
    title: 'preset’ler',
    newPreset: '+ yeni preset',
    edit: 'düzenle',
    delete: 'sil',
    deleteConfirm: 'bu preset’i sil?',
    name: 'isim',
    category: 'kategori',
    formatSpec: 'format şablonu (yt-dlp -f)',
    flagsLabel: 'ekstra bayraklar (boşlukla ayrılmış)',
    flagsPlaceholder: '--embed-metadata --sponsorblock-remove sponsor',
    hotkey: 'kısayol',
    hotkeyPlaceholder: '⌘1 · isteğe bağlı',
    defaultPreset: 'varsayılan preset',
    save: 'kaydet',
    saving: 'kaydediliyor…',
    cancel: 'iptal',
    editTitle: 'preset düzenle',
    newTitle: 'yeni preset'
  },
  playlist: {
    by: 'kanal',
    selected: 'seçili',
    selectedOf: (n, total) => `${n} / ${total} seçili`,
    all: 'tümü',
    none: 'hiçbiri',
    rangePlaceholder: 'aralık örn. 1-10, 15, 20-25',
    apply: 'uygula',
    preset: 'preset',
    cancel: 'iptal',
    queueing: 'kuyruğa ekleniyor…',
    queueN: (n) => `▸ kuyruğa ekle (${n})`,
    close: 'Kapat'
  },
  subtitle: {
    title: 'altyazılar',
    manualMode: 'manuel altyazılar (yazar tarafından)',
    autoMode: 'otomatik üretilen altyazılar',
    embedFile: 'video dosyasına göm (yoksa yanına .vtt olarak kaydet)',
    notAvailable: 'bu video için altyazı yok.',
    selectedOf: (n, total) => `${total} dilden ${n} tanesi seçili`,
    none: 'altyazı yok',
    apply: 'uygula',
    cancel: 'iptal',
    badgeManual: 'manuel',
    badgeAuto: 'otomatik',
    close: 'Kapat'
  },
  error: {
    kinds: {
      geo_blocked:   { title: 'Bölge engeli',          suggestion: 'VPN deneyin ya da Settings → cookie kaynağı’ndan oturum yükleyin.' },
      auth_required: { title: 'Oturum açma gerekli',   suggestion: 'Cookies görünümüne geçin ve oturumlu bir tarayıcı seçin.' },
      not_found:    { title: 'Video bulunamadı',      suggestion: 'Video kaldırılmış, özel ya da bölge kilitli olabilir.' },
      network:      { title: 'Ağ hatası',             suggestion: 'Bağlantınızı kontrol edin. yt-dlp otomatik tekrar dener, biraz bekleyin.' },
      io:           { title: 'Disk hatası',           suggestion: 'Çıktı klasörünün var olduğundan ve boş alan olduğundan emin olun.' },
      parse:        { title: 'Yanıt çözümleme hatası', suggestion: 'yt-dlp çıktısı beklenmedik. Settings’ten yt-dlp’yi güncellemeyi deneyin.' },
      shell:        { title: 'Binary başlatma hatası', suggestion: 'yt-dlp sidecar başlatılamadı. Yeniden kurun veya güncelleyin.' },
      unknown:      { title: 'yt-dlp hatası',          suggestion: 'Aşağıdaki mesajı kontrol edin ya da tekrar deneyin.' },
      fetch_failed: { title: 'Metadata getirilemedi',  suggestion: 'URL incelenemedi. Desteklenen bir site olduğundan emin olun.' }
    },
    openCookies: '→ cookies görünümünü aç',
    dismiss: 'Kapat'
  },
  update: {
    available: (v) => `Güncelleme mevcut · ${v}`,
    install: 'kur & yeniden başlat',
    later: 'sonra',
    downloading: 'Güncelleme indiriliyor…',
    installed: 'Güncelleme kuruldu.',
    restarting: 'yeniden başlatılıyor…',
    failed: 'Güncelleme başarısız:',
    dismiss: 'kapat'
  },
  history: {
    title: 'geçmiş',
    searchPlaceholder: 'başlık, url, codec ara…',
    empty: 'henüz tamamlanmış indirme yok.',
    copyPath: 'yolu kopyala',
    statusDone: 'bitti',
    statusError: 'hata',
    statusCancelled: 'iptal'
  },
  queueView: {
    title: 'kuyruk',
    empty: 'kuyruk boş — download görünümünden bir URL yapıştırın.',
    cancel: 'iptal',
    items: (n) => `${n} öğe`
  },
  cookies: {
    title: 'cookies',
    intro: 'yt-dlp özel veya age-gated içerik için tarayıcı cookie\'lerine ihtiyaç duyar. Aşağıdan bir kaynak seçin; --cookies-from-browser olarak otomatik aktarılır.',
    sourceFirefox: 'Firefox',
    sourceChromium: 'Chromium',
    sourceBrave: 'Brave',
    sourceCustom: 'Özel yol…',
    customPath: '/path/to/cookies.txt',
    test: 'test · özel video getir',
    futureNote: 'Gelişmiş (Faz 6): Firefox cookies.sqlite + Chromium Cookies (DPAPI / Keychain / libsecret çözümleme) ile otomatik içe aktarma.'
  },
  orphans: {
    summary: (n, mb, dir) => `${n} artık .part dosyası · ${dir} içinde ${mb}`,
    deleteAll: 'hepsini sil',
    cleaning: 'temizleniyor…',
    dismiss: 'kapat'
  },
  clipboard: {
    fetch: 'getir',
    dismiss: 'kapat'
  },
  shortcuts: {
    title: 'klavye kısayolları',
    focusUrl: 'URL alanına odaklan',
    presetIndex: (n) => `aktif kategori preset #${n}`,
    openSettings: 'ayarları aç',
    showCheatsheet: 'kısayol cetvelini göster',
    closeOverlay: 'overlay\'i kapat',
    switchVideo: 'video preset sekmesine geç',
    switchAudio: 'audio preset sekmesine geç',
    pressAnytime: 'her yerde',
    toClose: 'kapatmak için'
  },
  langSwitch: { en: 'EN', tr: 'TR' }
};

export const dictionaries = { en, tr } as const;
export type Locale = keyof typeof dictionaries;
