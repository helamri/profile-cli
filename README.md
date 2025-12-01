# Profile-CLI

A lightweight, cross-platform command-line tool built in **Rust** to manage local user profiles using a fast **binary database**.

This project works on **Windows**, **Linux**, and **macOS** without any changes.

---

## ✨ Features

- Add profiles (name + age)
- List all saved profiles
- Delete a profile by index
- Reset the entire database
- Uses a compact, fast binary file (`profiles.db`)
- Works on every major OS (Win/Linux/macOS)
- Clean and extensible Rust codebase
- `--help` explanation for every command

---

## 📦 Installation

### 1. Clone the repository
```bash
git clone https://github.com/YOUR_USERNAME/profile-cli
cd profile-cli
```

### 2. Build the project
```bash
cargo build --release
```

The compiled binary will be located in:
```
target/release/profile-cli
```

---

## 🚀 Usage

### Show help
```bash
profile-cli --help
```

### Add a new profile
```bash
profile-cli add "Hamza" 23
```

### List profiles
```bash
profile-cli list
```

### Delete a profile
```bash
profile-cli delete 1
```

### Reset (delete all profiles)
```bash
profile-cli reset
```

---

## 📁 Database

All profiles are stored in a binary file:

```
profiles.db
```

It is automatically created in the same directory where the program is executed.

---

## 🛠 Build for Other Platforms

### Windows (from Linux/macOS)
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

### Linux
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

### macOS
```bash
cargo build --release --target aarch64-apple-darwin
```

(You may need cross-compilers depending on your OS.)

---

## 📚 Technologies Used

- **Rust** (2021 Edition)
- **clap** for CLI parsing
- **serde** for serialization
- **bincode** for binary encoding

---

## 🤝 Contributing

Pull requests are welcome.  
For major changes, please open an issue first to discuss what you would like to improve.

---

## 📜 License

MIT License © 2025 YOUR_NAME
