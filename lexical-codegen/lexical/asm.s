/ (fcn) sym.parse_u8 75
|   sym.parse_u8 (unsigned int arg1);
|           ; arg unsigned int arg1 @ rdi
|           0x000032f0      test rsi, rsi
|       ,=< 0x000032f3      je   0x3335
|       |   0x000032f5      cmp  byte [rdi], 0x2b ; [0x2b:1]=0 ; '+' ; arg1
|      ,==< 0x000032f8      jne  0x3304
|      ||   0x000032fa      add  rsi, 0xffffffffffffffff
|     ,===< 0x000032fe      je   0x3335
|     |||   0x00003300      add  rdi, 1
|     |`--> 0x00003304      mov  rcx, rdi
|     | |   0x00003307      add  rcx, rsi ; '+'
|     | |   0x0000330a      xor  eax, eax
|     | |   0x0000330c      mov  r8b, 0xa
|     | |   0x0000330f      nop
|     |.--> 0x00003310      mov  edx, eax
|     |:|   0x00003312      cmp  rcx, rdi
|    ,====< 0x00003315      je   0x3338
|    ||:|   0x00003317      movzx esi, byte [rdi]
|    ||:|   0x0000331a      add  esi, 0xffffffffffffffd0
|    ||:|   0x0000331d      cmp  esi, 9
|   ,=====< 0x00003320      ja   0x3332
|   |||:|   0x00003322      mov  eax, edx
|   |||:|   0x00003324      mul  r8b
|  ,======< 0x00003327      jo   0x3332
|  ||||:|   0x00003329      add  rdi, 1
|  ||||:|   0x0000332d      add  al, sil ; '.'
|  ||||`==< 0x00003330      jae  0x3310
|  ``-----> 0x00003332      xor  eax, eax
|    || |   0x00003334      ret
|    |`-`-> 0x00003335      xor  eax, eax
|    |      0x00003337      ret
|    `----> 0x00003338      mov  al, 1
\           0x0000333a      ret
            0x0000333b      nop  dword [rax + rax]
