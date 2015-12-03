@echo off
if exist *.exe del *.exe

rustc thread_fail.rs


rustc thread.rs
thread.exe

pause