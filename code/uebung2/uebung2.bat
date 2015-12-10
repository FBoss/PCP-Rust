@echo off
if exist *.exe del *.exe

rustc uebung2.rs
uebung2.exe

pause