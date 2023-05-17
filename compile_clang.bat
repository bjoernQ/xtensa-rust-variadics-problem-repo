@echo off
set CHIP=esp32
call :compile
exit /b

:compile
echo Generating for %CHIP%

if %CHIP% equ esp32 (
    set TARGET=CONFIG_IDF_TARGET_ESP32
    set CC=xtensa-esp32-elf-clang
    set EXTRAS="-mlongcalls"
)

%CC% -isystem^
        -I. ^
        -D %TARGET% ^
        -O2 -c ^
        logging.c ^
        -o liblogging%CHIP%.a ^
        %EXTRAS%

copy liblogging%CHIP%.a .\libs\liblogging%CHIP%.a
del *.a /q
exit /b