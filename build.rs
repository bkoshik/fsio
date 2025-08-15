fn main() {
    cfg_aliases::cfg_aliases! {
        macos: { target_os = "macos" },
        ios: { target_os = "ios" },
        linux: { target_os = "linux" },

        apple_os: { any(macos, ios) }
    }
}
