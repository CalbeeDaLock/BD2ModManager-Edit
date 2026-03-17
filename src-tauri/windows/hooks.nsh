!macro NSIS_HOOK_POSTINSTALL
  CreateDirectory "$APPDATA\com.bruhnn.BD2ModManager\tools"
  CopyFiles "$INSTDIR\resources\BD2ModPreview.exe" "$APPDATA\com.bruhnn.BD2ModManager\tools\BD2ModPreview.exe"
  RMDir /r "$INSTDIR\resources"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  RMDir /r "$APPDATA\com.bruhnn.BD2ModManager\tools"
!macroend