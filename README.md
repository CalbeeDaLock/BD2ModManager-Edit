
# Brown Dust 2 Mod Manager

[![GitHub License](https://img.shields.io/github/license/bruhnn/BD2ModManager)](https://github.com/bruhnn/BD2ModManager/blob/main/LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/bruhnn/BD2ModManager)](https://github.com/bruhnn/BD2ModManager/releases)
[![Total Downloads](https://img.shields.io/github/downloads/bruhnn/BD2ModManager/total)](https://github.com/bruhnn/BD2ModManager/releases)


> 🛠 Easily manage, preview, and sync mods for Brown Dust 2.
> 
> 🎉 **Download the latest version:** [GitHub Releases](https://github.com/bruhnn/BD2ModManager/releases/latest)
>

If you have suggestions or run into any problems with the app, feel free to open an issue or contact me.

---

## ✨ What's New in v4.0.0+

### General
- Complete UI overhaul
- Added `.rar` and `.7z` mod installation support
- Much smaller file size (from ~164MB → ~60MB)
- Much faster performance (sync, unsync, and other operations)

### Mods Tab
- Conflict indicator (flags mods that edit the same character/files)
- Mod type icons
- Character icons (head)
- Context menu: select all, enabled, or disabled mods

### Characters Tab
- Grid view
- Filters by name, mod types (cutscene, dating, standing, etc.)
- Sort by name, release date, or number of mods
- Filter by release period (last week, 2 weeks, month, etc.)
- View and enable/disable a character's mods directly

### BrownDustX Tab *(new)*
- Detect BepInEx and BrownDustX plugin installation
- Install BepInEx and BrownDustX directly from the app

### Settings Tab
- Auto sync mods on enable/disable
- Show/hide columns (mod name, character, mod type, author)
- Mod name display: name only or with subfolders
- Character column: icon only, name only, or both
- Mod type column: icon only, name only, or both
- Optional color coding for mod types


---

## 🛠️ How to Use

1. **Download** the app from [GitHub Releases](https://github.com/bruhnn/BD2ModManager/releases).
2. **Select your Brown Dust 2 directory** (where `BrownDust II.exe` is located)
   - Example: F:\Neowiz\Browndust2\Browndust2_10000001
3. **Add your mods** by:
   - Noving them into the `mods/` folder  
     ⚠️ **Note:** This is *not* the BrownDustX `mods` directory. It's a separate folder used by this manager
4. **Enable or disable mods**.
5. **Sync your mods** to apply changes:
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
- ❌ Drag & Drop not available (Windows restrictions)  



---

## 📸 Screenshots

### Mods Page (v4.0.1)
![Mods Page](./screenshots/mods_page_v401.png)

### Characters Page (v4.0.1)
![Characters Page](./screenshots/characters_page_v401.png)

### Mod Preview (BD2ModPreview)
![Mod Preview](./screenshots/bd2modpreview.png)

---
## 🧰 Community & Related Projects  

- [**Brown Dust II Mod Manager (by kxdekxde)**](https://codeberg.org/kxdekxde/browndust2-mod-manager) – An alternative mod manager for Brown Dust 2
- [**BD2 Live2D Viewer (by jelosus2)**](https://jelosus2.github.io/BD2-L2D-Viewer) – Website to preview character animations
- [**BDroid_X (by Ark-Repoleved)**](https://github.com/Ark-Repoleved/BDroid_X) - Brown Dust II Mod Manager for Android

## ❓ FAQ

### Where can I get mods?
You can find mods on the BrownDustX Discord server: [https://discord.gg/B3Aqz6tDG2](https://discord.gg/B3Aqz6tDG2)

## 🤝 Credits

- Character assets by [myssal/Brown-Dust-2-Asset](https://github.com/myssal/Brown-Dust-2-Asset)
- Thanks to **Synae** for *Brown Dust X*

## Star History

<a href="https://www.star-history.com/#bruhnn/BD2ModManager&Date">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=bruhnn/BD2ModManager&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=bruhnn/BD2ModManager&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=bruhnn/BD2ModManager&type=Date" />
 </picture>
</a>