
# Brown Dust 2 Mod Manager (EditVer)

[![GitHub License](https://img.shields.io/github/license/CalbeeDaLock/BD2ModManager-Edit)](https://github.com/CalbeeDaLock/BD2ModManager-Edit/blob/main/LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/CalbeeDaLock/BD2ModManager-Edit)](https://github.com/CalbeeDaLock/BD2ModManager-Edit/releases)

> 🛠 An edited version of the Brown Dust 2 Mod Manager to manage, preview, and sync mods for Brown Dust 2.
> 
> 🎉 **Download the latest version:** [GitHub Releases](https://github.com/CalbeeDaLock/BD2ModManager-Edit/releases/latest)
>

If you have suggestions or run into any problems with the app, feel free to open an issue.

> [!WARNING]
> **Before Uninstalling Brown Dust II !!!**
>
> If you use the **Symlink** sync method, unsync all mods before uninstalling the game. Otherwise, the uninstaller may delete your mods from the staging folder.

---

## 🌟 What's Different in EditVer?

This version includes several enhancements and features not present in the original [bruhnn/BD2ModManager](https://github.com/bruhnn/BD2ModManager) v4.1.1:

### 👥 NPC Page
- **Full NPC Implementation**: Replaced the placeholder NPC stub with a fully functional page.
- **NPC Management**: Dedicated grid view for NPCs with search, sorting, and filtering.
- **NPC Details**: Added `NpcModal` to list and manage mods for specific NPCs.
- **Custom Assets**: Integrated specific NPC icons and name overrides (e.g., Shop Girl, Eleanor).

### 📦 Mods Tab Improvements
- **Error Filtering**: Added a "Not show Error Mods" toggle to hide mods with errors.
- **Selection Logic**: Added an "Only select one mod type" tickbox for faster filtering.
- **UX Polish**: Added the ability to deselect a mod row by clicking on empty space.
- **Preview Tip**: Added a context-menu reminder that double-clicking previews a mod.

### 🎭 Characters Tab Enhancements
- **Collab Filter**: Added an "Only Collab Characters" filter to easily find collaboration characters.
- **Path Sorting**: Character mods are now sorted A-Z by their folder path, with the path displayed as a subtitle for better organization.

### 🌐 Localization
- All the above features have been translated across all supported locales (English, Japanese, Korean, Portuguese, Chinese Simplified, Chinese Traditional).

---

## 🛠️ How to Use

1. **Download** the app from [GitHub Releases](https://github.com/CalbeeDaLock/BD2ModManager-Edit/releases).
2. **Select your Brown Dust 2 directory** (where `BrownDust II.exe` is located)
   - Example: F:\Neowiz\Browndust2\Browndust2_10000001
3. **Install BepInEx and BrownDustX**
   - Download both from the BrownDustX Discord: [discord.gg/B3Aqz6tDG2](https://discord.gg/B3Aqz6tDG2)
   - **Install BepInEx**: Extract the contents into the **game folder** (NOT the launcher folder).
   - **Install BrownDustX**: Extract the `BepInEx` folder from `BrownDustX-[VERSION].zip` into the same game folder.
4. **Verify the installation**
   - Launch the game. On the loading screen, the game version and BrownDustX version should appear in red at the top-right corner.
5. **Add your mods** by:
   - Moving them into the `mods/` folder  
     ⚠️ **Note:** This is *not* the BrownDustX `mods` directory. It's a separate folder used by this manager
6. **Enable or disable mods**.
7. **Sync your mods** to apply changes:
   - This will create a folder named `BD2MM` inside the `BrownDustX` mods folder with all your enabled mods.

> ⚠️ After making any changes (enable, disable, delete, rename), you **must sync** your mods to update the game folder.

### Sync Method: Copy vs Symlink

Choose how mods are synced to your BrownDust X `mods` folder:

#### 📁 Copy
Copies all enabled mods into the folder.  

- ✅ No admin rights needed
- ❌ Slower
- ❌ Uses more disk space

#### 🔗 Symlink  
Creates shortcuts instead of copying files.  

- ✅ Much faster
- ✅ Saves disk space
- ❌ Requires admin rights

#### Example Comparison (383 mods)

| Copy | Symlink |
|------|---------|
| <img src="./screenshots/sync_copy_v405.gif" width="260" /> | <img src="./screenshots/sync_symlink_v405.gif" width="260" /> |

---

## 📸 Screenshots

### Mods Page (v4.0.1)
![Mods Page](./screenshots/mods_page_v401.png)

### Characters Page (v4.0.1)
![Characters Page](./screenshots/characters_page_v401.png)

### Mod Preview (BD2ModPreview)
![Mod Preview](./screenshots/bd2modpreview.png)

---

## 🤝 Credits & Thanks

This project is an edited version of the original **Brown Dust 2 Mod Manager**.

- **Original Creator:** [bruhnn](https://github.com/bruhnn/) - Thank you for the amazing work and the original [BD2ModManager](https://github.com/bruhnn/BD2ModManager).
- **Character assets:** Provided by [myssal/Brown-Dust-2-Asset](https://github.com/myssal/Brown-Dust-2-Asset).
- **Special Thanks:** To **Synae** for *BrownDustX* and everyone who contributed to the original project.

---

## ❓ FAQ

### Where can I get mods?
You can find mods on the BrownDustX Discord server: [https://discord.gg/B3Aqz6tDG2](https://discord.gg/B3Aqz6tDG2)
