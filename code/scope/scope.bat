@echo off
if exist *.exe del *.exe

rustc scope.rs

pause