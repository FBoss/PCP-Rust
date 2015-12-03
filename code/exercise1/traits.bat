@echo off
if exist *.exe del *.exe

rustc traits.rs
traits.exe

pause