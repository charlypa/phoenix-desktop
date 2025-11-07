# Tauri 2.0 Migration Checklist for Phoenix Desktop

## Migration Status: ✅ FEASIBLE - All prerequisites met

**Estimated Effort:** 2-3 days
**Risk Level:** Low-Medium
**Last Updated:** 2025-11-07

---

## Prerequisites Verification

### ✅ System Requirements

- [ ] **Node.js Version:** v20+ (Current: v20 ✓)
- [ ] **Rust Version:** 1.70+ (Current: 1.70 ✓)
- [ ] **Git Status:** Clean working directory
- [ ] **Backup:** Create backup or ensure code is committed

### ✅ Compatibility Assessment

| Component | Current Version | Target Version | Status | Notes |
|-----------|----------------|----------------|--------|-------|
| tauri | 1.6.2 | 2.x | ✅ Compatible | Breaking changes documented |
| tauri-build | 1.5.1 | 2.x | ✅ Compatible | No changes needed |
| @tauri-apps/cli | 1.6.3 | 2.x | ✅ Compatible | Auto-migration tool available |
| tauri-plugin-window-state | v1 | 2.0.2 | ✅ Compatible | Official v2 available |
| tauri-plugin-single-instance | v1 | 2.0.1 | ✅ Compatible | Official v2 available |
| tauri-plugin-deep-link | 0.1.2 | 2.0.1 | ✅ Compatible | Official v2 available |
| tauri-plugin-fs-extra | v1 | N/A | ⚠️ **BLOCKER** | No official v2 - see alternatives |

---

## Phase 1: Pre-Migration Setup

### 1.1 Create Migration Branch
```bash
git checkout -b tauri-v2-migration
git push -u origin tauri-v2-migration
```

### 1.2 Document Current State
- [ ] Run and document current build: `npm run releaseDist`
- [ ] Test all major features and document functionality
- [ ] Screenshot or record working application
- [ ] Export list of all npm and cargo dependencies:
  ```bash
  npm list --depth=0 > pre-migration-npm-deps.txt
  cd src-tauri && cargo tree --depth=1 > ../pre-migration-cargo-deps.txt
  ```

### 1.3 Resolve fs-extra Plugin Issue

**Decision Required:** Choose one of the following approaches:

**Option A: Use tauri-plugin-fs-pro (Recommended)**
- [ ] Research tauri-plugin-fs-pro API compatibility
- [ ] Test that it provides needed functionality
- [ ] Document any API differences

**Option B: Remove fs-extra dependency**
- [ ] Audit code to confirm fs-extra is only used for:
  - [ ] Plugin registration (line 609 in main.rs)
  - [ ] No actual function calls found
- [ ] Check tauri.conf.json allowlist references (lines 296, 369)
- [ ] Test if functionality works without the plugin

**Option C: Wait for official v2 plugin**
- [ ] Monitor https://github.com/tauri-apps/plugins-workspace
- [ ] Delay migration until official v2 release

**Selected Option:** _______________

---

## Phase 2: Dependency Updates

### 2.1 Update Rust Dependencies (src-tauri/Cargo.toml)

**Current:**
```toml
[dependencies]
tauri = { version = "1.6.2", features = [ "updater", "cli", "api-all", "devtools", "linux-protocol-headers"] }
tauri-plugin-fs-extra = { git = "https://github.com/tauri-apps/plugins-workspace", branch = "v1" }
tauri-plugin-window-state = { git = "https://github.com/tauri-apps/plugins-workspace", branch = "v1" }
tauri-plugin-single-instance = { git = "https://github.com/tauri-apps/plugins-workspace", branch = "v1" }
tauri-plugin-deep-link = "0.1.2"

[build-dependencies]
tauri-build = { version = "1.5.1", features = [] }
```

**Target:**
```toml
[dependencies]
tauri = { version = "2", features = [ "devtools", "linux-protocol-headers"] }
# NOTE: updater, cli, api-all removed - now plugins
tauri-plugin-fs-pro = "2.0"  # OR remove if not needed
tauri-plugin-window-state = "2.0"
tauri-plugin-single-instance = "2.0"
tauri-plugin-deep-link = "2.0"

[build-dependencies]
tauri-build = { version = "2", features = [] }
```

