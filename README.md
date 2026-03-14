
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

## ✨ New UPDATE 4.0.0 ✨
* Remade the entire mod manager in another language (from pyside to tauri)
* Mod manager 3.5x smaller
* New UI Design
* Multiplatform (Windows, Linux)
* Much faster
* Characters Tab with grid, list view
* Install bepinex, browndustx inside the mod manager


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


### Example Comparison with 359 mods

| Copy | Symlink |
|--------|-------|
| ![](./screenshots/sync_copy_v322.gif) | ![](./screenshots/sync_symlink_v322.gif) |


---

## 📸 Screenshots

### Mods Page (v3.1.0)
![Mods Page](./screenshots/mods_page_v31.png)

### Characters Page (v3.1.0)
![Characters Page](./screenshots/characters_page_v31.png)

### Mod Preview (BD2ModPreview v0.4.1)
![Mod Preview](./screenshots/bd2modpreview.png)

---
## 🧰 Community & Related Projects  

- [**Brown Dust II Mod Manager (by kxdekxde)**](https://codeberg.org/kxdekxde/browndust2-mod-manager) – An alternative mod manager for Brown Dust 2
- [**BD2 Live2D Viewer (by jelosus2)**](https://jelosus2.github.io/BD2-L2D-Viewer) – Website to preview character animations

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