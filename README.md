```
███████╗███████╗ ██████╗██╗   ██╗██████╗ ███████╗
██╔════╝██╔════╝██╔════╝██║   ██║██╔══██╗██╔════╝
███████╗█████╗  ██║     ██║   ██║██████╔╝█████╗  
╚════██║██╔══╝  ██║     ██║   ██║██╔══██╗██╔══╝  
███████║███████╗╚██████╗╚██████╔╝██║  ██║███████╗
╚══════╝╚══════╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝

██╗   ██╗ █████╗ ██╗   ██╗██╗  ████████╗
██║   ██║██╔══██╗██║   ██║██║  ╚══██╔══╝
██║   ██║███████║██║   ██║██║     ██║   
╚██╗ ██╔╝██╔══██║██║   ██║██║     ██║   
 ╚████╔╝ ██║  ██║╚██████╔╝███████╗██║   
  ╚═══╝  ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝   
```

> **A local-first password manager built with Rust and Iced. AES-256-GCM encryption, Argon2 key derivation, zero-knowledge architecture.**

Your master password never touches disk. Credentials are encrypted locally, decrypted in memory, and never leave your machine. No cloud, no sync, no third-party — just cryptography.

---

## Context

Most password managers either live in the cloud (trust someone else's infra) or ship as bloated Electron apps burning 300MB of RAM to store 20 passwords. **Secure Vault** is a native Rust binary with a clean GUI — AES-256-GCM for data encryption, Argon2 for key derivation, and the Iced framework for rendering. Everything runs locally, everything is encrypted at rest.

---

## Security Architecture

```
Master Password + Random Salt ──► [ ARGON2 ] ──► 256-bit Encryption Key
                                                          │
                  Credentials (JSON) ──► [ AES-256-GCM ] ◄╯
                                                │
                                          secure.vault ◄──── Salt (16 bytes) + Ciphertext
```

- **AES-256-GCM** — authenticated encryption for all stored data. Tampering is detected.
- **Argon2** — memory-hard password hashing. Resistant to GPU/ASIC brute-force.
- **CSPRNG** — cryptographically secure random generation for salts and password generation.
- **Zero-knowledge** — the master password is never stored, only used to derive the encryption key at runtime.

### Storage Format

The `secure.vault` file is a raw binary blob: the first 16 bytes are the random salt used for Argon2 key derivation, and the remaining bytes are the AES-256-GCM ciphertext containing serialized JSON credential data.

---

## Features

- **Real-time search** across all stored services
- **Password generator** with configurable length (8–64 characters)
- **Visibility toggle** — show/hide passwords inline
- **One-click copy** to clipboard
- **Multiple built-in themes** for UI customization
- **Native performance** — no Electron, no WebView, minimal resource footprint

---

## Build & Run

```bash
# Clone
git clone https://github.com/Skeezko/Secure-Vault.git
cd Secure-Vault

# Build
cargo build --release

# Run
cargo run --release
```

**Requirements**: Rust 1.70+, Cargo.

---

## Usage

### First Launch

Launch the binary, set a strong master password (12+ characters, mixed case, numbers, symbols), and click "UNLOCK SYSTEM". This creates your encrypted vault.

### Managing Credentials

- **Add**: fill in Service / Username / Password at the bottom of the UI, submit.
- **Generate**: use the slider to set length, click "Gen" for a cryptographically random password.
- **Edit**: select a service in the sidebar, modify fields, click "Update".
- **Delete**: select a service, click "Delete Service".
- **Search**: real-time filtering via the search bar.
- **Copy**: click "Copy" next to any password field.

---

## Project Structure

```
.
├── src/
│   ├── main.rs        # Application entry point + UI logic (Iced)
│   ├── crypto.rs      # AES-256-GCM encryption / Argon2 key derivation
│   ├── storage.rs     # File persistence layer
│   └── models.rs      # Data structures
├── Cargo.toml         # Dependencies and project configuration
├── LICENSE
└── README.md
```

---

## Dependencies

| Crate | Role |
|-------|------|
| `iced` | Cross-platform native GUI framework |
| `aes-gcm` | AES-256-GCM authenticated encryption |
| `argon2` | Memory-hard password hashing / key derivation |
| `serde` / `serde_json` | Credential serialization |
| `rand` | Cryptographically secure RNG |

---

## Security Best Practices

- Use a master password of **12+ characters** with mixed case, digits, and symbols.
- Never share or reuse your master password.
- Keep encrypted backups of `secure.vault`.
- This application is only as secure as the device it runs on — lock your OS.

---

## License

MIT — see `LICENSE`.

---

## Disclaimer

Provided "as is" without warranty. While security best practices are followed, use at your own risk. Maintain backups of your encrypted data.

---

## Acknowledgments

Built with [Iced](https://github.com/iced-rs/iced), encryption by [RustCrypto](https://github.com/RustCrypto), password hashing by [Argon2](https://github.com/RustCrypto/password-hashes).
