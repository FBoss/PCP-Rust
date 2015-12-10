@echo off
if exist *.exe del *.exe

rustc uebung1.rs
uebung1.exe

pause