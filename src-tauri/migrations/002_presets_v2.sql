-- Categorised preset library. Run on every launch: INSERT OR IGNORE keeps
-- user-customised rows intact, the UPDATE lines only touch the seed set
-- shipped in 001 so existing installs pick up the new `category` column.

UPDATE presets SET category = 'video' WHERE id IN ('archive-av1', 'mobile-720');
UPDATE presets SET category = 'audio' WHERE id IN ('max-audio', 'podcast-split');

-- Video: highest-quality archive at top, mobile/small fallbacks, 4K optional.
INSERT OR IGNORE INTO presets (id, name, format_spec, flags, hotkey, is_default, category) VALUES
    ('video-archive-av1',  'archive · av1',     'bv[vcodec~=''av01'']+ba[acodec=''opus'']/b',                                   '[]',                                           '⌘1', 1, 'video'),
    ('video-archive-vp9',  'archive · vp9',     'bv[vcodec~=''vp09'']+ba[acodec=''opus'']/b',                                   '[]',                                           '⌘2', 0, 'video'),
    ('video-mobile-720',   'mobile · 720p',     'bv[vcodec~=''avc1''][height<=720]+ba[acodec~=''mp4a'']/b[ext=mp4][height<=720]', '[]',                                         '⌘3', 0, 'video'),
    ('video-small-480',    'small · 480p',      'bv[height<=480]+ba/b[height<=480]',                                             '[]',                                           NULL, 0, 'video'),
    ('video-4k-av1',       '4K · av1',          'bv[vcodec~=''av01''][height<=2160]+ba[acodec=''opus'']/bv+ba/b',                '[]',                                           NULL, 0, 'video');

-- Audio: "native" keeps container untouched (fastest), transcoded variants when you need specific codecs.
INSERT OR IGNORE INTO presets (id, name, format_spec, flags, hotkey, is_default, category) VALUES
    ('audio-opus',         'opus · native',     'ba[acodec=''opus'']/ba',                                                        '[]',                                                             '⌘1', 1, 'audio'),
    ('audio-m4a',          'm4a · native',      'ba[ext=m4a]/ba[acodec~=''mp4a'']',                                              '[]',                                                             '⌘2', 0, 'audio'),
    ('audio-mp3-v0',       'mp3 · v0 (320k)',   'ba/b',                                                                          '["--extract-audio","--audio-format","mp3","--audio-quality","0"]', '⌘3', 0, 'audio'),
    ('audio-flac',         'flac · lossless',   'ba/b',                                                                          '["--extract-audio","--audio-format","flac"]',                   NULL, 0, 'audio'),
    ('audio-podcast-chapters', 'podcast · chapters', 'ba/b',                                                                    '["--extract-audio","--split-chapters","--embed-metadata"]',     NULL, 0, 'audio');

-- Clean out the legacy placeholder rows if they still carry the default hotkeys
-- that now belong to the categorised set. (Safe: users can delete/modify them.)
UPDATE presets SET hotkey = NULL WHERE id IN ('archive-av1', 'max-audio', 'mobile-720', 'podcast-split');
UPDATE presets SET is_default = 0 WHERE id IN ('archive-av1', 'max-audio', 'mobile-720', 'podcast-split');
