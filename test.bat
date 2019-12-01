@echo off
REM timeout /T %1
PING -n %1 127.0.0.1>nul
echo Hello %2