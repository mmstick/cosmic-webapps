use crate::common::{Browser, BrowserType};

#[allow(dead_code)]
pub fn native_browsers() -> Vec<Browser> {
    vec![
        Browser::new(
            BrowserType::Firefox,
            "Firefox",
            "firefox",
            "/usr/bin/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Firefox Developer Edition",
            "firefox-developer-edition",
            "/usr/bin/firefox-developer-edition",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Firefox Nightly",
            "firefox-nightly",
            "/usr/bin/firefox-nightly",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Firefox ESR",
            "firefox-esr",
            "/usr/bin/firefox-esr",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Brave Browser",
            "brave-browser",
            "/usr/bin/brave-browser",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Brave (bin)",
            "brave-bin",
            "/usr/bin/brave-bin",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chrome",
            "google-chrome-stable",
            "/usr/bin/google-chrome-stable",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chrome Beta",
            "google-chrome-beta",
            "/usr/bin/google-chrome-beta",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chromium",
            "chromium",
            "/usr/bin/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chromium Browser",
            "chromium-browser",
            "/usr/bin/chromium-browser",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chromium (snap)",
            "chromium",
            "/snap/bin/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chromium (bin)",
            "chromium-bin",
            "/usr/bin/chromium-bin-browser",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Librewolf",
            "librewolf",
            "/usr/bin/librewolf",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Waterfox",
            "waterfox",
            "/usr/bin/waterfox",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Waterfox (current)",
            "waterfox-current",
            "/usr/bin/waterfox-current",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Waterfox (classic)",
            "waterfox-classic",
            "/usr/bin/waterfox-classic",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Waterfox 3rd Generation",
            "waterfox-g3",
            "/usr/bin/waterfox-g3",
        ),
        Browser::new(
            BrowserType::Firefox,
            "Waterfox 4rd Generation",
            "waterfox-g4",
            "/usr/bin/waterfox-g4",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Vivaldi",
            "vivaldi-stable",
            "/usr/bin/vivaldi-stable",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Vivaldi Snapshot",
            "vivaldi-snapshot",
            "/usr/bin/vivaldi-snapshot",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Microsoft Edge",
            "microsoft-edge-stable",
            "/usr/bin/microsoft-edge-stable",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Microsoft Edge Beta",
            "microsoft-edge-beta",
            "/usr/bin/microsoft-edge-beta",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Microsoft Edge Dev",
            "microsoft-edge-dev",
            "/usr/bin/microsoft-edge-dev",
        ),
        Browser::new(
            BrowserType::Chromium,
            "FlashPeak Slimjet",
            "flashpeak-slimjet",
            "/usr/bin/flashpeak-slimjet",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Yandex",
            "yandex-browser",
            "/usr/bin/yandex-browser",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Naver Whale",
            "naver-whale-stable",
            "/usr/bin/naver-whale-stable",
        ),
        Browser::new(BrowserType::Chromium, "Brave", "brave", "/usr/bin/brave"),
        Browser::new(BrowserType::Falkon, "Falkon", "falkon", "/usr/bin/falkon"),
    ]
}

pub fn supported_browsers() -> Vec<Browser> {
    vec![
        Browser::new(
            BrowserType::FirefoxFlatpak,
            "Firefox",
            "/var/lib/flatpak/exports/bin/org.mozilla.firefox",
            "/var/lib/flatpak/exports/bin/org.mozilla.firefox",
        ),
        Browser::new(
            BrowserType::FirefoxFlatpak,
            "Firefox",
            ".local/share/flatpak/exports/bin/org.mozilla.firefox",
            ".local/share/flatpak/exports/bin/org.mozilla.firefox",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chrome",
            "/var/lib/flatpak/exports/bin/com.google.Chrome",
            "/var/lib/flatpak/exports/bin/com.google.Chrome",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chrome",
            ".local/share/flatpak/exports/bin/com.google.Chrome",
            ".local/share/flatpak/exports/bin/com.google.Chrome",
        ),
        Browser::new(
            BrowserType::Librewolf,
            "Librewolf",
            "/var/lib/flatpak/exports/bin/io.gitlab.librewolf-community",
            "/var/lib/flatpak/exports/bin/io.gitlab.librewolf-community",
        ),
        Browser::new(
            BrowserType::Librewolf,
            "Librewolf",
            ".local/share/flatpak/exports/bin/io.gitlab.librewolf-community",
            ".local/share/flatpak/exports/bin/io.gitlab.librewolf-community",
        ),
        Browser::new(
            BrowserType::WaterfoxFlatpak,
            "Waterfox",
            "/var/lib/flatpak/exports/bin/net.waterfox.waterfox",
            "/var/lib/flatpak/exports/bin/net.waterfox.waterfox",
        ),
        Browser::new(
            BrowserType::WaterfoxFlatpak,
            "Waterfox",
            ".local/share/flatpak/exports/bin/net.waterfox.waterfox",
            ".local/share/flatpak/exports/bin/net.waterfox.waterfox",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Vivaldi",
            "/var/lib/flatpak/exports/bin/com.vivaldi.Vivaldi",
            "/var/lib/flatpak/exports/bin/com.vivaldi.Vivaldi",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Vivaldi",
            ".local/share/flatpak/exports/bin/com.vivaldi.Vivaldi",
            ".local/share/flatpak/exports/bin/com.vivaldi.Vivaldi",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Ungoogled Chromium",
            "/var/lib/flatpak/exports/bin/com.github.Eloston.UngoogledChromium",
            "/var/lib/flatpak/exports/bin/com.github.Eloston.UngoogledChromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Ungoogled Chromium",
            ".local/share/flatpak/exports/bin/com.github.Eloston.UngoogledChromium",
            ".local/share/flatpak/exports/bin/com.github.Eloston.UngoogledChromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chromium",
            "/var/lib/flatpak/exports/bin/org.chromium.Chromium",
            "/var/lib/flatpak/exports/bin/org.chromium.Chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Chromium",
            ".local/share/flatpak/exports/bin/org.chromium.Chromium",
            ".local/share/flatpak/exports/bin/org.chromium.Chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Microsoft Edge",
            "/var/lib/flatpak/exports/bin/com.microsoft.Edge",
            "/var/lib/flatpak/exports/bin/com.microsoft.Edge",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Microsoft Edge",
            ".local/share/flatpak/exports/bin/com.microsoft.Edge",
            ".local/share/flatpak/exports/bin/com.microsoft.Edge",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Brave",
            "/var/lib/flatpak/exports/bin/com.brave.Browser",
            "/var/lib/flatpak/exports/bin/com.brave.Browser",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Brave",
            ".local/share/flatpak/exports/bin/com.brave.Browser",
            ".local/share/flatpak/exports/bin/com.brave.Browser",
        ),
        Browser::new(
            BrowserType::Falkon,
            "Falkon",
            "/var/lib/flatpak/exports/bin/org.kde.falkon",
            "/var/lib/flatpak/exports/bin/org.kde.falkon",
        ),
        Browser::new(
            BrowserType::Falkon,
            "Falkon",
            ".local/share/flatpak/exports/bin/org.kde.falkon",
            ".local/share/flatpak/exports/bin/org.kde.falkon",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Yandex",
            "/var/lib/flatpak/exports/bin/ru.yandex.Browser",
            "/var/lib/flatpak/exports/bin/ru.yandex.Browser",
        ),
        Browser::new(
            BrowserType::Chromium,
            "Yandex",
            ".local/share/flatpak/exports/bin/ru.yandex.Browser",
            ".local/share/flatpak/exports/bin/ru.yandex.Browser",
        ),
    ]
}
