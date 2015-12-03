@echo off
if exist *.exe del *.exe

rustc patterns.rs
patterns.exe

pause