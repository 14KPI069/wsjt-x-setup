@echo off
setlocal
cd /d "%~dp0"

echo ============================================================
echo Nexus 1.10.3 - 11 m / DX Cluster - Windows installer
echo ============================================================
echo.
echo This launcher uses the project's official Windows build script.
echo It builds the full radio version and creates the Setup EXE.
echo.

where powershell.exe >nul 2>nul || (
  echo ERREUR : PowerShell est introuvable.
  pause
  exit /b 1
)

if not exist "scripts\build-windows.ps1" (
  echo ERREUR : scripts\build-windows.ps1 est introuvable.
  pause
  exit /b 1
)

powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0scripts\build-windows.ps1"
set "RC=%ERRORLEVEL%"

echo.
if not "%RC%"=="0" (
  echo ============================================================
  echo ECHEC DE LA COMPILATION.
  echo Lis le dernier message affiche dans la fenetre.
  echo ============================================================
  pause
  exit /b %RC%
)

echo ============================================================
echo COMPILATION TERMINEE
 echo.
echo L'installateur Setup EXE doit se trouver dans :
echo   src-tauri\target\release\bundle\nsis\
echo.
echo Le programme EXE se trouve dans :
echo   src-tauri\target\release\
echo ============================================================
pause
exit /b 0 
