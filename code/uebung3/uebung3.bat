@echo off
if exist *.exe del *.exe

rustc uebung3.rs
uebung3.exe

pause