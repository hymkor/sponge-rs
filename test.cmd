@echo off
setlocal
gawk "BEGIN{ for ( i=0 ; i<50000 ; i++ ){ printf \"%%d \",i } }" > original.txt
REM pause
call :testone cat -n || exit /b 1
call :testone gawk "{ print NR,$0 }" || exit /b 1
del original.txt
endlocal
exit /b 0

:testone
    echo TEST CASE: %*
    %* original.txt > expect.txt
    %* original.txt | .\target\release\sponge original.txt
    comp expect.txt original.txt /M || exit /b 1
    del expect.txt
    rem type original.txt
    exit /b 0
