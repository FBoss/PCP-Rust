@echo off
if exist *.exe del *.exe

rustc panic.rs
panic.exe

pause