@echo off
setlocal
CD /d "%~dp0"

echo =======================================================
echo Nexus 1.10.3 - 11m / DX Cluster - Windows Installer
echo =======================================================
echo.
echo This launcher uses the project's official Windows build script.
echo It builds the full radio version and creates the Setup EXE.
echo.

where powershell.exe >nul 2>nul || (
    echo ERROR : PowerShell is missing.
    pause
    exit /b1
)

if not exist "scripts\build-windows.ps1" (
    echo ERROR : scripts\build-windows.ps1 is missing.
    pause
    exit /b1
)

powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0scripts\build-windows.ps1"
set "RC=%ERRORLEVEL%"

echo.
if not "%RC%"=="0" (
    echo =======================================================
    echo BUILD FAILED.
    echo Read the last message displayed in the window.
    echo =======================================================
    pause
    exit /b %RC%
)

echo =======================================================
echo BUILD COMPLETED
echo.
echo The Setup installer should be located in :
echo   src-tauri\target\release\bundle\nsis\
echo.
echo The standalone EXE is located in :
echo   src-tauri\target\release\
echo =======================================================
pause
exit /b0