**Checklist:**
- [ ] Update tauri to v2
- [ ] Update tauri-build to v2
- [ ] Update tauri-plugin-window-state to v2
- [ ] Update tauri-plugin-single-instance to v2
- [ ] Update tauri-plugin-deep-link to v2
- [ ] Remove or replace tauri-plugin-fs-extra
- [ ] Run `cargo update` to update Cargo.lock

### 2.2 Update Node.js Dependencies (package.json)

**Current:**
```json
"devDependencies": {
    "@tauri-apps/cli": "1.6.3",
    "fs-extra": "11.3.0"
}
```

**Target:**
```json
"devDependencies": {
    "@tauri-apps/cli": "2.x",
    "fs-extra": "11.3.0"  // Keep for build scripts - see Phase 3.4
}
```

**Checklist:**
- [ ] Update @tauri-apps/cli to v2: `npm install -D @tauri-apps/cli@latest`
- [ ] Run `npm update` to update other dependencies
- [ ] Verify package-lock.json is updated

### 2.3 Run Automated Migration Tool

```bash
npm run tauri migrate
```

**What this does:**
- Automatically updates most configuration files
- Generates capability files from v1 allowlist
- Migrates tauri.conf.json structure
- **NOTE:** Does NOT migrate Rust code (manual changes required)

**Checklist:**
- [ ] Run migration command
- [ ] Review generated files in `src-tauri/capabilities/`
- [ ] Review changes to `src-tauri/tauri.conf.json`
- [ ] Commit migration tool changes before manual edits

---

## Phase 3: Code Changes

### 3.1 Update Custom Protocol Handler (src-tauri/src/main.rs:564-608)

**Breaking Change:** Return type changed from `Result<Response>` to `Response`

**Current Code:**
```rust
.register_uri_scheme_protocol("phtauri", move |app, request| {
    if path.is_none() {
        let not_found_response = ResponseBuilder::new()
            .status(404)
            .mimetype("text/html")
            .body("Asset not found".as_bytes().to_vec())
            .unwrap();
        return Ok(not_found_response);  // ← Remove Ok() wrapper
    }
    // ...
    let response = builder.body(asset.bytes)?;
    Ok(response)  // ← Remove Ok() wrapper
})
```

**Updated Code:**
```rust
.register_uri_scheme_protocol("phtauri", move |app, request| {
    if path.is_none() {
        return ResponseBuilder::new()
            .status(404)
            .mimetype("text/html")
            .body("Asset not found".as_bytes().to_vec())
            .unwrap();
    }
    // ...
    let response = builder.body(asset.bytes).unwrap();
    response  // ← No Ok() wrapper
})
```

**Checklist:**
- [ ] Remove `Ok()` wrapper from line 573
- [ ] Remove `Ok()` wrapper from line 590
- [ ] Change line 606 from `let response = builder.body(asset.bytes)?;` to `.unwrap()`
- [ ] Remove `Ok()` wrapper from line 607
- [ ] Update function signature if needed (remove Result)

**Files to modify:**
- `src-tauri/src/main.rs` (lines 564-608)

### 3.2 Update Path API Usage (src-tauri/src/init.rs)

**Breaking Change:** `tauri::api::path::*` moved to `app.path().*`

**Current Code (line 8-12):**
```rust
pub fn init_app(app: &mut tauri::App) {
    let config = app.config().clone();
    println!("Appdata path is {}",  tauri::api::path::app_local_data_dir(&config).expect("failed to retrieve app_local_data_dir").display());
    ensure_dir_exists(&tauri::api::path::app_local_data_dir(&config).unwrap());
    let _ = APP_CONSTANTS.set(AppConstants {
        tauri_config: config.clone(),
        app_local_data_dir: tauri::api::path::app_local_data_dir(&config).expect("failed to retrieve app_local_data_dir")
            .canonicalize().expect("Failed to canonicalize app_local_data_dir")
    });
}
```

