
<<<<<<< HEAD
# Brown Dust 2 Mod Manager (EditVer)

[![GitHub License](https://img.shields.io/github/license/CalbeeDaLock/BD2ModManager-Edit)](https://github.com/CalbeeDaLock/BD2ModManager-Edit/blob/main/LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/CalbeeDaLock/BD2ModManager-Edit)](https://github.com/CalbeeDaLock/BD2ModManager-Edit/releases)

> 🛠 An edited version of the Brown Dust 2 Mod Manager to manage, preview, and sync mods for Brown Dust 2.
> 
> 🎉 **Download the latest version:** [GitHub Releases](https://github.com/CalbeeDaLock/BD2ModManager-Edit/releases/latest)
>

If you have suggestions or run into any problems with the app, feel free to open an issue.

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
=======
# Brown Dust 2 Mod Manager

[![GitHub License](https://img.shields.io/github/license/bruhnn/BD2ModManager)](https://github.com/bruhnn/BD2ModManager/blob/main/LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/bruhnn/BD2ModManager)](https://github.com/bruhnn/BD2ModManager/releases)
[![Total Downloads](https://img.shields.io/endpoint?url=https://shy-waterfall-2797.bruhnn.workers.dev/downloads)](https://github.com/bruhnn/BD2ModManager/releases)


> 🛠 Easily manage, preview, and sync mods for Brown Dust 2.
> 
> 🎉 **Download the latest version:** [GitHub Releases](https://github.com/bruhnn/BD2ModManager/releases/latest)
>

If you have suggestions or run into any problems with the app, feel free to open an issue or contact me.

> [!WARNING]
> **Before Uninstalling Brown Dust II !!!**
>
> If you use the **Symlink** sync method, unsync all mods before uninstalling the game. Otherwise, the uninstaller may delete your mods from the staging folder.
>>>>>>> upstream/main

---

## 🛠️ How to Use
<<<<<<< HEAD

1. **Download** the app from [GitHub Releases](https://github.com/CalbeeDaLock/BD2ModManager-Edit/releases).
2. **Select your Brown Dust 2 directory** (where `BrownDust II.exe` is located)
   - Example: F:\Neowiz\Browndust2\Browndust2_10000001
3. **Add your mods** by:
   - Moving them into the `mods/` folder  
     ⚠️ **Note:** This is *not* the BrownDustX `mods` directory. It's a separate folder used by this manager
4. **Enable or disable mods**.
5. **Sync your mods** to apply changes:
=======
1. **Download** the app from [GitHub Releases](https://github.com/bruhnn/BD2ModManager/releases).
2. **Select your Brown Dust 2 directory** (where `BrownDust II.exe` is located)
   - Example: F:\Neowiz\Browndust2\Browndust2_10000001
3. **Install BepInEx and BrownDustX**
   - Download both from the BrownDustX Discord:
     [discord.gg/B3Aqz6tDG2](https://discord.gg/B3Aqz6tDG2)

   - Manual Steps:
        
      **Install BepInEx**
      
      Extract the contents of the BepInEx archive into the **game folder** (you can open the game folder from the mod manager)
      (**NOT** the launcher folder).
     
      Your folder should look like this:
      ```text
         Browndust2_10000001/
         ├─ BepInEx/
         ├─ winhttp.dll
         └─ BrownDust II.exe
      ```
  
      > The BepInEx archive from the BrownDustX Discord already includes ConfigurationManager.
   
      **Install BrownDustX**
     
      Extract the `BepInEx` folder from `BrownDustX-[VERSION].zip`
      into the same game folder.
     
      Your folder should look like this:
      ```text
         Browndust2_10000001/
         ├─ BepInEx/
         │  └─ plugins/
         │     └─ BrownDustX/
         └─ BrownDust II.exe
      ```
4. **Verify the installation**
   - Launch the game.
   - On the loading screen, the game version and BrownDustX version should appear in red at the top-right corner.
5. **Add your mods** by:
   - Moving them into the `mods/` folder  
     ⚠️ **Note:** This is *not* the BrownDustX `mods` directory. It's a separate folder used by this manager
6. **Enable or disable mods**.
7. **Sync your mods** to apply changes:
>>>>>>> upstream/main
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

<<<<<<< HEAD
## 🤝 Credits & Thanks

This project is an edited version of the original **Brown Dust 2 Mod Manager**.

- **Original Creator:** [bruhnn](https://github.com/bruhnn/) - Thank you for the amazing work and the original [BD2ModManager](https://github.com/bruhnn/BD2ModManager).
- **Character assets:** Provided by [myssal/Brown-Dust-2-Asset](https://github.com/myssal/Brown-Dust-2-Asset).
- **Special Thanks:** To **Synae** for *BrownDustX* and everyone who contributed to the original project.

---
=======
## ❤️ Support the Project

If you enjoy the mod manager and want to support development, you can support me here:

- Ko-Fi: https://ko-fi.com/bruhnn
- 🇨🇳: https://afdian.com/a/bruhnn

---
## 🧰 Community & Related Projects  

- [**Brown Dust II Mod Manager (by kxdekxde)**](https://codeberg.org/kxdekxde/browndust2-mod-manager) – An alternative mod manager for Brown Dust 2
- [**BD2 Live2D Viewer (by jelosus2)**](https://jelosus2.github.io/BD2-L2D-Viewer) – Website to preview character animations
- [**BDroid_X (by Ark-Repoleved)**](https://github.com/Ark-Repoleved/BDroid_X) - Brown Dust II Mod Manager for Android
>>>>>>> upstream/main

## ❓ FAQ

### Where can I get mods?
You can find mods on the BrownDustX Discord server: [https://discord.gg/B3Aqz6tDG2](https://discord.gg/B3Aqz6tDG2)
<<<<<<< HEAD
=======

## 🤝 Credits & Thanks

- Character assets by [myssal/Brown-Dust-2-Asset](https://github.com/myssal/Brown-Dust-2-Asset)
- Thanks to **Synae** for *BrownDustX*
- Thanks to everyone who contributed by reporting bugs and suggesting features.  
   ![Alt](https://repobeats.axiom.co/api/embed/ed16651a70e0b4d1937e8e346f394c6b78c9a231.svg "Repobeats analytics image")


## ✨ Star History

<a href="https://www.star-history.com/#bruhnn/BD2ModManager&Date">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=bruhnn/BD2ModManager&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=bruhnn/BD2ModManager&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=bruhnn/BD2ModManager&type=Date" />
 </picture>
</a>
>>>>>>> upstream/main
