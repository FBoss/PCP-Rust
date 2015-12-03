@echo off
if exist *.exe del *.exe

rustc channels.rs
channels.exe

pause