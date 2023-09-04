@echo off

REM Display the argument passed to the batch script
echo Argument passed: %1

REM Get the directory from the Rust program using the first argument passed to the batch script
FOR /F %%i IN ('rustcli qkdir %1') DO set DIR=%%i

REM Display the directory obtained from the Rust program
echo Directory from Rust program: %DIR%

REM Change to that directory
cd %DIR%
