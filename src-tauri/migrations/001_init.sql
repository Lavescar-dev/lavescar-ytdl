CREATE TABLE IF NOT EXISTS downloads (
    id            TEXT PRIMARY KEY,
    url           TEXT NOT NULL,
    title         TEXT NOT NULL,
    status        TEXT NOT NULL,
    codec         TEXT NOT NULL DEFAULT '',
    output_path   TEXT,
    started_at    INTEGER NOT NULL,
    finished_at   INTEGER,
    error         TEXT
);

CREATE INDEX IF NOT EXISTS idx_downloads_url    ON downloads(url);
CREATE INDEX IF NOT EXISTS idx_downloads_status ON downloads(status);
CREATE INDEX IF NOT EXISTS idx_downloads_finished_at ON downloads(finished_at DESC);

CREATE TABLE IF NOT EXISTS presets (
    id           TEXT PRIMARY KEY,
    name         TEXT NOT NULL,
    format_spec  TEXT NOT NULL,
    flags        TEXT NOT NULL DEFAULT '[]',
    hotkey       TEXT,
    is_default   INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS settings (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

INSERT OR IGNORE INTO presets (id, name, format_spec, flags, hotkey, is_default) VALUES
    ('archive-av1',    'archive · av1',   'bv[vcodec~=''av01'']+ba[acodec=''opus'']/b', '[]', NULL, 1),
    ('max-audio',      'max audio',       'ba/b',                                      '["--extract-audio","--sponsorblock-remove","sponsor"]', '⌘1', 0),
    ('mobile-720',     'mobile · 720p',   'b[ext=mp4][height<=720]',                    '[]', '⌘2', 0),
    ('podcast-split',  'podcast · split', 'ba/b',                                      '["--split-chapters","--embed-metadata"]', '⌘3', 0);
