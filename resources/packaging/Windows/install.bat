@echo off
echo ComputeDHT installation system for Windows
echo Starting installation of ComputeDHT Server program...
echo.
echo Press [Enter] to continue or [Ctrl+C] to cancel...
pause >nul

msiexec /i xxxx.msi /qn /norestart
if %ERRORLEVEL% EQU 0 (
    echo Installation completed successfully.
) else (
    echo Installation failed. Error code: %ERRORLEVEL%
)
pause
