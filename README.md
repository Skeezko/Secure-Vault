<div align="center">

  <h1>🔐 Secure Vault - Password Manager</h1>
  
  <p>
    <strong>A modern, secure password manager built with Rust and Iced GUI framework. Store and manage your credentials with military-grade encryption.</strong>
  </p>

  <p>
    <a href="https://www.rust-lang.org/">
      <img src="https://img.shields.io/badge/Made_with-Rust-orange?logo=rust" alt="Rust">
    </a>
    <a href="https://github.com/iced-rs/iced">
      <img src="https://img.shields.io/badge/GUI-Iced_0.14-blue?logo=icloud" alt="Iced">
    </a>
    <a href="#">
      <img src="https://img.shields.io/badge/License-MIT-green" alt="License">
    </a>
  </p>
  
  <br />
</div>

## ✨ Features

- **🔒 Military-Grade Encryption**: AES-256-GCM encryption with Argon2 key derivation
- **🎨 Modern UI**: Clean, intuitive interface built with Iced framework
- **🔍 Quick Search**: Instantly find your credentials with real-time search
- **🎲 Password Generator**: Create strong, random passwords with customizable length (8-64 characters)
- **👁️ Password Visibility Toggle**: Show/hide passwords as needed
- **📋 One-Click Copy**: Copy passwords to clipboard instantly
- **🎭 Theme Support**: Multiple built-in themes to customize your experience
- **💾 Persistent Storage**: Encrypted local storage of all credentials
- **🚀 Fast & Lightweight**: Native performance with minimal resource usage

## 🔐 Security

This password manager implements industry-standard security practices:

- **AES-256-GCM** encryption for all stored data
- **Argon2** password hashing for master password derivation
- **Cryptographically secure** random number generation for salts and passwords
- **Zero-knowledge architecture**: Your master password is never stored
- **Local storage only**: Your data never leaves your device

## 📋 Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

## 🚀 Installation

1. Clone the repository:
```bash
git clone https://github.com/Skeezko/Secure-Vault.git
cd Secure-Vault
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

## 💻 Usage

### First Time Setup

1. Launch the application
2. Enter a strong master password (this encrypts all your data)
3. Click "UNLOCK SYSTEM" to create your vault

### Managing Passwords

- **Add New Entry**: Fill in the form at the bottom (Service, Username, Password)
- **Generate Password**: Use the slider to set length, then click "Gen"
- **Edit Entry**: Click on a service in the sidebar, modify fields, and click "Update"
- **Delete Entry**: Select a service and click "Delete Service"
- **Search**: Use the search bar to quickly filter services
- **Copy Password**: Click the "Copy" button next to any password

## 🏗️ Project Structure

```
├── src/
│   ├── main.rs          # Application entry point and UI logic
│   ├── crypto.rs        # Encryption/decryption manager
│   ├── storage.rs       # File persistence layer
│   └── models.rs        # Data structures
├── Cargo.toml           # Dependencies and project configuration
├── README.md
└── LICENSE
```

## 🔧 Technical Details

### Dependencies

- **iced**: Modern GUI framework for Rust
- **aes-gcm**: AES-256-GCM encryption implementation
- **argon2**: Password hashing and key derivation
- **serde/serde_json**: Serialization for data persistence
- **rand**: Cryptographically secure random number generation

### Data Storage

Credentials are stored in an encrypted file (`credentials.encrypted`) with the following structure:
- First 16 bytes: Random salt for key derivation
- Remaining bytes: AES-256-GCM encrypted JSON data

### Encryption Flow

1. Master password + salt → Argon2 → 256-bit encryption key
2. Credentials → JSON serialization → AES-256-GCM encryption
3. Salt + encrypted data → saved to disk

## 🛡️ Security Best Practices

- **Choose a strong master password**: Use at least 12 characters with mixed case, numbers, and symbols
- **Never share your master password**: This is the only key to decrypt your vault
- **Regular backups**: Keep encrypted backups of your `credentials.encrypted` file
- **Secure your device**: This application is only as secure as the device it runs on


## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## ⚠️ Disclaimer

This software is provided "as is" without warranty of any kind. While every effort has been made to ensure security, use at your own risk. Always maintain backups of your encrypted data.

## 🙏 Acknowledgments

- Built with [Iced](https://github.com/iced-rs/iced) - A cross-platform GUI library for Rust
- Encryption powered by [RustCrypto](https://github.com/RustCrypto)
- Password hashing by [Argon2](https://github.com/RustCrypto/password-hashes)