**Updated Code:**
```rust
pub fn init_app(app: &mut tauri::App) {
    let app_handle = app.handle();
    let config = app.config().clone();

    println!("Appdata path is {}", app_handle.path().app_local_data_dir().expect("failed to retrieve app_local_data_dir").display());
    ensure_dir_exists(&app_handle.path().app_local_data_dir().unwrap());

    let _ = APP_CONSTANTS.set(AppConstants {
        tauri_config: config.clone(),
        app_local_data_dir: app_handle.path().app_local_data_dir()
            .expect("failed to retrieve app_local_data_dir")
            .canonicalize()
            .expect("Failed to canonicalize app_local_data_dir")
    });
}
```

**Checklist:**
- [ ] Replace all `tauri::api::path::app_local_data_dir(&config)` calls
- [ ] Use `app.handle().path().app_local_data_dir()` instead
- [ ] Test that paths resolve correctly on all platforms

**Files to modify:**
- `src-tauri/src/init.rs` (lines 8, 9, 12)

### 3.3 Update Plugin Registration Order (src-tauri/src/main.rs)

**Critical:** single-instance plugin MUST be registered first

**Current Code (around line 609):**
```rust
.plugin(tauri_plugin_fs_extra::init())
.plugin(tauri_plugin_window_state::init())
.plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
    // ...
}))
```

**Updated Code:**
```rust
.plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
    // single-instance callback
    let _ = app.get_webview_window("main")
        .expect("no main window")
        .set_focus();
}))
.plugin(tauri_plugin_window_state::init())
// .plugin(tauri_plugin_fs_pro::init())  // If using fs-pro
```

**Checklist:**
- [ ] Move single-instance plugin to FIRST position
- [ ] Remove or replace fs-extra plugin
- [ ] Add window focus logic to single-instance callback
- [ ] Verify deep-link plugin registration (macOS)

**Files to modify:**
- `src-tauri/src/main.rs` (lines 609-615)

### 3.4 Update Window Event Hook (src-tauri/src/main.rs:616)

**Current Code:**
```rust
.on_window_event(|event| {
    let app_handle = event.window().app_handle();
    let trust_state = app_handle.state::<WindowAesTrust>();
    process_window_event(&event, &trust_state);
})
```

**Action:** ✅ This code should work in Tauri 2.0 with minimal/no changes
- May need to change `window()` to `webview_window()` depending on new API

**Checklist:**
- [ ] Test that window event handler still fires
- [ ] Verify window close events work correctly
- [ ] Check if `event.window()` needs to change to `event.webview_window()`
- [ ] Test AES trust cleanup on window close

**Files to modify:**
- `src-tauri/src/main.rs` (line 616)
- `src-tauri/src/main.rs` (function at lines 342-355)

### 3.5 Review Custom Commands for Breaking Changes

All `#[tauri::command]` functions were audited. Most should work without changes.

**Potential Issues:**
- [ ] Commands using `tauri::Window` may need to use `tauri::WebviewWindow`
- [ ] Commands using `tauri::AppHandle` - verify API compatibility
- [ ] Test `toggle_devtools()` - verify unsafe block still works
- [ ] Test `zoom_window()` - verify webview access works

**Files to check:**
- `src-tauri/src/main.rs` (lines 70-427 - all command functions)

### 3.6 Remove api-all Feature Flag References

**Breaking Change:** The `api-all` feature no longer exists

**Checklist:**
- [ ] Remove `api-all` from Cargo.toml features (already in 2.1)
- [ ] Remove `updater` and `cli` features if present
- [ ] Add specific feature flags if needed

---

## Phase 4: Configuration Changes

### 4.1 Restructure tauri.conf.json

**Breaking Changes:**
1. Rename `tauri` → `app`
2. Add required `mainBinaryName` field
3. Move `productName` and `version` to top level
4. Migrate `allowlist` to `capabilities` (auto-generated by migrate tool)
5. Rename `systemTray` → `trayIcon`
6. Update asset protocol scope path

**Current Structure:**
```json
{
  "package": {
    "productName": "Phoenix Code Experimental Build",
    "version": "4.1.1"
  },
  "tauri": {
    "allowlist": { ... },
    "security": {
      "assetProtocol": { "scope": [...] }
    }
  }
}
```

**Target Structure:**
```json
{
  "productName": "Phoenix Code Experimental Build",
  "version": "4.1.1",
  "app": {
    "mainBinaryName": "phoenix-code-ide",
    "security": {
      "assetProtocol": {
        "scope": [...]
      }
    },
    "trayIcon": { ... }
  }
}
```

