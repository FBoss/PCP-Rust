@echo off
if exist *.exe del *.exe

rustc thread_fail.rs

pause