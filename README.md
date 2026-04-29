# 🚀 Dioxus GitHub Explorer

This project demonstrates GitHub repository explorer written in Dioxus patterns including asynchronous data fetching, reactive state management, and a premium UI using Tailwind CSS.

## 🛠 Tech Stack

- **Core**: [Dioxus 0.7](https://dioxuslabs.com)
- **Styling**: [Tailwind CSS](https://tailwindcss.com) & Vanilla CSS
- **Networking**: [reqwest](https://docs.rs/reqwest) with JSON support
- **Serialization**: [Serde](https://serde.rs)

## 📁 Project Structure

```text
u2020dx/
├─ assets/          # Static assets (icons, global styles)
│  ├─ main.css      # Custom styling & dark mode overrides
│  └─ tailwind.css  # Generated Tailwind styles
├─ src/
│  ├─ main.rs       # App entry point & UI components
│  └─ models.rs     # Data structures & API integration logic
├─ Cargo.toml       # Project dependencies
└─ dioxus.toml      # Dioxus CLI configuration
```

## 🚀 Getting Started

### Prerequisites

1.  **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
2.  **Dioxus CLI**:
    ```bash
    cargo install dioxus-cli
    ```

### Run Locally

To start the development server for your default platform:

```bash
dx serve
```

For specific platforms:

```bash


# Mobile (iOS)
dx serve --ios

# Mobile (Android)
dx serve --android


### Data Fetching
The project uses a clean separation between data models and UI. API calls are located in `src/models.rs` and consumed via the `use_resource` hook in `src/main.rs`:

### Styling
Styles are managed through a combination of Tailwind utility classes for layout and `assets/main.css` for global theme consistency.

