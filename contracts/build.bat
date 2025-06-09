@echo off
echo Building WASM contract...

REM Build contract
call wasm-pack build --target web

REM Optimize WASM
call wasm-opt -O4 -o token_opt.wasm pkg/zhtp_contracts_bg.wasm

REM Copy to main project
copy /Y token_opt.wasm ..\token.wasm

echo Build complete!