@echo off
if exist *.exe del *.exe

rustc matching.rs
matching.exe

pause