**Checklist:**
- [ ] Backup original tauri.conf.json
- [ ] Let migrate tool handle initial conversion
- [ ] Manually verify all fields migrated correctly
- [ ] Add `mainBinaryName` if not auto-added
- [ ] Check custom protocol configuration (`phtauri`)
- [ ] Verify shell scope for `phnode` sidecar
- [ ] Test dangerous remote domain IPC access settings

**Files to modify:**
- `src-tauri/tauri.conf.json`

### 4.2 Review Generated Capabilities Files

**Location:** `src-tauri/capabilities/`

The migrate tool will generate capability files from your v1 allowlist.

**Checklist:**
- [ ] Review generated capabilities for correctness
- [ ] Ensure all required permissions are granted:
  - [ ] File system access (all directories in allowlist)
  - [ ] Custom protocol access
  - [ ] Shell commands (phnode sidecar)
  - [ ] Window management
- [ ] Verify window-specific capabilities
- [ ] Test remote domain IPC permissions for healthData window

**Example Capability File:**
```json
{
  "identifier": "main-capability",
  "description": "Main window permissions",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "fs:default",
    "shell:execute",
    "window:default"
  ]
}
```

**Files to review:**
- All files in `src-tauri/capabilities/*.json`

### 4.3 Update Windows Custom Protocol (Windows only)

**Breaking Change:** Windows now uses `http` scheme instead of `https`

**Current (line 596-598 in main.rs):**
```rust
#[cfg(windows)]
let window_origin = "https://phtauri.localhost";
#[cfg(not(windows))]
let window_origin = "phtauri://localhost";
```

**Action:** May need to change to `http://phtauri.localhost` for Windows

**Checklist:**
- [ ] Test on Windows if origin needs updating
- [ ] Update tauri.conf.json if URL scheme changed
- [ ] Test asset loading on Windows
- [ ] Verify CORS headers still work

---

## Phase 5: Build Script Updates

### 5.1 Update Dynamic Config Generation Scripts

All build scripts in `src-build/` that modify `tauri.conf.json` must be updated for new structure.

**Files to modify:**
- [ ] `src-build/createDistReleaseConfig.js`
- [ ] `src-build/createSrcReleaseConfig.js`
- [ ] `src-build/createDistBundleReleaseConfig.js`
- [ ] `src-build/createDistTestReleaseConfig.js`
- [ ] `src-build/ci-createDistReleaseConfig.js`

**Changes needed in each file:**
```javascript
// OLD
configJson.tauri.bundle.active = false;
configJson.tauri.windows[0].url = "...";

// NEW
configJson.app.bundle.active = false;
configJson.app.windows[0].url = "...";
```

**Checklist:**
- [ ] Replace `configJson.tauri` with `configJson.app`
- [ ] Verify all nested property paths are correct
- [ ] Test each build variant generates correct config
- [ ] Ensure mainBinaryName is not overwritten

### 5.2 Optional: Replace fs-extra in Build Scripts

**Current usage in build scripts:**
- `fsExtra.copy()` - Copy directories (downloadNodeBinary.js:256)
- `fsExtra.pathExists()` - Check if path exists (utils.js:23)
- `fsExtra.remove()` - Remove directories (utils.js:29)

**Option A: Keep fs-extra**
- ✅ No changes needed
- Build scripts can still use fs-extra even if Rust plugin is removed

**Option B: Use Node.js native APIs (requires Node 22.3.0+)**

Replace in `src-build/utils.js`:
```javascript
// OLD
import * as fsExtra from "fs-extra";

async function removeDir(dirPath) {
    const exists = await fsExtra.pathExists(dirPath);
    if (!exists) return 'Directory not found!';
    await fsExtra.remove(dirPath);
    return 'Directory removed!';
}

// NEW
import { rm, access, constants } from 'fs/promises';

async function removeDir(dirPath) {
    try {
        await access(dirPath, constants.F_OK);
    } catch {
        return 'Directory not found!';
    }
    await rm(dirPath, { recursive: true, force: true });
    return 'Directory removed!';
}
```

