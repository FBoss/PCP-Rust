@echo off
if exist *.exe del *.exe

rustc thread_fail.rs

echo. 
echo. 

rustc thread.rs
thread.exe

pause