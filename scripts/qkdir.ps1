# Get the directory from the Rust program using the first argument passed to the script
$dir = & 'C:\Programming\rustcli\target\debug\rustcli.exe' qkdir $args[0]

# Start a new PowerShell window with the directory
Start-Process powershell -ArgumentList "-NoExit", "-Command cd '$dir'"