Replace in `src-build/downloadNodeBinary.js`:
```javascript
// OLD
await fsExtra.copy(source, destination);

// NEW
import { cp } from 'fs/promises';
await cp(source, destination, { recursive: true });
```

**Checklist (if doing Option B):**
- [ ] Verify Node.js version is 22.3.0+
- [ ] Update utils.js
- [ ] Update downloadNodeBinary.js
- [ ] Test all build scripts
- [ ] Remove fs-extra from package.json

---

## Phase 6: CI/CD Updates

### 6.1 Update GitHub Actions Workflows

**Files to update:**
- [ ] `.github/workflows/tauri-build-dev.yml`
- [ ] `.github/workflows/tauri-build-staging.yml`
- [ ] `.github/workflows/tauri-build-prod.yml`
- [ ] All test workflow files

**Changes needed:**

**1. Update Rust Toolchain Setup:**
```yaml
# OLD
- name: install Rust stable
  uses: actions-rs/toolchain@v1
  with:
    toolchain: stable

# NEW
- name: install Rust stable
  uses: dtolnay/rust-toolchain@stable
```

**2. Add Rust Cache:**
```yaml
- name: Rust cache
  uses: swatinem/rust-cache@v2
  with:
    workspaces: './src-tauri -> target'
```

**3. Update Ubuntu Version (if needed):**
```yaml
# Consider updating from ubuntu-20.04 to ubuntu-22.04
runs-on: ubuntu-22.04
```

**4. Update Actions Versions:**
```yaml
- uses: actions/checkout@v4  # was v3
- uses: actions/setup-node@v4  # was v3
- uses: actions/upload-artifact@v4  # was v3
- uses: actions/download-artifact@v4  # was v3
```

**Checklist:**
- [ ] Update all workflow files with new action versions
- [ ] Add rust-cache to improve build times
- [ ] Test workflows on all platforms (mac, windows, linux)
- [ ] Verify artifact uploads still work
- [ ] Test release creation and asset uploads

### 6.2 Test Build Matrix

**Platforms to test:**
- [ ] macOS 13 (Intel)
- [ ] macOS 14 (M1/ARM)
- [ ] Windows latest
- [ ] Linux (ubuntu-22.04)

**Build variants to test:**
- [ ] Dev build (`npm run releaseDist`)
- [ ] Bundle build (`npm run releaseDistBundle`)
- [ ] Test build (`npm run releaseDistTest`)
- [ ] Source build (`npm run releaseSrc`)

---

## Phase 7: Testing & Validation

### 7.1 Local Build Testing

**Checklist:**
- [ ] Clean build succeeds: `cargo clean && npm run releaseDist`
- [ ] Dev mode works: `npm run serve`
- [ ] Hot reload works in dev mode
- [ ] No Rust compilation errors
- [ ] No TypeScript/JavaScript errors

### 7.2 Feature Testing

**Core Functionality:**
- [ ] Application launches successfully
- [ ] Custom protocol loads assets (`phtauri://localhost/...`)
- [ ] Window state persists across restarts
- [ ] Single instance prevention works
- [ ] Deep linking works (macOS)

**Custom Commands:**
- [ ] `console_log` and `console_error` work
- [ ] `get_credential` / `store_credential` work
- [ ] `move_to_trash` works
- [ ] `show_in_folder` works
- [ ] `zoom_window` works
- [ ] `toggle_devtools` works
- [ ] AES window trust system works
- [ ] In-memory storage (put_item/get_item) works

**Platform-Specific:**
- [ ] Windows: Drives detection works
- [ ] macOS: Deep link requests work
- [ ] Linux: Keyboard handling works
- [ ] Node.js sidecar (`phnode`) launches correctly

**Security & State:**
- [ ] Credentials stored securely in system keyring
- [ ] Window AES trust cleaned up on close
- [ ] Boot config persists correctly
- [ ] App local data directory created correctly

### 7.3 Error Handling

**Checklist:**
- [ ] Panic hook shows error dialog
- [ ] Bugsnag reporting works (if user opts in)
- [ ] Custom protocol returns 404 for missing assets
- [ ] File operations handle errors gracefully

### 7.4 Performance Testing

