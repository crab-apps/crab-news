-- https://crates.io/crates/config
CREATE TABLE IF NOT EXISTS settings (
    theme_mode VARCHAR(50) CHECK (theme_mode IN ('light', 'dark', 'system')) NOT NULL DEFAULT 'system',
    theme_light VARCHAR(50) CHECK (
        theme_light IN (
            'light',
            'cupcake',
            'bumblebee',
            'emerald',
            'corporate',
            'retro',
            'cyberpunk',
            'valentine',
            'garden',
            'lofi',
            'pastel',
            'fantasy',
            'wireframe',
            'cmyk',
            'autumn',
            'acid',
            'lemonade',
            'winter',
            'nord',
            'caramellatte',
            'silk'
        )
    ) NOT NULL DEFAULT 'light',
    theme_dark VARCHAR(50) CHECK (
        theme_dark IN (
            'dark',
            'synthwave',
            'halloween',
            'forest',
            'aqua',
            'black',
            'luxury',
            'dracula',
            'business',
            'night',
            'coffee',
            'dim',
            'sunset',
            'abyss'
        )
    ) NOT NULL DEFAULT 'dark',
    text_size INTEGER CHECK (text_size IN (14, 16, 18, 20)) NOT NULL DEFAULT 16,
    browser VARCHAR(50) CHECK (
        browser IN (
            'default',
            'brave',
            'chrome',
            'duckduckgo',
            'edge',
            'firefox',
            'librewolf',
            'mullvad',
            'opera',
            'safari',
            'tor',
            'ungoogled',
            'vivaldi'
        )
    ) NOT NULL DEFAULT 'default',
    open_method VARCHAR(50) CHECK (open_method IN ('background', 'foreground')) NOT NULL DEFAULT 'background',
    refresh_interval INTEGER CHECK (refresh_interval IN (15, 30, 60, 120, 240, 480)) NOT NULL DEFAULT 60
);
