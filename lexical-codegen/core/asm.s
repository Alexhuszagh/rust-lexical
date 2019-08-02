/ (fcn) sym.core::num::__impl_core::str::FromStr_for_u8_::from_str::hce24b711719134f3 102
|   sym.core::num::__impl_core::str::FromStr_for_u8_::from_str::hce24b711719134f3 (int arg5, unsigned int arg1, unsigned int arg2);
|           ; arg int arg5 @ r8
|           ; arg unsigned int arg1 @ rdi
|           ; arg unsigned int arg2 @ rsi
|           0x00022230      mov  r8b, 1
|           0x00022233      test rsi, rsi
|       ,=< 0x00022236      je   0x2224c
|       |   0x00022238      cmp  byte [rdi], 0x2b ; [0x2b:1]=0 ; '+' ; arg1
|       |   0x0002223b      mov  rcx, rdi
|      ,==< 0x0002223e      jne  0x22256
|      ||   0x00022240      cmp  rsi, 1 ; "ELF\x02\x01\x01" ; arg2
|     ,===< 0x00022244      jne  0x22252
|     |||   0x00022246      xor  edx, edx
|     |||   0x00022248      mov  eax, r8d ; arg5
|     |||   0x0002224b      ret
|     ||`-> 0x0002224c      xor  edx, edx
|     ||    0x0002224e      mov  eax, r8d ; arg5
|     ||    0x00022251      ret
|     `---> 0x00022252      lea  rcx, [rdi + 1] ; "ELF\x02\x01\x01"
|      `--> 0x00022256      add  rdi, rsi ; '+'
|           0x00022259      xor  eax, eax
|           0x0002225b      mov  r9b, 0xa
|           0x0002225e      nop
|       .-> 0x00022260      cmp  rdi, rcx
|      ,==< 0x00022263      je   0x22287
|      |:   0x00022265      movzx esi, byte [rcx]
|      |:   0x00022268      add  esi, 0xffffffffffffffd0
|      |:   0x0002226b      mov  r8b, 1
|      |:   0x0002226e      cmp  esi, 9
|     ,===< 0x00022271      ja   0x22290
|     ||:   0x00022273      mul  r9b
|     ||:   0x00022276      mov  dl, 2
|    ,====< 0x00022278      jo   0x22283
|    |||:   0x0002227a      add  rcx, 1
|    |||:   0x0002227e      add  al, sil ; '.'
|    |||`=< 0x00022281      jae  0x22260
|    `----> 0x00022283      mov  eax, r8d ; arg5
|     ||    0x00022286      ret
|     |`--> 0x00022287      xor  r8d, r8d
|     |     0x0002228a      mov  edx, eax
|     |     0x0002228c      mov  eax, r8d ; arg5
|     |     0x0002228f      ret
|     `---> 0x00022290      mov  dl, 1
|           0x00022292      mov  eax, r8d ; arg5
\           0x00022295      ret
            0x00022296      nop  word cs:[rax + rax]

/ (fcn) sym.core::num::__impl_core::str::FromStr_for_u16_::from_str::h447c8ddc73e5df8e 125
|   sym.core::num::__impl_core::str::FromStr_for_u16_::from_str::h447c8ddc73e5df8e (int arg6, unsigned int arg1, unsigned int arg2);
|           ; arg int arg6 @ r9
|           ; arg unsigned int arg1 @ rdi
|           ; arg unsigned int arg2 @ rsi
|           0x000222a0      mov  r8d, 1
|           0x000222a6      xor  r9d, r9d
|           0x000222a9      test rsi, rsi
|       ,=< 0x000222ac      je   0x222bc
|       |   0x000222ae      cmp  byte [rdi], 0x2b ; [0x2b:1]=0 ; '+' ; arg1
|       |   0x000222b1      mov  rcx, rdi
|      ,==< 0x000222b4      jne  0x222cc
|      ||   0x000222b6      cmp  rsi, 1 ; "ELF\x02\x01\x01" ; arg2
|     ,===< 0x000222ba      jne  0x222c8
|     ||`-> 0x000222bc      xor  edx, edx
|  ...--.-> 0x000222be      or   r9d, r8d
|  :::||:   0x000222c1      or   r9d, edx
|  :::||:   0x000222c4      mov  eax, r9d ; arg6
|  :::||:   0x000222c7      ret
|  :::`---> 0x000222c8      lea  rcx, [rdi + 1] ; "ELF\x02\x01\x01"
|  ::: `--> 0x000222cc      add  rdi, rsi ; '+'
|  :::  :   0x000222cf      xor  eax, eax
|  :::  :   0x000222d1      mov  r10w, 0xa
|  :::  :   0x000222d6      nop  word cs:[rax + rax]
|  ::: .--> 0x000222e0      cmp  rdi, rcx
|  :::,===< 0x000222e3      je   0x22309
|  :::|::   0x000222e5      movzx esi, byte [rcx]
|  :::|::   0x000222e8      add  esi, 0xffffffffffffffd0
|  :::|::   0x000222eb      xor  r9d, r9d
|  :::|::   0x000222ee      cmp  esi, 9
| ,=======< 0x000222f1      ja   0x22316
| |:::|::   0x000222f3      mul  r10w
| |:::|::   0x000222f7      mov  edx, 0x200 ; "Q\xe5td\x06"
| |`======< 0x000222fc      jo   0x222be
| | ::|::   0x000222fe      add  rcx, 1
| | ::|::   0x00022302      add  ax, si ; '-'
| | ::|`==< 0x00022305      jae  0x222e0
| | `=====< 0x00022307      jmp  0x222be
| |  :`---> 0x00022309      shl  eax, 0x10
| |  :  :   0x0002230c      xor  edx, edx
| |  :  :   0x0002230e      mov  r9d, eax
| |  :  :   0x00022311      xor  r8d, r8d
| |  `====< 0x00022314      jmp  0x222be
| `-------> 0x00022316      mov  edx, 0x100
\       `=< 0x0002231b      jmp  0x222be
            0x0002231d      nop  dword [rax]

/ (fcn) sym.core::num::__impl_core::str::FromStr_for_u32_::from_str::hb2d9607963b46005 121
|   sym.core::num::__impl_core::str::FromStr_for_u32_::from_str::hb2d9607963b46005 (unsigned int arg1, unsigned int arg2);
|           ; arg unsigned int arg1 @ rdi
|           ; arg unsigned int arg2 @ rsi
|           0x00022320      mov  r8d, 1
|           0x00022326      test rsi, rsi
|       ,=< 0x00022329      je   0x22339
|       |   0x0002232b      cmp  byte [rdi], 0x2b ; [0x2b:1]=0 ; '+' ; arg1
|       |   0x0002232e      mov  rcx, rdi
|      ,==< 0x00022331      jne  0x2234c
|      ||   0x00022333      cmp  rsi, 1 ; "ELF\x02\x01\x01" ; arg2
|     ,===< 0x00022337      jne  0x22348
|     ||`-> 0x00022339      xor  r10d, r10d
|     ||    0x0002233c      xor  edx, edx
|  ...--.-> 0x0002233e      or   r10, r8
|  :::||:   0x00022341      or   r10, rdx
|  :::||:   0x00022344      mov  rax, r10
|  :::||:   0x00022347      ret
|  :::`---> 0x00022348      lea  rcx, [rdi + 1] ; "ELF\x02\x01\x01"
|  ::: `--> 0x0002234c      add  rdi, rsi ; '+'
|  :::  :   0x0002234f      xor  r10d, r10d
|  :::  :   0x00022352      mov  r9d, 0xa
|  :::  :   0x00022358      xor  eax, eax
|  :::  :   0x0002235a      nop  word [rax + rax]
|  ::: .--> 0x00022360      cmp  rdi, rcx
|  :::,===< 0x00022363      je   0x22384
|  :::|::   0x00022365      movzx esi, byte [rcx]
|  :::|::   0x00022368      add  esi, 0xffffffffffffffd0
|  :::|::   0x0002236b      cmp  esi, 9
| ,=======< 0x0002236e      ja   0x22392
| |:::|::   0x00022370      mul  r9d
| |:::|::   0x00022373      mov  edx, 0x200 ; "Q\xe5td\x06"
| |`======< 0x00022378      jo   0x2233e
| | ::|::   0x0002237a      add  rcx, 1
| | ::|::   0x0002237e      add  eax, esi
| | ::|`==< 0x00022380      jae  0x22360
| | `=====< 0x00022382      jmp  0x2233e
| |  :`---> 0x00022384      shl  rax, 0x20
| |  :  :   0x00022388      xor  edx, edx
| |  :  :   0x0002238a      mov  r10, rax
| |  :  :   0x0002238d      xor  r8d, r8d
| |  `====< 0x00022390      jmp  0x2233e
| `-------> 0x00022392      mov  edx, 0x100
\       `=< 0x00022397      jmp  0x2233e
            0x00022399      nop  dword [rax]

/ (fcn) sym.core::num::__impl_core::str::FromStr_for_usize_::from_str::hb41eb873edd26255 109
|   sym.core::num::__impl_core::str::FromStr_for_usize_::from_str::hb41eb873edd26255 (int arg1, unsigned int arg2, unsigned int arg3);
|           ; arg int arg1 @ rdi
|           ; arg unsigned int arg2 @ rsi
|           ; arg unsigned int arg3 @ rdx
|           0x000223a0      mov  r8, rdi ; arg1
|           0x000223a3      test rdx, rdx
|       ,=< 0x000223a6      je   0x223b6
|       |   0x000223a8      cmp  byte [rsi], 0x2b ; [0x2b:1]=0 ; '+' ; arg2
|       |   0x000223ab      mov  rcx, rsi
|      ,==< 0x000223ae      jne  0x223c1
|      ||   0x000223b0      cmp  rdx, 1 ; "ELF\x02\x01\x01" ; arg3
|     ,===< 0x000223b4      jne  0x223bd
|     ||`-> 0x000223b6      mov  byte [r8 + 1], 0
|     ||,=< 0x000223bb      jmp  0x22404
|     `---> 0x000223bd      lea  rcx, [rsi + 1] ; "ELF\x02\x01\x01"
|      `--> 0x000223c1      add  rsi, rdx
|       |   0x000223c4      xor  eax, eax
|       |   0x000223c6      mov  r9d, 0xa
|       |   0x000223cc      nop  dword [rax]
|      .--> 0x000223d0      cmp  rsi, rcx
|     ,===< 0x000223d3      je   0x223f7
|     |:|   0x000223d5      movzx edi, byte [rcx]
|     |:|   0x000223d8      add  edi, 0xffffffffffffffd0
|     |:|   0x000223db      cmp  edi, 0xa
|    ,====< 0x000223de      jae  0x223ff
|    ||:|   0x000223e0      mul  r9
|   ,=====< 0x000223e3      jo   0x223f0
|   |||:|   0x000223e5      add  rcx, 1
|   |||:|   0x000223e9      mov  edx, edi
|   |||:|   0x000223eb      add  rax, rdx
|   |||`==< 0x000223ee      jae  0x223d0
|   `-----> 0x000223f0      mov  byte [r8 + 1], 2
|    ||,==< 0x000223f5      jmp  0x22404
|    |`---> 0x000223f7      mov  qword [r8 + 8], rax
|    | ||   0x000223fb      xor  eax, eax
|    |,===< 0x000223fd      jmp  0x22406
|    `----> 0x000223ff      mov  byte [r8 + 1], 1
|     |``-> 0x00022404      mov  al, 1
|     `---> 0x00022406      mov  byte [r8], al
|           0x00022409      mov  rax, r8
\           0x0002240c      ret
            0x0002240d      nop  dword [rax]

/ (fcn) sym.core::num::__impl_core::str::FromStr_for_u128_::from_str::hb6606e086cf598b5 190
|   sym.core::num::__impl_core::str::FromStr_for_u128_::from_str::hb6606e086cf598b5 (int arg1, unsigned int arg2, unsigned int arg3, int arg4);
|           ; arg int arg1 @ rdi
|           ; arg unsigned int arg2 @ rsi
|           ; arg unsigned int arg3 @ rdx
|           ; arg int arg4 @ rcx
|           0x00022410      push rbp
|           0x00022411      push r15
|           0x00022413      push r14
|           0x00022415      push rbx
|           0x00022416      mov  r8, rdi ; arg1
|           0x00022419      test rdx, rdx
|       ,=< 0x0002241c      je   0x2242c
|       |   0x0002241e      cmp  byte [rsi], 0x2b ; [0x2b:1]=0 ; '+' ; arg2
|       |   0x00022421      mov  r11, rsi
|      ,==< 0x00022424      jne  0x2243a
|      ||   0x00022426      cmp  rdx, 1 ; "ELF\x02\x01\x01" ; arg3
|     ,===< 0x0002242a      jne  0x22436
|     ||`-> 0x0002242c      mov  byte [r8 + 1], 0
|     ||,=< 0x00022431      jmp  0x224bf
|     `---> 0x00022436      lea  r11, [rsi + 1] ; "ELF\x02\x01\x01"
|      `--> 0x0002243a      add  rsi, rdx
|       |   0x0002243d      xor  ecx, ecx
|       |   0x0002243f      mov  r14d, 0xa
|       |   0x00022445      xor  edi, edi
|       |   0x00022447      nop  word [rax + rax]
|      .--> 0x00022450      cmp  rsi, r11
|     ,===< 0x00022453      je   0x224ae
|     |:|   0x00022455      movzx ebp, byte [r11]
|     |:|   0x00022459      add  ebp, 0xffffffffffffffd0
|     |:|   0x0002245c      cmp  ebp, 0xa
|    ,====< 0x0002245f      jae  0x224ba
|    ||:|   0x00022461      xor  eax, eax
|    ||:|   0x00022463      mul  rcx
|    ||:|   0x00022466      mov  r9, rax
|    ||:|   0x00022469      seto r15b
|    ||:|   0x0002246d      mov  rax, rdi ; arg1
|    ||:|   0x00022470      mul  r14
|    ||:|   0x00022473      mov  r10, rax
|    ||:|   0x00022476      seto bl
|    ||:|   0x00022479      or   bl, r15b
|    ||:|   0x0002247c      add  r10, r9 ; 'k'
|    ||:|   0x0002247f      mov  rax, rcx ; arg4
|    ||:|   0x00022482      mul  r14
|    ||:|   0x00022485      mov  rdi, rdx
|    ||:|   0x00022488      add  rdi, r10 ; 'l'
|    ||:|   0x0002248b      setb dl
|    ||:|   0x0002248e      or   dl, bl
|    ||:|   0x00022490      cmp  dl, 1 ; "ELF\x02\x01\x01"
|   ,=====< 0x00022493      je   0x224a7
|   |||:|   0x00022495      mov  rcx, rax
|   |||:|   0x00022498      add  r11, 1
|   |||:|   0x0002249c      mov  eax, ebp
|   |||:|   0x0002249e      add  rcx, rax ; '#'
|   |||:|   0x000224a1      adc  rdi, 0
|   |||`==< 0x000224a5      jae  0x22450
|   `-----> 0x000224a7      mov  byte [r8 + 1], 2
|    ||,==< 0x000224ac      jmp  0x224bf
|    |`---> 0x000224ae      mov  qword [r8 + 8], rcx
|    | ||   0x000224b2      mov  qword [r8 + 0x10], rdi
|    | ||   0x000224b6      xor  eax, eax
|    |,===< 0x000224b8      jmp  0x224c1
|    `----> 0x000224ba      mov  byte [r8 + 1], 1
|     |``-> 0x000224bf      mov  al, 1
|     `---> 0x000224c1      mov  byte [r8], al
|           0x000224c4      mov  rax, r8
|           0x000224c7      pop  rbx
|           0x000224c8      pop  r14
|           0x000224ca      pop  r15
|           0x000224cc      pop  rbp
\           0x000224cd      ret
            0x000224ce      nop

/ (fcn) sym.core::num::__impl_core::str::FromStr_for_i8_::from_str::hcb78a62c9f97c037 166
|   sym.core::num::__impl_core::str::FromStr_for_i8_::from_str::hcb78a62c9f97c037 (int arg1, unsigned int arg2);
|           ; arg int arg1 @ rdi
|           ; arg unsigned int arg2 @ rsi
|           0x00021f90      mov  r9w, 1
|           0x00021f95      test rsi, rsi
|       ,=< 0x00021f98      je   0x21fac
|       |   0x00021f9a      mov  al, byte [rdi] ; arg1
|       |   0x00021f9c      cmp  al, 0x2b ; '+'
|      ,==< 0x00021f9e      je   0x21fb0
|      ||   0x00021fa0      cmp  al, 0x2d ; '-'
|     ,===< 0x00021fa2      jne  0x21fc5
|     |||   0x00021fa4      xor  eax, eax
|     |||   0x00021fa6      cmp  rsi, 1 ; "ELF\x02\x01\x01" ; arg2
|    ,====< 0x00021faa      jne  0x21fb8
|   .---`-> 0x00021fac      xor  eax, eax
|   :|||,=< 0x00021fae      jmp  0x22029
|   :||`--> 0x00021fb0      mov  al, 1
|   :|| |   0x00021fb2      cmp  rsi, 1 ; "ELF\x02\x01\x01"
|   `=====< 0x00021fb6      je   0x21fac
|    `----> 0x00021fb8      test al, al
|     |,==< 0x00021fba      je   0x21ff0
|     |||   0x00021fbc      add  rsi, rdi ; '''
|     |||   0x00021fbf      lea  rdi, [rdi + 1] ; "ELF\x02\x01\x01" ; arg1
|    ,====< 0x00021fc3      jmp  0x21fc8
|    |`---> 0x00021fc5      add  rsi, rdi ; '''
|    `----> 0x00021fc8      xor  eax, eax
|      ||   0x00021fca      mov  dl, 0xa
|      ||   0x00021fcc      nop  dword [rax]
|     .---> 0x00021fd0      cmp  rsi, rdi
|    ,====< 0x00021fd3      je   0x22022
|    |:||   0x00021fd5      movzx ecx, byte [rdi]
|    |:||   0x00021fd8      add  ecx, 0xffffffffffffffd0
|    |:||   0x00021fdb      cmp  ecx, 9
|   ,=====< 0x00021fde      ja   0x22027
|   ||:||   0x00021fe0      imul dl
|  ,======< 0x00021fe2      jo   0x21fec
|  |||:||   0x00021fe4      add  rdi, 1
|  |||:||   0x00021fe8      add  al, cl
|  |||`===< 0x00021fea      jno  0x21fd0
|  `------> 0x00021fec      mov  al, 2
|   ||,===< 0x00021fee      jmp  0x22029
|   |||`--> 0x00021ff0      mov  edx, 1
|   ||| |   0x00021ff5      xor  eax, eax
|   ||| |   0x00021ff7      mov  r8b, 0xa
|   ||| |   0x00021ffa      nop  word [rax + rax]
|   |||.--> 0x00022000      cmp  rsi, rdx
|  ,======< 0x00022003      je   0x22022
|  ||||:|   0x00022005      movzx ecx, byte [rdi + rdx]
|  ||||:|   0x00022009      add  ecx, 0xffffffffffffffd0
|  ||||:|   0x0002200c      cmp  ecx, 9
| ,=======< 0x0002200f      ja   0x22027
| |||||:|   0x00022011      imul r8b
| ========< 0x00022014      jo   0x2201e
| |||||:|   0x00022016      add  rdx, 1
| |||||:|   0x0002201a      sub  al, cl
| |||||`==< 0x0002201c      jno  0x22000
| --------> 0x0002201e      mov  al, 3
| |||||,==< 0x00022020      jmp  0x22029
| |`-`----> 0x00022022      xor  r9d, r9d
| | |,====< 0x00022025      jmp  0x22029
| `-`-----> 0x00022027      mov  al, 1
|    ````-> 0x00022029      movzx ecx, al
|           0x0002202c      shl  ecx, 8
|           0x0002202f      movzx eax, r9w
|           0x00022033      or   eax, ecx
\           0x00022035      ret
            0x00022036      nop  word cs:[rax + rax]

/ (fcn) sym.core::num::__impl_core::str::FromStr_for_i16_::from_str::h6a138a1ad1d474f9 221
|   sym.core::num::__impl_core::str::FromStr_for_i16_::from_str::h6a138a1ad1d474f9 (int arg1, unsigned int arg2, int arg3);
|           ; arg int arg1 @ rdi
|           ; arg unsigned int arg2 @ rsi
|           ; arg int arg3 @ rdx
|           0x00022040      mov  r8d, 1
|           0x00022046      xor  eax, eax
|           0x00022048      test rsi, rsi
|       ,=< 0x0002204b      je   0x22061
|       |   0x0002204d      mov  cl, byte [rdi] ; arg1
|       |   0x0002204f      cmp  cl, 0x2b ; '+'
|      ,==< 0x00022052      je   0x2206b
|      ||   0x00022054      cmp  cl, 0x2d ; '-'
|     ,===< 0x00022057      jne  0x22080
|     |||   0x00022059      xor  ecx, ecx
|     |||   0x0002205b      cmp  rsi, 1 ; "ELF\x02\x01\x01" ; arg2
|    ,====< 0x0002205f      jne  0x22073
|   .---`-> 0x00022061      xor  r10d, r10d
| ..----.-> 0x00022064      or   eax, r8d
| :::|||:   0x00022067      or   eax, r10d
| :::|||:   0x0002206a      ret
| :::||`--> 0x0002206b      mov  cl, 1
| :::|| :   0x0002206d      cmp  rsi, 1 ; "ELF\x02\x01\x01"
| ::`=====< 0x00022071      je   0x22061
| :: `----> 0x00022073      test cl, cl
| ::  |,==< 0x00022075      je   0x220b9
| ::  ||:   0x00022077      add  rsi, rdi ; '''
| ::  ||:   0x0002207a      lea  rdi, [rdi + 1] ; "ELF\x02\x01\x01" ; arg1
| :: ,====< 0x0002207e      jmp  0x22083
| :: |`---> 0x00022080      add  rsi, rdi ; '''
| :: `----> 0x00022083      xor  edx, edx
| ::   |:   0x00022085      nop  word cs:[rax + rax]
| ::   |:   0x0002208f      nop
| ::  .---> 0x00022090      cmp  rsi, rdi
| :: ,====< 0x00022093      je   0x22102
| :: |:|:   0x00022095      movzx ecx, byte [rdi]
| :: |:|:   0x00022098      add  ecx, 0xffffffffffffffd0
| :: |:|:   0x0002209b      xor  eax, eax
| :: |:|:   0x0002209d      cmp  ecx, 9
| ::,=====< 0x000220a0      ja   0x22112
| ::||:|:   0x000220a2      imul dx, dx, 0xa
| ::||:|:   0x000220a6      mov  r10d, 0x200 ; "Q\xe5td\x06"
| ========< 0x000220ac      jo   0x22064
| ::||:|:   0x000220ae      add  rdi, 1
| ::||:|:   0x000220b2      add  dx, cx
| ::||`===< 0x000220b5      jno  0x22090
| ========< 0x000220b7      jmp  0x22064
| ::|| `--> 0x000220b9      mov  r9d, 1
| ::||  :   0x000220bf      xor  edx, edx
| ::||  :   0x000220c1      nop  word cs:[rax + rax]
| ::||  :   0x000220cb      nop  dword [rax + rax]
| ::|| .--> 0x000220d0      cmp  rsi, r9
| ::||,===< 0x000220d3      je   0x22102
| ::|||::   0x000220d5      movzx ecx, byte [rdi + r9]
| ::|||::   0x000220da      add  ecx, 0xffffffffffffffd0
| ::|||::   0x000220dd      xor  eax, eax
| ::|||::   0x000220df      cmp  ecx, 9
| ========< 0x000220e2      ja   0x22112
| ::|||::   0x000220e4      imul dx, dx, 0xa
| ::|||::   0x000220e8      mov  r10d, 0x300
| ========< 0x000220ee      jo   0x22064
| ::|||::   0x000220f4      add  r9, 1
| ::|||::   0x000220f8      sub  dx, cx
| ::|||`==< 0x000220fb      jno  0x220d0
| `=======< 0x000220fd      jmp  0x22064
|  :|``---> 0x00022102      shl  edx, 0x10
|  :|   :   0x00022105      xor  r10d, r10d
|  :|   :   0x00022108      mov  eax, edx ; arg3
|  :|   :   0x0002210a      xor  r8d, r8d
|  `======< 0x0002210d      jmp  0x22064
| --`-----> 0x00022112      mov  r10d, 0x100
\       `=< 0x00022118      jmp  0x22064
            0x0002211d      nop  dword [rax]

/ (fcn) sym.core::num::__impl_core::str::FromStr_for_i32_::from_str::hdffbc2364705f1c5 202
|   sym.core::num::__impl_core::str::FromStr_for_i32_::from_str::hdffbc2364705f1c5 (int arg1, unsigned int arg2, int arg4);
|           ; arg int arg1 @ rdi
|           ; arg unsigned int arg2 @ rsi
|           ; arg int arg4 @ rcx
|           0x00022120      mov  r8d, 1
|           0x00022126      test rsi, rsi
|       ,=< 0x00022129      je   0x22152
|       |   0x0002212b      mov  al, byte [rdi] ; arg1
|       |   0x0002212d      cmp  al, 0x2b ; '+'
|      ,==< 0x0002212f      je   0x2214a
|      ||   0x00022131      cmp  al, 0x2d ; '-'
|     ,===< 0x00022133      jne  0x2215e
|     |||   0x00022135      xor  eax, eax
|     |||   0x00022137      cmp  rsi, 1 ; "ELF\x02\x01\x01" ; arg2
|    ,====< 0x0002213b      je   0x22152
|   .-----> 0x0002213d      test al, al
|  ,======< 0x0002213f      je   0x22195
|  |:||||   0x00022141      add  rsi, rdi ; '''
|  |:||||   0x00022144      lea  rdi, [rdi + 1] ; "ELF\x02\x01\x01" ; arg1
| ,=======< 0x00022148      jmp  0x22161
| ||:||`--> 0x0002214a      mov  al, 1
| ||:|| |   0x0002214c      cmp  rsi, 1 ; "ELF\x02\x01\x01"
| ||`=====< 0x00022150      jne  0x2213d
| || `--`-> 0x00022152      xor  eax, eax
| ||  |     0x00022154      xor  r10d, r10d
| --..-..-> 0x00022157      or   rax, r8
| ||::|::   0x0002215a      or   rax, r10
| ||::|::   0x0002215d      ret
| ||::`---> 0x0002215e      add  rsi, rdi ; '''
| `-------> 0x00022161      xor  eax, eax
|  |:: ::   0x00022163      xor  ecx, ecx
|  |:: ::   0x00022165      nop  word cs:[rax + rax]
|  |:: ::   0x0002216f      nop
|  |::.---> 0x00022170      cmp  rsi, rdi
| ,=======< 0x00022173      je   0x221cd
| ||:::::   0x00022175      movzx edx, byte [rdi]
| ||:::::   0x00022178      add  edx, 0xffffffffffffffd0
| ||:::::   0x0002217b      cmp  edx, 9
| ========< 0x0002217e      ja   0x221df
| ||:::::   0x00022180      imul ecx, ecx, 0xa
| ||:::::   0x00022183      mov  r10d, 0x200 ; "Q\xe5td\x06"
| ========< 0x00022189      jo   0x22157
| ||:::::   0x0002218b      add  rdi, 1
| ||:::::   0x0002218f      add  ecx, edx
| ||::`===< 0x00022191      jno  0x22170
| ========< 0x00022193      jmp  0x22157
| |`------> 0x00022195      xor  eax, eax
| | :: ::   0x00022197      mov  r9d, 1
| | :: ::   0x0002219d      xor  ecx, ecx
| | :: ::   0x0002219f      nop
| | ::.---> 0x000221a0      cmp  rsi, r9
| |,======< 0x000221a3      je   0x221cd
| ||:::::   0x000221a5      movzx edx, byte [rdi + r9]
| ||:::::   0x000221aa      add  edx, 0xffffffffffffffd0
| ||:::::   0x000221ad      mov  r8d, 1
| ||:::::   0x000221b3      cmp  edx, 9
| ========< 0x000221b6      ja   0x221df
| ||:::::   0x000221b8      imul ecx, ecx, 0xa
| ||:::::   0x000221bb      mov  r10d, 0x300
| ||`=====< 0x000221c1      jo   0x22157
| || ::::   0x000221c3      add  r9, 1
| || ::::   0x000221c7      sub  ecx, edx
| || :`===< 0x000221c9      jno  0x221a0
| || `====< 0x000221cb      jmp  0x22157
| ``------> 0x000221cd      shl  rcx, 0x20
|      ::   0x000221d1      xor  r10d, r10d
|      ::   0x000221d4      mov  rax, rcx ; arg4
|      ::   0x000221d7      xor  r8d, r8d
|      `==< 0x000221da      jmp  0x22157
| --------> 0x000221df      mov  r10d, 0x100
\       `=< 0x000221e5      jmp  0x22157
            0x000221ea      nop  word [rax + rax]
