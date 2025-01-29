@echo off
echo Starting installation of xxxx.msi
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
