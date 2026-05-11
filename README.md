# Portfolio

A portfolio website built with Rust and WebAssembly.

## 🌐 Live Demo

Visit the live site: [https://eniyanyosuva.github.io/portfolio/](https://eniyanyosuva.github.io/portfolio/)

## 🛠️ Tech Stack

- **Rust** - Core programming language
- **WebAssembly** - For web deployment
- **Nix** - Development environment and build system
- **GitHub Pages** - Hosting
- **GitHub Actions** - CI/CD pipeline

## 📦 Project Structure

```
portfolio/
├── src/              # Source code
├── assets/           # Static assets (images, fonts, etc.)
├── .github/          # GitHub Actions workflows
│   └── workflows/    
│       ├── deploy.yml    # Deployment workflow
│       ├── check.yml     # Code checks
│       └── format.yml    # Code formatting
├── Cargo.toml        # Rust dependencies
├── flake.nix         # Nix configuration
└── README.md         # This file
```

## 🏗️ Development

### Prerequisites

- [Nix](https://nixos.org/download.html) package manager
- Git

### Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/Eniyanyosuva/portfolio.git
   cd portfolio
   ```

2. Enter the Nix development environment:
   ```bash
   nix develop
   ```

3. Build the project:
   ```bash
   nix build
   ```

4. The built site will be in the `./result` directory

## 📝 Customization

To customize this portfolio for your own use:

1. Update personal information in `src/` files
2. Replace assets in the `assets/` directory
3. Modify content in the relevant source files
4. Commit and push your changes

## 🚢 Deployment

The site automatically deploys to GitHub Pages when you push to the `main` branch.

### Manual Deployment

You can also trigger deployment manually:

1. Go to the [Actions tab](https://github.com/Eniyanyosuva/portfolio/actions)
2. Select "Deploy to GitHub Pages"
3. Click "Run workflow"

**Note:** This portfolio was built using rust and customized for personal use. 
