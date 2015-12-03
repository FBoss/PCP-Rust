@echo off
if exist *.exe del *.exe

rustc thread.rs
thread.exe

pause