**Checklist:**
- [ ] Application startup time acceptable
- [ ] Asset loading speed unchanged or improved
- [ ] Memory usage comparable to v1
- [ ] No memory leaks in long-running sessions

### 7.5 Platform-Specific Testing

**Windows:**
- [ ] WebView2 works correctly
- [ ] Custom protocol uses http scheme
- [ ] File explorer integration works
- [ ] Windows updater commands work

**macOS:**
- [ ] Both Intel and ARM builds work
- [ ] Deep linking configured correctly
- [ ] Rosetta detection works (if applicable)
- [ ] App bundle signed correctly

**Linux:**
- [ ] GTK integration works
- [ ] AppImage works
- [ ] Update terminal commands work
- [ ] File permissions correct

---

## Phase 8: Deployment Preparation

### 8.1 Update Documentation

**Checklist:**
- [ ] Update README.md with new Tauri version
- [ ] Update build instructions if changed
- [ ] Document any new dependencies
- [ ] Update troubleshooting section
- [ ] Document migration notes for other developers

### 8.2 Version Bump

**Checklist:**
- [ ] Update version in package.json (if doing major release)
- [ ] Update version in Cargo.toml (if doing major release)
- [ ] Update version in tauri.conf.json
- [ ] Update CHANGELOG.md

### 8.3 Beta Release

**Checklist:**
- [ ] Create beta release branch
- [ ] Build installers for all platforms
- [ ] Test installers on clean systems
- [ ] Deploy to subset of beta users
- [ ] Monitor for issues
- [ ] Collect feedback

### 8.4 Production Rollout

**Checklist:**
- [ ] Fix all beta issues
- [ ] Merge to main branch
- [ ] Create release tag
- [ ] Build production installers
- [ ] Update auto-updater configuration
- [ ] Deploy to all users
- [ ] Monitor crash reports and metrics

---

## Rollback Plan

If migration fails or critical issues are found:

### Immediate Rollback
```bash
git checkout main
git branch -D tauri-v2-migration
# Deploy last known good build
```

### Partial Rollback
- Keep new branch for investigation
- Deploy v1 builds to users
- Debug issues in parallel

**Rollback Triggers:**
- Critical functionality broken
- Performance degradation >20%
- Platform-specific crashes
- Data loss or corruption
- Security vulnerabilities introduced

---

## Success Criteria

Migration is complete and successful when:

- [ ] All builds (dev, staging, prod) compile without errors
- [ ] All platforms (Windows, macOS Intel, macOS ARM, Linux) work
- [ ] All custom commands function correctly
- [ ] All custom hooks work as expected
- [ ] Performance is equal or better than v1
- [ ] No regressions in functionality
- [ ] Beta users report no critical issues
- [ ] CI/CD pipeline works for all variants
- [ ] Auto-updater works correctly

---

## Resources

### Official Documentation
- [Tauri 2.0 Migration Guide](https://v2.tauri.app/start/migrate/from-tauri-1/)
- [Tauri 2.0 Release Notes](https://v2.tauri.app/blog/tauri-20/)
- [Plugin Catalog](https://v2.tauri.app/plugin/)
- [Permissions System](https://v2.tauri.app/security/permissions/)

### Plugin Documentation
- [Window State Plugin](https://v2.tauri.app/plugin/window-state/)
- [Single Instance Plugin](https://v2.tauri.app/plugin/single-instance/)
- [Deep Link Plugin](https://v2.tauri.app/plugin/deep-linking/)

### Community Resources
- [Tauri Discord](https://discord.gg/tauri)
- [GitHub Discussions](https://github.com/tauri-apps/tauri/discussions)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/tauri)

---

## Notes & Decisions Log

| Date | Decision | Rationale | Made By |
|------|----------|-----------|---------|
| 2025-11-07 | fs-extra plugin: _____________ | (Document decision here) | _______ |
| | | | |
| | | | |

---

## Contact & Support

For questions or issues during migration:
1. Check official migration guide first
2. Search GitHub issues: https://github.com/tauri-apps/tauri/issues
3. Ask in Tauri Discord: https://discord.gg/tauri
4. Review this checklist and research findings

---

**Good luck with your migration! Remember to test thoroughly on all platforms.**
