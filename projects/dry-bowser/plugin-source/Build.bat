@echo off
title Building Skyline...

:start
echo Building Skyline...
cargo skyline build --release

:question
echo.
set /p choice=Do you want to rebuild the plugin? (y/n):

if /i "%choice%"=="y" goto start
if /i "%choice%"=="Y" goto start
if /i "%choice%"=="" goto start

if /i "%choice%"=="N" goto end
if /i "%choice%"=="n" goto end

echo Please make your choice : 
goto question

:end
echo Ending session...
pause