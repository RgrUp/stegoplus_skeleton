## Build & Generate Installer (Windows)

These steps build the StegoPlus desktop app and generate the Windows installer locally.

### Prerequisites

Install the following:

1. **Git**
2. **Rust** (stable) via rustup
3. **Node.js (LTS)** + npm
4. **Tauri prerequisites for Windows**
   - Microsoft Visual Studio Build Tools (Desktop development with C++)
   - WebView2 Runtime (usually already installed on Windows 10/11)

> Note: The first Tauri build may download WiX toolset automatically for MSI packaging.

---

### 1) Clone the Repository

```bash
git clone <YOUR_REPO_URL_HERE>
cd STEGOPLUS_SKELETON_V1.0.0

### 2) Confirm Rust Workspace Builds (Optional Sanity Check)

cargo build

### 3) Build the CLI (Optional)

cargo build -p stegoplus_cli --release

### 4) Install Frontend Dependencies (Desktop App)

cd apps/stegoplus_desktop
npm install

### 5) Run Desktop App in Dev Mode

npm run tauri dev

### 6) Build Release + Generate Installer (MSI + NSIS)

npm run tauri build

### Troubleshooting ###

# Build fails due to missing Visual C++ tools

# Install Visual Studio Build Tools

# Ensure “Desktop development with C++” is selected

# WebView2 errors

# Install Microsoft Edge WebView2 Runtime

# WiX packaging issues

# Re-run the build; Tauri may download WiX automatically

# Ensure you have permissions to write into the build folders

## 1) **“Verify it worked”**

```markdown

### Verify the Installer

1. Run the `.msi`
2. Launch StegoPlus from the Start Menu/Desktop
3. Use the app to embed a short payload into a PNG and extract it back

## 2) “Release build artifacts”

Release artifacts are located in:
- bundle/msi/
- bundle/nsis/
