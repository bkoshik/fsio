fn main() {
    cfg_aliases::cfg_aliases! {
        macos: { target_os = "macos" },
        ios: { target_os = "ios" },

        apple_os: { any(macos, ios) }
    }
}
