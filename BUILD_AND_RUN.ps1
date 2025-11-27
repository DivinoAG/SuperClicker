# SuperClicker Build and Run Script
# Location: D:\Projects\SuperClicker\super_clicker

# Ensure we're in the correct directory
Set-Location D:\Projects\SuperClicker\super_clicker

Write-Host "Current directory: D:\Projects\SuperClicker" -ForegroundColor Green
Write-Host "Building SuperClicker..." -ForegroundColor Cyan

cargo build --quiet 2>&1

if ( -eq 0) {
    Write-Host "Build successful!" -ForegroundColor Green
    Write-Host "Launching SuperClicker..." -ForegroundColor Cyan
    Write-Host "(Close the window to stop)" -ForegroundColor Yellow
    Write-Host ""
    cargo run 2>&1
} else {
    Write-Host "Build failed!" -ForegroundColor Red
}
