                           .data:0000c000 01 00 05                         ld bc,0x0500
                           .data:0000c003 00                               nop
                           .data:0000c004 72                               ld (hl),d
                           .data:0000c005 c0                               ret nz
                           .data:0000c006 c3 bc c1                         jp 0xc1bc
                           .data:0000c009 c3 b2 c1                         jp 0xc1b2
                           .data:0000c00c c3 d1 cc                         jp 0xccd1
                           .data:0000c00f c3 d5 cc                         jp 0xccd5
                           .data:0000c012 c3 e4 cc                         jp 0xcce4
                           .data:0000c015 c3 fd cc                         jp 0xccfd
                           .data:0000c018 c3 01 cd                         jp 0xcd01
                           .data:0000c01b c3 18 cd                         jp 0xcd18
                           .data:0000c01e c3 da cd                         jp 0xcdda
                           .data:0000c021 c3 dd cd                         jp 0xcddd
                           .data:0000c024 c3 e4 cd                         jp 0xcde4
                           .data:0000c027 c3 fe cd                         jp 0xcdfe
                           .data:0000c02a c3 2e d4                         jp 0xd42e
                           .data:0000c02d c3 8a d4                         jp 0xd48a
                           .data:0000c030 c3 c4 d4                         jp 0xd4c4
                           .data:0000c033 c3 72 ca                         jp 0xca72
                           .data:0000c036 c3 0d c6                         jp 0xc60d
                           .data:0000c039 c3 81 c5                         jp 0xc581
                           .data:0000c03c c3 66 c6                         jp 0xc666
                           .data:0000c03f c3 4e c6                         jp 0xc64e
                           .data:0000c042 c3 52 c6                         jp 0xc652
                           .data:0000c045 c3 63 c7                         jp 0xc763
                           .data:0000c048 c3 30 c6                         jp 0xc630
                           .data:0000c04b c3 03 c6                         jp 0xc603
                           .data:0000c04e c3 68 c1                         jp 0xc168
                           .data:0000c051 c3 db c0                         jp 0xc0db
                           .data:0000c054 c3 89 c3                         jp 0xc389
                           .data:0000c057 c3 01 c3                         jp 0xc301
                           .data:0000c05a c3 db c3                         jp 0xc3db
                           .data:0000c05d c3 f7 c3                         jp 0xc3f7
                           .data:0000c060 c3 35 c4                         jp 0xc435
                           .data:0000c063 c3 45 c4                         jp 0xc445
                           .data:0000c066 c3 e3 c3                         jp 0xc3e3
                           .data:0000c069 c3 ff c3                         jp 0xc3ff
                           .data:0000c06c c3 3a c4                         jp 0xc43a
                           .data:0000c06f c3 4b c4                         jp 0xc44b
                           .data:0000c072 43                               ld b,e
                           .data:0000c073 50                               ld d,b
                           .data:0000c074 4d                               ld c,l
                           .data:0000c075 20 52                            jr nz,0xc0c9
                           .data:0000c077 4f                               ld c,a
                           .data:0000c078 cd 43 50                         call 0x5043
                           .data:0000c07b cd 44 49                         call 0x4944
                           .data:0000c07e 53                               ld d,e
                           .data:0000c07f c3 44 49                         jp 0x4944
                           .data:0000c082 53                               ld d,e
                           .data:0000c083 43                               ld b,e
                           .data:0000c084 2e 49                            ld l,0x49
                           .data:0000c086 ce 44                            adc a,0x44
                           .data:0000c088 49                               ld c,c
                           .data:0000c089 53                               ld d,e
                           .data:0000c08a 43                               ld b,e
                           .data:0000c08b 2e 4f                            ld l,0x4f
                           .data:0000c08d 55                               ld d,l
                           .data:0000c08e d4 54 41                         call nc,0x4154
                           .data:0000c091 50                               ld d,b
                           .data:0000c092 c5                               push bc
                           .data:0000c093 54                               ld d,h
                           .data:0000c094 41                               ld b,c
                           .data:0000c095 50                               ld d,b
                           .data:0000c096 45                               ld b,l
                           .data:0000c097 2e 49                            ld l,0x49
                           .data:0000c099 ce 54                            adc a,0x54
                           .data:0000c09b 41                               ld b,c
                           .data:0000c09c 50                               ld d,b
                           .data:0000c09d 45                               ld b,l
                           .data:0000c09e 2e 4f                            ld l,0x4f
                           .data:0000c0a0 55                               ld d,l
                           .data:0000c0a1 d4 c1 c2                         call nc,0xc2c1
                           .data:0000c0a4 44                               ld b,h
                           .data:0000c0a5 52                               ld d,d
                           .data:0000c0a6 49                               ld c,c
                           .data:0000c0a7 56                               ld d,(hl)
                           .data:0000c0a8 c5                               push bc
                           .data:0000c0a9 55                               ld d,l
                           .data:0000c0aa 53                               ld d,e
                           .data:0000c0ab 45                               ld b,l
                           .data:0000c0ac d2 44 49                         jp nc,0x4944
                           .data:0000c0af d2 45 52                         jp nc,0x5245
                           .data:0000c0b2 c1                               pop bc
                           .data:0000c0b3 52                               ld d,d
                           .data:0000c0b4 45                               ld b,l
                           .data:0000c0b5 ce 81                            adc a,0x81
                           .data:0000c0b7 82                               add a,d
                           .data:0000c0b8 83                               add a,e
                           .data:0000c0b9 84                               add a,h
                           .data:0000c0ba 85                               add a,l
                           .data:0000c0bb 86                               add a,(hl)
                           .data:0000c0bc 87                               add a,a
                           .data:0000c0bd 88                               adc a,b
                           .data:0000c0be 89                               adc a,c
                           .data:0000c0bf 00                               nop
                           .data:0000c0c0 2a 39 00                         ld hl,(0x0039)
                           .data:0000c0c3 22 3e ad                         ld (0xad3e),hl
                           .data:0000c0c6 3e c3                            ld a,0xc3
                           .data:0000c0c8 32 33 ad                         ld (0xad33),a
                           .data:0000c0cb af                               xor a
                           .data:0000c0cc 32 40 ad                         ld (0xad40),a
                           .data:0000c0cf f3                               di
                           .data:0000c0d0 d9                               exx
                           .data:0000c0d1 ed 43 3c ad                      ld (0xad3c),bc
                           .data:0000c0d5 d9                               exx
                           .data:0000c0d6 21 fa c0                         ld hl,0xc0fa
                           .data:0000c0d9 18 1a                            jr 0xc0f5
                           .data:0000c0db 21 40 ad                         ld hl,0xad40
                           .data:0000c0de be                               cp (hl)
                           .data:0000c0df c8                               ret z
                           .data:0000c0e0 c5                               push bc
                           .data:0000c0e1 46                               ld b,(hl)
                           .data:0000c0e2 77                               ld (hl),a
                           .data:0000c0e3 b7                               or a
                           .data:0000c0e4 78                               ld a,b
                           .data:0000c0e5 c1                               pop bc
                           .data:0000c0e6 28 e7                            jr z,0xc0cf
                           .data:0000c0e8 f3                               di
                           .data:0000c0e9 08                               ex af,af'
                           .data:0000c0ea d9                               exx
                           .data:0000c0eb ed 4b 3c ad                      ld bc,(0xad3c)
                           .data:0000c0ef b7                               or a
                           .data:0000c0f0 08                               ex af,af'
                           .data:0000c0f1 d9                               exx
                           .data:0000c0f2 21 32 c1                         ld hl,0xc132
                           .data:0000c0f5 22 34 ad                         ld (0xad34),hl
                           .data:0000c0f8 fb                               ei
                           .data:0000c0f9 c9                               ret
                           .data:0000c0fa f3                               di
                           .data:0000c0fb 08                               ex af,af'
                           .data:0000c0fc d9                               exx
                           .data:0000c0fd 22 38 ad                         ld (0xad38),hl
                           .data:0000c100 e1                               pop hl
                           .data:0000c101 ed 73 36 ad                      ld (0xad36),sp
                           .data:0000c105 31 00 c0                         ld sp,0xc000
                           .data:0000c108 d5                               push de
                           .data:0000c109 c5                               push bc
                           .data:0000c10a f5                               push af
                           .data:0000c10b fd e5                            push iy
                           .data:0000c10d ed 4b 3c ad                      ld bc,(0xad3c)
                           .data:0000c111 b7                               or a
                           .data:0000c112 cd 4f c1                         call 0xc14f
                           .data:0000c115 f3                               di
                           .data:0000c116 08                               ex af,af'
                           .data:0000c117 d9                               exx
                           .data:0000c118 ed 43 3c ad                      ld (0xad3c),bc
                           .data:0000c11c 21 63 c1                         ld hl,0xc163
                           .data:0000c11f 22 39 00                         ld (0x0039),hl
                           .data:0000c122 fd e1                            pop iy
                           .data:0000c124 f1                               pop af
                           .data:0000c125 c1                               pop bc
                           .data:0000c126 d1                               pop de
                           .data:0000c127 2a 38 ad                         ld hl,(0xad38)
                           .data:0000c12a 08                               ex af,af'
                           .data:0000c12b d9                               exx
                           .data:0000c12c ed 7b 36 ad                      ld sp,(0xad36)
                           .data:0000c130 fb                               ei
                           .data:0000c131 c9                               ret
                           .data:0000c132 f3                               di
                           .data:0000c133 08                               ex af,af'
                           .data:0000c134 d9                               exx
                           .data:0000c135 e1                               pop hl
                           .data:0000c136 ed 73 36 ad                      ld (0xad36),sp
                           .data:0000c13a 31 00 c0                         ld sp,0xc000
                           .data:0000c13d cd 4f c1                         call 0xc14f
                           .data:0000c140 f3                               di
                           .data:0000c141 d9                               exx
                           .data:0000c142 21 63 c1                         ld hl,0xc163
                           .data:0000c145 22 39 00                         ld (0x0039),hl
                           .data:0000c148 d9                               exx
                           .data:0000c149 ed 7b 36 ad                      ld sp,(0xad36)
                           .data:0000c14d fb                               ei
                           .data:0000c14e c9                               ret
                           .data:0000c14f ed 5b 3e ad                      ld de,(0xad3e)
                           .data:0000c153 ed 53 39 00                      ld (0x0039),de
                           .data:0000c157 fd 21 48 ac                      ld iy,0xac48
                           .data:0000c15b 5e                               ld e,(hl)
                           .data:0000c15c 23                               inc hl
                           .data:0000c15d 56                               ld d,(hl)
                           .data:0000c15e d5                               push de
                           .data:0000c15f 08                               ex af,af'
                           .data:0000c160 d9                               exx
                           .data:0000c161 fb                               ei
                           .data:0000c162 c9                               ret
                           .data:0000c163 cd 33 ad                         call 0xad33
                           .data:0000c166 38 00                            jr c,0xc168
                           .data:0000c168 22 3a ad                         ld (0xad3a),hl
                           .data:0000c16b e1                               pop hl
                           .data:0000c16c e5                               push hl
                           .data:0000c16d 23                               inc hl
                           .data:0000c16e 23                               inc hl
                           .data:0000c16f e3                               ex (sp),hl
                           .data:0000c170 e5                               push hl
                           .data:0000c171 2a 3a ad                         ld hl,(0xad3a)
                           .data:0000c174 c3 33 ad                         jp 0xad33
                           .data:0000c177 21 63 c1                         ld hl,0xc163
                           .data:0000c17a 22 39 00                         ld (0x0039),hl
                           .data:0000c17d eb                               ex de,hl
                           .data:0000c17e e9                               jp (hl)
                           .data:0000c17f c3 b2 c1                         jp 0xc1b2
                           .data:0000c182 c3 be c2                         jp 0xc2be
                           .data:0000c185 c3 e1 c2                         jp 0xc2e1
                           .data:0000c188 c3 c3 c2                         jp 0xc2c3
                           .data:0000c18b c3 c8 c2                         jp 0xc2c8
                           .data:0000c18e c3 d2 c2                         jp 0xc2d2
                           .data:0000c191 c3 d7 c2                         jp 0xc2d7
                           .data:0000c194 c3 dc c2                         jp 0xc2dc
                           .data:0000c197 c3 e9 c2                         jp 0xc2e9
                           .data:0000c19a c3 f2 c2                         jp 0xc2f2
                           .data:0000c19d c3 24 c5                         jp 0xc524
                           .data:0000c1a0 c3 29 c5                         jp 0xc529
                           .data:0000c1a3 c3 1a c5                         jp 0xc51a
                           .data:0000c1a6 c3 f7 c2                         jp 0xc2f7
                           .data:0000c1a9 c3 fc c2                         jp 0xc2fc
                           .data:0000c1ac c3 cd c2                         jp 0xc2cd
                           .data:0000c1af c3 5a c5                         jp 0xc55a
                           .data:0000c1b2 cd 12 b9                         call 0xb912
                           .data:0000c1b5 4f                               ld c,a
                           .data:0000c1b6 21 dc c1                         ld hl,0xc1dc
                           .data:0000c1b9 c3 16 bd                         jp 0xbd16
                           .data:0000c1bc 38 06                            jr c,0xc1c4
                           .data:0000c1be cd 12 b9                         call 0xb912
                           .data:0000c1c1 b7                               or a
                           .data:0000c1c2 28 18                            jr z,0xc1dc
                           .data:0000c1c4 fd e5                            push iy
                           .data:0000c1c6 d5                               push de
                           .data:0000c1c7 11 00 fb                         ld de,0xfb00
                           .data:0000c1ca 19                               add hl,de
                           .data:0000c1cb e5                               push hl
                           .data:0000c1cc 23                               inc hl
                           .data:0000c1cd e5                               push hl
                           .data:0000c1ce fd e1                            pop iy
                           .data:0000c1d0 cd dd c5                         call 0xc5dd
                           .data:0000c1d3 cd a0 cc                         call 0xcca0
                           .data:0000c1d6 e1                               pop hl
                           .data:0000c1d7 d1                               pop de
                           .data:0000c1d8 fd e1                            pop iy
                           .data:0000c1da 37                               scf
                           .data:0000c1db c9                               ret
                           .data:0000c1dc 31 00 c0                         ld sp,0xc000
                           .data:0000c1df fd 21 48 ac                      ld iy,0xac48
                           .data:0000c1e3 11 33 ad                         ld de,0xad33
                           .data:0000c1e6 01 a5 00                         ld bc,0x00a5
                           .data:0000c1e9 cd af ca                         call 0xcaaf
                           .data:0000c1ec 21 41 ad                         ld hl,0xad41
                           .data:0000c1ef 35                               dec (hl)
                           .data:0000c1f0 3e 81                            ld a,0x81
                           .data:0000c1f2 32 03 00                         ld (0x0003),a
                           .data:0000c1f5 af                               xor a
                           .data:0000c1f6 32 04 00                         ld (0x0004),a
                           .data:0000c1f9 21 33 c0                         ld hl,0xc033
                           .data:0000c1fc 11 80 be                         ld de,0xbe80
                           .data:0000c1ff 01 3f 00                         ld bc,0x003f
                           .data:0000c202 ed b0                            ldir
                           .data:0000c204 cd c0 c0                         call 0xc0c0
                           .data:0000c207 cd dd c5                         call 0xc5dd
                           .data:0000c20a 0e 41                            ld c,0x41
                           .data:0000c20c 11 00 00                         ld de,0x0000
                           .data:0000c20f 21 00 01                         ld hl,0x0100
                           .data:0000c212 cd 66 c6                         call 0xc666
                           .data:0000c215 dc ac c2                         call c,0xc2ac
                           .data:0000c218 30 0a                            jr nc,0xc224
                           .data:0000c21a eb                               ex de,hl
                           .data:0000c21b 01 7f c1                         ld bc,0xc17f
                           .data:0000c21e 31 33 ad                         ld sp,0xad33
                           .data:0000c221 c3 77 c1                         jp 0xc177
                           .data:0000c224 3e 0f                            ld a,0x0f
                           .data:0000c226 cd b8 ca                         call 0xcab8
                           .data:0000c229 18 df                            jr 0xc20a
                           .data:0000c22b cd 6f c8                         call 0xc86f
                           .data:0000c22e cd b0 c8                         call 0xc8b0
                           .data:0000c231 01 48 01                         ld bc,0x0148
                           .data:0000c234 11 00 00                         ld de,0x0000
                           .data:0000c237 e5                               push hl
                           .data:0000c238 cd 99 c2                         call 0xc299
                           .data:0000c23b e1                               pop hl
                           .data:0000c23c dc ac c2                         call c,0xc2ac
                           .data:0000c23f 30 51                            jr nc,0xc292
                           .data:0000c241 e5                               push hl
                           .data:0000c242 23                               inc hl
                           .data:0000c243 5e                               ld e,(hl)
                           .data:0000c244 23                               inc hl
                           .data:0000c245 56                               ld d,(hl)
                           .data:0000c246 21 a4 fc                         ld hl,0xfca4
                           .data:0000c249 19                               add hl,de
                           .data:0000c24a eb                               ex de,hl
                           .data:0000c24b e1                               pop hl
                           .data:0000c24c 01 00 02                         ld bc,0x0200
                           .data:0000c24f ed b0                            ldir
                           .data:0000c251 eb                               ex de,hl
                           .data:0000c252 01 49 0a                         ld bc,0x0a49
                           .data:0000c255 11 00 00                         ld de,0x0000
                           .data:0000c258 cd 99 c2                         call 0xc299
                           .data:0000c25b 30 35                            jr nc,0xc292
                           .data:0000c25d eb                               ex de,hl
                           .data:0000c25e 21 00 ea                         ld hl,0xea00
                           .data:0000c261 19                               add hl,de
                           .data:0000c262 e5                               push hl
                           .data:0000c263 21 06 f2                         ld hl,0xf206
                           .data:0000c266 19                               add hl,de
                           .data:0000c267 3e c3                            ld a,0xc3
                           .data:0000c269 32 05 00                         ld (0x0005),a
                           .data:0000c26c 22 06 00                         ld (0x0006),hl
                           .data:0000c26f 32 00 00                         ld (0x0000),a
                           .data:0000c272 21 03 00                         ld hl,0x0003
                           .data:0000c275 19                               add hl,de
                           .data:0000c276 22 01 00                         ld (0x0001),hl
                           .data:0000c279 21 7f c1                         ld hl,0xc17f
                           .data:0000c27c 01 33 00                         ld bc,0x0033
                           .data:0000c27f ed b0                            ldir
                           .data:0000c281 21 04 00                         ld hl,0x0004
                           .data:0000c284 7e                               ld a,(hl)
                           .data:0000c285 e6 0f                            and 0x0f
                           .data:0000c287 fe 02                            cp 0x02
                           .data:0000c289 38 02                            jr c,0xc28d
                           .data:0000c28b 36 00                            ld (hl),0x00
                           .data:0000c28d 4e                               ld c,(hl)
                           .data:0000c28e d1                               pop de
                           .data:0000c28f c3 77 c1                         jp 0xc177
                           .data:0000c292 3e 0e                            ld a,0x0e
                           .data:0000c294 cd b8 ca                         call 0xcab8
                           .data:0000c297 18 92                            jr 0xc22b
                           .data:0000c299 cd 66 c6                         call 0xc666
                           .data:0000c29c d0                               ret nc
                           .data:0000c29d 79                               ld a,c
                           .data:0000c29e 0c                               inc c
                           .data:0000c29f fe 49                            cp 0x49
                           .data:0000c2a1 38 03                            jr c,0xc2a6
                           .data:0000c2a3 0e 41                            ld c,0x41
                           .data:0000c2a5 14                               inc d
                           .data:0000c2a6 24                               inc h
                           .data:0000c2a7 24                               inc h
                           .data:0000c2a8 10 ef                            djnz 0xc299
                           .data:0000c2aa 37                               scf
                           .data:0000c2ab c9                               ret
                           .data:0000c2ac e5                               push hl
                           .data:0000c2ad 01 02 00                         ld bc,0x0002
                           .data:0000c2b0 7e                               ld a,(hl)
                           .data:0000c2b1 be                               cp (hl)
                           .data:0000c2b2 23                               inc hl
                           .data:0000c2b3 37                               scf
                           .data:0000c2b4 20 06                            jr nz,0xc2bc
                           .data:0000c2b6 10 f9                            djnz 0xc2b1
                           .data:0000c2b8 0d                               dec c
                           .data:0000c2b9 20 f6                            jr nz,0xc2b1
                           .data:0000c2bb b7                               or a
                           .data:0000c2bc e1                               pop hl
                           .data:0000c2bd c9                               ret
                           .data:0000c2be cd 33 ad                         call 0xad33
                           .data:0000c2c1 2b                               dec hl
                           .data:0000c2c2 c2 21 86                         jp nz,0x8621
                           .data:0000c2c5 c4 18 1c                         call nz,0x1c18
                           .data:0000c2c8 21 8f c4                         ld hl,0xc48f
                           .data:0000c2cb 18 17                            jr 0xc2e4
                           .data:0000c2cd 21 98 c4                         ld hl,0xc498
                           .data:0000c2d0 18 12                            jr 0xc2e4
                           .data:0000c2d2 21 a1 c4                         ld hl,0xc4a1
                           .data:0000c2d5 18 0d                            jr 0xc2e4
                           .data:0000c2d7 21 aa c4                         ld hl,0xc4aa
                           .data:0000c2da 18 08                            jr 0xc2e4
                           .data:0000c2dc 21 bc c4                         ld hl,0xc4bc
                           .data:0000c2df 18 03                            jr 0xc2e4
                           .data:0000c2e1 21 7d c4                         ld hl,0xc47d
                           .data:0000c2e4 cd 33 ad                         call 0xad33
                           .data:0000c2e7 6a                               ld l,d
                           .data:0000c2e8 c4 cd 68                         call nz,0x68cd
                           .data:0000c2eb c1                               pop bc
                           .data:0000c2ec 1f                               rra
                           .data:0000c2ed c5                               push bc
                           .data:0000c2ee 21 89 be                         ld hl,0xbe89
                           .data:0000c2f1 c9                               ret
                           .data:0000c2f2 cd 33 ad                         call 0xad33
                           .data:0000c2f5 f0                               ret p
                           .data:0000c2f6 c4 cd 33                         call nz,0x33cd
                           .data:0000c2f9 ad                               xor l
                           .data:0000c2fa 4c                               ld c,h
                           .data:0000c2fb c5                               push bc
                           .data:0000c2fc cd 33 ad                         call 0xad33
                           .data:0000c2ff 2e c5                            ld l,0xc5
                           .data:0000c301 32 c5 ad                         ld (0xadc5),a
                           .data:0000c304 01 81 00                         ld bc,0x0081
                           .data:0000c307 11 42 ad                         ld de,0xad42
                           .data:0000c30a ed b0                            ldir
                           .data:0000c30c 21 43 ad                         ld hl,0xad43
                           .data:0000c30f 22 c3 ad                         ld (0xadc3),hl
                           .data:0000c312 c9                               ret
                           .data:0000c313 21 41 ad                         ld hl,0xad41
                           .data:0000c316 7e                               ld a,(hl)
                           .data:0000c317 b7                               or a
                           .data:0000c318 28 04                            jr z,0xc31e
                           .data:0000c31a 35                               dec (hl)
                           .data:0000c31b cc 81 bb                         call z,0xbb81
                           .data:0000c31e cd 09 bb                         call 0xbb09
                           .data:0000c321 dc 0c bb                         call c,0xbb0c
                           .data:0000c324 9f                               sbc a,a
                           .data:0000c325 c9                               ret
                           .data:0000c326 21 42 ad                         ld hl,0xad42
                           .data:0000c329 7e                               ld a,(hl)
                           .data:0000c32a b7                               or a
                           .data:0000c32b 28 1b                            jr z,0xc348
                           .data:0000c32d cd 09 bb                         call 0xbb09
                           .data:0000c330 30 0c                            jr nc,0xc33e
                           .data:0000c332 21 c5 ad                         ld hl,0xadc5
                           .data:0000c335 34                               inc (hl)
                           .data:0000c336 35                               dec (hl)
                           .data:0000c337 c0                               ret nz
                           .data:0000c338 21 42 ad                         ld hl,0xad42
                           .data:0000c33b 36 00                            ld (hl),0x00
                           .data:0000c33d c9                               ret
                           .data:0000c33e 35                               dec (hl)
                           .data:0000c33f 2a c3 ad                         ld hl,(0xadc3)
                           .data:0000c342 7e                               ld a,(hl)
                           .data:0000c343 23                               inc hl
                           .data:0000c344 22 c3 ad                         ld (0xadc3),hl
                           .data:0000c347 c9                               ret
                           .data:0000c348 21 41 ad                         ld hl,0xad41
                           .data:0000c34b 7e                               ld a,(hl)
                           .data:0000c34c b7                               or a
                           .data:0000c34d c4 81 bb                         call nz,0xbb81
                           .data:0000c350 36 00                            ld (hl),0x00
                           .data:0000c352 c3 06 bb                         jp 0xbb06
                           .data:0000c355 3e 1a                            ld a,0x1a
                           .data:0000c357 c9                               ret
                           .data:0000c358 3e ff                            ld a,0xff
                           .data:0000c35a c9                               ret
                           .data:0000c35b 21 41 ad                         ld hl,0xad41
                           .data:0000c35e 7e                               ld a,(hl)
                           .data:0000c35f b7                               or a
                           .data:0000c360 cc 84 bb                         call z,0xbb84
                           .data:0000c363 36 ff                            ld (hl),0xff
                           .data:0000c365 79                               ld a,c
                           .data:0000c366 cd 5a bb                         call 0xbb5a
                           .data:0000c369 fe 20                            cp 0x20
                           .data:0000c36b d0                               ret nc
                           .data:0000c36c cd 78 bb                         call 0xbb78
                           .data:0000c36f cd 87 bb                         call 0xbb87
                           .data:0000c372 d8                               ret c
                           .data:0000c373 cd 8a bb                         call 0xbb8a
                           .data:0000c376 c3 8d bb                         jp 0xbb8d
                           .data:0000c379 cd 2e bd                         call 0xbd2e
                           .data:0000c37c 3f                               ccf
                           .data:0000c37d 9f                               sbc a,a
                           .data:0000c37e c9                               ret
                           .data:0000c37f 79                               ld a,c
                           .data:0000c380 cd 2b bd                         call 0xbd2b
                           .data:0000c383 d8                               ret c
                           .data:0000c384 cd d3 c4                         call 0xc4d3
                           .data:0000c387 18 f6                            jr 0xc37f
                           .data:0000c389 f3                               di
                           .data:0000c38a 01 dd fa                         ld bc,0xfadd
                           .data:0000c38d 11 c6 ad                         ld de,0xadc6
                           .data:0000c390 cd bd c3                         call 0xc3bd
                           .data:0000c393 03                               inc bc
                           .data:0000c394 03                               inc bc
                           .data:0000c395 13                               inc de
                           .data:0000c396 cd bd c3                         call 0xc3bd
                           .data:0000c399 3e 36                            ld a,0x36
                           .data:0000c39b 1e dc                            ld e,0xdc
                           .data:0000c39d cd ae c3                         call 0xc3ae
                           .data:0000c3a0 3e 76                            ld a,0x76
                           .data:0000c3a2 1c                               inc e
                           .data:0000c3a3 cd ae c3                         call 0xc3ae
                           .data:0000c3a6 3e b6                            ld a,0xb6
                           .data:0000c3a8 1c                               inc e
                           .data:0000c3a9 cd ae c3                         call 0xc3ae
                           .data:0000c3ac fb                               ei
                           .data:0000c3ad c9                               ret
                           .data:0000c3ae 01 df fb                         ld bc,0xfbdf
                           .data:0000c3b1 ed 79                            out (c),a
                           .data:0000c3b3 4b                               ld c,e
                           .data:0000c3b4 7e                               ld a,(hl)
                           .data:0000c3b5 23                               inc hl
                           .data:0000c3b6 ed 79                            out (c),a
                           .data:0000c3b8 7e                               ld a,(hl)
                           .data:0000c3b9 23                               inc hl
                           .data:0000c3ba ed 79                            out (c),a
                           .data:0000c3bc c9                               ret
                           .data:0000c3bd 3e 18                            ld a,0x18
                           .data:0000c3bf ed 79                            out (c),a
                           .data:0000c3c1 3e 04                            ld a,0x04
                           .data:0000c3c3 ed 79                            out (c),a
                           .data:0000c3c5 7e                               ld a,(hl)
                           .data:0000c3c6 23                               inc hl
                           .data:0000c3c7 ed 79                            out (c),a
                           .data:0000c3c9 3e 05                            ld a,0x05
                           .data:0000c3cb ed 79                            out (c),a
                           .data:0000c3cd 7e                               ld a,(hl)
                           .data:0000c3ce 12                               ld (de),a
                           .data:0000c3cf 23                               inc hl
                           .data:0000c3d0 ed 79                            out (c),a
                           .data:0000c3d2 3e 03                            ld a,0x03
                           .data:0000c3d4 ed 79                            out (c),a
                           .data:0000c3d6 7e                               ld a,(hl)
                           .data:0000c3d7 23                               inc hl
                           .data:0000c3d8 ed 79                            out (c),a
                           .data:0000c3da c9                               ret
                           .data:0000c3db 01 dd fa                         ld bc,0xfadd
                           .data:0000c3de 21 c6 ad                         ld hl,0xadc6
                           .data:0000c3e1 18 06                            jr 0xc3e9
                           .data:0000c3e3 01 df fa                         ld bc,0xfadf
                           .data:0000c3e6 21 c7 ad                         ld hl,0xadc7
                           .data:0000c3e9 ed 78                            in a,(c)
                           .data:0000c3eb 0f                               rrca
                           .data:0000c3ec 9f                               sbc a,a
                           .data:0000c3ed d8                               ret c
                           .data:0000c3ee cd 24 c4                         call 0xc424
                           .data:0000c3f1 ed 78                            in a,(c)
                           .data:0000c3f3 0f                               rrca
                           .data:0000c3f4 9f                               sbc a,a
                           .data:0000c3f5 18 29                            jr 0xc420
                           .data:0000c3f7 01 dd fa                         ld bc,0xfadd
                           .data:0000c3fa 21 c6 ad                         ld hl,0xadc6
                           .data:0000c3fd 18 06                            jr 0xc405
                           .data:0000c3ff 01 df fa                         ld bc,0xfadf
                           .data:0000c402 21 c7 ad                         ld hl,0xadc7
                           .data:0000c405 ed 78                            in a,(c)
                           .data:0000c407 0f                               rrca
                           .data:0000c408 38 12                            jr c,0xc41c
                           .data:0000c40a cd 24 c4                         call 0xc424
                           .data:0000c40d cd c5 c4                         call 0xc4c5
                           .data:0000c410 fe 1a                            cp 0x1a
                           .data:0000c412 28 0c                            jr z,0xc420
                           .data:0000c414 ed 78                            in a,(c)
                           .data:0000c416 0f                               rrca
                           .data:0000c417 30 f4                            jr nc,0xc40d
                           .data:0000c419 cd 20 c4                         call 0xc420
                           .data:0000c41c 0b                               dec bc
                           .data:0000c41d ed 78                            in a,(c)
                           .data:0000c41f c9                               ret
                           .data:0000c420 1e 00                            ld e,0x00
                           .data:0000c422 18 02                            jr 0xc426
                           .data:0000c424 1e 80                            ld e,0x80
                           .data:0000c426 f3                               di
                           .data:0000c427 f5                               push af
                           .data:0000c428 3e 05                            ld a,0x05
                           .data:0000c42a ed 79                            out (c),a
                           .data:0000c42c 7e                               ld a,(hl)
                           .data:0000c42d e6 7f                            and 0x7f
                           .data:0000c42f b3                               or e
                           .data:0000c430 ed 79                            out (c),a
                           .data:0000c432 f1                               pop af
                           .data:0000c433 fb                               ei
                           .data:0000c434 c9                               ret
                           .data:0000c435 01 dd fa                         ld bc,0xfadd
                           .data:0000c438 18 03                            jr 0xc43d
                           .data:0000c43a 01 df fa                         ld bc,0xfadf
                           .data:0000c43d ed 78                            in a,(c)
                           .data:0000c43f e6 04                            and 0x04
                           .data:0000c441 c8                               ret z
                           .data:0000c442 37                               scf
                           .data:0000c443 9f                               sbc a,a
                           .data:0000c444 c9                               ret
                           .data:0000c445 79                               ld a,c
                           .data:0000c446 01 dd fa                         ld bc,0xfadd
                           .data:0000c449 18 04                            jr 0xc44f
                           .data:0000c44b 79                               ld a,c
                           .data:0000c44c 01 df fa                         ld bc,0xfadf
                           .data:0000c44f f5                               push af
                           .data:0000c450 cd d3 c4                         call 0xc4d3
                           .data:0000c453 cd 3d c4                         call 0xc43d
                           .data:0000c456 30 f8                            jr nc,0xc450
                           .data:0000c458 f1                               pop af
                           .data:0000c459 0b                               dec bc
                           .data:0000c45a ed 79                            out (c),a
                           .data:0000c45c c9                               ret
                           .data:0000c45d 21 b3 c4                         ld hl,0xc4b3
                           .data:0000c460 18 08                            jr 0xc46a
                           .data:0000c462 21 bc c4                         ld hl,0xc4bc
                           .data:0000c465 18 03                            jr 0xc46a
                           .data:0000c467 21 a1 c4                         ld hl,0xc4a1
                           .data:0000c46a 46                               ld b,(hl)
                           .data:0000c46b 23                               inc hl
                           .data:0000c46c 3a 03 00                         ld a,(0x0003)
                           .data:0000c46f 07                               rlca
                           .data:0000c470 10 fd                            djnz 0xc46f
                           .data:0000c472 e6 06                            and 0x06
                           .data:0000c474 16 00                            ld d,0x00
                           .data:0000c476 5f                               ld e,a
                           .data:0000c477 19                               add hl,de
                           .data:0000c478 5e                               ld e,(hl)
                           .data:0000c479 23                               inc hl
                           .data:0000c47a 56                               ld d,(hl)
                           .data:0000c47b eb                               ex de,hl
                           .data:0000c47c e9                               jp (hl)
                           .data:0000c47d 01 a7 be                         ld bc,0xbea7
                           .data:0000c480 13                               inc de
                           .data:0000c481 c3 5d c4                         jp 0xc45d
                           .data:0000c484 b3                               or e
                           .data:0000c485 be                               cp (hl)
                           .data:0000c486 01 aa be                         ld bc,0xbeaa
                           .data:0000c489 26 c3                            ld h,0xc3
                           .data:0000c48b 62                               ld h,d
                           .data:0000c48c c4 b6 be                         call nz,0xbeb6
                           .data:0000c48f 01 b0 be                         ld bc,0xbeb0
                           .data:0000c492 5b                               ld e,e
                           .data:0000c493 c3 67 c4                         jp 0xc467
                           .data:0000c496 bc                               cp h
                           .data:0000c497 be                               cp (hl)
                           .data:0000c498 03                               inc bc
                           .data:0000c499 ad                               xor l
                           .data:0000c49a be                               cp (hl)
                           .data:0000c49b 58                               ld e,b
                           .data:0000c49c c3 79 c3                         jp 0xc379
                           .data:0000c49f b9                               cp c
                           .data:0000c4a0 be                               cp (hl)
                           .data:0000c4a1 03                               inc bc
                           .data:0000c4a2 b0                               or b
                           .data:0000c4a3 be                               cp (hl)
                           .data:0000c4a4 5b                               ld e,e
                           .data:0000c4a5 c3 7f c3                         jp 0xc37f
                           .data:0000c4a8 bc                               cp h
                           .data:0000c4a9 be                               cp (hl)
                           .data:0000c4aa 05                               dec b
                           .data:0000c4ab b0                               or b
                           .data:0000c4ac be                               cp (hl)
                           .data:0000c4ad 5a                               ld e,d
                           .data:0000c4ae c3 bc be                         jp 0xbebc
                           .data:0000c4b1 5b                               ld e,e
                           .data:0000c4b2 c3 07 a7                         jp 0xa707
                           .data:0000c4b5 be                               cp (hl)
                           .data:0000c4b6 58                               ld e,b
                           .data:0000c4b7 c3 b3 be                         jp 0xbeb3
                           .data:0000c4ba 13                               inc de
                           .data:0000c4bb c3 07 aa                         jp 0xaa07
                           .data:0000c4be be                               cp (hl)
                           .data:0000c4bf 55                               ld d,l
                           .data:0000c4c0 c3 b6 be                         jp 0xbeb6
                           .data:0000c4c3 26 c3                            ld h,0xc3
                           .data:0000c4c5 cd d3 c4                         call 0xc4d3
                           .data:0000c4c8 fe 13                            cp 0x13
                           .data:0000c4ca c0                               ret nz
                           .data:0000c4cb e5                               push hl
                           .data:0000c4cc c5                               push bc
                           .data:0000c4cd cd 26 c3                         call 0xc326
                           .data:0000c4d0 c1                               pop bc
                           .data:0000c4d1 e1                               pop hl
                           .data:0000c4d2 c9                               ret
                           .data:0000c4d3 e5                               push hl
                           .data:0000c4d4 d5                               push de
                           .data:0000c4d5 c5                               push bc
                           .data:0000c4d6 cd 13 c3                         call 0xc313
                           .data:0000c4d9 b7                               or a
                           .data:0000c4da 28 0f                            jr z,0xc4eb
                           .data:0000c4dc cd 26 c3                         call 0xc326
                           .data:0000c4df fe 03                            cp 0x03
                           .data:0000c4e1 20 08                            jr nz,0xc4eb
                           .data:0000c4e3 3e 0d                            ld a,0x0d
                           .data:0000c4e5 cd eb ca                         call 0xcaeb
                           .data:0000c4e8 c3 2b c2                         jp 0xc22b
                           .data:0000c4eb c1                               pop bc
                           .data:0000c4ec d1                               pop de
                           .data:0000c4ed e1                               pop hl
                           .data:0000c4ee c9                               ret
                           .data:0000c4ef ff                               rst 0x38
                           .data:0000c4f0 79                               ld a,c
                           .data:0000c4f1 fe 02                            cp 0x02
                           .data:0000c4f3 21 00 00                         ld hl,0x0000
                           .data:0000c4f6 d0                               ret nc
                           .data:0000c4f7 7b                               ld a,e
                           .data:0000c4f8 1f                               rra
                           .data:0000c4f9 38 0f                            jr c,0xc50a
                           .data:0000c4fb 59                               ld e,c
                           .data:0000c4fc 3e 18                            ld a,0x18
                           .data:0000c4fe cd 5c ca                         call 0xca5c
                           .data:0000c501 b7                               or a
                           .data:0000c502 20 06                            jr nz,0xc50a
                           .data:0000c504 e5                               push hl
                           .data:0000c505 cd 6c c5                         call 0xc56c
                           .data:0000c508 e1                               pop hl
                           .data:0000c509 d0                               ret nc
                           .data:0000c50a 79                               ld a,c
                           .data:0000c50b 32 53 be                         ld (0xbe53),a
                           .data:0000c50e 21 10 02                         ld hl,0x0210
                           .data:0000c511 b7                               or a
                           .data:0000c512 28 03                            jr z,0xc517
                           .data:0000c514 21 20 02                         ld hl,0x0220
                           .data:0000c517 c3 9f ca                         jp 0xca9f
                           .data:0000c51a ed 43 60 be                      ld (0xbe60),bc
                           .data:0000c51e c9                               ret
                           .data:0000c51f cd 6f c8                         call 0xc86f
                           .data:0000c522 0e 00                            ld c,0x00
                           .data:0000c524 79                               ld a,c
                           .data:0000c525 32 54 be                         ld (0xbe54),a
                           .data:0000c528 c9                               ret
                           .data:0000c529 79                               ld a,c
                           .data:0000c52a 32 55 be                         ld (0xbe55),a
                           .data:0000c52d c9                               ret
                           .data:0000c52e c5                               push bc
                           .data:0000c52f 79                               ld a,c
                           .data:0000c530 fe 02                            cp 0x02
                           .data:0000c532 cc eb c7                         call z,0xc7eb
                           .data:0000c535 cd 00 c8                         call 0xc800
                           .data:0000c538 dc 1b c8                         call c,0xc81b
                           .data:0000c53b cd 32 c8                         call 0xc832
                           .data:0000c53e c1                               pop bc
                           .data:0000c53f d0                               ret nc
                           .data:0000c540 cd b6 c8                         call 0xc8b6
                           .data:0000c543 0d                               dec c
                           .data:0000c544 37                               scf
                           .data:0000c545 cc 6f c8                         call z,0xc86f
                           .data:0000c548 d0                               ret nc
                           .data:0000c549 3e 00                            ld a,0x00
                           .data:0000c54b c9                               ret
                           .data:0000c54c af                               xor a
                           .data:0000c54d 32 59 be                         ld (0xbe59),a
                           .data:0000c550 cd 32 c8                         call 0xc832
                           .data:0000c553 cd c7 c8                         call 0xc8c7
                           .data:0000c556 d0                               ret nc
                           .data:0000c557 3e 00                            ld a,0x00
                           .data:0000c559 c9                               ret
                           .data:0000c55a 60                               ld h,b
                           .data:0000c55b 69                               ld l,c
                           .data:0000c55c c9                               ret
                           .data:0000c55d 01 7e fb                         ld bc,0xfb7e
                           .data:0000c560 3e 4a                            ld a,0x4a
                           .data:0000c562 cd 5c c9                         call 0xc95c
                           .data:0000c565 7b                               ld a,e
                           .data:0000c566 cd 5c c9                         call 0xc95c
                           .data:0000c569 c3 f9 c8                         jp 0xc8f9
                           .data:0000c56c cd 76 c9                         call 0xc976
                           .data:0000c56f 3e 16                            ld a,0x16
                           .data:0000c571 cd 5c ca                         call 0xca5c
                           .data:0000c574 57                               ld d,a
                           .data:0000c575 0e 10                            ld c,0x10
                           .data:0000c577 21 5d c5                         ld hl,0xc55d
                           .data:0000c57a cd ff c6                         call 0xc6ff
                           .data:0000c57d d0                               ret nc
                           .data:0000c57e 3a 51 be                         ld a,(0xbe51)
                           .data:0000c581 f5                               push af
                           .data:0000c582 af                               xor a
                           .data:0000c583 cd 63 ca                         call 0xca63
                           .data:0000c586 e5                               push hl
                           .data:0000c587 eb                               ex de,hl
                           .data:0000c588 21 43 ca                         ld hl,0xca43
                           .data:0000c58b 01 16 00                         ld bc,0x0016
                           .data:0000c58e ed b0                            ldir
                           .data:0000c590 e1                               pop hl
                           .data:0000c591 f1                               pop af
                           .data:0000c592 e6 c0                            and 0xc0
                           .data:0000c594 fe 40                            cp 0x40
                           .data:0000c596 37                               scf
                           .data:0000c597 c8                               ret z
                           .data:0000c598 11 ca c5                         ld de,0xc5ca
                           .data:0000c59b fe c0                            cp 0xc0
                           .data:0000c59d 28 03                            jr z,0xc5a2
                           .data:0000c59f 11 c0 c5                         ld de,0xc5c0
                           .data:0000c5a2 1a                               ld a,(de)
                           .data:0000c5a3 13                               inc de
                           .data:0000c5a4 77                               ld (hl),a
                           .data:0000c5a5 23                               inc hl
                           .data:0000c5a6 1a                               ld a,(de)
                           .data:0000c5a7 13                               inc de
                           .data:0000c5a8 77                               ld (hl),a
                           .data:0000c5a9 01 04 00                         ld bc,0x0004
                           .data:0000c5ac 09                               add hl,bc
                           .data:0000c5ad 1a                               ld a,(de)
                           .data:0000c5ae 13                               inc de
                           .data:0000c5af 77                               ld (hl),a
                           .data:0000c5b0 23                               inc hl
                           .data:0000c5b1 1a                               ld a,(de)
                           .data:0000c5b2 13                               inc de
                           .data:0000c5b3 77                               ld (hl),a
                           .data:0000c5b4 01 07 00                         ld bc,0x0007
                           .data:0000c5b7 09                               add hl,bc
                           .data:0000c5b8 eb                               ex de,hl
                           .data:0000c5b9 01 06 00                         ld bc,0x0006
                           .data:0000c5bc ed b0                            ldir
                           .data:0000c5be 37                               scf
                           .data:0000c5bf c9                               ret
                           .data:0000c5c0 20 00                            jr nz,0xc5c2
                           .data:0000c5c2 9b                               sbc a,e
                           .data:0000c5c3 00                               nop
                           .data:0000c5c4 01 00 01                         ld bc,0x0100
                           .data:0000c5c7 08                               ex af,af'
                           .data:0000c5c8 2a 50 24                         ld hl,(0x2450)
                           .data:0000c5cb 00                               nop
                           .data:0000c5cc b3                               or e
                           .data:0000c5cd 00                               nop
                           .data:0000c5ce 00                               nop
                           .data:0000c5cf 00                               nop
                           .data:0000c5d0 c1                               pop bc
                           .data:0000c5d1 09                               add hl,bc
                           .data:0000c5d2 2a 52 32                         ld hl,(0x3252)
                           .data:0000c5d5 00                               nop
                           .data:0000c5d6 fa 00 af                         jp m,0xaf00
                           .data:0000c5d9 0f                               rrca
                           .data:0000c5da 0c                               inc c
                           .data:0000c5db 01 03 11                         ld bc,0x1103
                           .data:0000c5de 40                               ld b,b
                           .data:0000c5df be                               cp (hl)
                           .data:0000c5e0 01 3d 00                         ld bc,0x003d
                           .data:0000c5e3 cd af ca                         call 0xcaaf
                           .data:0000c5e6 cd f4 c9                         call 0xc9f4
                           .data:0000c5e9 cd e8 c9                         call 0xc9e8
                           .data:0000c5ec 21 d4 c5                         ld hl,0xc5d4
                           .data:0000c5ef cd 0d c6                         call 0xc60d
                           .data:0000c5f2 cd 12 b9                         call 0xb912
                           .data:0000c5f5 4f                               ld c,a
                           .data:0000c5f6 06 80                            ld b,0x80
                           .data:0000c5f8 21 6d be                         ld hl,0xbe6d
                           .data:0000c5fb 11 d6 c9                         ld de,0xc9d6
                           .data:0000c5fe cd ef bc                         call 0xbcef
                           .data:0000c601 3e 10                            ld a,0x10
                           .data:0000c603 e5                               push hl
                           .data:0000c604 2a 66 be                         ld hl,(0xbe66)
                           .data:0000c607 32 66 be                         ld (0xbe66),a
                           .data:0000c60a 7d                               ld a,l
                           .data:0000c60b e1                               pop hl
                           .data:0000c60c c9                               ret
                           .data:0000c60d 11 44 be                         ld de,0xbe44
                           .data:0000c610 01 07 00                         ld bc,0x0007
                           .data:0000c613 ed b0                            ldir
                           .data:0000c615 01 7e fb                         ld bc,0xfb7e
                           .data:0000c618 3e 03                            ld a,0x03
                           .data:0000c61a cd 5c c9                         call 0xc95c
                           .data:0000c61d 3a 4a be                         ld a,(0xbe4a)
                           .data:0000c620 3d                               dec a
                           .data:0000c621 07                               rlca
                           .data:0000c622 07                               rlca
                           .data:0000c623 07                               rlca
                           .data:0000c624 2f                               cpl
                           .data:0000c625 e6 f0                            and 0xf0
                           .data:0000c627 b6                               or (hl)
                           .data:0000c628 cd 5c c9                         call 0xc95c
                           .data:0000c62b 23                               inc hl
                           .data:0000c62c 7e                               ld a,(hl)
                           .data:0000c62d c3 5c c9                         jp 0xc95c
                           .data:0000c630 cd 38 c6                         call 0xc638
                           .data:0000c633 d0                               ret nc
                           .data:0000c634 3a 4c be                         ld a,(0xbe4c)
                           .data:0000c637 c9                               ret
                           .data:0000c638 cd 76 c9                         call 0xc976
                           .data:0000c63b f5                               push af
                           .data:0000c63c cd 47 c9                         call 0xc947
                           .data:0000c63f 01 7e fb                         ld bc,0xfb7e
                           .data:0000c642 3e 04                            ld a,0x04
                           .data:0000c644 cd 5c c9                         call 0xc95c
                           .data:0000c647 f1                               pop af
                           .data:0000c648 cd 5c c9                         call 0xc95c
                           .data:0000c64b c3 1c c9                         jp 0xc91c
                           .data:0000c64e 3e 45                            ld a,0x45
                           .data:0000c650 18 02                            jr 0xc654
                           .data:0000c652 3e 4d                            ld a,0x4d
                           .data:0000c654 cd 76 c9                         call 0xc976
                           .data:0000c657 06 11                            ld b,0x11
                           .data:0000c659 cd 6d c6                         call 0xc66d
                           .data:0000c65c 3a 48 be                         ld a,(0xbe48)
                           .data:0000c65f 3d                               dec a
                           .data:0000c660 03                               inc bc
                           .data:0000c661 03                               inc bc
                           .data:0000c662 03                               inc bc
                           .data:0000c663 20 fa                            jr nz,0xc65f
                           .data:0000c665 c9                               ret
                           .data:0000c666 cd 76 c9                         call 0xc976
                           .data:0000c669 3e 66                            ld a,0x66
                           .data:0000c66b 06 10                            ld b,0x10
                           .data:0000c66d 22 62 be                         ld (0xbe62),hl
                           .data:0000c670 67                               ld h,a
                           .data:0000c671 69                               ld l,c
                           .data:0000c672 22 74 be                         ld (0xbe74),hl
                           .data:0000c675 48                               ld c,b
                           .data:0000c676 21 7c c6                         ld hl,0xc67c
                           .data:0000c679 c3 ff c6                         jp 0xc6ff
                           .data:0000c67c 2a 74 be                         ld hl,(0xbe74)
                           .data:0000c67f 01 7e fb                         ld bc,0xfb7e
                           .data:0000c682 7c                               ld a,h
                           .data:0000c683 cd 5c c9                         call 0xc95c
                           .data:0000c686 7b                               ld a,e
                           .data:0000c687 cd 5c c9                         call 0xc95c
                           .data:0000c68a 7c                               ld a,h
                           .data:0000c68b fe 4d                            cp 0x4d
                           .data:0000c68d 20 16                            jr nz,0xc6a5
                           .data:0000c68f 3e 14                            ld a,0x14
                           .data:0000c691 cd 59 c9                         call 0xc959
                           .data:0000c694 3e 10                            ld a,0x10
                           .data:0000c696 cd 59 c9                         call 0xc959
                           .data:0000c699 3e 12                            ld a,0x12
                           .data:0000c69b cd 59 c9                         call 0xc959
                           .data:0000c69e 3e 13                            ld a,0x13
                           .data:0000c6a0 cd 5c ca                         call 0xca5c
                           .data:0000c6a3 18 1c                            jr 0xc6c1
                           .data:0000c6a5 7a                               ld a,d
                           .data:0000c6a6 cd 5c c9                         call 0xc95c
                           .data:0000c6a9 af                               xor a
                           .data:0000c6aa cd 5c c9                         call 0xc95c
                           .data:0000c6ad 7d                               ld a,l
                           .data:0000c6ae cd 5c c9                         call 0xc95c
                           .data:0000c6b1 3e 14                            ld a,0x14
                           .data:0000c6b3 cd 59 c9                         call 0xc959
                           .data:0000c6b6 7d                               ld a,l
                           .data:0000c6b7 cd 5c c9                         call 0xc95c
                           .data:0000c6ba 3e 11                            ld a,0x11
                           .data:0000c6bc cd 59 c9                         call 0xc959
                           .data:0000c6bf 3e ff                            ld a,0xff
                           .data:0000c6c1 cd d1 c6                         call 0xc6d1
                           .data:0000c6c4 fb                               ei
                           .data:0000c6c5 cd 07 c9                         call 0xc907
                           .data:0000c6c8 d8                               ret c
                           .data:0000c6c9 c0                               ret nz
                           .data:0000c6ca 3a 4d be                         ld a,(0xbe4d)
                           .data:0000c6cd 87                               add a,a
                           .data:0000c6ce d8                               ret c
                           .data:0000c6cf af                               xor a
                           .data:0000c6d0 c9                               ret
                           .data:0000c6d1 f3                               di
                           .data:0000c6d2 cd 5c c9                         call 0xc95c
                           .data:0000c6d5 7c                               ld a,h
                           .data:0000c6d6 2a 62 be                         ld hl,(0xbe62)
                           .data:0000c6d9 fe 66                            cp 0x66
                           .data:0000c6db 20 18                            jr nz,0xc6f5
                           .data:0000c6dd 18 06                            jr 0xc6e5
                           .data:0000c6df 0c                               inc c
                           .data:0000c6e0 ed 78                            in a,(c)
                           .data:0000c6e2 77                               ld (hl),a
                           .data:0000c6e3 0d                               dec c
                           .data:0000c6e4 23                               inc hl
                           .data:0000c6e5 ed 78                            in a,(c)
                           .data:0000c6e7 f2 e5 c6                         jp p,0xc6e5
                           .data:0000c6ea e6 20                            and 0x20
                           .data:0000c6ec 20 f1                            jr nz,0xc6df
                           .data:0000c6ee c9                               ret
                           .data:0000c6ef 0c                               inc c
                           .data:0000c6f0 7e                               ld a,(hl)
                           .data:0000c6f1 ed 79                            out (c),a
                           .data:0000c6f3 0d                               dec c
                           .data:0000c6f4 23                               inc hl
                           .data:0000c6f5 ed 78                            in a,(c)
                           .data:0000c6f7 f2 f5 c6                         jp p,0xc6f5
                           .data:0000c6fa e6 20                            and 0x20
                           .data:0000c6fc 20 f1                            jr nz,0xc6ef
                           .data:0000c6fe c9                               ret
                           .data:0000c6ff 3a 66 be                         ld a,(0xbe66)
                           .data:0000c702 47                               ld b,a
                           .data:0000c703 cd 2b c7                         call 0xc72b
                           .data:0000c706 d8                               ret c
                           .data:0000c707 28 19                            jr z,0xc722
                           .data:0000c709 78                               ld a,b
                           .data:0000c70a e6 04                            and 0x04
                           .data:0000c70c 28 09                            jr z,0xc717
                           .data:0000c70e d5                               push de
                           .data:0000c70f 16 27                            ld d,0x27
                           .data:0000c711 cd 66 c7                         call 0xc766
                           .data:0000c714 d1                               pop de
                           .data:0000c715 18 ec                            jr 0xc703
                           .data:0000c717 e5                               push hl
                           .data:0000c718 3e 17                            ld a,0x17
                           .data:0000c71a cd 63 ca                         call 0xca63
                           .data:0000c71d 36 00                            ld (hl),0x00
                           .data:0000c71f e1                               pop hl
                           .data:0000c720 18 e1                            jr 0xc703
                           .data:0000c722 79                               ld a,c
                           .data:0000c723 c5                               push bc
                           .data:0000c724 cd 7a ca                         call 0xca7a
                           .data:0000c727 c1                               pop bc
                           .data:0000c728 20 d5                            jr nz,0xc6ff
                           .data:0000c72a c9                               ret
                           .data:0000c72b cd 54 c7                         call 0xc754
                           .data:0000c72e d8                               ret c
                           .data:0000c72f c8                               ret z
                           .data:0000c730 cd 47 c9                         call 0xc947
                           .data:0000c733 cd 54 c7                         call 0xc754
                           .data:0000c736 d8                               ret c
                           .data:0000c737 c8                               ret z
                           .data:0000c738 7a                               ld a,d
                           .data:0000c739 fe 27                            cp 0x27
                           .data:0000c73b 05                               dec b
                           .data:0000c73c 30 0a                            jr nc,0xc748
                           .data:0000c73e 04                               inc b
                           .data:0000c73f 14                               inc d
                           .data:0000c740 cd 66 c7                         call 0xc766
                           .data:0000c743 15                               dec d
                           .data:0000c744 cd 54 c7                         call 0xc754
                           .data:0000c747 d8                               ret c
                           .data:0000c748 c8                               ret z
                           .data:0000c749 7a                               ld a,d
                           .data:0000c74a b7                               or a
                           .data:0000c74b 20 02                            jr nz,0xc74f
                           .data:0000c74d 05                               dec b
                           .data:0000c74e c9                               ret
                           .data:0000c74f 15                               dec d
                           .data:0000c750 cd 66 c7                         call 0xc766
                           .data:0000c753 14                               inc d
                           .data:0000c754 cd 66 c7                         call 0xc766
                           .data:0000c757 e5                               push hl
                           .data:0000c758 c5                               push bc
                           .data:0000c759 cd 1e 00                         call 0x001e
                           .data:0000c75c c1                               pop bc
                           .data:0000c75d e1                               pop hl
                           .data:0000c75e d8                               ret c
                           .data:0000c75f 20 f3                            jr nz,0xc754
                           .data:0000c761 05                               dec b
                           .data:0000c762 c9                               ret
                           .data:0000c763 cd 76 c9                         call 0xc976
                           .data:0000c766 e5                               push hl
                           .data:0000c767 d5                               push de
                           .data:0000c768 c5                               push bc
                           .data:0000c769 3a 66 be                         ld a,(0xbe66)
                           .data:0000c76c 47                               ld b,a
                           .data:0000c76d 3e 17                            ld a,0x17
                           .data:0000c76f cd 63 ca                         call 0xca63
                           .data:0000c772 7e                               ld a,(hl)
                           .data:0000c773 b7                               or a
                           .data:0000c774 20 1f                            jr nz,0xc795
                           .data:0000c776 c5                               push bc
                           .data:0000c777 01 7e fb                         ld bc,0xfb7e
                           .data:0000c77a 3e 07                            ld a,0x07
                           .data:0000c77c cd 5c c9                         call 0xc95c
                           .data:0000c77f 7b                               ld a,e
                           .data:0000c780 cd 5c c9                         call 0xc95c
                           .data:0000c783 3e 28                            ld a,0x28
                           .data:0000c785 cd c7 c7                         call 0xc7c7
                           .data:0000c788 30 2a                            jr nc,0xc7b4
                           .data:0000c78a 3e 16                            ld a,0x16
                           .data:0000c78c cd 63 ca                         call 0xca63
                           .data:0000c78f 36 00                            ld (hl),0x00
                           .data:0000c791 23                               inc hl
                           .data:0000c792 36 ff                            ld (hl),0xff
                           .data:0000c794 c1                               pop bc
                           .data:0000c795 2b                               dec hl
                           .data:0000c796 7e                               ld a,(hl)
                           .data:0000c797 92                               sub d
                           .data:0000c798 28 28                            jr z,0xc7c2
                           .data:0000c79a c5                               push bc
                           .data:0000c79b 01 7e fb                         ld bc,0xfb7e
                           .data:0000c79e 3e 0f                            ld a,0x0f
                           .data:0000c7a0 cd 5c c9                         call 0xc95c
                           .data:0000c7a3 7b                               ld a,e
                           .data:0000c7a4 cd 5c c9                         call 0xc95c
                           .data:0000c7a7 7a                               ld a,d
                           .data:0000c7a8 cd 5c c9                         call 0xc95c
                           .data:0000c7ab 96                               sub (hl)
                           .data:0000c7ac 30 02                            jr nc,0xc7b0
                           .data:0000c7ae 7e                               ld a,(hl)
                           .data:0000c7af 92                               sub d
                           .data:0000c7b0 72                               ld (hl),d
                           .data:0000c7b1 cd c7 c7                         call 0xc7c7
                           .data:0000c7b4 c1                               pop bc
                           .data:0000c7b5 38 0b                            jr c,0xc7c2
                           .data:0000c7b7 20 bd                            jr nz,0xc776
                           .data:0000c7b9 05                               dec b
                           .data:0000c7ba ca ad c9                         jp z,0xc9ad
                           .data:0000c7bd cd 47 c9                         call 0xc947
                           .data:0000c7c0 18 b4                            jr 0xc776
                           .data:0000c7c2 c1                               pop bc
                           .data:0000c7c3 d1                               pop de
                           .data:0000c7c4 e1                               pop hl
                           .data:0000c7c5 37                               scf
                           .data:0000c7c6 c9                               ret
                           .data:0000c7c7 f5                               push af
                           .data:0000c7c8 3a 4a be                         ld a,(0xbe4a)
                           .data:0000c7cb cd e0 c7                         call 0xc7e0
                           .data:0000c7ce f1                               pop af
                           .data:0000c7cf 3d                               dec a
                           .data:0000c7d0 20 f5                            jr nz,0xc7c7
                           .data:0000c7d2 3a 49 be                         ld a,(0xbe49)
                           .data:0000c7d5 cd e0 c7                         call 0xc7e0
                           .data:0000c7d8 3e 08                            ld a,0x08
                           .data:0000c7da cd 5c c9                         call 0xc95c
                           .data:0000c7dd c3 f9 c8                         jp 0xc8f9
                           .data:0000c7e0 f5                               push af
                           .data:0000c7e1 3e f6                            ld a,0xf6
                           .data:0000c7e3 3d                               dec a
                           .data:0000c7e4 20 fd                            jr nz,0xc7e3
                           .data:0000c7e6 f1                               pop af
                           .data:0000c7e7 3d                               dec a
                           .data:0000c7e8 20 f6                            jr nz,0xc7e0
                           .data:0000c7ea c9                               ret
                           .data:0000c7eb 21 53 be                         ld hl,0xbe53
                           .data:0000c7ee 5e                               ld e,(hl)
                           .data:0000c7ef 3e 03                            ld a,0x03
                           .data:0000c7f1 cd 5c ca                         call 0xca5c
                           .data:0000c7f4 3c                               inc a
                           .data:0000c7f5 11 59 be                         ld de,0xbe59
                           .data:0000c7f8 12                               ld (de),a
                           .data:0000c7f9 13                               inc de
                           .data:0000c7fa 01 03 00                         ld bc,0x0003
                           .data:0000c7fd ed b0                            ldir
                           .data:0000c7ff c9                               ret
                           .data:0000c800 11 59 be                         ld de,0xbe59
                           .data:0000c803 1a                               ld a,(de)
                           .data:0000c804 b7                               or a
                           .data:0000c805 c8                               ret z
                           .data:0000c806 13                               inc de
                           .data:0000c807 21 53 be                         ld hl,0xbe53
                           .data:0000c80a 06 03                            ld b,0x03
                           .data:0000c80c 1a                               ld a,(de)
                           .data:0000c80d ae                               xor (hl)
                           .data:0000c80e 20 06                            jr nz,0xc816
                           .data:0000c810 13                               inc de
                           .data:0000c811 23                               inc hl
                           .data:0000c812 10 f8                            djnz 0xc80c
                           .data:0000c814 37                               scf
                           .data:0000c815 c9                               ret
                           .data:0000c816 af                               xor a
                           .data:0000c817 32 59 be                         ld (0xbe59),a
                           .data:0000c81a c9                               ret
                           .data:0000c81b f5                               push af
                           .data:0000c81c 21 59 be                         ld hl,0xbe59
                           .data:0000c81f 35                               dec (hl)
                           .data:0000c820 23                               inc hl
                           .data:0000c821 5e                               ld e,(hl)
                           .data:0000c822 23                               inc hl
                           .data:0000c823 23                               inc hl
                           .data:0000c824 34                               inc (hl)
                           .data:0000c825 af                               xor a
                           .data:0000c826 cd 5c ca                         call 0xca5c
                           .data:0000c829 be                               cp (hl)
                           .data:0000c82a 20 04                            jr nz,0xc830
                           .data:0000c82c 36 00                            ld (hl),0x00
                           .data:0000c82e 2b                               dec hl
                           .data:0000c82f 34                               inc (hl)
                           .data:0000c830 f1                               pop af
                           .data:0000c831 c9                               ret
                           .data:0000c832 f5                               push af
                           .data:0000c833 cd 54 c8                         call 0xc854
                           .data:0000c836 38 19                            jr c,0xc851
                           .data:0000c838 cd 6f c8                         call 0xc86f
                           .data:0000c83b c1                               pop bc
                           .data:0000c83c d0                               ret nc
                           .data:0000c83d c5                               push bc
                           .data:0000c83e cd 80 c8                         call 0xc880
                           .data:0000c841 f1                               pop af
                           .data:0000c842 38 06                            jr c,0xc84a
                           .data:0000c844 cd a2 c8                         call 0xc8a2
                           .data:0000c847 cd 66 c6                         call 0xc666
                           .data:0000c84a f5                               push af
                           .data:0000c84b 9f                               sbc a,a
                           .data:0000c84c 32 5e be                         ld (0xbe5e),a
                           .data:0000c84f f1                               pop af
                           .data:0000c850 c9                               ret
                           .data:0000c851 f1                               pop af
                           .data:0000c852 37                               scf
                           .data:0000c853 c9                               ret
                           .data:0000c854 3a 5e be                         ld a,(0xbe5e)
                           .data:0000c857 b7                               or a
                           .data:0000c858 c8                               ret z
                           .data:0000c859 01 53 be                         ld bc,0xbe53
                           .data:0000c85c 21 56 be                         ld hl,0xbe56
                           .data:0000c85f 5e                               ld e,(hl)
                           .data:0000c860 0a                               ld a,(bc)
                           .data:0000c861 ae                               xor (hl)
                           .data:0000c862 c0                               ret nz
                           .data:0000c863 03                               inc bc
                           .data:0000c864 23                               inc hl
                           .data:0000c865 0a                               ld a,(bc)
                           .data:0000c866 ae                               xor (hl)
                           .data:0000c867 c0                               ret nz
                           .data:0000c868 cd 92 c8                         call 0xc892
                           .data:0000c86b ae                               xor (hl)
                           .data:0000c86c c0                               ret nz
                           .data:0000c86d 37                               scf
                           .data:0000c86e c9                               ret
                           .data:0000c86f 21 5e be                         ld hl,0xbe5e
                           .data:0000c872 36 00                            ld (hl),0x00
                           .data:0000c874 2b                               dec hl
                           .data:0000c875 7e                               ld a,(hl)
                           .data:0000c876 b7                               or a
                           .data:0000c877 37                               scf
                           .data:0000c878 c8                               ret z
                           .data:0000c879 34                               inc (hl)
                           .data:0000c87a cd a2 c8                         call 0xc8a2
                           .data:0000c87d c3 4e c6                         jp 0xc64e
                           .data:0000c880 21 56 be                         ld hl,0xbe56
                           .data:0000c883 01 53 be                         ld bc,0xbe53
                           .data:0000c886 0a                               ld a,(bc)
                           .data:0000c887 77                               ld (hl),a
                           .data:0000c888 5f                               ld e,a
                           .data:0000c889 23                               inc hl
                           .data:0000c88a 03                               inc bc
                           .data:0000c88b 0a                               ld a,(bc)
                           .data:0000c88c 77                               ld (hl),a
                           .data:0000c88d cd 92 c8                         call 0xc892
                           .data:0000c890 77                               ld (hl),a
                           .data:0000c891 c9                               ret
                           .data:0000c892 03                               inc bc
                           .data:0000c893 23                               inc hl
                           .data:0000c894 3e 15                            ld a,0x15
                           .data:0000c896 cd 5c ca                         call 0xca5c
                           .data:0000c899 57                               ld d,a
                           .data:0000c89a 0a                               ld a,(bc)
                           .data:0000c89b cb 3a                            srl d
                           .data:0000c89d d8                               ret c
                           .data:0000c89e cb 3f                            srl a
                           .data:0000c8a0 18 f9                            jr 0xc89b
                           .data:0000c8a2 ed 5b 56 be                      ld de,(0xbe56)
                           .data:0000c8a6 3e 0f                            ld a,0x0f
                           .data:0000c8a8 cd 5c ca                         call 0xca5c
                           .data:0000c8ab 21 58 be                         ld hl,0xbe58
                           .data:0000c8ae 86                               add a,(hl)
                           .data:0000c8af 4f                               ld c,a
                           .data:0000c8b0 21 b0 02                         ld hl,0x02b0
                           .data:0000c8b3 c3 9f ca                         jp 0xca9f
                           .data:0000c8b6 e5                               push hl
                           .data:0000c8b7 d5                               push de
                           .data:0000c8b8 c5                               push bc
                           .data:0000c8b9 f5                               push af
                           .data:0000c8ba 3e ff                            ld a,0xff
                           .data:0000c8bc 32 5d be                         ld (0xbe5d),a
                           .data:0000c8bf cd d6 c8                         call 0xc8d6
                           .data:0000c8c2 cd 1b b9                         call 0xb91b
                           .data:0000c8c5 18 0a                            jr 0xc8d1
                           .data:0000c8c7 e5                               push hl
                           .data:0000c8c8 d5                               push de
                           .data:0000c8c9 c5                               push bc
                           .data:0000c8ca f5                               push af
                           .data:0000c8cb cd d6 c8                         call 0xc8d6
                           .data:0000c8ce eb                               ex de,hl
                           .data:0000c8cf ed b0                            ldir
                           .data:0000c8d1 f1                               pop af
                           .data:0000c8d2 c1                               pop bc
                           .data:0000c8d3 d1                               pop de
                           .data:0000c8d4 e1                               pop hl
                           .data:0000c8d5 c9                               ret
                           .data:0000c8d6 21 53 be                         ld hl,0xbe53
                           .data:0000c8d9 5e                               ld e,(hl)
                           .data:0000c8da 3e 15                            ld a,0x15
                           .data:0000c8dc cd 5c ca                         call 0xca5c
                           .data:0000c8df 3d                               dec a
                           .data:0000c8e0 23                               inc hl
                           .data:0000c8e1 23                               inc hl
                           .data:0000c8e2 a6                               and (hl)
                           .data:0000c8e3 11 80 00                         ld de,0x0080
                           .data:0000c8e6 21 30 02                         ld hl,0x0230
                           .data:0000c8e9 3c                               inc a
                           .data:0000c8ea 19                               add hl,de
                           .data:0000c8eb 3d                               dec a
                           .data:0000c8ec 20 fc                            jr nz,0xc8ea
                           .data:0000c8ee eb                               ex de,hl
                           .data:0000c8ef cd 98 ca                         call 0xca98
                           .data:0000c8f2 2a 60 be                         ld hl,(0xbe60)
                           .data:0000c8f5 01 80 00                         ld bc,0x0080
                           .data:0000c8f8 c9                               ret
                           .data:0000c8f9 cd 1c c9                         call 0xc91c
                           .data:0000c8fc d8                               ret c
                           .data:0000c8fd 3a 4c be                         ld a,(0xbe4c)
                           .data:0000c900 e6 08                            and 0x08
                           .data:0000c902 c8                               ret z
                           .data:0000c903 3e 13                            ld a,0x13
                           .data:0000c905 18 0d                            jr 0xc914
                           .data:0000c907 cd f9 c8                         call 0xc8f9
                           .data:0000c90a d8                               ret c
                           .data:0000c90b c0                               ret nz
                           .data:0000c90c 3a 4d be                         ld a,(0xbe4d)
                           .data:0000c90f e6 02                            and 0x02
                           .data:0000c911 c8                               ret z
                           .data:0000c912 3e 12                            ld a,0x12
                           .data:0000c914 cd 7a ca                         call 0xca7a
                           .data:0000c917 d8                               ret c
                           .data:0000c918 ca ad c9                         jp z,0xc9ad
                           .data:0000c91b c9                               ret
                           .data:0000c91c e5                               push hl
                           .data:0000c91d d5                               push de
                           .data:0000c91e 16 00                            ld d,0x00
                           .data:0000c920 21 4c be                         ld hl,0xbe4c
                           .data:0000c923 e5                               push hl
                           .data:0000c924 ed 78                            in a,(c)
                           .data:0000c926 fe c0                            cp 0xc0
                           .data:0000c928 38 fa                            jr c,0xc924
                           .data:0000c92a 0c                               inc c
                           .data:0000c92b ed 78                            in a,(c)
                           .data:0000c92d 0d                               dec c
                           .data:0000c92e 77                               ld (hl),a
                           .data:0000c92f 23                               inc hl
                           .data:0000c930 14                               inc d
                           .data:0000c931 3e 05                            ld a,0x05
                           .data:0000c933 3d                               dec a
                           .data:0000c934 20 fd                            jr nz,0xc933
                           .data:0000c936 ed 78                            in a,(c)
                           .data:0000c938 e6 10                            and 0x10
                           .data:0000c93a 20 e8                            jr nz,0xc924
                           .data:0000c93c e1                               pop hl
                           .data:0000c93d 7e                               ld a,(hl)
                           .data:0000c93e e6 c0                            and 0xc0
                           .data:0000c940 2b                               dec hl
                           .data:0000c941 72                               ld (hl),d
                           .data:0000c942 d1                               pop de
                           .data:0000c943 e1                               pop hl
                           .data:0000c944 c0                               ret nz
                           .data:0000c945 37                               scf
                           .data:0000c946 c9                               ret
                           .data:0000c947 c5                               push bc
                           .data:0000c948 01 7e fb                         ld bc,0xfb7e
                           .data:0000c94b 3e 08                            ld a,0x08
                           .data:0000c94d cd 5c c9                         call 0xc95c
                           .data:0000c950 cd 1c c9                         call 0xc91c
                           .data:0000c953 fe 80                            cp 0x80
                           .data:0000c955 20 f4                            jr nz,0xc94b
                           .data:0000c957 c1                               pop bc
                           .data:0000c958 c9                               ret
                           .data:0000c959 cd 5c ca                         call 0xca5c
                           .data:0000c95c f5                               push af
                           .data:0000c95d f5                               push af
                           .data:0000c95e ed 78                            in a,(c)
                           .data:0000c960 87                               add a,a
                           .data:0000c961 30 fb                            jr nc,0xc95e
                           .data:0000c963 87                               add a,a
                           .data:0000c964 30 03                            jr nc,0xc969
                           .data:0000c966 f1                               pop af
                           .data:0000c967 f1                               pop af
                           .data:0000c968 c9                               ret
                           .data:0000c969 f1                               pop af
                           .data:0000c96a 0c                               inc c
                           .data:0000c96b ed 79                            out (c),a
                           .data:0000c96d 0d                               dec c
                           .data:0000c96e 3e 05                            ld a,0x05
                           .data:0000c970 3d                               dec a
                           .data:0000c971 00                               nop
                           .data:0000c972 20 fc                            jr nz,0xc970
                           .data:0000c974 f1                               pop af
                           .data:0000c975 c9                               ret
                           .data:0000c976 22 76 be                         ld (0xbe76),hl
                           .data:0000c979 e3                               ex (sp),hl
                           .data:0000c97a d5                               push de
                           .data:0000c97b c5                               push bc
                           .data:0000c97c ed 73 64 be                      ld (0xbe64),sp
                           .data:0000c980 e5                               push hl
                           .data:0000c981 21 ad c9                         ld hl,0xc9ad
                           .data:0000c984 e3                               ex (sp),hl
                           .data:0000c985 e5                               push hl
                           .data:0000c986 d5                               push de
                           .data:0000c987 c5                               push bc
                           .data:0000c988 f5                               push af
                           .data:0000c989 cd df c9                         call 0xc9df
                           .data:0000c98c 3a 5f be                         ld a,(0xbe5f)
                           .data:0000c98f b7                               or a
                           .data:0000c990 20 14                            jr nz,0xc9a6
                           .data:0000c992 01 7e fa                         ld bc,0xfa7e
                           .data:0000c995 3e 01                            ld a,0x01
                           .data:0000c997 ed 79                            out (c),a
                           .data:0000c999 ed 5b 44 be                      ld de,(0xbe44)
                           .data:0000c99d cd cd c9                         call 0xc9cd
                           .data:0000c9a0 3a 5f be                         ld a,(0xbe5f)
                           .data:0000c9a3 b7                               or a
                           .data:0000c9a4 28 fa                            jr z,0xc9a0
                           .data:0000c9a6 f1                               pop af
                           .data:0000c9a7 c1                               pop bc
                           .data:0000c9a8 d1                               pop de
                           .data:0000c9a9 2a 76 be                         ld hl,(0xbe76)
                           .data:0000c9ac c9                               ret
                           .data:0000c9ad ed 7b 64 be                      ld sp,(0xbe64)
                           .data:0000c9b1 f5                               push af
                           .data:0000c9b2 ed 5b 46 be                      ld de,(0xbe46)
                           .data:0000c9b6 cd cd c9                         call 0xc9cd
                           .data:0000c9b9 f1                               pop af
                           .data:0000c9ba c1                               pop bc
                           .data:0000c9bb d1                               pop de
                           .data:0000c9bc e1                               pop hl
                           .data:0000c9bd 3e 00                            ld a,0x00
                           .data:0000c9bf d8                               ret c
                           .data:0000c9c0 21 4c be                         ld hl,0xbe4c
                           .data:0000c9c3 7e                               ld a,(hl)
                           .data:0000c9c4 e6 08                            and 0x08
                           .data:0000c9c6 23                               inc hl
                           .data:0000c9c7 b6                               or (hl)
                           .data:0000c9c8 f6 40                            or 0x40
                           .data:0000c9ca 2b                               dec hl
                           .data:0000c9cb 2b                               dec hl
                           .data:0000c9cc c9                               ret
                           .data:0000c9cd 21 67 be                         ld hl,0xbe67
                           .data:0000c9d0 01 00 00                         ld bc,0x0000
                           .data:0000c9d3 c3 e9 bc                         jp 0xbce9
                           .data:0000c9d6 21 5f be                         ld hl,0xbe5f
                           .data:0000c9d9 7e                               ld a,(hl)
                           .data:0000c9da 2f                               cpl
                           .data:0000c9db 77                               ld (hl),a
                           .data:0000c9dc b7                               or a
                           .data:0000c9dd 28 06                            jr z,0xc9e5
                           .data:0000c9df 21 67 be                         ld hl,0xbe67
                           .data:0000c9e2 c3 ec bc                         jp 0xbcec
                           .data:0000c9e5 cd df c9                         call 0xc9df
                           .data:0000c9e8 3e 00                            ld a,0x00
                           .data:0000c9ea 01 7e fa                         ld bc,0xfa7e
                           .data:0000c9ed ed 79                            out (c),a
                           .data:0000c9ef af                               xor a
                           .data:0000c9f0 32 5f be                         ld (0xbe5f),a
                           .data:0000c9f3 c9                               ret
                           .data:0000c9f4 21 20 02                         ld hl,0x0220
                           .data:0000c9f7 11 d0 01                         ld de,0x01d0
                           .data:0000c9fa cd 03 ca                         call 0xca03
                           .data:0000c9fd 21 10 02                         ld hl,0x0210
                           .data:0000ca00 11 90 01                         ld de,0x0190
                           .data:0000ca03 cd 98 ca                         call 0xca98
                           .data:0000ca06 ed 53 42 be                      ld (0xbe42),de
                           .data:0000ca0a d5                               push de
                           .data:0000ca0b cd 9f ca                         call 0xca9f
                           .data:0000ca0e 22 40 be                         ld (0xbe40),hl
                           .data:0000ca11 e5                               push hl
                           .data:0000ca12 21 43 ca                         ld hl,0xca43
                           .data:0000ca15 01 19 00                         ld bc,0x0019
                           .data:0000ca18 ed b0                            ldir
                           .data:0000ca1a 4b                               ld c,e
                           .data:0000ca1b 42                               ld b,d
                           .data:0000ca1c e1                               pop hl
                           .data:0000ca1d 36 00                            ld (hl),0x00
                           .data:0000ca1f 23                               inc hl
                           .data:0000ca20 36 00                            ld (hl),0x00
                           .data:0000ca22 11 07 00                         ld de,0x0007
                           .data:0000ca25 19                               add hl,de
                           .data:0000ca26 11 30 02                         ld de,0x0230
                           .data:0000ca29 cd 98 ca                         call 0xca98
                           .data:0000ca2c 73                               ld (hl),e
                           .data:0000ca2d 23                               inc hl
                           .data:0000ca2e 72                               ld (hl),d
                           .data:0000ca2f 23                               inc hl
                           .data:0000ca30 d1                               pop de
                           .data:0000ca31 73                               ld (hl),e
                           .data:0000ca32 23                               inc hl
                           .data:0000ca33 72                               ld (hl),d
                           .data:0000ca34 23                               inc hl
                           .data:0000ca35 71                               ld (hl),c
                           .data:0000ca36 23                               inc hl
                           .data:0000ca37 70                               ld (hl),b
                           .data:0000ca38 23                               inc hl
                           .data:0000ca39 eb                               ex de,hl
                           .data:0000ca3a 21 10 00                         ld hl,0x0010
                           .data:0000ca3d 09                               add hl,bc
                           .data:0000ca3e eb                               ex de,hl
                           .data:0000ca3f 73                               ld (hl),e
                           .data:0000ca40 23                               inc hl
                           .data:0000ca41 72                               ld (hl),d
                           .data:0000ca42 c9                               ret
                           .data:0000ca43 24                               inc h
                           .data:0000ca44 00                               nop
                           .data:0000ca45 03                               inc bc
                           .data:0000ca46 07                               rlca
                           .data:0000ca47 00                               nop
                           .data:0000ca48 aa                               xor d
                           .data:0000ca49 00                               nop
                           .data:0000ca4a 3f                               ccf
                           .data:0000ca4b 00                               nop
                           .data:0000ca4c c0                               ret nz
                           .data:0000ca4d 00                               nop
                           .data:0000ca4e 10 00                            djnz 0xca50
                           .data:0000ca50 02                               ld (bc),a
                           .data:0000ca51 00                               nop
                           .data:0000ca52 41                               ld b,c
                           .data:0000ca53 09                               add hl,bc
                           .data:0000ca54 2a 52 e5                         ld hl,(0xe552)
                           .data:0000ca57 02                               ld (bc),a
                           .data:0000ca58 04                               inc b
                           .data:0000ca59 00                               nop
                           .data:0000ca5a 00                               nop
                           .data:0000ca5b 00                               nop
                           .data:0000ca5c e5                               push hl
                           .data:0000ca5d cd 63 ca                         call 0xca63
                           .data:0000ca60 7e                               ld a,(hl)
                           .data:0000ca61 e1                               pop hl
                           .data:0000ca62 c9                               ret
                           .data:0000ca63 d5                               push de
                           .data:0000ca64 2a 42 be                         ld hl,(0xbe42)
                           .data:0000ca67 1d                               dec e
                           .data:0000ca68 11 40 00                         ld de,0x0040
                           .data:0000ca6b 20 01                            jr nz,0xca6e
                           .data:0000ca6d 19                               add hl,de
                           .data:0000ca6e 5f                               ld e,a
                           .data:0000ca6f 19                               add hl,de
                           .data:0000ca70 d1                               pop de
                           .data:0000ca71 c9                               ret
                           .data:0000ca72 2a 78 be                         ld hl,(0xbe78)
                           .data:0000ca75 32 78 be                         ld (0xbe78),a
                           .data:0000ca78 7d                               ld a,l
                           .data:0000ca79 c9                               ret
                           .data:0000ca7a f5                               push af
                           .data:0000ca7b 3a 78 be                         ld a,(0xbe78)
                           .data:0000ca7e b7                               or a
                           .data:0000ca7f 20 05                            jr nz,0xca86
                           .data:0000ca81 f1                               pop af
                           .data:0000ca82 4b                               ld c,e
                           .data:0000ca83 c3 b8 ca                         jp 0xcab8
                           .data:0000ca86 f1                               pop af
                           .data:0000ca87 af                               xor a
                           .data:0000ca88 c9                               ret
                           .data:0000ca89 ff                               rst 0x38
                           .data:0000ca8a ff                               rst 0x38
                           .data:0000ca8b ff                               rst 0x38
                           .data:0000ca8c ff                               rst 0x38
                           .data:0000ca8d ff                               rst 0x38
                           .data:0000ca8e ff                               rst 0x38
                           .data:0000ca8f ff                               rst 0x38
                           .data:0000ca90 fd e5                            push iy
                           .data:0000ca92 e3                               ex (sp),hl
                           .data:0000ca93 09                               add hl,bc
                           .data:0000ca94 44                               ld b,h
                           .data:0000ca95 4d                               ld c,l
                           .data:0000ca96 e1                               pop hl
                           .data:0000ca97 c9                               ret
                           .data:0000ca98 fd e5                            push iy
                           .data:0000ca9a e3                               ex (sp),hl
                           .data:0000ca9b 19                               add hl,de
                           .data:0000ca9c eb                               ex de,hl
                           .data:0000ca9d e1                               pop hl
                           .data:0000ca9e c9                               ret
                           .data:0000ca9f d5                               push de
                           .data:0000caa0 fd e5                            push iy
                           .data:0000caa2 d1                               pop de
                           .data:0000caa3 19                               add hl,de
                           .data:0000caa4 d1                               pop de
                           .data:0000caa5 c9                               ret
                           .data:0000caa6 fe 61                            cp 0x61
                           .data:0000caa8 d8                               ret c
                           .data:0000caa9 fe 7b                            cp 0x7b
                           .data:0000caab d0                               ret nc
                           .data:0000caac c6 e0                            add a,0xe0
                           .data:0000caae c9                               ret
                           .data:0000caaf af                               xor a
                           .data:0000cab0 12                               ld (de),a
                           .data:0000cab1 13                               inc de
                           .data:0000cab2 0b                               dec bc
                           .data:0000cab3 78                               ld a,b
                           .data:0000cab4 b1                               or c
                           .data:0000cab5 20 f8                            jr nz,0xcaaf
                           .data:0000cab7 c9                               ret
                           .data:0000cab8 cd eb ca                         call 0xcaeb
                           .data:0000cabb 3e 14                            ld a,0x14
                           .data:0000cabd cd eb ca                         call 0xcaeb
                           .data:0000cac0 cd 09 bb                         call 0xbb09
                           .data:0000cac3 38 fb                            jr c,0xcac0
                           .data:0000cac5 cd 81 bb                         call 0xbb81
                           .data:0000cac8 cd 06 bb                         call 0xbb06
                           .data:0000cacb cd a6 ca                         call 0xcaa6
                           .data:0000cace fe 43                            cp 0x43
                           .data:0000cad0 28 11                            jr z,0xcae3
                           .data:0000cad2 fe 49                            cp 0x49
                           .data:0000cad4 37                               scf
                           .data:0000cad5 28 0c                            jr z,0xcae3
                           .data:0000cad7 fe 52                            cp 0x52
                           .data:0000cad9 28 07                            jr z,0xcae2
                           .data:0000cadb 3e 07                            ld a,0x07
                           .data:0000cadd cd 5a bb                         call 0xbb5a
                           .data:0000cae0 18 e6                            jr 0xcac8
                           .data:0000cae2 b7                               or a
                           .data:0000cae3 cd 5a bb                         call 0xbb5a
                           .data:0000cae6 cd 84 bb                         call 0xbb84
                           .data:0000cae9 3e 00                            ld a,0x00
                           .data:0000caeb e5                               push hl
                           .data:0000caec c5                               push bc
                           .data:0000caed f5                               push af
                           .data:0000caee e6 7f                            and 0x7f
                           .data:0000caf0 21 86 cb                         ld hl,0xcb86
                           .data:0000caf3 47                               ld b,a
                           .data:0000caf4 04                               inc b
                           .data:0000caf5 18 05                            jr 0xcafc
                           .data:0000caf7 7e                               ld a,(hl)
                           .data:0000caf8 23                               inc hl
                           .data:0000caf9 3c                               inc a
                           .data:0000cafa 20 fb                            jr nz,0xcaf7
                           .data:0000cafc 10 f9                            djnz 0xcaf7
                           .data:0000cafe 7e                               ld a,(hl)
                           .data:0000caff 23                               inc hl
                           .data:0000cb00 fe ff                            cp 0xff
                           .data:0000cb02 28 0b                            jr z,0xcb0f
                           .data:0000cb04 e5                               push hl
                           .data:0000cb05 d5                               push de
                           .data:0000cb06 c5                               push bc
                           .data:0000cb07 cd 13 cb                         call 0xcb13
                           .data:0000cb0a c1                               pop bc
                           .data:0000cb0b d1                               pop de
                           .data:0000cb0c e1                               pop hl
                           .data:0000cb0d 18 ef                            jr 0xcafe
                           .data:0000cb0f f1                               pop af
                           .data:0000cb10 c1                               pop bc
                           .data:0000cb11 e1                               pop hl
                           .data:0000cb12 c9                               ret
                           .data:0000cb13 b7                               or a
                           .data:0000cb14 f2 66 cb                         jp p,0xcb66
                           .data:0000cb17 fe fe                            cp 0xfe
                           .data:0000cb19 28 46                            jr z,0xcb61
                           .data:0000cb1b fe fc                            cp 0xfc
                           .data:0000cb1d 28 1a                            jr z,0xcb39
                           .data:0000cb1f fe fd                            cp 0xfd
                           .data:0000cb21 20 c8                            jr nz,0xcaeb
                           .data:0000cb23 06 08                            ld b,0x08
                           .data:0000cb25 cd 2f cb                         call 0xcb2f
                           .data:0000cb28 3e 2e                            ld a,0x2e
                           .data:0000cb2a cd 83 cb                         call 0xcb83
                           .data:0000cb2d 06 03                            ld b,0x03
                           .data:0000cb2f 13                               inc de
                           .data:0000cb30 1a                               ld a,(de)
                           .data:0000cb31 e6 7f                            and 0x7f
                           .data:0000cb33 cd 83 cb                         call 0xcb83
                           .data:0000cb36 10 f7                            djnz 0xcb2f
                           .data:0000cb38 c9                               ret
                           .data:0000cb39 eb                               ex de,hl
                           .data:0000cb3a 16 20                            ld d,0x20
                           .data:0000cb3c 01 9c ff                         ld bc,0xff9c
                           .data:0000cb3f cd 4d cb                         call 0xcb4d
                           .data:0000cb42 01 f6 ff                         ld bc,0xfff6
                           .data:0000cb45 cd 4d cb                         call 0xcb4d
                           .data:0000cb48 7d                               ld a,l
                           .data:0000cb49 c6 30                            add a,0x30
                           .data:0000cb4b 18 36                            jr 0xcb83
                           .data:0000cb4d 3e ff                            ld a,0xff
                           .data:0000cb4f e5                               push hl
                           .data:0000cb50 3c                               inc a
                           .data:0000cb51 09                               add hl,bc
                           .data:0000cb52 30 04                            jr nc,0xcb58
                           .data:0000cb54 e3                               ex (sp),hl
                           .data:0000cb55 e1                               pop hl
                           .data:0000cb56 18 f7                            jr 0xcb4f
                           .data:0000cb58 e1                               pop hl
                           .data:0000cb59 b7                               or a
                           .data:0000cb5a 28 02                            jr z,0xcb5e
                           .data:0000cb5c 16 30                            ld d,0x30
                           .data:0000cb5e 82                               add a,d
                           .data:0000cb5f 18 22                            jr 0xcb83
                           .data:0000cb61 79                               ld a,c
                           .data:0000cb62 c6 41                            add a,0x41
                           .data:0000cb64 18 1d                            jr 0xcb83
                           .data:0000cb66 f5                               push af
                           .data:0000cb67 fe 20                            cp 0x20
                           .data:0000cb69 20 17                            jr nz,0xcb82
                           .data:0000cb6b e5                               push hl
                           .data:0000cb6c d5                               push de
                           .data:0000cb6d cd 69 bb                         call 0xbb69
                           .data:0000cb70 cd 78 bb                         call 0xbb78
                           .data:0000cb73 7a                               ld a,d
                           .data:0000cb74 d6 04                            sub 0x04
                           .data:0000cb76 3f                               ccf
                           .data:0000cb77 30 01                            jr nc,0xcb7a
                           .data:0000cb79 bc                               cp h
                           .data:0000cb7a d1                               pop de
                           .data:0000cb7b e1                               pop hl
                           .data:0000cb7c 30 04                            jr nc,0xcb82
                           .data:0000cb7e f1                               pop af
                           .data:0000cb7f c3 e9 ca                         jp 0xcae9
                           .data:0000cb82 f1                               pop af
                           .data:0000cb83 c3 5a bb                         jp 0xbb5a
                           .data:0000cb86 0d                               dec c
                           .data:0000cb87 0a                               ld a,(bc)
                           .data:0000cb88 ff                               rst 0x38
                           .data:0000cb89 20 20                            jr nz,0xcbab
                           .data:0000cb8b 20 ff                            jr nz,0xcb8c
                           .data:0000cb8d fc 4b ff                         call m,0xff4b
                           .data:0000cb90 97                               sub a
                           .data:0000cb91 82                               add a,d
                           .data:0000cb92 20 66                            jr nz,0xcbfa
                           .data:0000cb94 72                               ld (hl),d
                           .data:0000cb95 65                               ld h,l
                           .data:0000cb96 65                               ld h,l
                           .data:0000cb97 97                               sub a
                           .data:0000cb98 ff                               rst 0x38
                           .data:0000cb99 80                               add a,b
                           .data:0000cb9a 42                               ld b,d
                           .data:0000cb9b 61                               ld h,c
                           .data:0000cb9c 64                               ld h,h
                           .data:0000cb9d 20 63                            jr nz,0xcc02
                           .data:0000cb9f 6f                               ld l,a
                           .data:0000cba0 6d                               ld l,l
                           .data:0000cba1 6d                               ld l,l
                           .data:0000cba2 61                               ld h,c
                           .data:0000cba3 6e                               ld l,(hl)
                           .data:0000cba4 64                               ld h,h
                           .data:0000cba5 80                               add a,b
                           .data:0000cba6 ff                               rst 0x38
                           .data:0000cba7 9b                               sbc a,e
                           .data:0000cba8 61                               ld h,c
                           .data:0000cba9 6c                               ld l,h
                           .data:0000cbaa 72                               ld (hl),d
                           .data:0000cbab 65                               ld h,l
                           .data:0000cbac 61                               ld h,c
                           .data:0000cbad 64                               ld h,h
                           .data:0000cbae 79                               ld a,c
                           .data:0000cbaf 20 65                            jr nz,0xcc16
                           .data:0000cbb1 78                               ld a,b
                           .data:0000cbb2 69                               ld l,c
                           .data:0000cbb3 73                               ld (hl),e
                           .data:0000cbb4 74                               ld (hl),h
                           .data:0000cbb5 73                               ld (hl),e
                           .data:0000cbb6 80                               add a,b
                           .data:0000cbb7 ff                               rst 0x38
                           .data:0000cbb8 9b                               sbc a,e
                           .data:0000cbb9 6e                               ld l,(hl)
                           .data:0000cbba 6f                               ld l,a
                           .data:0000cbbb 74                               ld (hl),h
                           .data:0000cbbc 20 66                            jr nz,0xcc24
                           .data:0000cbbe 6f                               ld l,a
                           .data:0000cbbf 75                               ld (hl),l
                           .data:0000cbc0 6e                               ld l,(hl)
                           .data:0000cbc1 64                               ld h,h
                           .data:0000cbc2 80                               add a,b
                           .data:0000cbc3 ff                               rst 0x38
                           .data:0000cbc4 95                               sub l
                           .data:0000cbc5 64                               ld h,h
                           .data:0000cbc6 69                               ld l,c
                           .data:0000cbc7 72                               ld (hl),d
                           .data:0000cbc8 65                               ld h,l
                           .data:0000cbc9 63                               ld h,e
                           .data:0000cbca 74                               ld (hl),h
                           .data:0000cbcb 6f                               ld l,a
                           .data:0000cbcc 72                               ld (hl),d
                           .data:0000cbcd 79                               ld a,c
                           .data:0000cbce 20 9a                            jr nz,0xcb6a
                           .data:0000cbd0 ff                               rst 0x38
                           .data:0000cbd1 98                               sbc a,b
                           .data:0000cbd2 9a                               sbc a,d
                           .data:0000cbd3 ff                               rst 0x38
                           .data:0000cbd4 98                               sbc a,b
                           .data:0000cbd5 63                               ld h,e
                           .data:0000cbd6 68                               ld l,b
                           .data:0000cbd7 61                               ld h,c
                           .data:0000cbd8 6e                               ld l,(hl)
                           .data:0000cbd9 67                               ld h,a
                           .data:0000cbda 65                               ld h,l
                           .data:0000cbdb 64                               ld h,h
                           .data:0000cbdc 2c                               inc l
                           .data:0000cbdd 20 63                            jr nz,0xcc42
                           .data:0000cbdf 6c                               ld l,h
                           .data:0000cbe0 6f                               ld l,a
                           .data:0000cbe1 73                               ld (hl),e
                           .data:0000cbe2 69                               ld l,c
                           .data:0000cbe3 6e                               ld l,(hl)
                           .data:0000cbe4 67                               ld h,a
                           .data:0000cbe5 20 fd                            jr nz,0xcbe4
                           .data:0000cbe7 80                               add a,b
                           .data:0000cbe8 ff                               rst 0x38
                           .data:0000cbe9 9b                               sbc a,e
                           .data:0000cbea 69                               ld l,c
                           .data:0000cbeb 73                               ld (hl),e
                           .data:0000cbec 20 9d                            jr nz,0xcb8b
                           .data:0000cbee 20 6f                            jr nz,0xcc5f
                           .data:0000cbf0 6e                               ld l,(hl)
                           .data:0000cbf1 6c                               ld l,h
                           .data:0000cbf2 79                               ld a,c
                           .data:0000cbf3 80                               add a,b
                           .data:0000cbf4 ff                               rst 0x38
                           .data:0000cbf5 fd                               defb 0xfd
                           .data:0000cbf6 ff                               rst 0x38
                           .data:0000cbf7 95                               sub l
                           .data:0000cbf8 75                               ld (hl),l
                           .data:0000cbf9 73                               ld (hl),e
                           .data:0000cbfa 65                               ld h,l
                           .data:0000cbfb 72                               ld (hl),d
                           .data:0000cbfc fc 80 ff                         call m,0xff80
                           .data:0000cbff 2e 2e                            ld l,0x2e
                           .data:0000cc01 2e 5e                            ld l,0x5e
                           .data:0000cc03 43                               ld b,e
                           .data:0000cc04 ff                               rst 0x38
                           .data:0000cc05 96                               sub (hl)
                           .data:0000cc06 43                               ld b,e
                           .data:0000cc07 50                               ld d,b
                           .data:0000cc08 2f                               cpl
                           .data:0000cc09 4d                               ld c,l
                           .data:0000cc0a 80                               add a,b
                           .data:0000cc0b ff                               rst 0x38
                           .data:0000cc0c 96                               sub (hl)
                           .data:0000cc0d 62                               ld h,d
                           .data:0000cc0e 6f                               ld l,a
                           .data:0000cc0f 6f                               ld l,a
                           .data:0000cc10 74                               ld (hl),h
                           .data:0000cc11 20 73                            jr nz,0xcc86
                           .data:0000cc13 65                               ld h,l
                           .data:0000cc14 63                               ld h,e
                           .data:0000cc15 74                               ld (hl),h
                           .data:0000cc16 6f                               ld l,a
                           .data:0000cc17 72                               ld (hl),d
                           .data:0000cc18 80                               add a,b
                           .data:0000cc19 ff                               rst 0x38
                           .data:0000cc1a 95                               sub l
                           .data:0000cc1b 9d                               sbc a,l
                           .data:0000cc1c 99                               sbc a,c
                           .data:0000cc1d ff                               rst 0x38
                           .data:0000cc1e 95                               sub l
                           .data:0000cc1f 9c                               sbc a,h
                           .data:0000cc20 99                               sbc a,c
                           .data:0000cc21 ff                               rst 0x38
                           .data:0000cc22 98                               sbc a,b
                           .data:0000cc23 69                               ld l,c
                           .data:0000cc24 73                               ld (hl),e
                           .data:0000cc25 20 9c                            jr nz,0xcbc3
                           .data:0000cc27 20 70                            jr nz,0xcc99
                           .data:0000cc29 72                               ld (hl),d
                           .data:0000cc2a 6f                               ld l,a
                           .data:0000cc2b 74                               ld (hl),h
                           .data:0000cc2c 65                               ld h,l
                           .data:0000cc2d 63                               ld h,e
                           .data:0000cc2e 74                               ld (hl),h
                           .data:0000cc2f 65                               ld h,l
                           .data:0000cc30 64                               ld h,h
                           .data:0000cc31 80                               add a,b
                           .data:0000cc32 ff                               rst 0x38
                           .data:0000cc33 98                               sbc a,b
                           .data:0000cc34 6d                               ld l,l
                           .data:0000cc35 69                               ld l,c
                           .data:0000cc36 73                               ld (hl),e
                           .data:0000cc37 73                               ld (hl),e
                           .data:0000cc38 69                               ld l,c
                           .data:0000cc39 6e                               ld l,(hl)
                           .data:0000cc3a 67                               ld h,a
                           .data:0000cc3b 80                               add a,b
                           .data:0000cc3c ff                               rst 0x38
                           .data:0000cc3d 80                               add a,b
                           .data:0000cc3e 52                               ld d,d
                           .data:0000cc3f 65                               ld h,l
                           .data:0000cc40 74                               ld (hl),h
                           .data:0000cc41 72                               ld (hl),d
                           .data:0000cc42 79                               ld a,c
                           .data:0000cc43 2c                               inc l
                           .data:0000cc44 20 49                            jr nz,0xcc8f
                           .data:0000cc46 67                               ld h,a
                           .data:0000cc47 6e                               ld l,(hl)
                           .data:0000cc48 6f                               ld l,a
                           .data:0000cc49 72                               ld (hl),d
                           .data:0000cc4a 65                               ld h,l
                           .data:0000cc4b 20 6f                            jr nz,0xccbc
                           .data:0000cc4d 72                               ld (hl),d
                           .data:0000cc4e 20 43                            jr nz,0xcc93
                           .data:0000cc50 61                               ld h,c
                           .data:0000cc51 6e                               ld l,(hl)
                           .data:0000cc52 63                               ld h,e
                           .data:0000cc53 65                               ld h,l
                           .data:0000cc54 6c                               ld l,h
                           .data:0000cc55 3f                               ccf
                           .data:0000cc56 20 ff                            jr nz,0xcc57
                           .data:0000cc58 80                               add a,b
                           .data:0000cc59 44                               ld b,h
                           .data:0000cc5a 72                               ld (hl),d
                           .data:0000cc5b 69                               ld l,c
                           .data:0000cc5c 76                               halt
                           .data:0000cc5d 65                               ld h,l
                           .data:0000cc5e 20 fe                            jr nz,0xcc5e
                           .data:0000cc60 3a 20 ff                         ld a,(0xff20)
                           .data:0000cc63 80                               add a,b
                           .data:0000cc64 46                               ld b,(hl)
                           .data:0000cc65 61                               ld h,c
                           .data:0000cc66 69                               ld l,c
                           .data:0000cc67 6c                               ld l,h
                           .data:0000cc68 65                               ld h,l
                           .data:0000cc69 64                               ld h,h
                           .data:0000cc6a 20 74                            jr nz,0xcce0
                           .data:0000cc6c 6f                               ld l,a
                           .data:0000cc6d 20 6c                            jr nz,0xccdb
                           .data:0000cc6f 6f                               ld l,a
                           .data:0000cc70 61                               ld h,c
                           .data:0000cc71 64                               ld h,h
                           .data:0000cc72 20 ff                            jr nz,0xcc73
                           .data:0000cc74 80                               add a,b
                           .data:0000cc75 80                               add a,b
                           .data:0000cc76 ff                               rst 0x38
                           .data:0000cc77 95                               sub l
                           .data:0000cc78 64                               ld h,h
                           .data:0000cc79 69                               ld l,c
                           .data:0000cc7a 73                               ld (hl),e
                           .data:0000cc7b 63                               ld h,e
                           .data:0000cc7c 20 ff                            jr nz,0xcc7d
                           .data:0000cc7e 20 66                            jr nz,0xcce6
                           .data:0000cc80 61                               ld h,c
                           .data:0000cc81 69                               ld l,c
                           .data:0000cc82 6c                               ld l,h
                           .data:0000cc83 80                               add a,b
                           .data:0000cc84 ff                               rst 0x38
                           .data:0000cc85 66                               ld h,(hl)
                           .data:0000cc86 75                               ld (hl),l
                           .data:0000cc87 6c                               ld l,h
                           .data:0000cc88 6c                               ld l,h
                           .data:0000cc89 80                               add a,b
                           .data:0000cc8a ff                               rst 0x38
                           .data:0000cc8b 80                               add a,b
                           .data:0000cc8c fd                               defb 0xfd
                           .data:0000cc8d 20 ff                            jr nz,0xcc8e
                           .data:0000cc8f 77                               ld (hl),a
                           .data:0000cc90 72                               ld (hl),d
                           .data:0000cc91 69                               ld l,c
                           .data:0000cc92 74                               ld (hl),h
                           .data:0000cc93 65                               ld h,l
                           .data:0000cc94 ff                               rst 0x38
                           .data:0000cc95 72                               ld (hl),d
                           .data:0000cc96 65                               ld h,l
                           .data:0000cc97 61                               ld h,c
                           .data:0000cc98 64                               ld h,h
                           .data:0000cc99 ff                               rst 0x38
                           .data:0000cc9a ff                               rst 0x38
                           .data:0000cc9b ff                               rst 0x38
                           .data:0000cc9c ff                               rst 0x38
                           .data:0000cc9d ff                               rst 0x38
                           .data:0000cc9e ff                               rst 0x38
                           .data:0000cc9f ff                               rst 0x38
                           .data:0000cca0 af                               xor a
                           .data:0000cca1 fd 77 00                         ld (iy+0),a
                           .data:0000cca4 fd 77 01                         ld (iy+1),a
                           .data:0000cca7 3d                               dec a
                           .data:0000cca8 fd 77 08                         ld (iy+8),a
                           .data:0000ccab fd 77 2c                         ld (iy+44),a
                           .data:0000ccae fd 22 7d be                      ld (0xbe7d),iy
                           .data:0000ccb2 21 77 bc                         ld hl,0xbc77
                           .data:0000ccb5 11 64 01                         ld de,0x0164
                           .data:0000ccb8 cd 98 ca                         call 0xca98
                           .data:0000ccbb 01 27 00                         ld bc,0x0027
                           .data:0000ccbe ed b0                            ldir
                           .data:0000ccc0 eb                               ex de,hl
                           .data:0000ccc1 36 30                            ld (hl),0x30
                           .data:0000ccc3 23                               inc hl
                           .data:0000ccc4 36 cd                            ld (hl),0xcd
                           .data:0000ccc6 23                               inc hl
                           .data:0000ccc7 cd 12 b9                         call 0xb912
                           .data:0000ccca 77                               ld (hl),a
                           .data:0000cccb 3e c9                            ld a,0xc9
                           .data:0000cccd 32 7f be                         ld (0xbe7f),a
                           .data:0000ccd0 af                               xor a
                           .data:0000ccd1 cd e4 cc                         call 0xcce4
                           .data:0000ccd4 d0                               ret nc
                           .data:0000ccd5 21 77 bc                         ld hl,0xbc77
                           .data:0000ccd8 06 07                            ld b,0x07
                           .data:0000ccda cd e9 cc                         call 0xcce9
                           .data:0000ccdd d0                               ret nc
                           .data:0000ccde 21 9b bc                         ld hl,0xbc9b
                           .data:0000cce1 04                               inc b
                           .data:0000cce2 18 05                            jr 0xcce9
                           .data:0000cce4 21 8c bc                         ld hl,0xbc8c
                           .data:0000cce7 06 05                            ld b,0x05
                           .data:0000cce9 b7                               or a
                           .data:0000ccea 20 3f                            jr nz,0xcd2b
                           .data:0000ccec 11 8b 01                         ld de,0x018b
                           .data:0000ccef cd 98 ca                         call 0xca98
                           .data:0000ccf2 36 df                            ld (hl),0xdf
                           .data:0000ccf4 23                               inc hl
                           .data:0000ccf5 73                               ld (hl),e
                           .data:0000ccf6 23                               inc hl
                           .data:0000ccf7 72                               ld (hl),d
                           .data:0000ccf8 23                               inc hl
                           .data:0000ccf9 10 f7                            djnz 0xccf2
                           .data:0000ccfb 37                               scf
                           .data:0000ccfc c9                               ret
                           .data:0000ccfd cd 18 cd                         call 0xcd18
                           .data:0000cd00 d0                               ret nc
                           .data:0000cd01 21 64 01                         ld hl,0x0164
                           .data:0000cd04 11 77 bc                         ld de,0xbc77
                           .data:0000cd07 01 15 00                         ld bc,0x0015
                           .data:0000cd0a cd 21 cd                         call 0xcd21
                           .data:0000cd0d d0                               ret nc
                           .data:0000cd0e 21 88 01                         ld hl,0x0188
                           .data:0000cd11 11 9b bc                         ld de,0xbc9b
                           .data:0000cd14 0e 03                            ld c,0x03
                           .data:0000cd16 18 09                            jr 0xcd21
                           .data:0000cd18 21 79 01                         ld hl,0x0179
                           .data:0000cd1b 11 8c bc                         ld de,0xbc8c
                           .data:0000cd1e 01 0f 00                         ld bc,0x000f
                           .data:0000cd21 b7                               or a
                           .data:0000cd22 20 07                            jr nz,0xcd2b
                           .data:0000cd24 cd 9f ca                         call 0xca9f
                           .data:0000cd27 ed b0                            ldir
                           .data:0000cd29 37                               scf
                           .data:0000cd2a c9                               ret
                           .data:0000cd2b 3e 04                            ld a,0x04
                           .data:0000cd2d c3 eb ca                         jp 0xcaeb
                           .data:0000cd30 fd 2a 7d be                      ld iy,(0xbe7d)
                           .data:0000cd34 f3                               di
                           .data:0000cd35 08                               ex af,af'
                           .data:0000cd36 d9                               exx
                           .data:0000cd37 79                               ld a,c
                           .data:0000cd38 d1                               pop de
                           .data:0000cd39 c1                               pop bc
                           .data:0000cd3a e1                               pop hl
                           .data:0000cd3b e3                               ex (sp),hl
                           .data:0000cd3c c5                               push bc
                           .data:0000cd3d d5                               push de
                           .data:0000cd3e 4f                               ld c,a
                           .data:0000cd3f 06 7f                            ld b,0x7f
                           .data:0000cd41 11 d2 10                         ld de,0x10d2
                           .data:0000cd44 19                               add hl,de
                           .data:0000cd45 e5                               push hl
                           .data:0000cd46 d9                               exx
                           .data:0000cd47 08                               ex af,af'
                           .data:0000cd48 fb                               ei
                           .data:0000cd49 c3 7f be                         jp 0xbe7f
                           .data:0000cd4c c3 af ce                         jp 0xceaf
                           .data:0000cd4f c3 b6 d1                         jp 0xd1b6
                           .data:0000cd52 c3 bc d1                         jp 0xd1bc
                           .data:0000cd55 c3 64 cf                         jp 0xcf64
                           .data:0000cd58 c3 f5 cf                         jp 0xcff5
                           .data:0000cd5b c3 69 d0                         jp 0xd069
                           .data:0000cd5e c3 65 d0                         jp 0xd065
                           .data:0000cd61 c3 37 cf                         jp 0xcf37
                           .data:0000cd64 c3 d8 d1                         jp 0xd1d8
                           .data:0000cd67 c3 c2 d1                         jp 0xd1c2
                           .data:0000cd6a c3 8f d0                         jp 0xd08f
                           .data:0000cd6d c3 d8 d0                         jp 0xd0d8
                           .data:0000cd70 c3 13 d5                         jp 0xd513
                           .data:0000cd73 cd 77 cd                         call 0xcd77
                           .data:0000cd76 c9                               ret
                           .data:0000cd77 e5                               push hl
                           .data:0000cd78 21 06 00                         ld hl,0x0006
                           .data:0000cd7b 39                               add hl,sp
                           .data:0000cd7c fd 75 06                         ld (iy+6),l
                           .data:0000cd7f fd 74 07                         ld (iy+7),h
                           .data:0000cd82 e1                               pop hl
                           .data:0000cd83 c9                               ret
                           .data:0000cd84 cd 77 cd                         call 0xcd77
                           .data:0000cd87 f5                               push af
                           .data:0000cd88 fd 7e 08                         ld a,(iy+8)
                           .data:0000cd8b 18 07                            jr 0xcd94
                           .data:0000cd8d cd 77 cd                         call 0xcd77
                           .data:0000cd90 f5                               push af
                           .data:0000cd91 fd 7e 2c                         ld a,(iy+44)
                           .data:0000cd94 fe ff                            cp 0xff
                           .data:0000cd96 28 12                            jr z,0xcdaa
                           .data:0000cd98 cd 16 ce                         call 0xce16
                           .data:0000cd9b f1                               pop af
                           .data:0000cd9c c9                               ret
                           .data:0000cd9d fd 7e 08                         ld a,(iy+8)
                           .data:0000cda0 18 03                            jr 0xcda5
                           .data:0000cda2 fd 7e 2c                         ld a,(iy+44)
                           .data:0000cda5 cd 77 cd                         call 0xcd77
                           .data:0000cda8 3c                               inc a
                           .data:0000cda9 c8                               ret z
                           .data:0000cdaa 3e 0e                            ld a,0x0e
                           .data:0000cdac b7                               or a
                           .data:0000cdad 18 0a                            jr 0xcdb9
                           .data:0000cdaf 3e 04                            ld a,0x04
                           .data:0000cdb1 cd ca db                         call 0xdbca
                           .data:0000cdb4 c6 0c                            add a,0x0c
                           .data:0000cdb6 f6 80                            or 0x80
                           .data:0000cdb8 bf                               cp a
                           .data:0000cdb9 fd 6e 06                         ld l,(iy+6)
                           .data:0000cdbc fd 66 07                         ld h,(iy+7)
                           .data:0000cdbf f9                               ld sp,hl
                           .data:0000cdc0 c9                               ret
                           .data:0000cdc1 3d                               dec a
                           .data:0000cdc2 3d                               dec a
                           .data:0000cdc3 c8                               ret z
                           .data:0000cdc4 c3 af cd                         jp 0xcdaf
                           .data:0000cdc7 cd cf cd                         call 0xcdcf
                           .data:0000cdca 46                               ld b,(hl)
                           .data:0000cdcb 23                               inc hl
                           .data:0000cdcc c3 f9 db                         jp 0xdbf9
                           .data:0000cdcf dd 6e 00                         ld l,(ix+0)
                           .data:0000cdd2 dd 66 01                         ld h,(ix+1)
                           .data:0000cdd5 dd 23                            inc ix
                           .data:0000cdd7 dd 23                            inc ix
                           .data:0000cdd9 c9                               ret
                           .data:0000cdda af                               xor a
                           .data:0000cddb 18 02                            jr 0xcddf
                           .data:0000cddd 3e 01                            ld a,0x01
                           .data:0000cddf cd 73 cd                         call 0xcd73
                           .data:0000cde2 18 13                            jr 0xcdf7
                           .data:0000cde4 cd 73 cd                         call 0xcd73
                           .data:0000cde7 cd c2 cd                         call 0xcdc2
                           .data:0000cdea cd c7 cd                         call 0xcdc7
                           .data:0000cded 05                               dec b
                           .data:0000cdee c2 af cd                         jp nz,0xcdaf
                           .data:0000cdf1 7e                               ld a,(hl)
                           .data:0000cdf2 cd a6 ca                         call 0xcaa6
                           .data:0000cdf5 d6 41                            sub 0x41
                           .data:0000cdf7 cd 16 ce                         call 0xce16
                           .data:0000cdfa fd 77 00                         ld (iy+0),a
                           .data:0000cdfd c9                               ret
                           .data:0000cdfe cd 73 cd                         call 0xcd73
                           .data:0000ce01 cd c2 cd                         call 0xcdc2
                           .data:0000ce04 cd cf cd                         call 0xcdcf
                           .data:0000ce07 11 10 00                         ld de,0x0010
                           .data:0000ce0a cd f3 db                         call 0xdbf3
                           .data:0000ce0d d2 af cd                         jp nc,0xcdaf
                           .data:0000ce10 fd 75 01                         ld (iy+1),l
                           .data:0000ce13 c9                               ret
                           .data:0000ce14 0a                               ld a,(bc)
                           .data:0000ce15 03                               inc bc
                           .data:0000ce16 e5                               push hl
                           .data:0000ce17 d5                               push de
                           .data:0000ce18 c5                               push bc
                           .data:0000ce19 f5                               push af
                           .data:0000ce1a 4f                               ld c,a
                           .data:0000ce1b 1e ff                            ld e,0xff
                           .data:0000ce1d fd 7e 08                         ld a,(iy+8)
                           .data:0000ce20 b9                               cp c
                           .data:0000ce21 28 08                            jr z,0xce2b
                           .data:0000ce23 fd 7e 2c                         ld a,(iy+44)
                           .data:0000ce26 b9                               cp c
                           .data:0000ce27 28 02                            jr z,0xce2b
                           .data:0000ce29 1e 00                            ld e,0x00
                           .data:0000ce2b d5                               push de
                           .data:0000ce2c c5                               push bc
                           .data:0000ce2d cd f0 c4                         call 0xc4f0
                           .data:0000ce30 c1                               pop bc
                           .data:0000ce31 d1                               pop de
                           .data:0000ce32 7c                               ld a,h
                           .data:0000ce33 b5                               or l
                           .data:0000ce34 ca af cd                         jp z,0xcdaf
                           .data:0000ce37 fd 75 03                         ld (iy+3),l
                           .data:0000ce3a fd 74 04                         ld (iy+4),h
                           .data:0000ce3d fd 73 05                         ld (iy+5),e
                           .data:0000ce40 fd 71 02                         ld (iy+2),c
                           .data:0000ce43 f1                               pop af
                           .data:0000ce44 c1                               pop bc
                           .data:0000ce45 d1                               pop de
                           .data:0000ce46 e1                               pop hl
                           .data:0000ce47 c9                               ret
                           .data:0000ce48 21 50 00                         ld hl,0x0050
                           .data:0000ce4b cd 5a ce                         call 0xce5a
                           .data:0000ce4e e5                               push hl
                           .data:0000ce4f 11 42 00                         ld de,0x0042
                           .data:0000ce52 19                               add hl,de
                           .data:0000ce53 36 80                            ld (hl),0x80
                           .data:0000ce55 e1                               pop hl
                           .data:0000ce56 c9                               ret
                           .data:0000ce57 21 9a 00                         ld hl,0x009a
                           .data:0000ce5a c5                               push bc
                           .data:0000ce5b d5                               push de
                           .data:0000ce5c cd 9f ca                         call 0xca9f
                           .data:0000ce5f 36 00                            ld (hl),0x00
                           .data:0000ce61 23                               inc hl
                           .data:0000ce62 73                               ld (hl),e
                           .data:0000ce63 23                               inc hl
                           .data:0000ce64 72                               ld (hl),d
                           .data:0000ce65 23                               inc hl
                           .data:0000ce66 73                               ld (hl),e
                           .data:0000ce67 23                               inc hl
                           .data:0000ce68 72                               ld (hl),d
                           .data:0000ce69 23                               inc hl
                           .data:0000ce6a e5                               push hl
                           .data:0000ce6b c5                               push bc
                           .data:0000ce6c 01 45 00                         ld bc,0x0045
                           .data:0000ce6f eb                               ex de,hl
                           .data:0000ce70 cd af ca                         call 0xcaaf
                           .data:0000ce73 c1                               pop bc
                           .data:0000ce74 60                               ld h,b
                           .data:0000ce75 69                               ld l,c
                           .data:0000ce76 d1                               pop de
                           .data:0000ce77 d5                               push de
                           .data:0000ce78 01 0c 00                         ld bc,0x000c
                           .data:0000ce7b ed b0                            ldir
                           .data:0000ce7d e1                               pop hl
                           .data:0000ce7e d1                               pop de
                           .data:0000ce7f e5                               push hl
                           .data:0000ce80 01 12 00                         ld bc,0x0012
                           .data:0000ce83 09                               add hl,bc
                           .data:0000ce84 36 16                            ld (hl),0x16
                           .data:0000ce86 23                               inc hl
                           .data:0000ce87 23                               inc hl
                           .data:0000ce88 23                               inc hl
                           .data:0000ce89 73                               ld (hl),e
                           .data:0000ce8a 23                               inc hl
                           .data:0000ce8b 72                               ld (hl),d
                           .data:0000ce8c 23                               inc hl
                           .data:0000ce8d 36 ff                            ld (hl),0xff
                           .data:0000ce8f e1                               pop hl
                           .data:0000ce90 c1                               pop bc
                           .data:0000ce91 c9                               ret
                           .data:0000ce92 e5                               push hl
                           .data:0000ce93 21 00 00                         ld hl,0x0000
                           .data:0000ce96 54                               ld d,h
                           .data:0000ce97 06 43                            ld b,0x43
                           .data:0000ce99 e3                               ex (sp),hl
                           .data:0000ce9a 7e                               ld a,(hl)
                           .data:0000ce9b 23                               inc hl
                           .data:0000ce9c e3                               ex (sp),hl
                           .data:0000ce9d 5f                               ld e,a
                           .data:0000ce9e 19                               add hl,de
                           .data:0000ce9f 10 f8                            djnz 0xce99
                           .data:0000cea1 eb                               ex de,hl
                           .data:0000cea2 e1                               pop hl
                           .data:0000cea3 c9                               ret
                           .data:0000cea4 e5                               push hl
                           .data:0000cea5 cd 92 ce                         call 0xce92
                           .data:0000cea8 73                               ld (hl),e
                           .data:0000cea9 23                               inc hl
                           .data:0000ceaa 72                               ld (hl),d
                           .data:0000ceab e1                               pop hl
                           .data:0000ceac c3 f9 d3                         jp 0xd3f9
                           .data:0000ceaf cd 9d cd                         call 0xcd9d
                           .data:0000ceb2 d5                               push de
                           .data:0000ceb3 cd 6f da                         call 0xda6f
                           .data:0000ceb6 cd 14 ce                         call 0xce14
                           .data:0000ceb9 21 09 00                         ld hl,0x0009
                           .data:0000cebc 09                               add hl,bc
                           .data:0000cebd 7e                               ld a,(hl)
                           .data:0000cebe 3c                               inc a
                           .data:0000cebf 28 08                            jr z,0xcec9
                           .data:0000cec1 cd 51 d6                         call 0xd651
                           .data:0000cec4 d2 0c d5                         jp nc,0xd50c
                           .data:0000cec7 18 1e                            jr 0xcee7
                           .data:0000cec9 cd a8 d2                         call 0xd2a8
                           .data:0000cecc cd 51 d6                         call 0xd651
                           .data:0000cecf 38 16                            jr c,0xcee7
                           .data:0000ced1 cd b3 d2                         call 0xd2b3
                           .data:0000ced4 cd 51 d6                         call 0xd651
                           .data:0000ced7 38 0e                            jr c,0xcee7
                           .data:0000ced9 cd b7 d2                         call 0xd2b7
                           .data:0000cedc cd 51 d6                         call 0xd651
                           .data:0000cedf f5                               push af
                           .data:0000cee0 d4 a8 d2                         call nc,0xd2a8
                           .data:0000cee3 f1                               pop af
                           .data:0000cee4 d2 0c d5                         jp nc,0xd50c
                           .data:0000cee7 d1                               pop de
                           .data:0000cee8 cd 48 ce                         call 0xce48
                           .data:0000ceeb e5                               push hl
                           .data:0000ceec 11 08 00                         ld de,0x0008
                           .data:0000ceef cd 98 ca                         call 0xca98
                           .data:0000cef2 0b                               dec bc
                           .data:0000cef3 0a                               ld a,(bc)
                           .data:0000cef4 12                               ld (de),a
                           .data:0000cef5 cd 9c d7                         call 0xd79c
                           .data:0000cef8 21 e4 00                         ld hl,0x00e4
                           .data:0000cefb cd 9f ca                         call 0xca9f
                           .data:0000cefe cd 92 d3                         call 0xd392
                           .data:0000cf01 30 1f                            jr nc,0xcf22
                           .data:0000cf03 e5                               push hl
                           .data:0000cf04 d5                               push de
                           .data:0000cf05 cd 92 ce                         call 0xce92
                           .data:0000cf08 cd f9 db                         call 0xdbf9
                           .data:0000cf0b cd f3 db                         call 0xdbf3
                           .data:0000cf0e d1                               pop de
                           .data:0000cf0f e1                               pop hl
                           .data:0000cf10 20 0d                            jr nz,0xcf1f
                           .data:0000cf12 11 55 00                         ld de,0x0055
                           .data:0000cf15 cd 98 ca                         call 0xca98
                           .data:0000cf18 01 45 00                         ld bc,0x0045
                           .data:0000cf1b ed b0                            ldir
                           .data:0000cf1d 18 03                            jr 0xcf22
                           .data:0000cf1f cd 9c d7                         call 0xd79c
                           .data:0000cf22 e1                               pop hl
                           .data:0000cf23 e5                               push hl
                           .data:0000cf24 11 15 00                         ld de,0x0015
                           .data:0000cf27 19                               add hl,de
                           .data:0000cf28 5e                               ld e,(hl)
                           .data:0000cf29 23                               inc hl
                           .data:0000cf2a 56                               ld d,(hl)
                           .data:0000cf2b 23                               inc hl
                           .data:0000cf2c 23                               inc hl
                           .data:0000cf2d 4e                               ld c,(hl)
                           .data:0000cf2e 23                               inc hl
                           .data:0000cf2f 46                               ld b,(hl)
                           .data:0000cf30 e1                               pop hl
                           .data:0000cf31 37                               scf
                           .data:0000cf32 9f                               sbc a,a
                           .data:0000cf33 fd 7e 67                         ld a,(iy+103)
                           .data:0000cf36 c9                               ret
                           .data:0000cf37 cd a2 cd                         call 0xcda2
                           .data:0000cf3a d5                               push de
                           .data:0000cf3b cd 6a da                         call 0xda6a
                           .data:0000cf3e cd 14 ce                         call 0xce14
                           .data:0000cf41 d1                               pop de
                           .data:0000cf42 cd 57 ce                         call 0xce57
                           .data:0000cf45 e5                               push hl
                           .data:0000cf46 cd ab d2                         call 0xd2ab
                           .data:0000cf49 cd 76 d6                         call 0xd676
                           .data:0000cf4c 60                               ld h,b
                           .data:0000cf4d 69                               ld l,c
                           .data:0000cf4e 2b                               dec hl
                           .data:0000cf4f 11 2c 00                         ld de,0x002c
                           .data:0000cf52 cd 98 ca                         call 0xca98
                           .data:0000cf55 01 0d 00                         ld bc,0x000d
                           .data:0000cf58 ed b0                            ldir
                           .data:0000cf5a 01 17 00                         ld bc,0x0017
                           .data:0000cf5d cd af ca                         call 0xcaaf
                           .data:0000cf60 e1                               pop hl
                           .data:0000cf61 37                               scf
                           .data:0000cf62 9f                               sbc a,a
                           .data:0000cf63 c9                               ret
                           .data:0000cf64 e5                               push hl
                           .data:0000cf65 d5                               push de
                           .data:0000cf66 c5                               push bc
                           .data:0000cf67 cd 74 cf                         call 0xcf74
                           .data:0000cf6a c1                               pop bc
                           .data:0000cf6b d1                               pop de
                           .data:0000cf6c e1                               pop hl
                           .data:0000cf6d d0                               ret nc
                           .data:0000cf6e fe 1a                            cp 0x1a
                           .data:0000cf70 37                               scf
                           .data:0000cf71 c0                               ret nz
                           .data:0000cf72 b7                               or a
                           .data:0000cf73 c9                               ret
                           .data:0000cf74 cd 84 cd                         call 0xcd84
                           .data:0000cf77 fd e5                            push iy
                           .data:0000cf79 d1                               pop de
                           .data:0000cf7a 21 50 00                         ld hl,0x0050
                           .data:0000cf7d 19                               add hl,de
                           .data:0000cf7e 7e                               ld a,(hl)
                           .data:0000cf7f fe 02                            cp 0x02
                           .data:0000cf81 ca aa cd                         jp z,0xcdaa
                           .data:0000cf84 36 01                            ld (hl),0x01
                           .data:0000cf86 21 95 00                         ld hl,0x0095
                           .data:0000cf89 19                               add hl,de
                           .data:0000cf8a 7e                               ld a,(hl)
                           .data:0000cf8b 23                               inc hl
                           .data:0000cf8c b6                               or (hl)
                           .data:0000cf8d 23                               inc hl
                           .data:0000cf8e b6                               or (hl)
                           .data:0000cf8f 28 36                            jr z,0xcfc7
                           .data:0000cf91 21 68 00                         ld hl,0x0068
                           .data:0000cf94 19                               add hl,de
                           .data:0000cf95 7e                               ld a,(hl)
                           .data:0000cf96 23                               inc hl
                           .data:0000cf97 b6                               or (hl)
                           .data:0000cf98 2b                               dec hl
                           .data:0000cf99 cc cb cf                         call z,0xcfcb
                           .data:0000cf9c 7e                               ld a,(hl)
                           .data:0000cf9d 23                               inc hl
                           .data:0000cf9e b6                               or (hl)
                           .data:0000cf9f 28 26                            jr z,0xcfc7
                           .data:0000cfa1 46                               ld b,(hl)
                           .data:0000cfa2 2b                               dec hl
                           .data:0000cfa3 4e                               ld c,(hl)
                           .data:0000cfa4 0b                               dec bc
                           .data:0000cfa5 71                               ld (hl),c
                           .data:0000cfa6 23                               inc hl
                           .data:0000cfa7 70                               ld (hl),b
                           .data:0000cfa8 21 95 00                         ld hl,0x0095
                           .data:0000cfab 19                               add hl,de
                           .data:0000cfac 06 03                            ld b,0x03
                           .data:0000cfae 7e                               ld a,(hl)
                           .data:0000cfaf d6 01                            sub 0x01
                           .data:0000cfb1 77                               ld (hl),a
                           .data:0000cfb2 30 03                            jr nc,0xcfb7
                           .data:0000cfb4 23                               inc hl
                           .data:0000cfb5 10 f7                            djnz 0xcfae
                           .data:0000cfb7 21 53 00                         ld hl,0x0053
                           .data:0000cfba 19                               add hl,de
                           .data:0000cfbb 5e                               ld e,(hl)
                           .data:0000cfbc 23                               inc hl
                           .data:0000cfbd 56                               ld d,(hl)
                           .data:0000cfbe eb                               ex de,hl
                           .data:0000cfbf e7                               rst 0x20
                           .data:0000cfc0 eb                               ex de,hl
                           .data:0000cfc1 13                               inc de
                           .data:0000cfc2 72                               ld (hl),d
                           .data:0000cfc3 2b                               dec hl
                           .data:0000cfc4 73                               ld (hl),e
                           .data:0000cfc5 37                               scf
                           .data:0000cfc6 c9                               ret
                           .data:0000cfc7 3e 0f                            ld a,0x0f
                           .data:0000cfc9 b7                               or a
                           .data:0000cfca c9                               ret
                           .data:0000cfcb e5                               push hl
                           .data:0000cfcc d5                               push de
                           .data:0000cfcd e5                               push hl
                           .data:0000cfce 21 51 00                         ld hl,0x0051
                           .data:0000cfd1 19                               add hl,de
                           .data:0000cfd2 cd f9 db                         call 0xdbf9
                           .data:0000cfd5 e5                               push hl
                           .data:0000cfd6 01 10 00                         ld bc,0x0010
                           .data:0000cfd9 cd 49 d0                         call 0xd049
                           .data:0000cfdc 3e 10                            ld a,0x10
                           .data:0000cfde 91                               sub c
                           .data:0000cfdf 47                               ld b,a
                           .data:0000cfe0 0e 00                            ld c,0x00
                           .data:0000cfe2 cb 38                            srl b
                           .data:0000cfe4 cb 19                            rr c
                           .data:0000cfe6 d1                               pop de
                           .data:0000cfe7 e1                               pop hl
                           .data:0000cfe8 71                               ld (hl),c
                           .data:0000cfe9 23                               inc hl
                           .data:0000cfea 70                               ld (hl),b
                           .data:0000cfeb 01 ea ff                         ld bc,0xffea
                           .data:0000cfee 09                               add hl,bc
                           .data:0000cfef 73                               ld (hl),e
                           .data:0000cff0 23                               inc hl
                           .data:0000cff1 72                               ld (hl),d
                           .data:0000cff2 d1                               pop de
                           .data:0000cff3 e1                               pop hl
                           .data:0000cff4 c9                               ret
                           .data:0000cff5 cd 84 cd                         call 0xcd84
                           .data:0000cff8 e5                               push hl
                           .data:0000cff9 21 50 00                         ld hl,0x0050
                           .data:0000cffc cd 9f ca                         call 0xca9f
                           .data:0000cfff 7e                               ld a,(hl)
                           .data:0000d000 fe 01                            cp 0x01
                           .data:0000d002 ca aa cd                         jp z,0xcdaa
                           .data:0000d005 36 02                            ld (hl),0x02
                           .data:0000d007 11 45 00                         ld de,0x0045
                           .data:0000d00a 19                               add hl,de
                           .data:0000d00b 5e                               ld e,(hl)
                           .data:0000d00c 23                               inc hl
                           .data:0000d00d 56                               ld d,(hl)
                           .data:0000d00e e1                               pop hl
                           .data:0000d00f d5                               push de
                           .data:0000d010 e5                               push hl
                           .data:0000d011 eb                               ex de,hl
                           .data:0000d012 3e 07                            ld a,0x07
                           .data:0000d014 cd eb db                         call 0xdbeb
                           .data:0000d017 44                               ld b,h
                           .data:0000d018 4d                               ld c,l
                           .data:0000d019 e1                               pop hl
                           .data:0000d01a cd 49 d0                         call 0xd049
                           .data:0000d01d d1                               pop de
                           .data:0000d01e 30 1e                            jr nc,0xd03e
                           .data:0000d020 7b                               ld a,e
                           .data:0000d021 e6 7f                            and 0x7f
                           .data:0000d023 28 19                            jr z,0xd03e
                           .data:0000d025 f5                               push af
                           .data:0000d026 e5                               push hl
                           .data:0000d027 21 e4 00                         ld hl,0x00e4
                           .data:0000d02a cd 9f ca                         call 0xca9f
                           .data:0000d02d e5                               push hl
                           .data:0000d02e 01 01 00                         ld bc,0x0001
                           .data:0000d031 cd 49 d0                         call 0xd049
                           .data:0000d034 e1                               pop hl
                           .data:0000d035 d1                               pop de
                           .data:0000d036 c1                               pop bc
                           .data:0000d037 30 05                            jr nc,0xd03e
                           .data:0000d039 48                               ld c,b
                           .data:0000d03a 06 00                            ld b,0x00
                           .data:0000d03c ed b0                            ldir
                           .data:0000d03e 21 6f 00                         ld hl,0x006f
                           .data:0000d041 cd 9f ca                         call 0xca9f
                           .data:0000d044 37                               scf
                           .data:0000d045 9f                               sbc a,a
                           .data:0000d046 c3 f9 db                         jp 0xdbf9
                           .data:0000d049 18 14                            jr 0xd05f
                           .data:0000d04b cd 92 d3                         call 0xd392
                           .data:0000d04e d0                               ret nc
                           .data:0000d04f 11 67 00                         ld de,0x0067
                           .data:0000d052 cd 98 ca                         call 0xca98
                           .data:0000d055 1a                               ld a,(de)
                           .data:0000d056 1f                               rra
                           .data:0000d057 dc 52 d2                         call c,0xd252
                           .data:0000d05a 11 80 00                         ld de,0x0080
                           .data:0000d05d 19                               add hl,de
                           .data:0000d05e 0b                               dec bc
                           .data:0000d05f 78                               ld a,b
                           .data:0000d060 b1                               or c
                           .data:0000d061 20 e8                            jr nz,0xd04b
                           .data:0000d063 37                               scf
                           .data:0000d064 c9                               ret
                           .data:0000d065 cd 64 cf                         call 0xcf64
                           .data:0000d068 d0                               ret nc
                           .data:0000d069 e5                               push hl
                           .data:0000d06a d5                               push de
                           .data:0000d06b f5                               push af
                           .data:0000d06c 21 53 00                         ld hl,0x0053
                           .data:0000d06f cd 9f ca                         call 0xca9f
                           .data:0000d072 5e                               ld e,(hl)
                           .data:0000d073 23                               inc hl
                           .data:0000d074 56                               ld d,(hl)
                           .data:0000d075 1b                               dec de
                           .data:0000d076 72                               ld (hl),d
                           .data:0000d077 2b                               dec hl
                           .data:0000d078 73                               ld (hl),e
                           .data:0000d079 54                               ld d,h
                           .data:0000d07a 5d                               ld e,l
                           .data:0000d07b 21 42 00                         ld hl,0x0042
                           .data:0000d07e 19                               add hl,de
                           .data:0000d07f cd ab d7                         call 0xd7ab
                           .data:0000d082 21 15 00                         ld hl,0x0015
                           .data:0000d085 19                               add hl,de
                           .data:0000d086 34                               inc (hl)
                           .data:0000d087 20 02                            jr nz,0xd08b
                           .data:0000d089 23                               inc hl
                           .data:0000d08a 34                               inc (hl)
                           .data:0000d08b f1                               pop af
                           .data:0000d08c d1                               pop de
                           .data:0000d08d e1                               pop hl
                           .data:0000d08e c9                               ret
                           .data:0000d08f cd 8d cd                         call 0xcd8d
                           .data:0000d092 e5                               push hl
                           .data:0000d093 d5                               push de
                           .data:0000d094 c5                               push bc
                           .data:0000d095 f5                               push af
                           .data:0000d096 fd e5                            push iy
                           .data:0000d098 d1                               pop de
                           .data:0000d099 21 9a 00                         ld hl,0x009a
                           .data:0000d09c 19                               add hl,de
                           .data:0000d09d 7e                               ld a,(hl)
                           .data:0000d09e fe 02                            cp 0x02
                           .data:0000d0a0 ca aa cd                         jp z,0xcdaa
                           .data:0000d0a3 36 01                            ld (hl),0x01
                           .data:0000d0a5 21 b2 00                         ld hl,0x00b2
                           .data:0000d0a8 19                               add hl,de
                           .data:0000d0a9 e5                               push hl
                           .data:0000d0aa cd f9 db                         call 0xdbf9
                           .data:0000d0ad 01 00 f8                         ld bc,0xf800
                           .data:0000d0b0 09                               add hl,bc
                           .data:0000d0b1 d5                               push de
                           .data:0000d0b2 dc 18 d1                         call c,0xd118
                           .data:0000d0b5 d1                               pop de
                           .data:0000d0b6 e1                               pop hl
                           .data:0000d0b7 34                               inc (hl)
                           .data:0000d0b8 23                               inc hl
                           .data:0000d0b9 20 01                            jr nz,0xd0bc
                           .data:0000d0bb 34                               inc (hl)
                           .data:0000d0bc 21 df 00                         ld hl,0x00df
                           .data:0000d0bf 19                               add hl,de
                           .data:0000d0c0 cd ab d7                         call 0xd7ab
                           .data:0000d0c3 21 9d 00                         ld hl,0x009d
                           .data:0000d0c6 19                               add hl,de
                           .data:0000d0c7 f1                               pop af
                           .data:0000d0c8 4e                               ld c,(hl)
                           .data:0000d0c9 23                               inc hl
                           .data:0000d0ca 46                               ld b,(hl)
                           .data:0000d0cb 2b                               dec hl
                           .data:0000d0cc 02                               ld (bc),a
                           .data:0000d0cd 34                               inc (hl)
                           .data:0000d0ce 20 02                            jr nz,0xd0d2
                           .data:0000d0d0 23                               inc hl
                           .data:0000d0d1 34                               inc (hl)
                           .data:0000d0d2 c1                               pop bc
                           .data:0000d0d3 d1                               pop de
                           .data:0000d0d4 e1                               pop hl
                           .data:0000d0d5 37                               scf
                           .data:0000d0d6 9f                               sbc a,a
                           .data:0000d0d7 c9                               ret
                           .data:0000d0d8 cd 8d cd                         call 0xcd8d
                           .data:0000d0db f5                               push af
                           .data:0000d0dc e5                               push hl
                           .data:0000d0dd d5                               push de
                           .data:0000d0de 21 9a 00                         ld hl,0x009a
                           .data:0000d0e1 cd 9f ca                         call 0xca9f
                           .data:0000d0e4 7e                               ld a,(hl)
                           .data:0000d0e5 fe 01                            cp 0x01
                           .data:0000d0e7 ca aa cd                         jp z,0xcdaa
                           .data:0000d0ea 36 02                            ld (hl),0x02
                           .data:0000d0ec 11 20 00                         ld de,0x0020
                           .data:0000d0ef 19                               add hl,de
                           .data:0000d0f0 70                               ld (hl),b
                           .data:0000d0f1 2b                               dec hl
                           .data:0000d0f2 71                               ld (hl),c
                           .data:0000d0f3 c1                               pop bc
                           .data:0000d0f4 2b                               dec hl
                           .data:0000d0f5 70                               ld (hl),b
                           .data:0000d0f6 2b                               dec hl
                           .data:0000d0f7 71                               ld (hl),c
                           .data:0000d0f8 11 29 00                         ld de,0x0029
                           .data:0000d0fb 19                               add hl,de
                           .data:0000d0fc 70                               ld (hl),b
                           .data:0000d0fd 2b                               dec hl
                           .data:0000d0fe 71                               ld (hl),c
                           .data:0000d0ff 11 d3 ff                         ld de,0xffd3
                           .data:0000d102 19                               add hl,de
                           .data:0000d103 71                               ld (hl),c
                           .data:0000d104 23                               inc hl
                           .data:0000d105 70                               ld (hl),b
                           .data:0000d106 c1                               pop bc
                           .data:0000d107 23                               inc hl
                           .data:0000d108 71                               ld (hl),c
                           .data:0000d109 23                               inc hl
                           .data:0000d10a 70                               ld (hl),b
                           .data:0000d10b 11 e6 ff                         ld de,0xffe6
                           .data:0000d10e 19                               add hl,de
                           .data:0000d10f 71                               ld (hl),c
                           .data:0000d110 23                               inc hl
                           .data:0000d111 70                               ld (hl),b
                           .data:0000d112 f1                               pop af
                           .data:0000d113 11 15 00                         ld de,0x0015
                           .data:0000d116 19                               add hl,de
                           .data:0000d117 77                               ld (hl),a
                           .data:0000d118 fd e5                            push iy
                           .data:0000d11a d1                               pop de
                           .data:0000d11b 21 b6 00                         ld hl,0x00b6
                           .data:0000d11e 19                               add hl,de
                           .data:0000d11f 7e                               ld a,(hl)
                           .data:0000d120 b7                               or a
                           .data:0000d121 28 18                            jr z,0xd13b
                           .data:0000d123 21 b1 00                         ld hl,0x00b1
                           .data:0000d126 19                               add hl,de
                           .data:0000d127 7e                               ld a,(hl)
                           .data:0000d128 e6 0f                            and 0x0f
                           .data:0000d12a fe 06                            cp 0x06
                           .data:0000d12c 28 0d                            jr z,0xd13b
                           .data:0000d12e 21 2c 00                         ld hl,0x002c
                           .data:0000d131 19                               add hl,de
                           .data:0000d132 d5                               push de
                           .data:0000d133 eb                               ex de,hl
                           .data:0000d134 cd a7 d7                         call 0xd7a7
                           .data:0000d137 cd 7d d7                         call 0xd77d
                           .data:0000d13a d1                               pop de
                           .data:0000d13b 21 b2 00                         ld hl,0x00b2
                           .data:0000d13e 19                               add hl,de
                           .data:0000d13f e5                               push hl
                           .data:0000d140 5e                               ld e,(hl)
                           .data:0000d141 23                               inc hl
                           .data:0000d142 56                               ld d,(hl)
                           .data:0000d143 01 e8 ff                         ld bc,0xffe8
                           .data:0000d146 09                               add hl,bc
                           .data:0000d147 cd f9 db                         call 0xdbf9
                           .data:0000d14a e5                               push hl
                           .data:0000d14b cd 64 d1                         call 0xd164
                           .data:0000d14e c1                               pop bc
                           .data:0000d14f e1                               pop hl
                           .data:0000d150 36 00                            ld (hl),0x00
                           .data:0000d152 23                               inc hl
                           .data:0000d153 36 00                            ld (hl),0x00
                           .data:0000d155 23                               inc hl
                           .data:0000d156 23                               inc hl
                           .data:0000d157 23                               inc hl
                           .data:0000d158 36 00                            ld (hl),0x00
                           .data:0000d15a 11 e7 ff                         ld de,0xffe7
                           .data:0000d15d 19                               add hl,de
                           .data:0000d15e 71                               ld (hl),c
                           .data:0000d15f 23                               inc hl
                           .data:0000d160 70                               ld (hl),b
                           .data:0000d161 37                               scf
                           .data:0000d162 9f                               sbc a,a
                           .data:0000d163 c9                               ret
                           .data:0000d164 d5                               push de
                           .data:0000d165 3e 07                            ld a,0x07
                           .data:0000d167 eb                               ex de,hl
                           .data:0000d168 cd eb db                         call 0xdbeb
                           .data:0000d16b eb                               ex de,hl
                           .data:0000d16c 42                               ld b,d
                           .data:0000d16d 4b                               ld c,e
                           .data:0000d16e cd 88 d1                         call 0xd188
                           .data:0000d171 c1                               pop bc
                           .data:0000d172 79                               ld a,c
                           .data:0000d173 e6 7f                            and 0x7f
                           .data:0000d175 c8                               ret z
                           .data:0000d176 4f                               ld c,a
                           .data:0000d177 06 00                            ld b,0x00
                           .data:0000d179 11 e4 00                         ld de,0x00e4
                           .data:0000d17c cd 98 ca                         call 0xca98
                           .data:0000d17f d5                               push de
                           .data:0000d180 cd 1b b9                         call 0xb91b
                           .data:0000d183 3e 1a                            ld a,0x1a
                           .data:0000d185 12                               ld (de),a
                           .data:0000d186 e1                               pop hl
                           .data:0000d187 03                               inc bc
                           .data:0000d188 18 27                            jr 0xd1b1
                           .data:0000d18a e5                               push hl
                           .data:0000d18b 11 b1 00                         ld de,0x00b1
                           .data:0000d18e cd 98 ca                         call 0xca98
                           .data:0000d191 1a                               ld a,(de)
                           .data:0000d192 1f                               rra
                           .data:0000d193 30 13                            jr nc,0xd1a8
                           .data:0000d195 c5                               push bc
                           .data:0000d196 11 e4 00                         ld de,0x00e4
                           .data:0000d199 cd 98 ca                         call 0xca98
                           .data:0000d19c d5                               push de
                           .data:0000d19d 01 80 00                         ld bc,0x0080
                           .data:0000d1a0 cd 1b b9                         call 0xb91b
                           .data:0000d1a3 e1                               pop hl
                           .data:0000d1a4 c1                               pop bc
                           .data:0000d1a5 cd 52 d2                         call 0xd252
                           .data:0000d1a8 cd af d3                         call 0xd3af
                           .data:0000d1ab e1                               pop hl
                           .data:0000d1ac 11 80 00                         ld de,0x0080
                           .data:0000d1af 19                               add hl,de
                           .data:0000d1b0 0b                               dec bc
                           .data:0000d1b1 78                               ld a,b
                           .data:0000d1b2 b1                               or c
                           .data:0000d1b3 20 d5                            jr nz,0xd18a
                           .data:0000d1b5 c9                               ret
                           .data:0000d1b6 cd 84 cd                         call 0xcd84
                           .data:0000d1b9 cd e5 c9                         call 0xc9e5
                           .data:0000d1bc fd 36 08 ff                      ld (iy+8),0xff
                           .data:0000d1c0 18 6e                            jr 0xd230
                           .data:0000d1c2 cd 8d cd                         call 0xcd8d
                           .data:0000d1c5 11 2d 00                         ld de,0x002d
                           .data:0000d1c8 cd 98 ca                         call 0xca98
                           .data:0000d1cb af                               xor a
                           .data:0000d1cc cd 3c d8                         call 0xd83c
                           .data:0000d1cf 1b                               dec de
                           .data:0000d1d0 3e ff                            ld a,0xff
                           .data:0000d1d2 12                               ld (de),a
                           .data:0000d1d3 cd 1f c5                         call 0xc51f
                           .data:0000d1d6 18 58                            jr 0xd230
                           .data:0000d1d8 21 df 00                         ld hl,0x00df
                           .data:0000d1db cd 9f ca                         call 0xca9f
                           .data:0000d1de 7e                               ld a,(hl)
                           .data:0000d1df 23                               inc hl
                           .data:0000d1e0 b6                               or (hl)
                           .data:0000d1e1 23                               inc hl
                           .data:0000d1e2 b6                               or (hl)
                           .data:0000d1e3 28 dd                            jr z,0xd1c2
                           .data:0000d1e5 cd 8d cd                         call 0xcd8d
                           .data:0000d1e8 cd 18 d1                         call 0xd118
                           .data:0000d1eb 11 2c 00                         ld de,0x002c
                           .data:0000d1ee cd 98 ca                         call 0xca98
                           .data:0000d1f1 d5                               push de
                           .data:0000d1f2 cd 8c d7                         call 0xd78c
                           .data:0000d1f5 01 9f 00                         ld bc,0x009f
                           .data:0000d1f8 cd 90 ca                         call 0xca90
                           .data:0000d1fb 21 12 00                         ld hl,0x0012
                           .data:0000d1fe 09                               add hl,bc
                           .data:0000d1ff 5e                               ld e,(hl)
                           .data:0000d200 21 09 00                         ld hl,0x0009
                           .data:0000d203 09                               add hl,bc
                           .data:0000d204 7e                               ld a,(hl)
                           .data:0000d205 3c                               inc a
                           .data:0000d206 20 16                            jr nz,0xd21e
                           .data:0000d208 7b                               ld a,e
                           .data:0000d209 e6 0e                            and 0x0e
                           .data:0000d20b 20 05                            jr nz,0xd212
                           .data:0000d20d cd b3 d2                         call 0xd2b3
                           .data:0000d210 18 0c                            jr 0xd21e
                           .data:0000d212 fe 02                            cp 0x02
                           .data:0000d214 20 05                            jr nz,0xd21b
                           .data:0000d216 cd b7 d2                         call 0xd2b7
                           .data:0000d219 18 03                            jr 0xd21e
                           .data:0000d21b cd a8 d2                         call 0xd2a8
                           .data:0000d21e 60                               ld h,b
                           .data:0000d21f 69                               ld l,c
                           .data:0000d220 7b                               ld a,e
                           .data:0000d221 e6 0f                            and 0x0f
                           .data:0000d223 fe 06                            cp 0x06
                           .data:0000d225 c4 a4 ce                         call nz,0xcea4
                           .data:0000d228 c1                               pop bc
                           .data:0000d229 3e ff                            ld a,0xff
                           .data:0000d22b 02                               ld (bc),a
                           .data:0000d22c 03                               inc bc
                           .data:0000d22d cd da d2                         call 0xd2da
                           .data:0000d230 37                               scf
                           .data:0000d231 9f                               sbc a,a
                           .data:0000d232 c9                               ret
                           .data:0000d233 fd 66 02                         ld h,(iy+2)
                           .data:0000d236 fd 36 05 00                      ld (iy+5),0x00
                           .data:0000d23a 11 08 00                         ld de,0x0008
                           .data:0000d23d cd 43 d2                         call 0xd243
                           .data:0000d240 11 2c 00                         ld de,0x002c
                           .data:0000d243 cd 98 ca                         call 0xca98
                           .data:0000d246 1a                               ld a,(de)
                           .data:0000d247 bc                               cp h
                           .data:0000d248 c0                               ret nz
                           .data:0000d249 3e ff                            ld a,0xff
                           .data:0000d24b 12                               ld (de),a
                           .data:0000d24c 13                               inc de
                           .data:0000d24d 3e 09                            ld a,0x09
                           .data:0000d24f c3 ca db                         jp 0xdbca
                           .data:0000d252 e5                               push hl
                           .data:0000d253 c5                               push bc
                           .data:0000d254 e5                               push hl
                           .data:0000d255 11 01 01                         ld de,0x0101
                           .data:0000d258 06 81                            ld b,0x81
                           .data:0000d25a 18 0e                            jr 0xd26a
                           .data:0000d25c e3                               ex (sp),hl
                           .data:0000d25d e7                               rst 0x20
                           .data:0000d25e e3                               ex (sp),hl
                           .data:0000d25f ae                               xor (hl)
                           .data:0000d260 dd ae 00                         xor (ix+0)
                           .data:0000d263 e3                               ex (sp),hl
                           .data:0000d264 77                               ld (hl),a
                           .data:0000d265 23                               inc hl
                           .data:0000d266 e3                               ex (sp),hl
                           .data:0000d267 dd 23                            inc ix
                           .data:0000d269 23                               inc hl
                           .data:0000d26a 15                               dec d
                           .data:0000d26b 20 06                            jr nz,0xd273
                           .data:0000d26d 16 0b                            ld d,0x0b
                           .data:0000d26f dd 21 81 d2                      ld ix,0xd281
                           .data:0000d273 1d                               dec e
                           .data:0000d274 20 05                            jr nz,0xd27b
                           .data:0000d276 1e 0d                            ld e,0x0d
                           .data:0000d278 21 8c d2                         ld hl,0xd28c
                           .data:0000d27b 10 df                            djnz 0xd25c
                           .data:0000d27d e1                               pop hl
                           .data:0000d27e d1                               pop de
                           .data:0000d27f e1                               pop hl
                           .data:0000d280 c9                               ret
                           .data:0000d281 49                               ld c,c
                           .data:0000d282 b1                               or c
                           .data:0000d283 36 f0                            ld (hl),0xf0
                           .data:0000d285 2e 1e                            ld l,0x1e
                           .data:0000d287 06 2a                            ld b,0x2a
                           .data:0000d289 28 19                            jr z,0xd2a4
                           .data:0000d28b ea e2 9d                         jp pe,0x9de2
                           .data:0000d28e db 1a                            in a,(0x1a)
                           .data:0000d290 42                               ld b,d
                           .data:0000d291 29                               add hl,hl
                           .data:0000d292 39                               add hl,sp
                           .data:0000d293 c6 b3                            add a,0xb3
                           .data:0000d295 c6 90                            add a,0x90
                           .data:0000d297 45                               ld b,l
                           .data:0000d298 8a                               adc a,d
                           .data:0000d299 20 20                            jr nz,0xd2bb
                           .data:0000d29b 20 24                            jr nz,0xd2c1
                           .data:0000d29d 24                               inc h
                           .data:0000d29e 24                               inc h
                           .data:0000d29f 42                               ld b,d
                           .data:0000d2a0 41                               ld b,c
                           .data:0000d2a1 4b                               ld c,e
                           .data:0000d2a2 42                               ld b,d
                           .data:0000d2a3 41                               ld b,c
                           .data:0000d2a4 53                               ld d,e
                           .data:0000d2a5 42                               ld b,d
                           .data:0000d2a6 49                               ld c,c
                           .data:0000d2a7 4e                               ld c,(hl)
                           .data:0000d2a8 af                               xor a
                           .data:0000d2a9 18 0e                            jr 0xd2b9
                           .data:0000d2ab 3e 03                            ld a,0x03
                           .data:0000d2ad 18 0a                            jr 0xd2b9
                           .data:0000d2af 3e 06                            ld a,0x06
                           .data:0000d2b1 18 06                            jr 0xd2b9
                           .data:0000d2b3 3e 09                            ld a,0x09
                           .data:0000d2b5 18 02                            jr 0xd2b9
                           .data:0000d2b7 3e 0c                            ld a,0x0c
                           .data:0000d2b9 d5                               push de
                           .data:0000d2ba c6 99                            add a,0x99
                           .data:0000d2bc 5f                               ld e,a
                           .data:0000d2bd ce d2                            adc a,0xd2
                           .data:0000d2bf 93                               sub e
                           .data:0000d2c0 57                               ld d,a
                           .data:0000d2c1 18 07                            jr 0xd2ca
                           .data:0000d2c3 d5                               push de
                           .data:0000d2c4 11 a8 00                         ld de,0x00a8
                           .data:0000d2c7 cd 98 ca                         call 0xca98
                           .data:0000d2ca e5                               push hl
                           .data:0000d2cb c5                               push bc
                           .data:0000d2cc 21 09 00                         ld hl,0x0009
                           .data:0000d2cf 09                               add hl,bc
                           .data:0000d2d0 01 03 00                         ld bc,0x0003
                           .data:0000d2d3 eb                               ex de,hl
                           .data:0000d2d4 ed b0                            ldir
                           .data:0000d2d6 c1                               pop bc
                           .data:0000d2d7 e1                               pop hl
                           .data:0000d2d8 d1                               pop de
                           .data:0000d2d9 c9                               ret
                           .data:0000d2da 21 0c 00                         ld hl,0x000c
                           .data:0000d2dd 09                               add hl,bc
                           .data:0000d2de 36 ff                            ld (hl),0xff
                           .data:0000d2e0 23                               inc hl
                           .data:0000d2e1 23                               inc hl
                           .data:0000d2e2 36 ff                            ld (hl),0xff
                           .data:0000d2e4 cd 83 d6                         call 0xd683
                           .data:0000d2e7 e5                               push hl
                           .data:0000d2e8 21 00 00                         ld hl,0x0000
                           .data:0000d2eb e3                               ex (sp),hl
                           .data:0000d2ec cd a2 d6                         call 0xd6a2
                           .data:0000d2ef e3                               ex (sp),hl
                           .data:0000d2f0 30 28                            jr nc,0xd31a
                           .data:0000d2f2 cd af d2                         call 0xd2af
                           .data:0000d2f5 cd d8 d7                         call 0xd7d8
                           .data:0000d2f8 30 08                            jr nc,0xd302
                           .data:0000d2fa 26 01                            ld h,0x01
                           .data:0000d2fc cd d9 d9                         call 0xd9d9
                           .data:0000d2ff 38 01                            jr c,0xd302
                           .data:0000d301 24                               inc h
                           .data:0000d302 cd c3 d2                         call 0xd2c3
                           .data:0000d305 cd d8 d7                         call 0xd7d8
                           .data:0000d308 30 08                            jr nc,0xd312
                           .data:0000d30a 2e 01                            ld l,0x01
                           .data:0000d30c cd d9 d9                         call 0xd9d9
                           .data:0000d30f 38 01                            jr c,0xd312
                           .data:0000d311 2c                               inc l
                           .data:0000d312 7c                               ld a,h
                           .data:0000d313 b7                               or a
                           .data:0000d314 28 d5                            jr z,0xd2eb
                           .data:0000d316 7d                               ld a,l
                           .data:0000d317 b7                               or a
                           .data:0000d318 28 d1                            jr z,0xd2eb
                           .data:0000d31a f1                               pop af
                           .data:0000d31b 7d                               ld a,l
                           .data:0000d31c b7                               or a
                           .data:0000d31d 28 43                            jr z,0xd362
                           .data:0000d31f 3d                               dec a
                           .data:0000d320 28 66                            jr z,0xd388
                           .data:0000d322 7c                               ld a,h
                           .data:0000d323 b7                               or a
                           .data:0000d324 28 3c                            jr z,0xd362
                           .data:0000d326 3d                               dec a
                           .data:0000d327 28 45                            jr z,0xd36e
                           .data:0000d329 cd 83 d6                         call 0xd683
                           .data:0000d32c cd a2 d6                         call 0xd6a2
                           .data:0000d32f d0                               ret nc
                           .data:0000d330 cd 35 d3                         call 0xd335
                           .data:0000d333 18 f7                            jr 0xd32c
                           .data:0000d335 cd af d2                         call 0xd2af
                           .data:0000d338 cd d8 d7                         call 0xd7d8
                           .data:0000d33b da aa d4                         jp c,0xd4aa
                           .data:0000d33e cd 51 d3                         call 0xd351
                           .data:0000d341 d8                               ret c
                           .data:0000d342 cd c3 d2                         call 0xd2c3
                           .data:0000d345 cd d8 d7                         call 0xd7d8
                           .data:0000d348 d0                               ret nc
                           .data:0000d349 c5                               push bc
                           .data:0000d34a 42                               ld b,d
                           .data:0000d34b 4b                               ld c,e
                           .data:0000d34c cd af d2                         call 0xd2af
                           .data:0000d34f 18 0d                            jr 0xd35e
                           .data:0000d351 cd ab d2                         call 0xd2ab
                           .data:0000d354 cd d8 d7                         call 0xd7d8
                           .data:0000d357 d0                               ret nc
                           .data:0000d358 c5                               push bc
                           .data:0000d359 42                               ld b,d
                           .data:0000d35a 4b                               ld c,e
                           .data:0000d35b cd c3 d2                         call 0xd2c3
                           .data:0000d35e c1                               pop bc
                           .data:0000d35f c3 7a d9                         jp 0xd97a
                           .data:0000d362 cd 83 d6                         call 0xd683
                           .data:0000d365 cd a2 d6                         call 0xd6a2
                           .data:0000d368 d0                               ret nc
                           .data:0000d369 cd 3e d3                         call 0xd33e
                           .data:0000d36c 18 f7                            jr 0xd365
                           .data:0000d36e cd 83 d6                         call 0xd683
                           .data:0000d371 cd a2 d6                         call 0xd6a2
                           .data:0000d374 d0                               ret nc
                           .data:0000d375 cd 7a d3                         call 0xd37a
                           .data:0000d378 18 f7                            jr 0xd371
                           .data:0000d37a cd 51 d3                         call 0xd351
                           .data:0000d37d d8                               ret c
                           .data:0000d37e cd c3 d2                         call 0xd2c3
                           .data:0000d381 cd d8 d7                         call 0xd7d8
                           .data:0000d384 da aa d4                         jp c,0xd4aa
                           .data:0000d387 c9                               ret
                           .data:0000d388 cd c3 d2                         call 0xd2c3
                           .data:0000d38b 50                               ld d,b
                           .data:0000d38c 59                               ld e,c
                           .data:0000d38d 3e 0a                            ld a,0x0a
                           .data:0000d38f c3 b1 cd                         jp 0xcdb1
                           .data:0000d392 e5                               push hl
                           .data:0000d393 d5                               push de
                           .data:0000d394 c5                               push bc
                           .data:0000d395 e5                               push hl
                           .data:0000d396 11 08 00                         ld de,0x0008
                           .data:0000d399 cd 98 ca                         call 0xca98
                           .data:0000d39c cd 10 d4                         call 0xd410
                           .data:0000d39f 30 08                            jr nc,0xd3a9
                           .data:0000d3a1 eb                               ex de,hl
                           .data:0000d3a2 e3                               ex (sp),hl
                           .data:0000d3a3 cd e8 d9                         call 0xd9e8
                           .data:0000d3a6 d1                               pop de
                           .data:0000d3a7 18 48                            jr 0xd3f1
                           .data:0000d3a9 e1                               pop hl
                           .data:0000d3aa c1                               pop bc
                           .data:0000d3ab d1                               pop de
                           .data:0000d3ac e1                               pop hl
                           .data:0000d3ad b7                               or a
                           .data:0000d3ae c9                               ret
                           .data:0000d3af e5                               push hl
                           .data:0000d3b0 d5                               push de
                           .data:0000d3b1 c5                               push bc
                           .data:0000d3b2 e5                               push hl
                           .data:0000d3b3 11 2c 00                         ld de,0x002c
                           .data:0000d3b6 cd 98 ca                         call 0xca98
                           .data:0000d3b9 cd c8 d6                         call 0xd6c8
                           .data:0000d3bc 38 0b                            jr c,0xd3c9
                           .data:0000d3be 3e 08                            ld a,0x08
                           .data:0000d3c0 c2 b1 cd                         jp nz,0xcdb1
                           .data:0000d3c3 cd 8c d7                         call 0xd78c
                           .data:0000d3c6 cd fa d6                         call 0xd6fa
                           .data:0000d3c9 cd 2f d7                         call 0xd72f
                           .data:0000d3cc 0e 00                            ld c,0x00
                           .data:0000d3ce 38 18                            jr c,0xd3e8
                           .data:0000d3d0 d5                               push de
                           .data:0000d3d1 eb                               ex de,hl
                           .data:0000d3d2 cd 93 d8                         call 0xd893
                           .data:0000d3d5 eb                               ex de,hl
                           .data:0000d3d6 3e 08                            ld a,0x08
                           .data:0000d3d8 d2 b1 cd                         jp nc,0xcdb1
                           .data:0000d3db 73                               ld (hl),e
                           .data:0000d3dc 78                               ld a,b
                           .data:0000d3dd b7                               or a
                           .data:0000d3de 28 02                            jr z,0xd3e2
                           .data:0000d3e0 23                               inc hl
                           .data:0000d3e1 72                               ld (hl),d
                           .data:0000d3e2 d1                               pop de
                           .data:0000d3e3 cd 2f d7                         call 0xd72f
                           .data:0000d3e6 0e 02                            ld c,0x02
                           .data:0000d3e8 eb                               ex de,hl
                           .data:0000d3e9 e3                               ex (sp),hl
                           .data:0000d3ea cd f3 d9                         call 0xd9f3
                           .data:0000d3ed d1                               pop de
                           .data:0000d3ee cd 7d d7                         call 0xd77d
                           .data:0000d3f1 cd a7 d7                         call 0xd7a7
                           .data:0000d3f4 c1                               pop bc
                           .data:0000d3f5 d1                               pop de
                           .data:0000d3f6 e1                               pop hl
                           .data:0000d3f7 37                               scf
                           .data:0000d3f8 c9                               ret
                           .data:0000d3f9 e5                               push hl
                           .data:0000d3fa 11 2c 00                         ld de,0x002c
                           .data:0000d3fd cd 98 ca                         call 0xca98
                           .data:0000d400 cd 9c d7                         call 0xd79c
                           .data:0000d403 cd 10 d4                         call 0xd410
                           .data:0000d406 eb                               ex de,hl
                           .data:0000d407 e1                               pop hl
                           .data:0000d408 0e 00                            ld c,0x00
                           .data:0000d40a da f3 d9                         jp c,0xd9f3
                           .data:0000d40d c3 af cd                         jp 0xcdaf
                           .data:0000d410 cd c8 d6                         call 0xd6c8
                           .data:0000d413 38 12                            jr c,0xd427
                           .data:0000d415 c0                               ret nz
                           .data:0000d416 cd fa d6                         call 0xd6fa
                           .data:0000d419 d5                               push de
                           .data:0000d41a 42                               ld b,d
                           .data:0000d41b 4b                               ld c,e
                           .data:0000d41c 03                               inc bc
                           .data:0000d41d c5                               push bc
                           .data:0000d41e cd b3 d7                         call 0xd7b3
                           .data:0000d421 eb                               ex de,hl
                           .data:0000d422 d1                               pop de
                           .data:0000d423 dc df db                         call c,0xdbdf
                           .data:0000d426 d1                               pop de
                           .data:0000d427 dc 0c d7                         call c,0xd70c
                           .data:0000d42a da 2f d7                         jp c,0xd72f
                           .data:0000d42d c9                               ret
                           .data:0000d42e cd 73 cd                         call 0xcd73
                           .data:0000d431 06 00                            ld b,0x00
                           .data:0000d433 b7                               or a
                           .data:0000d434 28 06                            jr z,0xd43c
                           .data:0000d436 cd c2 cd                         call 0xcdc2
                           .data:0000d439 cd c7 cd                         call 0xcdc7
                           .data:0000d43c cd a6 da                         call 0xdaa6
                           .data:0000d43f cd 14 ce                         call 0xce14
                           .data:0000d442 cd d0 db                         call 0xdbd0
                           .data:0000d445 3e 0c                            ld a,0x0c
                           .data:0000d447 cd 72 d4                         call 0xd472
                           .data:0000d44a 65                               ld h,l
                           .data:0000d44b e5                               push hl
                           .data:0000d44c cd 83 d6                         call 0xd683
                           .data:0000d44f cd 98 d6                         call 0xd698
                           .data:0000d452 30 1a                            jr nc,0xd46e
                           .data:0000d454 cd df d9                         call 0xd9df
                           .data:0000d457 38 f6                            jr c,0xd44f
                           .data:0000d459 e3                               ex (sp),hl
                           .data:0000d45a c5                               push bc
                           .data:0000d45b 7c                               ld a,h
                           .data:0000d45c bd                               cp l
                           .data:0000d45d c4 c4 db                         call nz,0xdbc4
                           .data:0000d460 cc e9 ca                         call z,0xcae9
                           .data:0000d463 cd c8 db                         call 0xdbc8
                           .data:0000d466 2d                               dec l
                           .data:0000d467 20 01                            jr nz,0xd46a
                           .data:0000d469 6c                               ld l,h
                           .data:0000d46a c1                               pop bc
                           .data:0000d46b e3                               ex (sp),hl
                           .data:0000d46c 18 e1                            jr 0xd44f
                           .data:0000d46e e1                               pop hl
                           .data:0000d46f c3 71 d5                         jp 0xd571
                           .data:0000d472 c6 03                            add a,0x03
                           .data:0000d474 67                               ld h,a
                           .data:0000d475 d5                               push de
                           .data:0000d476 e5                               push hl
                           .data:0000d477 cd 69 bb                         call 0xbb69
                           .data:0000d47a 7a                               ld a,d
                           .data:0000d47b e1                               pop hl
                           .data:0000d47c d1                               pop de
                           .data:0000d47d c6 04                            add a,0x04
                           .data:0000d47f 2e 00                            ld l,0x00
                           .data:0000d481 2c                               inc l
                           .data:0000d482 94                               sub h
                           .data:0000d483 30 fc                            jr nc,0xd481
                           .data:0000d485 2d                               dec l
                           .data:0000d486 c0                               ret nz
                           .data:0000d487 2e 01                            ld l,0x01
                           .data:0000d489 c9                               ret
                           .data:0000d48a cd 73 cd                         call 0xcd73
                           .data:0000d48d cd c2 cd                         call 0xcdc2
                           .data:0000d490 cd c7 cd                         call 0xcdc7
                           .data:0000d493 cd 8d da                         call 0xda8d
                           .data:0000d496 cd 14 ce                         call 0xce14
                           .data:0000d499 cd 83 d6                         call 0xd683
                           .data:0000d49c cd 98 d6                         call 0xd698
                           .data:0000d49f 30 6b                            jr nc,0xd50c
                           .data:0000d4a1 cd b1 d4                         call 0xd4b1
                           .data:0000d4a4 cd 98 d6                         call 0xd698
                           .data:0000d4a7 38 f8                            jr c,0xd4a1
                           .data:0000d4a9 c9                               ret
                           .data:0000d4aa cd b1 d4                         call 0xd4b1
                           .data:0000d4ad d2 b8 cd                         jp nc,0xcdb8
                           .data:0000d4b0 c9                               ret
                           .data:0000d4b1 cd d9 d9                         call 0xd9d9
                           .data:0000d4b4 3f                               ccf
                           .data:0000d4b5 3e 0a                            ld a,0x0a
                           .data:0000d4b7 d2 ca db                         jp nc,0xdbca
                           .data:0000d4ba af                               xor a
                           .data:0000d4bb cd 3c d8                         call 0xd83c
                           .data:0000d4be 3e e5                            ld a,0xe5
                           .data:0000d4c0 12                               ld (de),a
                           .data:0000d4c1 c3 7a d9                         jp 0xd97a
                           .data:0000d4c4 cd 73 cd                         call 0xcd73
                           .data:0000d4c7 cd c1 cd                         call 0xcdc1
                           .data:0000d4ca cd c7 cd                         call 0xcdc7
                           .data:0000d4cd cd 5b da                         call 0xda5b
                           .data:0000d4d0 c5                               push bc
                           .data:0000d4d1 cd c7 cd                         call 0xcdc7
                           .data:0000d4d4 cd 60 da                         call 0xda60
                           .data:0000d4d7 e1                               pop hl
                           .data:0000d4d8 0a                               ld a,(bc)
                           .data:0000d4d9 be                               cp (hl)
                           .data:0000d4da c2 af cd                         jp nz,0xcdaf
                           .data:0000d4dd cd 14 ce                         call 0xce14
                           .data:0000d4e0 23                               inc hl
                           .data:0000d4e1 e5                               push hl
                           .data:0000d4e2 cd 44 d6                         call 0xd644
                           .data:0000d4e5 e1                               pop hl
                           .data:0000d4e6 c5                               push bc
                           .data:0000d4e7 44                               ld b,h
                           .data:0000d4e8 4d                               ld c,l
                           .data:0000d4e9 cd 83 d6                         call 0xd683
                           .data:0000d4ec cd 98 d6                         call 0xd698
                           .data:0000d4ef 30 1b                            jr nc,0xd50c
                           .data:0000d4f1 cd d9 d9                         call 0xd9d9
                           .data:0000d4f4 da 8d d3                         jp c,0xd38d
                           .data:0000d4f7 e3                               ex (sp),hl
                           .data:0000d4f8 e5                               push hl
                           .data:0000d4f9 c5                               push bc
                           .data:0000d4fa 01 0c 00                         ld bc,0x000c
                           .data:0000d4fd ed b0                            ldir
                           .data:0000d4ff c1                               pop bc
                           .data:0000d500 e1                               pop hl
                           .data:0000d501 e3                               ex (sp),hl
                           .data:0000d502 cd 7a d9                         call 0xd97a
                           .data:0000d505 cd 98 d6                         call 0xd698
                           .data:0000d508 38 e7                            jr c,0xd4f1
                           .data:0000d50a e1                               pop hl
                           .data:0000d50b c9                               ret
                           .data:0000d50c 50                               ld d,b
                           .data:0000d50d 59                               ld e,c
                           .data:0000d50e 3e 06                            ld a,0x06
                           .data:0000d510 c3 b1 cd                         jp 0xcdb1
                           .data:0000d513 cd 73 cd                         call 0xcd73
                           .data:0000d516 d5                               push de
                           .data:0000d517 dd e1                            pop ix
                           .data:0000d519 01 00 08                         ld bc,0x0800
                           .data:0000d51c cd af ca                         call 0xcaaf
                           .data:0000d51f cd 86 da                         call 0xda86
                           .data:0000d522 cd 14 ce                         call 0xce14
                           .data:0000d525 cd d0 db                         call 0xdbd0
                           .data:0000d528 af                               xor a
                           .data:0000d529 f5                               push af
                           .data:0000d52a cd 83 d6                         call 0xd683
                           .data:0000d52d cd 98 d6                         call 0xd698
                           .data:0000d530 30 0c                            jr nc,0xd53e
                           .data:0000d532 cd df d9                         call 0xd9df
                           .data:0000d535 38 f6                            jr c,0xd52d
                           .data:0000d537 e3                               ex (sp),hl
                           .data:0000d538 cd aa d5                         call 0xd5aa
                           .data:0000d53b e3                               ex (sp),hl
                           .data:0000d53c 38 ef                            jr c,0xd52d
                           .data:0000d53e 3e 11                            ld a,0x11
                           .data:0000d540 cd 72 d4                         call 0xd472
                           .data:0000d543 55                               ld d,l
                           .data:0000d544 f1                               pop af
                           .data:0000d545 1e 00                            ld e,0x00
                           .data:0000d547 1c                               inc e
                           .data:0000d548 92                               sub d
                           .data:0000d549 30 fc                            jr nc,0xd547
                           .data:0000d54b 82                               add a,d
                           .data:0000d54c 20 01                            jr nz,0xd54f
                           .data:0000d54e 1d                               dec e
                           .data:0000d54f dd e5                            push ix
                           .data:0000d551 e1                               pop hl
                           .data:0000d552 4b                               ld c,e
                           .data:0000d553 42                               ld b,d
                           .data:0000d554 e5                               push hl
                           .data:0000d555 cd 7a d5                         call 0xd57a
                           .data:0000d558 d5                               push de
                           .data:0000d559 eb                               ex de,hl
                           .data:0000d55a 26 00                            ld h,0x00
                           .data:0000d55c cd 3a d6                         call 0xd63a
                           .data:0000d55f 19                               add hl,de
                           .data:0000d560 d1                               pop de
                           .data:0000d561 10 f2                            djnz 0xd555
                           .data:0000d563 e1                               pop hl
                           .data:0000d564 d5                               push de
                           .data:0000d565 11 0e 00                         ld de,0x000e
                           .data:0000d568 19                               add hl,de
                           .data:0000d569 d1                               pop de
                           .data:0000d56a 0d                               dec c
                           .data:0000d56b 28 04                            jr z,0xd571
                           .data:0000d56d 7e                               ld a,(hl)
                           .data:0000d56e b7                               or a
                           .data:0000d56f 20 e2                            jr nz,0xd553
                           .data:0000d571 cd c2 d8                         call 0xd8c2
                           .data:0000d574 3e 03                            ld a,0x03
                           .data:0000d576 b7                               or a
                           .data:0000d577 c3 eb ca                         jp 0xcaeb
                           .data:0000d57a e7                               rst 0x20
                           .data:0000d57b b7                               or a
                           .data:0000d57c c8                               ret z
                           .data:0000d57d e5                               push hl
                           .data:0000d57e d5                               push de
                           .data:0000d57f c5                               push bc
                           .data:0000d580 78                               ld a,b
                           .data:0000d581 ba                               cp d
                           .data:0000d582 c4 c4 db                         call nz,0xdbc4
                           .data:0000d585 cc e9 ca                         call z,0xcae9
                           .data:0000d588 eb                               ex de,hl
                           .data:0000d589 cd c8 db                         call 0xdbc8
                           .data:0000d58c cd d9 d9                         call 0xd9d9
                           .data:0000d58f 3e 2a                            ld a,0x2a
                           .data:0000d591 38 02                            jr c,0xd595
                           .data:0000d593 3e 20                            ld a,0x20
                           .data:0000d595 cd 5a bb                         call 0xbb5a
                           .data:0000d598 21 0c 00                         ld hl,0x000c
                           .data:0000d59b 19                               add hl,de
                           .data:0000d59c e7                               rst 0x20
                           .data:0000d59d 5f                               ld e,a
                           .data:0000d59e 23                               inc hl
                           .data:0000d59f e7                               rst 0x20
                           .data:0000d5a0 57                               ld d,a
                           .data:0000d5a1 3e 02                            ld a,0x02
                           .data:0000d5a3 cd eb ca                         call 0xcaeb
                           .data:0000d5a6 c1                               pop bc
                           .data:0000d5a7 d1                               pop de
                           .data:0000d5a8 e1                               pop hl
                           .data:0000d5a9 c9                               ret
                           .data:0000d5aa c5                               push bc
                           .data:0000d5ab 4c                               ld c,h
                           .data:0000d5ac 06 00                            ld b,0x00
                           .data:0000d5ae dd e5                            push ix
                           .data:0000d5b0 e1                               pop hl
                           .data:0000d5b1 e7                               rst 0x20
                           .data:0000d5b2 b7                               or a
                           .data:0000d5b3 28 50                            jr z,0xd605
                           .data:0000d5b5 04                               inc b
                           .data:0000d5b6 cd 23 d6                         call 0xd623
                           .data:0000d5b9 28 0f                            jr z,0xd5ca
                           .data:0000d5bb 30 26                            jr nc,0xd5e3
                           .data:0000d5bd d5                               push de
                           .data:0000d5be 11 0e 00                         ld de,0x000e
                           .data:0000d5c1 19                               add hl,de
                           .data:0000d5c2 d1                               pop de
                           .data:0000d5c3 78                               ld a,b
                           .data:0000d5c4 fe 92                            cp 0x92
                           .data:0000d5c6 38 e9                            jr c,0xd5b1
                           .data:0000d5c8 18 56                            jr 0xd620
                           .data:0000d5ca e5                               push hl
                           .data:0000d5cb cd f2 d8                         call 0xd8f2
                           .data:0000d5ce e3                               ex (sp),hl
                           .data:0000d5cf 11 0c 00                         ld de,0x000c
                           .data:0000d5d2 19                               add hl,de
                           .data:0000d5d3 e7                               rst 0x20
                           .data:0000d5d4 5f                               ld e,a
                           .data:0000d5d5 23                               inc hl
                           .data:0000d5d6 e7                               rst 0x20
                           .data:0000d5d7 57                               ld d,a
                           .data:0000d5d8 2b                               dec hl
                           .data:0000d5d9 e3                               ex (sp),hl
                           .data:0000d5da 19                               add hl,de
                           .data:0000d5db eb                               ex de,hl
                           .data:0000d5dc e1                               pop hl
                           .data:0000d5dd 73                               ld (hl),e
                           .data:0000d5de 23                               inc hl
                           .data:0000d5df 72                               ld (hl),d
                           .data:0000d5e0 37                               scf
                           .data:0000d5e1 18 3d                            jr 0xd620
                           .data:0000d5e3 79                               ld a,c
                           .data:0000d5e4 fe 92                            cp 0x92
                           .data:0000d5e6 28 38                            jr z,0xd620
                           .data:0000d5e8 e5                               push hl
                           .data:0000d5e9 d5                               push de
                           .data:0000d5ea c5                               push bc
                           .data:0000d5eb eb                               ex de,hl
                           .data:0000d5ec 79                               ld a,c
                           .data:0000d5ed 90                               sub b
                           .data:0000d5ee 3c                               inc a
                           .data:0000d5ef 6f                               ld l,a
                           .data:0000d5f0 26 00                            ld h,0x00
                           .data:0000d5f2 cd 3a d6                         call 0xd63a
                           .data:0000d5f5 44                               ld b,h
                           .data:0000d5f6 4d                               ld c,l
                           .data:0000d5f7 19                               add hl,de
                           .data:0000d5f8 2b                               dec hl
                           .data:0000d5f9 eb                               ex de,hl
                           .data:0000d5fa 21 0e 00                         ld hl,0x000e
                           .data:0000d5fd 19                               add hl,de
                           .data:0000d5fe eb                               ex de,hl
                           .data:0000d5ff cd 1e b9                         call 0xb91e
                           .data:0000d602 c1                               pop bc
                           .data:0000d603 d1                               pop de
                           .data:0000d604 e1                               pop hl
                           .data:0000d605 0c                               inc c
                           .data:0000d606 c5                               push bc
                           .data:0000d607 d5                               push de
                           .data:0000d608 36 ff                            ld (hl),0xff
                           .data:0000d60a 23                               inc hl
                           .data:0000d60b 13                               inc de
                           .data:0000d60c eb                               ex de,hl
                           .data:0000d60d 01 0b 00                         ld bc,0x000b
                           .data:0000d610 cd 1b b9                         call 0xb91b
                           .data:0000d613 eb                               ex de,hl
                           .data:0000d614 e3                               ex (sp),hl
                           .data:0000d615 eb                               ex de,hl
                           .data:0000d616 cd f2 d8                         call 0xd8f2
                           .data:0000d619 eb                               ex de,hl
                           .data:0000d61a e1                               pop hl
                           .data:0000d61b 73                               ld (hl),e
                           .data:0000d61c 23                               inc hl
                           .data:0000d61d 72                               ld (hl),d
                           .data:0000d61e c1                               pop bc
                           .data:0000d61f 37                               scf
                           .data:0000d620 61                               ld h,c
                           .data:0000d621 c1                               pop bc
                           .data:0000d622 c9                               ret
                           .data:0000d623 e5                               push hl
                           .data:0000d624 d5                               push de
                           .data:0000d625 c5                               push bc
                           .data:0000d626 06 0b                            ld b,0x0b
                           .data:0000d628 13                               inc de
                           .data:0000d629 23                               inc hl
                           .data:0000d62a 1a                               ld a,(de)
                           .data:0000d62b e6 7f                            and 0x7f
                           .data:0000d62d 4f                               ld c,a
                           .data:0000d62e e7                               rst 0x20
                           .data:0000d62f e6 7f                            and 0x7f
                           .data:0000d631 b9                               cp c
                           .data:0000d632 20 02                            jr nz,0xd636
                           .data:0000d634 10 f2                            djnz 0xd628
                           .data:0000d636 c1                               pop bc
                           .data:0000d637 d1                               pop de
                           .data:0000d638 e1                               pop hl
                           .data:0000d639 c9                               ret
                           .data:0000d63a d5                               push de
                           .data:0000d63b 54                               ld d,h
                           .data:0000d63c 5d                               ld e,l
                           .data:0000d63d 29                               add hl,hl
                           .data:0000d63e 19                               add hl,de
                           .data:0000d63f 29                               add hl,hl
                           .data:0000d640 19                               add hl,de
                           .data:0000d641 29                               add hl,hl
                           .data:0000d642 d1                               pop de
                           .data:0000d643 c9                               ret
                           .data:0000d644 cd 83 d6                         call 0xd683
                           .data:0000d647 cd 98 d6                         call 0xd698
                           .data:0000d64a 30 25                            jr nc,0xd671
                           .data:0000d64c 3e 05                            ld a,0x05
                           .data:0000d64e c3 b1 cd                         jp 0xcdb1
                           .data:0000d651 cd 83 d6                         call 0xd683
                           .data:0000d654 cd 98 d6                         call 0xd698
                           .data:0000d657 30 18                            jr nc,0xd671
                           .data:0000d659 e5                               push hl
                           .data:0000d65a 21 09 00                         ld hl,0x0009
                           .data:0000d65d cd 9f ca                         call 0xca9f
                           .data:0000d660 eb                               ex de,hl
                           .data:0000d661 cd df db                         call 0xdbdf
                           .data:0000d664 e1                               pop hl
                           .data:0000d665 fd 7e 05                         ld a,(iy+5)
                           .data:0000d668 b7                               or a
                           .data:0000d669 37                               scf
                           .data:0000d66a c0                               ret nz
                           .data:0000d66b cd a2 d6                         call 0xd6a2
                           .data:0000d66e 38 fb                            jr c,0xd66b
                           .data:0000d670 37                               scf
                           .data:0000d671 fd 36 05 ff                      ld (iy+5),0xff
                           .data:0000d675 c9                               ret
                           .data:0000d676 cd 83 d6                         call 0xd683
                           .data:0000d679 cd 98 d6                         call 0xd698
                           .data:0000d67c 30 f3                            jr nc,0xd671
                           .data:0000d67e cd aa d4                         call 0xd4aa
                           .data:0000d681 18 f6                            jr 0xd679
                           .data:0000d683 c5                               push bc
                           .data:0000d684 cd 1f c5                         call 0xc51f
                           .data:0000d687 c1                               pop bc
                           .data:0000d688 21 ff ff                         ld hl,0xffff
                           .data:0000d68b fd 7e 05                         ld a,(iy+5)
                           .data:0000d68e b7                               or a
                           .data:0000d68f c0                               ret nz
                           .data:0000d690 e5                               push hl
                           .data:0000d691 cd 14 d8                         call 0xd814
                           .data:0000d694 e1                               pop hl
                           .data:0000d695 c3 a8 d9                         jp 0xd9a8
                           .data:0000d698 cd a2 d6                         call 0xd6a2
                           .data:0000d69b d0                               ret nc
                           .data:0000d69c cd d8 d7                         call 0xd7d8
                           .data:0000d69f 30 f7                            jr nc,0xd698
                           .data:0000d6a1 c9                               ret
                           .data:0000d6a2 23                               inc hl
                           .data:0000d6a3 fd 7e 05                         ld a,(iy+5)
                           .data:0000d6a6 b7                               or a
                           .data:0000d6a7 20 11                            jr nz,0xd6ba
                           .data:0000d6a9 cd 1c d9                         call 0xd91c
                           .data:0000d6ac d0                               ret nc
                           .data:0000d6ad 1a                               ld a,(de)
                           .data:0000d6ae fe e5                            cp 0xe5
                           .data:0000d6b0 37                               scf
                           .data:0000d6b1 c8                               ret z
                           .data:0000d6b2 cd a8 d9                         call 0xd9a8
                           .data:0000d6b5 3e ff                            ld a,0xff
                           .data:0000d6b7 c3 3c d8                         jp 0xd83c
                           .data:0000d6ba cd b8 d9                         call 0xd9b8
                           .data:0000d6bd d0                               ret nc
                           .data:0000d6be c3 1c d9                         jp 0xd91c
                           .data:0000d6c1 21 21 00                         ld hl,0x0021
                           .data:0000d6c4 19                               add hl,de
                           .data:0000d6c5 c3 f9 db                         jp 0xdbf9
                           .data:0000d6c8 21 23 00                         ld hl,0x0023
                           .data:0000d6cb 19                               add hl,de
                           .data:0000d6cc 7e                               ld a,(hl)
                           .data:0000d6cd b7                               or a
                           .data:0000d6ce c0                               ret nz
                           .data:0000d6cf cd c1 d6                         call 0xd6c1
                           .data:0000d6d2 7c                               ld a,h
                           .data:0000d6d3 1f                               rra
                           .data:0000d6d4 1f                               rra
                           .data:0000d6d5 1f                               rra
                           .data:0000d6d6 1f                               rra
                           .data:0000d6d7 e6 0f                            and 0x0f
                           .data:0000d6d9 47                               ld b,a
                           .data:0000d6da 29                               add hl,hl
                           .data:0000d6db 7c                               ld a,h
                           .data:0000d6dc e6 1f                            and 0x1f
                           .data:0000d6de 4f                               ld c,a
                           .data:0000d6df c5                               push bc
                           .data:0000d6e0 21 0f 00                         ld hl,0x000f
                           .data:0000d6e3 19                               add hl,de
                           .data:0000d6e4 7e                               ld a,(hl)
                           .data:0000d6e5 a8                               xor b
                           .data:0000d6e6 20 0f                            jr nz,0xd6f7
                           .data:0000d6e8 3e 04                            ld a,0x04
                           .data:0000d6ea cd 54 da                         call 0xda54
                           .data:0000d6ed 2f                               cpl
                           .data:0000d6ee 47                               ld b,a
                           .data:0000d6ef 2b                               dec hl
                           .data:0000d6f0 2b                               dec hl
                           .data:0000d6f1 7e                               ld a,(hl)
                           .data:0000d6f2 a9                               xor c
                           .data:0000d6f3 a0                               and b
                           .data:0000d6f4 20 01                            jr nz,0xd6f7
                           .data:0000d6f6 37                               scf
                           .data:0000d6f7 c1                               pop bc
                           .data:0000d6f8 9f                               sbc a,a
                           .data:0000d6f9 c9                               ret
                           .data:0000d6fa 21 0d 00                         ld hl,0x000d
                           .data:0000d6fd 19                               add hl,de
                           .data:0000d6fe 71                               ld (hl),c
                           .data:0000d6ff 23                               inc hl
                           .data:0000d700 23                               inc hl
                           .data:0000d701 70                               ld (hl),b
                           .data:0000d702 23                               inc hl
                           .data:0000d703 eb                               ex de,hl
                           .data:0000d704 01 11 00                         ld bc,0x0011
                           .data:0000d707 cd af ca                         call 0xcaaf
                           .data:0000d70a eb                               ex de,hl
                           .data:0000d70b c9                               ret
                           .data:0000d70c d5                               push de
                           .data:0000d70d cd c1 d6                         call 0xd6c1
                           .data:0000d710 7c                               ld a,h
                           .data:0000d711 e6 0f                            and 0x0f
                           .data:0000d713 67                               ld h,a
                           .data:0000d714 e5                               push hl
                           .data:0000d715 21 10 00                         ld hl,0x0010
                           .data:0000d718 19                               add hl,de
                           .data:0000d719 4e                               ld c,(hl)
                           .data:0000d71a 06 00                            ld b,0x00
                           .data:0000d71c 2b                               dec hl
                           .data:0000d71d 2b                               dec hl
                           .data:0000d71e 2b                               dec hl
                           .data:0000d71f 66                               ld h,(hl)
                           .data:0000d720 68                               ld l,b
                           .data:0000d721 3e 01                            ld a,0x01
                           .data:0000d723 cd eb db                         call 0xdbeb
                           .data:0000d726 09                               add hl,bc
                           .data:0000d727 d1                               pop de
                           .data:0000d728 13                               inc de
                           .data:0000d729 cd f3 db                         call 0xdbf3
                           .data:0000d72c 3f                               ccf
                           .data:0000d72d d1                               pop de
                           .data:0000d72e c9                               ret
                           .data:0000d72f cd c1 d6                         call 0xd6c1
                           .data:0000d732 3e 03                            ld a,0x03
                           .data:0000d734 cd 54 da                         call 0xda54
                           .data:0000d737 a5                               and l
                           .data:0000d738 4f                               ld c,a
                           .data:0000d739 3e 02                            ld a,0x02
                           .data:0000d73b cd 54 da                         call 0xda54
                           .data:0000d73e cd eb db                         call 0xdbeb
                           .data:0000d741 3e 06                            ld a,0x06
                           .data:0000d743 cd 54 da                         call 0xda54
                           .data:0000d746 47                               ld b,a
                           .data:0000d747 b7                               or a
                           .data:0000d748 7d                               ld a,l
                           .data:0000d749 21 11 00                         ld hl,0x0011
                           .data:0000d74c 19                               add hl,de
                           .data:0000d74d 28 0e                            jr z,0xd75d
                           .data:0000d74f e6 07                            and 0x07
                           .data:0000d751 87                               add a,a
                           .data:0000d752 85                               add a,l
                           .data:0000d753 6f                               ld l,a
                           .data:0000d754 8c                               adc a,h
                           .data:0000d755 95                               sub l
                           .data:0000d756 67                               ld h,a
                           .data:0000d757 e5                               push hl
                           .data:0000d758 cd f9 db                         call 0xdbf9
                           .data:0000d75b 18 0b                            jr 0xd768
                           .data:0000d75d e6 0f                            and 0x0f
                           .data:0000d75f 85                               add a,l
                           .data:0000d760 6f                               ld l,a
                           .data:0000d761 8c                               adc a,h
                           .data:0000d762 95                               sub l
                           .data:0000d763 67                               ld h,a
                           .data:0000d764 e5                               push hl
                           .data:0000d765 6e                               ld l,(hl)
                           .data:0000d766 26 00                            ld h,0x00
                           .data:0000d768 7c                               ld a,h
                           .data:0000d769 b5                               or l
                           .data:0000d76a 28 0f                            jr z,0xd77b
                           .data:0000d76c f1                               pop af
                           .data:0000d76d 3e 02                            ld a,0x02
                           .data:0000d76f cd 54 da                         call 0xda54
                           .data:0000d772 29                               add hl,hl
                           .data:0000d773 3d                               dec a
                           .data:0000d774 20 fc                            jr nz,0xd772
                           .data:0000d776 79                               ld a,c
                           .data:0000d777 b5                               or l
                           .data:0000d778 6f                               ld l,a
                           .data:0000d779 37                               scf
                           .data:0000d77a c9                               ret
                           .data:0000d77b e1                               pop hl
                           .data:0000d77c c9                               ret
                           .data:0000d77d 21 10 00                         ld hl,0x0010
                           .data:0000d780 19                               add hl,de
                           .data:0000d781 7e                               ld a,(hl)
                           .data:0000d782 34                               inc (hl)
                           .data:0000d783 b7                               or a
                           .data:0000d784 f0                               ret p
                           .data:0000d785 36 01                            ld (hl),0x01
                           .data:0000d787 2b                               dec hl
                           .data:0000d788 2b                               dec hl
                           .data:0000d789 2b                               dec hl
                           .data:0000d78a 34                               inc (hl)
                           .data:0000d78b c9                               ret
                           .data:0000d78c d5                               push de
                           .data:0000d78d d5                               push de
                           .data:0000d78e cd bb d7                         call 0xd7bb
                           .data:0000d791 e3                               ex (sp),hl
                           .data:0000d792 23                               inc hl
                           .data:0000d793 cd df db                         call 0xdbdf
                           .data:0000d796 e1                               pop hl
                           .data:0000d797 cd 7a d9                         call 0xd97a
                           .data:0000d79a d1                               pop de
                           .data:0000d79b c9                               ret
                           .data:0000d79c 21 21 00                         ld hl,0x0021
                           .data:0000d79f 19                               add hl,de
                           .data:0000d7a0 af                               xor a
                           .data:0000d7a1 77                               ld (hl),a
                           .data:0000d7a2 23                               inc hl
                           .data:0000d7a3 77                               ld (hl),a
                           .data:0000d7a4 23                               inc hl
                           .data:0000d7a5 77                               ld (hl),a
                           .data:0000d7a6 c9                               ret
                           .data:0000d7a7 21 21 00                         ld hl,0x0021
                           .data:0000d7aa 19                               add hl,de
                           .data:0000d7ab 34                               inc (hl)
                           .data:0000d7ac c0                               ret nz
                           .data:0000d7ad 23                               inc hl
                           .data:0000d7ae 34                               inc (hl)
                           .data:0000d7af c0                               ret nz
                           .data:0000d7b0 23                               inc hl
                           .data:0000d7b1 34                               inc (hl)
                           .data:0000d7b2 c9                               ret
                           .data:0000d7b3 cd 83 d6                         call 0xd683
                           .data:0000d7b6 cd 98 d6                         call 0xd698
                           .data:0000d7b9 18 11                            jr 0xd7cc
                           .data:0000d7bb 21 ff ff                         ld hl,0xffff
                           .data:0000d7be 23                               inc hl
                           .data:0000d7bf cd 1c d9                         call 0xd91c
                           .data:0000d7c2 3e 07                            ld a,0x07
                           .data:0000d7c4 d2 b1 cd                         jp nc,0xcdb1
                           .data:0000d7c7 1a                               ld a,(de)
                           .data:0000d7c8 fe e5                            cp 0xe5
                           .data:0000d7ca 20 f2                            jr nz,0xd7be
                           .data:0000d7cc f5                               push af
                           .data:0000d7cd fd 7e 05                         ld a,(iy+5)
                           .data:0000d7d0 b7                               or a
                           .data:0000d7d1 3e 09                            ld a,0x09
                           .data:0000d7d3 ca b8 cd                         jp z,0xcdb8
                           .data:0000d7d6 f1                               pop af
                           .data:0000d7d7 c9                               ret
                           .data:0000d7d8 c5                               push bc
                           .data:0000d7d9 d5                               push de
                           .data:0000d7da e5                               push hl
                           .data:0000d7db 60                               ld h,b
                           .data:0000d7dc 69                               ld l,c
                           .data:0000d7dd 1a                               ld a,(de)
                           .data:0000d7de ae                               xor (hl)
                           .data:0000d7df 20 2d                            jr nz,0xd80e
                           .data:0000d7e1 23                               inc hl
                           .data:0000d7e2 13                               inc de
                           .data:0000d7e3 06 0b                            ld b,0x0b
                           .data:0000d7e5 7e                               ld a,(hl)
                           .data:0000d7e6 fe 3f                            cp 0x3f
                           .data:0000d7e8 28 06                            jr z,0xd7f0
                           .data:0000d7ea 1a                               ld a,(de)
                           .data:0000d7eb ae                               xor (hl)
                           .data:0000d7ec e6 7f                            and 0x7f
                           .data:0000d7ee 20 1e                            jr nz,0xd80e
                           .data:0000d7f0 23                               inc hl
                           .data:0000d7f1 13                               inc de
                           .data:0000d7f2 10 f1                            djnz 0xd7e5
                           .data:0000d7f4 7e                               ld a,(hl)
                           .data:0000d7f5 3c                               inc a
                           .data:0000d7f6 28 0c                            jr z,0xd804
                           .data:0000d7f8 3e 04                            ld a,0x04
                           .data:0000d7fa cd 54 da                         call 0xda54
                           .data:0000d7fd 2f                               cpl
                           .data:0000d7fe 47                               ld b,a
                           .data:0000d7ff 1a                               ld a,(de)
                           .data:0000d800 ae                               xor (hl)
                           .data:0000d801 a0                               and b
                           .data:0000d802 20 0a                            jr nz,0xd80e
                           .data:0000d804 23                               inc hl
                           .data:0000d805 13                               inc de
                           .data:0000d806 23                               inc hl
                           .data:0000d807 13                               inc de
                           .data:0000d808 7e                               ld a,(hl)
                           .data:0000d809 3c                               inc a
                           .data:0000d80a 28 02                            jr z,0xd80e
                           .data:0000d80c 1a                               ld a,(de)
                           .data:0000d80d ae                               xor (hl)
                           .data:0000d80e e1                               pop hl
                           .data:0000d80f d1                               pop de
                           .data:0000d810 c1                               pop bc
                           .data:0000d811 c0                               ret nz
                           .data:0000d812 37                               scf
                           .data:0000d813 c9                               ret
                           .data:0000d814 3e 05                            ld a,0x05
                           .data:0000d816 cd 45 da                         call 0xda45
                           .data:0000d819 3e 03                            ld a,0x03
                           .data:0000d81b cd eb db                         call 0xdbeb
                           .data:0000d81e 23                               inc hl
                           .data:0000d81f eb                               ex de,hl
                           .data:0000d820 3e 0e                            ld a,0x0e
                           .data:0000d822 cd 3f da                         call 0xda3f
                           .data:0000d825 36 00                            ld (hl),0x00
                           .data:0000d827 23                               inc hl
                           .data:0000d828 1b                               dec de
                           .data:0000d829 7a                               ld a,d
                           .data:0000d82a b3                               or e
                           .data:0000d82b 20 f8                            jr nz,0xd825
                           .data:0000d82d 3e 09                            ld a,0x09
                           .data:0000d82f cd 45 da                         call 0xda45
                           .data:0000d832 eb                               ex de,hl
                           .data:0000d833 3e 0e                            ld a,0x0e
                           .data:0000d835 cd 3f da                         call 0xda3f
                           .data:0000d838 73                               ld (hl),e
                           .data:0000d839 23                               inc hl
                           .data:0000d83a 72                               ld (hl),d
                           .data:0000d83b c9                               ret
                           .data:0000d83c e5                               push hl
                           .data:0000d83d d5                               push de
                           .data:0000d83e c5                               push bc
                           .data:0000d83f 4f                               ld c,a
                           .data:0000d840 21 10 00                         ld hl,0x0010
                           .data:0000d843 19                               add hl,de
                           .data:0000d844 06 10                            ld b,0x10
                           .data:0000d846 5e                               ld e,(hl)
                           .data:0000d847 23                               inc hl
                           .data:0000d848 3e 06                            ld a,0x06
                           .data:0000d84a cd 54 da                         call 0xda54
                           .data:0000d84d b7                               or a
                           .data:0000d84e 28 03                            jr z,0xd853
                           .data:0000d850 05                               dec b
                           .data:0000d851 7e                               ld a,(hl)
                           .data:0000d852 23                               inc hl
                           .data:0000d853 57                               ld d,a
                           .data:0000d854 b3                               or e
                           .data:0000d855 28 0e                            jr z,0xd865
                           .data:0000d857 e5                               push hl
                           .data:0000d858 3e 05                            ld a,0x05
                           .data:0000d85a cd 45 da                         call 0xda45
                           .data:0000d85d 7d                               ld a,l
                           .data:0000d85e 93                               sub e
                           .data:0000d85f 7c                               ld a,h
                           .data:0000d860 9a                               sbc a,d
                           .data:0000d861 d4 6c d8                         call nc,0xd86c
                           .data:0000d864 e1                               pop hl
                           .data:0000d865 10 df                            djnz 0xd846
                           .data:0000d867 c1                               pop bc
                           .data:0000d868 d1                               pop de
                           .data:0000d869 e1                               pop hl
                           .data:0000d86a 37                               scf
                           .data:0000d86b c9                               ret
                           .data:0000d86c c5                               push bc
                           .data:0000d86d d5                               push de
                           .data:0000d86e d5                               push de
                           .data:0000d86f eb                               ex de,hl
                           .data:0000d870 3e 03                            ld a,0x03
                           .data:0000d872 cd eb db                         call 0xdbeb
                           .data:0000d875 eb                               ex de,hl
                           .data:0000d876 3e 0e                            ld a,0x0e
                           .data:0000d878 cd 3f da                         call 0xda3f
                           .data:0000d87b 19                               add hl,de
                           .data:0000d87c d1                               pop de
                           .data:0000d87d 7b                               ld a,e
                           .data:0000d87e e6 07                            and 0x07
                           .data:0000d880 5f                               ld e,a
                           .data:0000d881 3e 01                            ld a,0x01
                           .data:0000d883 1c                               inc e
                           .data:0000d884 0f                               rrca
                           .data:0000d885 1d                               dec e
                           .data:0000d886 20 fc                            jr nz,0xd884
                           .data:0000d888 47                               ld b,a
                           .data:0000d889 a1                               and c
                           .data:0000d88a 4f                               ld c,a
                           .data:0000d88b 78                               ld a,b
                           .data:0000d88c 2f                               cpl
                           .data:0000d88d a6                               and (hl)
                           .data:0000d88e b1                               or c
                           .data:0000d88f 77                               ld (hl),a
                           .data:0000d890 d1                               pop de
                           .data:0000d891 c1                               pop bc
                           .data:0000d892 c9                               ret
                           .data:0000d893 c5                               push bc
                           .data:0000d894 d5                               push de
                           .data:0000d895 3e 05                            ld a,0x05
                           .data:0000d897 cd 45 da                         call 0xda45
                           .data:0000d89a eb                               ex de,hl
                           .data:0000d89b 3e 0e                            ld a,0x0e
                           .data:0000d89d cd 3f da                         call 0xda3f
                           .data:0000d8a0 01 80 08                         ld bc,0x0880
                           .data:0000d8a3 7e                               ld a,(hl)
                           .data:0000d8a4 a1                               and c
                           .data:0000d8a5 28 0c                            jr z,0xd8b3
                           .data:0000d8a7 0f                               rrca
                           .data:0000d8a8 4f                               ld c,a
                           .data:0000d8a9 7a                               ld a,d
                           .data:0000d8aa b3                               or e
                           .data:0000d8ab 28 12                            jr z,0xd8bf
                           .data:0000d8ad 1b                               dec de
                           .data:0000d8ae 10 f3                            djnz 0xd8a3
                           .data:0000d8b0 23                               inc hl
                           .data:0000d8b1 18 ed                            jr 0xd8a0
                           .data:0000d8b3 7e                               ld a,(hl)
                           .data:0000d8b4 b1                               or c
                           .data:0000d8b5 77                               ld (hl),a
                           .data:0000d8b6 3e 05                            ld a,0x05
                           .data:0000d8b8 cd 45 da                         call 0xda45
                           .data:0000d8bb b7                               or a
                           .data:0000d8bc ed 52                            sbc hl,de
                           .data:0000d8be 37                               scf
                           .data:0000d8bf d1                               pop de
                           .data:0000d8c0 c1                               pop bc
                           .data:0000d8c1 c9                               ret
                           .data:0000d8c2 c5                               push bc
                           .data:0000d8c3 e5                               push hl
                           .data:0000d8c4 21 00 00                         ld hl,0x0000
                           .data:0000d8c7 e5                               push hl
                           .data:0000d8c8 3e 05                            ld a,0x05
                           .data:0000d8ca cd 45 da                         call 0xda45
                           .data:0000d8cd eb                               ex de,hl
                           .data:0000d8ce 3e 0e                            ld a,0x0e
                           .data:0000d8d0 cd 3f da                         call 0xda3f
                           .data:0000d8d3 01 80 08                         ld bc,0x0880
                           .data:0000d8d6 7e                               ld a,(hl)
                           .data:0000d8d7 a1                               and c
                           .data:0000d8d8 20 03                            jr nz,0xd8dd
                           .data:0000d8da e3                               ex (sp),hl
                           .data:0000d8db 23                               inc hl
                           .data:0000d8dc e3                               ex (sp),hl
                           .data:0000d8dd 79                               ld a,c
                           .data:0000d8de 0f                               rrca
                           .data:0000d8df 4f                               ld c,a
                           .data:0000d8e0 7a                               ld a,d
                           .data:0000d8e1 b3                               or e
                           .data:0000d8e2 28 06                            jr z,0xd8ea
                           .data:0000d8e4 1b                               dec de
                           .data:0000d8e5 10 ef                            djnz 0xd8d6
                           .data:0000d8e7 23                               inc hl
                           .data:0000d8e8 18 e9                            jr 0xd8d3
                           .data:0000d8ea e1                               pop hl
                           .data:0000d8eb cd 10 d9                         call 0xd910
                           .data:0000d8ee eb                               ex de,hl
                           .data:0000d8ef e1                               pop hl
                           .data:0000d8f0 c1                               pop bc
                           .data:0000d8f1 c9                               ret
                           .data:0000d8f2 d5                               push de
                           .data:0000d8f3 21 10 00                         ld hl,0x0010
                           .data:0000d8f6 19                               add hl,de
                           .data:0000d8f7 11 00 10                         ld de,0x1000
                           .data:0000d8fa 3e 06                            ld a,0x06
                           .data:0000d8fc cd 54 da                         call 0xda54
                           .data:0000d8ff b7                               or a
                           .data:0000d900 7e                               ld a,(hl)
                           .data:0000d901 23                               inc hl
                           .data:0000d902 28 03                            jr z,0xd907
                           .data:0000d904 b6                               or (hl)
                           .data:0000d905 15                               dec d
                           .data:0000d906 23                               inc hl
                           .data:0000d907 b7                               or a
                           .data:0000d908 28 01                            jr z,0xd90b
                           .data:0000d90a 1c                               inc e
                           .data:0000d90b 15                               dec d
                           .data:0000d90c 20 ec                            jr nz,0xd8fa
                           .data:0000d90e eb                               ex de,hl
                           .data:0000d90f d1                               pop de
                           .data:0000d910 3e 02                            ld a,0x02
                           .data:0000d912 cd 54 da                         call 0xda54
                           .data:0000d915 3d                               dec a
                           .data:0000d916 3d                               dec a
                           .data:0000d917 3d                               dec a
                           .data:0000d918 c8                               ret z
                           .data:0000d919 29                               add hl,hl
                           .data:0000d91a 18 fb                            jr 0xd917
                           .data:0000d91c e5                               push hl
                           .data:0000d91d c5                               push bc
                           .data:0000d91e 7d                               ld a,l
                           .data:0000d91f e6 03                            and 0x03
                           .data:0000d921 20 11                            jr nz,0xd934
                           .data:0000d923 eb                               ex de,hl
                           .data:0000d924 3e 07                            ld a,0x07
                           .data:0000d926 cd 45 da                         call 0xda45
                           .data:0000d929 cd f3 db                         call 0xdbf3
                           .data:0000d92c 3f                               ccf
                           .data:0000d92d eb                               ex de,hl
                           .data:0000d92e 30 15                            jr nc,0xd945
                           .data:0000d930 cd 48 d9                         call 0xd948
                           .data:0000d933 af                               xor a
                           .data:0000d934 47                               ld b,a
                           .data:0000d935 3e 08                            ld a,0x08
                           .data:0000d937 cd 3f da                         call 0xda3f
                           .data:0000d93a 11 20 00                         ld de,0x0020
                           .data:0000d93d 04                               inc b
                           .data:0000d93e 18 01                            jr 0xd941
                           .data:0000d940 19                               add hl,de
                           .data:0000d941 10 fd                            djnz 0xd940
                           .data:0000d943 eb                               ex de,hl
                           .data:0000d944 37                               scf
                           .data:0000d945 c1                               pop bc
                           .data:0000d946 e1                               pop hl
                           .data:0000d947 c9                               ret
                           .data:0000d948 3e 02                            ld a,0x02
                           .data:0000d94a cd eb db                         call 0xdbeb
                           .data:0000d94d eb                               ex de,hl
                           .data:0000d94e 3e 08                            ld a,0x08
                           .data:0000d950 cd 3f da                         call 0xda3f
                           .data:0000d953 cd e8 d9                         call 0xd9e8
                           .data:0000d956 3e 0b                            ld a,0x0b
                           .data:0000d958 cd 45 da                         call 0xda45
                           .data:0000d95b eb                               ex de,hl
                           .data:0000d95c cd f3 db                         call 0xdbf3
                           .data:0000d95f eb                               ex de,hl
                           .data:0000d960 d0                               ret nc
                           .data:0000d961 3e 0c                            ld a,0x0c
                           .data:0000d963 cd 3f da                         call 0xda3f
                           .data:0000d966 19                               add hl,de
                           .data:0000d967 cd c8 d9                         call 0xd9c8
                           .data:0000d96a be                               cp (hl)
                           .data:0000d96b c8                               ret z
                           .data:0000d96c f5                               push af
                           .data:0000d96d eb                               ex de,hl
                           .data:0000d96e 29                               add hl,hl
                           .data:0000d96f 29                               add hl,hl
                           .data:0000d970 cd b8 d9                         call 0xd9b8
                           .data:0000d973 eb                               ex de,hl
                           .data:0000d974 d1                               pop de
                           .data:0000d975 da 33 d2                         jp c,0xd233
                           .data:0000d978 72                               ld (hl),d
                           .data:0000d979 c9                               ret
                           .data:0000d97a e5                               push hl
                           .data:0000d97b c5                               push bc
                           .data:0000d97c 3e 02                            ld a,0x02
                           .data:0000d97e cd eb db                         call 0xdbeb
                           .data:0000d981 eb                               ex de,hl
                           .data:0000d982 3e 08                            ld a,0x08
                           .data:0000d984 cd 3f da                         call 0xda3f
                           .data:0000d987 0e 01                            ld c,0x01
                           .data:0000d989 cd f3 d9                         call 0xd9f3
                           .data:0000d98c 3e 0b                            ld a,0x0b
                           .data:0000d98e cd 45 da                         call 0xda45
                           .data:0000d991 eb                               ex de,hl
                           .data:0000d992 cd f3 db                         call 0xdbf3
                           .data:0000d995 eb                               ex de,hl
                           .data:0000d996 30 0a                            jr nc,0xd9a2
                           .data:0000d998 3e 0c                            ld a,0x0c
                           .data:0000d99a cd 3f da                         call 0xda3f
                           .data:0000d99d 19                               add hl,de
                           .data:0000d99e cd c8 d9                         call 0xd9c8
                           .data:0000d9a1 77                               ld (hl),a
                           .data:0000d9a2 c1                               pop bc
                           .data:0000d9a3 e1                               pop hl
                           .data:0000d9a4 cd b8 d9                         call 0xd9b8
                           .data:0000d9a7 d8                               ret c
                           .data:0000d9a8 d5                               push de
                           .data:0000d9a9 e5                               push hl
                           .data:0000d9aa eb                               ex de,hl
                           .data:0000d9ab 13                               inc de
                           .data:0000d9ac 3e 02                            ld a,0x02
                           .data:0000d9ae cd 35 da                         call 0xda35
                           .data:0000d9b1 73                               ld (hl),e
                           .data:0000d9b2 23                               inc hl
                           .data:0000d9b3 72                               ld (hl),d
                           .data:0000d9b4 e1                               pop hl
                           .data:0000d9b5 d1                               pop de
                           .data:0000d9b6 37                               scf
                           .data:0000d9b7 c9                               ret
                           .data:0000d9b8 d5                               push de
                           .data:0000d9b9 e5                               push hl
                           .data:0000d9ba 3e 02                            ld a,0x02
                           .data:0000d9bc cd 35 da                         call 0xda35
                           .data:0000d9bf 5e                               ld e,(hl)
                           .data:0000d9c0 23                               inc hl
                           .data:0000d9c1 56                               ld d,(hl)
                           .data:0000d9c2 e1                               pop hl
                           .data:0000d9c3 cd f3 db                         call 0xdbf3
                           .data:0000d9c6 d1                               pop de
                           .data:0000d9c7 c9                               ret
                           .data:0000d9c8 c5                               push bc
                           .data:0000d9c9 e5                               push hl
                           .data:0000d9ca 06 80                            ld b,0x80
                           .data:0000d9cc 3e 08                            ld a,0x08
                           .data:0000d9ce cd 3f da                         call 0xda3f
                           .data:0000d9d1 af                               xor a
                           .data:0000d9d2 86                               add a,(hl)
                           .data:0000d9d3 23                               inc hl
                           .data:0000d9d4 10 fc                            djnz 0xd9d2
                           .data:0000d9d6 e1                               pop hl
                           .data:0000d9d7 c1                               pop bc
                           .data:0000d9d8 c9                               ret
                           .data:0000d9d9 e5                               push hl
                           .data:0000d9da 21 09 00                         ld hl,0x0009
                           .data:0000d9dd 18 04                            jr 0xd9e3
                           .data:0000d9df e5                               push hl
                           .data:0000d9e0 21 0a 00                         ld hl,0x000a
                           .data:0000d9e3 19                               add hl,de
                           .data:0000d9e4 7e                               ld a,(hl)
                           .data:0000d9e5 87                               add a,a
                           .data:0000d9e6 e1                               pop hl
                           .data:0000d9e7 c9                               ret
                           .data:0000d9e8 c5                               push bc
                           .data:0000d9e9 d5                               push de
                           .data:0000d9ea e5                               push hl
                           .data:0000d9eb cd 06 da                         call 0xda06
                           .data:0000d9ee cd 4c c5                         call 0xc54c
                           .data:0000d9f1 18 0b                            jr 0xd9fe
                           .data:0000d9f3 c5                               push bc
                           .data:0000d9f4 d5                               push de
                           .data:0000d9f5 e5                               push hl
                           .data:0000d9f6 c5                               push bc
                           .data:0000d9f7 cd 06 da                         call 0xda06
                           .data:0000d9fa c1                               pop bc
                           .data:0000d9fb cd 2e c5                         call 0xc52e
                           .data:0000d9fe b7                               or a
                           .data:0000d9ff c2 b6 cd                         jp nz,0xcdb6
                           .data:0000da02 e1                               pop hl
                           .data:0000da03 d1                               pop de
                           .data:0000da04 c1                               pop bc
                           .data:0000da05 c9                               ret
                           .data:0000da06 d5                               push de
                           .data:0000da07 44                               ld b,h
                           .data:0000da08 4d                               ld c,l
                           .data:0000da09 cd 1a c5                         call 0xc51a
                           .data:0000da0c d1                               pop de
                           .data:0000da0d 3e 0d                            ld a,0x0d
                           .data:0000da0f cd 45 da                         call 0xda45
                           .data:0000da12 44                               ld b,h
                           .data:0000da13 4d                               ld c,l
                           .data:0000da14 af                               xor a
                           .data:0000da15 cd 45 da                         call 0xda45
                           .data:0000da18 0b                               dec bc
                           .data:0000da19 03                               inc bc
                           .data:0000da1a 7b                               ld a,e
                           .data:0000da1b 95                               sub l
                           .data:0000da1c 5f                               ld e,a
                           .data:0000da1d 7a                               ld a,d
                           .data:0000da1e 9c                               sbc a,h
                           .data:0000da1f 57                               ld d,a
                           .data:0000da20 30 f7                            jr nc,0xda19
                           .data:0000da22 19                               add hl,de
                           .data:0000da23 e5                               push hl
                           .data:0000da24 cd 24 c5                         call 0xc524
                           .data:0000da27 c1                               pop bc
                           .data:0000da28 af                               xor a
                           .data:0000da29 cd 3f da                         call 0xda3f
                           .data:0000da2c eb                               ex de,hl
                           .data:0000da2d cd 5a c5                         call 0xc55a
                           .data:0000da30 4d                               ld c,l
                           .data:0000da31 44                               ld b,h
                           .data:0000da32 c3 29 c5                         jp 0xc529
                           .data:0000da35 fd 86 03                         add a,(iy+3)
                           .data:0000da38 6f                               ld l,a
                           .data:0000da39 fd 8e 04                         adc a,(iy+4)
                           .data:0000da3c 95                               sub l
                           .data:0000da3d 67                               ld h,a
                           .data:0000da3e c9                               ret
                           .data:0000da3f cd 35 da                         call 0xda35
                           .data:0000da42 c3 f9 db                         jp 0xdbf9
                           .data:0000da45 f5                               push af
                           .data:0000da46 3e 0a                            ld a,0x0a
                           .data:0000da48 cd 3f da                         call 0xda3f
                           .data:0000da4b f1                               pop af
                           .data:0000da4c 85                               add a,l
                           .data:0000da4d 6f                               ld l,a
                           .data:0000da4e 8c                               adc a,h
                           .data:0000da4f 95                               sub l
                           .data:0000da50 67                               ld h,a
                           .data:0000da51 c3 f9 db                         jp 0xdbf9
                           .data:0000da54 e5                               push hl
                           .data:0000da55 cd 45 da                         call 0xda45
                           .data:0000da58 7d                               ld a,l
                           .data:0000da59 e1                               pop hl
                           .data:0000da5a c9                               ret
                           .data:0000da5b 11 e4 00                         ld de,0x00e4
                           .data:0000da5e 18 03                            jr 0xda63
                           .data:0000da60 11 f4 00                         ld de,0x00f4
                           .data:0000da63 0e 20                            ld c,0x20
                           .data:0000da65 cd 74 da                         call 0xda74
                           .data:0000da68 18 2b                            jr 0xda95
                           .data:0000da6a cd 6f da                         call 0xda6f
                           .data:0000da6d 18 26                            jr 0xda95
                           .data:0000da6f 0e ff                            ld c,0xff
                           .data:0000da71 11 e4 00                         ld de,0x00e4
                           .data:0000da74 cd a0 da                         call 0xdaa0
                           .data:0000da77 c5                               push bc
                           .data:0000da78 16 0b                            ld d,0x0b
                           .data:0000da7a 03                               inc bc
                           .data:0000da7b 03                               inc bc
                           .data:0000da7c 0a                               ld a,(bc)
                           .data:0000da7d fe 3f                            cp 0x3f
                           .data:0000da7f 28 69                            jr z,0xdaea
                           .data:0000da81 15                               dec d
                           .data:0000da82 20 f7                            jr nz,0xda7b
                           .data:0000da84 c1                               pop bc
                           .data:0000da85 c9                               ret
                           .data:0000da86 06 00                            ld b,0x00
                           .data:0000da88 cd a6 da                         call 0xdaa6
                           .data:0000da8b 18 08                            jr 0xda95
                           .data:0000da8d 0e 20                            ld c,0x20
                           .data:0000da8f 11 e4 00                         ld de,0x00e4
                           .data:0000da92 cd a0 da                         call 0xdaa0
                           .data:0000da95 21 0d 00                         ld hl,0x000d
                           .data:0000da98 09                               add hl,bc
                           .data:0000da99 36 ff                            ld (hl),0xff
                           .data:0000da9b 23                               inc hl
                           .data:0000da9c 23                               inc hl
                           .data:0000da9d 36 ff                            ld (hl),0xff
                           .data:0000da9f c9                               ret
                           .data:0000daa0 cd b6 da                         call 0xdab6
                           .data:0000daa3 28 45                            jr z,0xdaea
                           .data:0000daa5 c9                               ret
                           .data:0000daa6 0e 20                            ld c,0x20
                           .data:0000daa8 11 e4 00                         ld de,0x00e4
                           .data:0000daab cd b6 da                         call 0xdab6
                           .data:0000daae c5                               push bc
                           .data:0000daaf 0e 0b                            ld c,0x0b
                           .data:0000dab1 cc 8e db                         call z,0xdb8e
                           .data:0000dab4 c1                               pop bc
                           .data:0000dab5 c9                               ret
                           .data:0000dab6 e5                               push hl
                           .data:0000dab7 cd 98 ca                         call 0xca98
                           .data:0000daba d5                               push de
                           .data:0000dabb fd 7e 00                         ld a,(iy+0)
                           .data:0000dabe 12                               ld (de),a
                           .data:0000dabf 13                               inc de
                           .data:0000dac0 fd 7e 01                         ld a,(iy+1)
                           .data:0000dac3 12                               ld (de),a
                           .data:0000dac4 13                               inc de
                           .data:0000dac5 c5                               push bc
                           .data:0000dac6 41                               ld b,c
                           .data:0000dac7 0e 08                            ld c,0x08
                           .data:0000dac9 cd 85 db                         call 0xdb85
                           .data:0000dacc 78                               ld a,b
                           .data:0000dacd 0e 03                            ld c,0x03
                           .data:0000dacf cd 90 db                         call 0xdb90
                           .data:0000dad2 01 03 00                         ld bc,0x0003
                           .data:0000dad5 cd af ca                         call 0xcaaf
                           .data:0000dad8 c1                               pop bc
                           .data:0000dad9 d1                               pop de
                           .data:0000dada e1                               pop hl
                           .data:0000dadb d5                               push de
                           .data:0000dadc cd ed da                         call 0xdaed
                           .data:0000dadf d1                               pop de
                           .data:0000dae0 30 08                            jr nc,0xdaea
                           .data:0000dae2 42                               ld b,d
                           .data:0000dae3 4b                               ld c,e
                           .data:0000dae4 13                               inc de
                           .data:0000dae5 13                               inc de
                           .data:0000dae6 1a                               ld a,(de)
                           .data:0000dae7 fe 20                            cp 0x20
                           .data:0000dae9 c9                               ret
                           .data:0000daea c3 af cd                         jp 0xcdaf
                           .data:0000daed 2b                               dec hl
                           .data:0000daee cd 97 db                         call 0xdb97
                           .data:0000daf1 3f                               ccf
                           .data:0000daf2 d8                               ret c
                           .data:0000daf3 4f                               ld c,a
                           .data:0000daf4 e5                               push hl
                           .data:0000daf5 c5                               push bc
                           .data:0000daf6 fe 3a                            cp 0x3a
                           .data:0000daf8 28 06                            jr z,0xdb00
                           .data:0000dafa cd a5 db                         call 0xdba5
                           .data:0000dafd 38 f7                            jr c,0xdaf6
                           .data:0000daff 37                               scf
                           .data:0000db00 c1                               pop bc
                           .data:0000db01 e1                               pop hl
                           .data:0000db02 79                               ld a,c
                           .data:0000db03 38 3e                            jr c,0xdb43
                           .data:0000db05 13                               inc de
                           .data:0000db06 fe 30                            cp 0x30
                           .data:0000db08 38 1f                            jr c,0xdb29
                           .data:0000db0a fe 3a                            cp 0x3a
                           .data:0000db0c 30 1b                            jr nc,0xdb29
                           .data:0000db0e d6 30                            sub 0x30
                           .data:0000db10 4f                               ld c,a
                           .data:0000db11 12                               ld (de),a
                           .data:0000db12 cd a5 db                         call 0xdba5
                           .data:0000db15 fe 30                            cp 0x30
                           .data:0000db17 38 10                            jr c,0xdb29
                           .data:0000db19 fe 3a                            cp 0x3a
                           .data:0000db1b 30 0c                            jr nc,0xdb29
                           .data:0000db1d b7                               or a
                           .data:0000db1e 0d                               dec c
                           .data:0000db1f c0                               ret nz
                           .data:0000db20 c6 da                            add a,0xda
                           .data:0000db22 fe 10                            cp 0x10
                           .data:0000db24 d0                               ret nc
                           .data:0000db25 12                               ld (de),a
                           .data:0000db26 cd a5 db                         call 0xdba5
                           .data:0000db29 1b                               dec de
                           .data:0000db2a fe 51                            cp 0x51
                           .data:0000db2c 30 0a                            jr nc,0xdb38
                           .data:0000db2e fe 41                            cp 0x41
                           .data:0000db30 38 06                            jr c,0xdb38
                           .data:0000db32 d6 41                            sub 0x41
                           .data:0000db34 12                               ld (de),a
                           .data:0000db35 cd a5 db                         call 0xdba5
                           .data:0000db38 cd 9b db                         call 0xdb9b
                           .data:0000db3b ee 3a                            xor 0x3a
                           .data:0000db3d c0                               ret nz
                           .data:0000db3e cd 97 db                         call 0xdb97
                           .data:0000db41 3f                               ccf
                           .data:0000db42 d8                               ret c
                           .data:0000db43 13                               inc de
                           .data:0000db44 13                               inc de
                           .data:0000db45 fe 2e                            cp 0x2e
                           .data:0000db47 c8                               ret z
                           .data:0000db48 0e 08                            ld c,0x08
                           .data:0000db4a cd 58 db                         call 0xdb58
                           .data:0000db4d d8                               ret c
                           .data:0000db4e ee 2e                            xor 0x2e
                           .data:0000db50 c0                               ret nz
                           .data:0000db51 cd 97 db                         call 0xdb97
                           .data:0000db54 0e 03                            ld c,0x03
                           .data:0000db56 30 2d                            jr nc,0xdb85
                           .data:0000db58 fe 20                            cp 0x20
                           .data:0000db5a 38 29                            jr c,0xdb85
                           .data:0000db5c e5                               push hl
                           .data:0000db5d c5                               push bc
                           .data:0000db5e 47                               ld b,a
                           .data:0000db5f 21 b2 db                         ld hl,0xdbb2
                           .data:0000db62 7e                               ld a,(hl)
                           .data:0000db63 23                               inc hl
                           .data:0000db64 b7                               or a
                           .data:0000db65 28 04                            jr z,0xdb6b
                           .data:0000db67 b8                               cp b
                           .data:0000db68 20 f8                            jr nz,0xdb62
                           .data:0000db6a 37                               scf
                           .data:0000db6b 78                               ld a,b
                           .data:0000db6c c1                               pop bc
                           .data:0000db6d e1                               pop hl
                           .data:0000db6e 38 15                            jr c,0xdb85
                           .data:0000db70 0d                               dec c
                           .data:0000db71 f8                               ret m
                           .data:0000db72 fe 2a                            cp 0x2a
                           .data:0000db74 cc 8e db                         call z,0xdb8e
                           .data:0000db77 12                               ld (de),a
                           .data:0000db78 13                               inc de
                           .data:0000db79 cd a5 db                         call 0xdba5
                           .data:0000db7c 30 07                            jr nc,0xdb85
                           .data:0000db7e fe 20                            cp 0x20
                           .data:0000db80 20 d6                            jr nz,0xdb58
                           .data:0000db82 cd 9b db                         call 0xdb9b
                           .data:0000db85 f5                               push af
                           .data:0000db86 3e 20                            ld a,0x20
                           .data:0000db88 cd 90 db                         call 0xdb90
                           .data:0000db8b f1                               pop af
                           .data:0000db8c 3f                               ccf
                           .data:0000db8d c9                               ret
                           .data:0000db8e 3e 3f                            ld a,0x3f
                           .data:0000db90 0c                               inc c
                           .data:0000db91 0d                               dec c
                           .data:0000db92 c8                               ret z
                           .data:0000db93 12                               ld (de),a
                           .data:0000db94 13                               inc de
                           .data:0000db95 18 fa                            jr 0xdb91
                           .data:0000db97 cd a5 db                         call 0xdba5
                           .data:0000db9a d0                               ret nc
                           .data:0000db9b fe 20                            cp 0x20
                           .data:0000db9d 37                               scf
                           .data:0000db9e c0                               ret nz
                           .data:0000db9f cd a5 db                         call 0xdba5
                           .data:0000dba2 38 f7                            jr c,0xdb9b
                           .data:0000dba4 c9                               ret
                           .data:0000dba5 78                               ld a,b
                           .data:0000dba6 b7                               or a
                           .data:0000dba7 c8                               ret z
                           .data:0000dba8 23                               inc hl
                           .data:0000dba9 05                               dec b
                           .data:0000dbaa e7                               rst 0x20
                           .data:0000dbab e6 7f                            and 0x7f
                           .data:0000dbad cd a6 ca                         call 0xcaa6
                           .data:0000dbb0 37                               scf
                           .data:0000dbb1 c9                               ret
                           .data:0000dbb2 3c                               inc a
                           .data:0000dbb3 3e 2e                            ld a,0x2e
                           .data:0000dbb5 2c                               inc l
                           .data:0000dbb6 3b                               dec sp
                           .data:0000dbb7 3a 3d 5b                         ld a,(0x5b3d)
                           .data:0000dbba 5d                               ld e,l
                           .data:0000dbbb 5f                               ld e,a
                           .data:0000dbbc 25                               dec h
                           .data:0000dbbd 7c                               ld a,h
                           .data:0000dbbe 28 29                            jr z,0xdbe9
                           .data:0000dbc0 2f                               cpl
                           .data:0000dbc1 5c                               ld e,h
                           .data:0000dbc2 7f                               ld a,a
                           .data:0000dbc3 00                               nop
                           .data:0000dbc4 3e 01                            ld a,0x01
                           .data:0000dbc6 18 02                            jr 0xdbca
                           .data:0000dbc8 3e 0b                            ld a,0x0b
                           .data:0000dbca c5                               push bc
                           .data:0000dbcb fd 4e 02                         ld c,(iy+2)
                           .data:0000dbce 18 0a                            jr 0xdbda
                           .data:0000dbd0 c5                               push bc
                           .data:0000dbd1 0a                               ld a,(bc)
                           .data:0000dbd2 5f                               ld e,a
                           .data:0000dbd3 16 00                            ld d,0x00
                           .data:0000dbd5 0b                               dec bc
                           .data:0000dbd6 0a                               ld a,(bc)
                           .data:0000dbd7 4f                               ld c,a
                           .data:0000dbd8 3e 0c                            ld a,0x0c
                           .data:0000dbda cd eb ca                         call 0xcaeb
                           .data:0000dbdd c1                               pop bc
                           .data:0000dbde c9                               ret
                           .data:0000dbdf e5                               push hl
                           .data:0000dbe0 d5                               push de
                           .data:0000dbe1 c5                               push bc
                           .data:0000dbe2 01 20 00                         ld bc,0x0020
                           .data:0000dbe5 ed b0                            ldir
                           .data:0000dbe7 c1                               pop bc
                           .data:0000dbe8 d1                               pop de
                           .data:0000dbe9 e1                               pop hl
                           .data:0000dbea c9                               ret
                           .data:0000dbeb cb 3c                            srl h
                           .data:0000dbed cb 1d                            rr l
                           .data:0000dbef 3d                               dec a
                           .data:0000dbf0 20 f9                            jr nz,0xdbeb
                           .data:0000dbf2 c9                               ret
                           .data:0000dbf3 e5                               push hl
                           .data:0000dbf4 b7                               or a
                           .data:0000dbf5 ed 52                            sbc hl,de
                           .data:0000dbf7 e1                               pop hl
                           .data:0000dbf8 c9                               ret
                           .data:0000dbf9 d5                               push de
                           .data:0000dbfa 5e                               ld e,(hl)
                           .data:0000dbfb 23                               inc hl
                           .data:0000dbfc 56                               ld d,(hl)
                           .data:0000dbfd eb                               ex de,hl
                           .data:0000dbfe d1                               pop de
                           .data:0000dbff c9                               ret
                           .data:0000dc00 ff                               rst 0x38
                           .data:0000dc01 ff                               rst 0x38
                           .data:0000dc02 ff                               rst 0x38
                           .data:0000dc03 ff                               rst 0x38
                           .data:0000dc04 ff                               rst 0x38
                           .data:0000dc05 ff                               rst 0x38
                           .data:0000dc06 ff                               rst 0x38
                           .data:0000dc07 ff                               rst 0x38
                           .data:0000dc08 ff                               rst 0x38
                           .data:0000dc09 ff                               rst 0x38
                           .data:0000dc0a ff                               rst 0x38
                           .data:0000dc0b ff                               rst 0x38
                           .data:0000dc0c ff                               rst 0x38
                           .data:0000dc0d ff                               rst 0x38
                           .data:0000dc0e ff                               rst 0x38
                           .data:0000dc0f ff                               rst 0x38
                           .data:0000dc10 ff                               rst 0x38
                           .data:0000dc11 ff                               rst 0x38
                           .data:0000dc12 ff                               rst 0x38
                           .data:0000dc13 ff                               rst 0x38
                           .data:0000dc14 ff                               rst 0x38
                           .data:0000dc15 ff                               rst 0x38
                           .data:0000dc16 ff                               rst 0x38
                           .data:0000dc17 ff                               rst 0x38
                           .data:0000dc18 ff                               rst 0x38
                           .data:0000dc19 ff                               rst 0x38
                           .data:0000dc1a ff                               rst 0x38
                           .data:0000dc1b ff                               rst 0x38
                           .data:0000dc1c ff                               rst 0x38
                           .data:0000dc1d ff                               rst 0x38
                           .data:0000dc1e ff                               rst 0x38
                           .data:0000dc1f ff                               rst 0x38
                           .data:0000dc20 ff                               rst 0x38
                           .data:0000dc21 ff                               rst 0x38
                           .data:0000dc22 ff                               rst 0x38
                           .data:0000dc23 ff                               rst 0x38
                           .data:0000dc24 ff                               rst 0x38
                           .data:0000dc25 ff                               rst 0x38
                           .data:0000dc26 ff                               rst 0x38
                           .data:0000dc27 ff                               rst 0x38
                           .data:0000dc28 ff                               rst 0x38
                           .data:0000dc29 ff                               rst 0x38
                           .data:0000dc2a ff                               rst 0x38
                           .data:0000dc2b ff                               rst 0x38
                           .data:0000dc2c ff                               rst 0x38
                           .data:0000dc2d ff                               rst 0x38
                           .data:0000dc2e ff                               rst 0x38
                           .data:0000dc2f ff                               rst 0x38
                           .data:0000dc30 ff                               rst 0x38
                           .data:0000dc31 ff                               rst 0x38
                           .data:0000dc32 ff                               rst 0x38
                           .data:0000dc33 ff                               rst 0x38
                           .data:0000dc34 ff                               rst 0x38
                           .data:0000dc35 ff                               rst 0x38
                           .data:0000dc36 ff                               rst 0x38
                           .data:0000dc37 ff                               rst 0x38
                           .data:0000dc38 ff                               rst 0x38
                           .data:0000dc39 ff                               rst 0x38
                           .data:0000dc3a ff                               rst 0x38
                           .data:0000dc3b ff                               rst 0x38
                           .data:0000dc3c ff                               rst 0x38
                           .data:0000dc3d ff                               rst 0x38
                           .data:0000dc3e ff                               rst 0x38
                           .data:0000dc3f ff                               rst 0x38
                           .data:0000dc40 ff                               rst 0x38
                           .data:0000dc41 ff                               rst 0x38
                           .data:0000dc42 ff                               rst 0x38
                           .data:0000dc43 ff                               rst 0x38
                           .data:0000dc44 ff                               rst 0x38
                           .data:0000dc45 ff                               rst 0x38
                           .data:0000dc46 ff                               rst 0x38
                           .data:0000dc47 ff                               rst 0x38
                           .data:0000dc48 ff                               rst 0x38
                           .data:0000dc49 ff                               rst 0x38
                           .data:0000dc4a ff                               rst 0x38
                           .data:0000dc4b ff                               rst 0x38
                           .data:0000dc4c ff                               rst 0x38
                           .data:0000dc4d ff                               rst 0x38
                           .data:0000dc4e ff                               rst 0x38
                           .data:0000dc4f ff                               rst 0x38
                           .data:0000dc50 ff                               rst 0x38
                           .data:0000dc51 ff                               rst 0x38
                           .data:0000dc52 ff                               rst 0x38
                           .data:0000dc53 ff                               rst 0x38
                           .data:0000dc54 ff                               rst 0x38
                           .data:0000dc55 ff                               rst 0x38
                           .data:0000dc56 ff                               rst 0x38
                           .data:0000dc57 ff                               rst 0x38
                           .data:0000dc58 ff                               rst 0x38
                           .data:0000dc59 ff                               rst 0x38
                           .data:0000dc5a ff                               rst 0x38
                           .data:0000dc5b ff                               rst 0x38
                           .data:0000dc5c ff                               rst 0x38
                           .data:0000dc5d ff                               rst 0x38
                           .data:0000dc5e ff                               rst 0x38
                           .data:0000dc5f ff                               rst 0x38
                           .data:0000dc60 ff                               rst 0x38
                           .data:0000dc61 ff                               rst 0x38
                           .data:0000dc62 ff                               rst 0x38
                           .data:0000dc63 ff                               rst 0x38
                           .data:0000dc64 ff                               rst 0x38
                           .data:0000dc65 ff                               rst 0x38
                           .data:0000dc66 ff                               rst 0x38
                           .data:0000dc67 ff                               rst 0x38
                           .data:0000dc68 ff                               rst 0x38
                           .data:0000dc69 ff                               rst 0x38
                           .data:0000dc6a ff                               rst 0x38
                           .data:0000dc6b ff                               rst 0x38
                           .data:0000dc6c ff                               rst 0x38
                           .data:0000dc6d ff                               rst 0x38
                           .data:0000dc6e ff                               rst 0x38
                           .data:0000dc6f ff                               rst 0x38
                           .data:0000dc70 ff                               rst 0x38
                           .data:0000dc71 ff                               rst 0x38
                           .data:0000dc72 ff                               rst 0x38
                           .data:0000dc73 ff                               rst 0x38
                           .data:0000dc74 ff                               rst 0x38
                           .data:0000dc75 ff                               rst 0x38
                           .data:0000dc76 ff                               rst 0x38
                           .data:0000dc77 ff                               rst 0x38
                           .data:0000dc78 ff                               rst 0x38
                           .data:0000dc79 ff                               rst 0x38
                           .data:0000dc7a ff                               rst 0x38
                           .data:0000dc7b ff                               rst 0x38
                           .data:0000dc7c ff                               rst 0x38
                           .data:0000dc7d ff                               rst 0x38
                           .data:0000dc7e ff                               rst 0x38
                           .data:0000dc7f ff                               rst 0x38
                           .data:0000dc80 ff                               rst 0x38
                           .data:0000dc81 ff                               rst 0x38
                           .data:0000dc82 ff                               rst 0x38
                           .data:0000dc83 ff                               rst 0x38
                           .data:0000dc84 ff                               rst 0x38
                           .data:0000dc85 ff                               rst 0x38
                           .data:0000dc86 ff                               rst 0x38
                           .data:0000dc87 ff                               rst 0x38
                           .data:0000dc88 ff                               rst 0x38
                           .data:0000dc89 ff                               rst 0x38
                           .data:0000dc8a ff                               rst 0x38
                           .data:0000dc8b ff                               rst 0x38
                           .data:0000dc8c ff                               rst 0x38
                           .data:0000dc8d ff                               rst 0x38
                           .data:0000dc8e ff                               rst 0x38
                           .data:0000dc8f ff                               rst 0x38
                           .data:0000dc90 ff                               rst 0x38
                           .data:0000dc91 ff                               rst 0x38
                           .data:0000dc92 ff                               rst 0x38
                           .data:0000dc93 ff                               rst 0x38
                           .data:0000dc94 ff                               rst 0x38
                           .data:0000dc95 ff                               rst 0x38
                           .data:0000dc96 ff                               rst 0x38
                           .data:0000dc97 ff                               rst 0x38
                           .data:0000dc98 ff                               rst 0x38
                           .data:0000dc99 ff                               rst 0x38
                           .data:0000dc9a ff                               rst 0x38
                           .data:0000dc9b ff                               rst 0x38
                           .data:0000dc9c ff                               rst 0x38
                           .data:0000dc9d ff                               rst 0x38
                           .data:0000dc9e ff                               rst 0x38
                           .data:0000dc9f ff                               rst 0x38
                           .data:0000dca0 ff                               rst 0x38
                           .data:0000dca1 ff                               rst 0x38
                           .data:0000dca2 ff                               rst 0x38
                           .data:0000dca3 ff                               rst 0x38
                           .data:0000dca4 ff                               rst 0x38
                           .data:0000dca5 ff                               rst 0x38
                           .data:0000dca6 ff                               rst 0x38
                           .data:0000dca7 ff                               rst 0x38
                           .data:0000dca8 ff                               rst 0x38
                           .data:0000dca9 ff                               rst 0x38
                           .data:0000dcaa ff                               rst 0x38
                           .data:0000dcab ff                               rst 0x38
                           .data:0000dcac ff                               rst 0x38
                           .data:0000dcad ff                               rst 0x38
                           .data:0000dcae ff                               rst 0x38
                           .data:0000dcaf ff                               rst 0x38
                           .data:0000dcb0 ff                               rst 0x38
                           .data:0000dcb1 ff                               rst 0x38
                           .data:0000dcb2 ff                               rst 0x38
                           .data:0000dcb3 ff                               rst 0x38
                           .data:0000dcb4 ff                               rst 0x38
                           .data:0000dcb5 ff                               rst 0x38
                           .data:0000dcb6 ff                               rst 0x38
                           .data:0000dcb7 ff                               rst 0x38
                           .data:0000dcb8 ff                               rst 0x38
                           .data:0000dcb9 ff                               rst 0x38
                           .data:0000dcba ff                               rst 0x38
                           .data:0000dcbb ff                               rst 0x38
                           .data:0000dcbc ff                               rst 0x38
                           .data:0000dcbd ff                               rst 0x38
                           .data:0000dcbe ff                               rst 0x38
                           .data:0000dcbf ff                               rst 0x38
                           .data:0000dcc0 ff                               rst 0x38
                           .data:0000dcc1 ff                               rst 0x38
                           .data:0000dcc2 ff                               rst 0x38
                           .data:0000dcc3 ff                               rst 0x38
                           .data:0000dcc4 ff                               rst 0x38
                           .data:0000dcc5 ff                               rst 0x38
                           .data:0000dcc6 ff                               rst 0x38
                           .data:0000dcc7 ff                               rst 0x38
                           .data:0000dcc8 ff                               rst 0x38
                           .data:0000dcc9 ff                               rst 0x38
                           .data:0000dcca ff                               rst 0x38
                           .data:0000dccb ff                               rst 0x38
                           .data:0000dccc ff                               rst 0x38
                           .data:0000dccd ff                               rst 0x38
                           .data:0000dcce ff                               rst 0x38
                           .data:0000dccf ff                               rst 0x38
                           .data:0000dcd0 ff                               rst 0x38
                           .data:0000dcd1 ff                               rst 0x38
                           .data:0000dcd2 ff                               rst 0x38
                           .data:0000dcd3 ff                               rst 0x38
                           .data:0000dcd4 ff                               rst 0x38
                           .data:0000dcd5 ff                               rst 0x38
                           .data:0000dcd6 ff                               rst 0x38
                           .data:0000dcd7 ff                               rst 0x38
                           .data:0000dcd8 ff                               rst 0x38
                           .data:0000dcd9 ff                               rst 0x38
                           .data:0000dcda ff                               rst 0x38
                           .data:0000dcdb ff                               rst 0x38
                           .data:0000dcdc ff                               rst 0x38
                           .data:0000dcdd ff                               rst 0x38
                           .data:0000dcde ff                               rst 0x38
                           .data:0000dcdf ff                               rst 0x38
                           .data:0000dce0 ff                               rst 0x38
                           .data:0000dce1 ff                               rst 0x38
                           .data:0000dce2 ff                               rst 0x38
                           .data:0000dce3 ff                               rst 0x38
                           .data:0000dce4 ff                               rst 0x38
                           .data:0000dce5 ff                               rst 0x38
                           .data:0000dce6 ff                               rst 0x38
                           .data:0000dce7 ff                               rst 0x38
                           .data:0000dce8 ff                               rst 0x38
                           .data:0000dce9 ff                               rst 0x38
                           .data:0000dcea ff                               rst 0x38
                           .data:0000dceb ff                               rst 0x38
                           .data:0000dcec ff                               rst 0x38
                           .data:0000dced ff                               rst 0x38
                           .data:0000dcee ff                               rst 0x38
                           .data:0000dcef ff                               rst 0x38
                           .data:0000dcf0 ff                               rst 0x38
                           .data:0000dcf1 ff                               rst 0x38
                           .data:0000dcf2 ff                               rst 0x38
                           .data:0000dcf3 ff                               rst 0x38
                           .data:0000dcf4 ff                               rst 0x38
                           .data:0000dcf5 ff                               rst 0x38
                           .data:0000dcf6 ff                               rst 0x38
                           .data:0000dcf7 ff                               rst 0x38
                           .data:0000dcf8 ff                               rst 0x38
                           .data:0000dcf9 ff                               rst 0x38
                           .data:0000dcfa ff                               rst 0x38
                           .data:0000dcfb ff                               rst 0x38
                           .data:0000dcfc ff                               rst 0x38
                           .data:0000dcfd ff                               rst 0x38
                           .data:0000dcfe ff                               rst 0x38
                           .data:0000dcff ff                               rst 0x38
                           .data:0000dd00 ff                               rst 0x38
                           .data:0000dd01 ff                               rst 0x38
                           .data:0000dd02 ff                               rst 0x38
                           .data:0000dd03 ff                               rst 0x38
                           .data:0000dd04 ff                               rst 0x38
                           .data:0000dd05 ff                               rst 0x38
                           .data:0000dd06 ff                               rst 0x38
                           .data:0000dd07 ff                               rst 0x38
                           .data:0000dd08 ff                               rst 0x38
                           .data:0000dd09 ff                               rst 0x38
                           .data:0000dd0a ff                               rst 0x38
                           .data:0000dd0b ff                               rst 0x38
                           .data:0000dd0c ff                               rst 0x38
                           .data:0000dd0d ff                               rst 0x38
                           .data:0000dd0e ff                               rst 0x38
                           .data:0000dd0f ff                               rst 0x38
                           .data:0000dd10 ff                               rst 0x38
                           .data:0000dd11 ff                               rst 0x38
                           .data:0000dd12 ff                               rst 0x38
                           .data:0000dd13 ff                               rst 0x38
                           .data:0000dd14 ff                               rst 0x38
                           .data:0000dd15 ff                               rst 0x38
                           .data:0000dd16 ff                               rst 0x38
                           .data:0000dd17 ff                               rst 0x38
                           .data:0000dd18 ff                               rst 0x38
                           .data:0000dd19 ff                               rst 0x38
                           .data:0000dd1a ff                               rst 0x38
                           .data:0000dd1b ff                               rst 0x38
                           .data:0000dd1c ff                               rst 0x38
                           .data:0000dd1d ff                               rst 0x38
                           .data:0000dd1e ff                               rst 0x38
                           .data:0000dd1f ff                               rst 0x38
                           .data:0000dd20 ff                               rst 0x38
                           .data:0000dd21 ff                               rst 0x38
                           .data:0000dd22 ff                               rst 0x38
                           .data:0000dd23 ff                               rst 0x38
                           .data:0000dd24 ff                               rst 0x38
                           .data:0000dd25 ff                               rst 0x38
                           .data:0000dd26 ff                               rst 0x38
                           .data:0000dd27 ff                               rst 0x38
                           .data:0000dd28 ff                               rst 0x38
                           .data:0000dd29 ff                               rst 0x38
                           .data:0000dd2a ff                               rst 0x38
                           .data:0000dd2b ff                               rst 0x38
                           .data:0000dd2c ff                               rst 0x38
                           .data:0000dd2d ff                               rst 0x38
                           .data:0000dd2e ff                               rst 0x38
                           .data:0000dd2f ff                               rst 0x38
                           .data:0000dd30 ff                               rst 0x38
                           .data:0000dd31 ff                               rst 0x38
                           .data:0000dd32 ff                               rst 0x38
                           .data:0000dd33 ff                               rst 0x38
                           .data:0000dd34 ff                               rst 0x38
                           .data:0000dd35 ff                               rst 0x38
                           .data:0000dd36 ff                               rst 0x38
                           .data:0000dd37 ff                               rst 0x38
                           .data:0000dd38 ff                               rst 0x38
                           .data:0000dd39 ff                               rst 0x38
                           .data:0000dd3a ff                               rst 0x38
                           .data:0000dd3b ff                               rst 0x38
                           .data:0000dd3c ff                               rst 0x38
                           .data:0000dd3d ff                               rst 0x38
                           .data:0000dd3e ff                               rst 0x38
                           .data:0000dd3f ff                               rst 0x38
                           .data:0000dd40 ff                               rst 0x38
                           .data:0000dd41 ff                               rst 0x38
                           .data:0000dd42 ff                               rst 0x38
                           .data:0000dd43 ff                               rst 0x38
                           .data:0000dd44 ff                               rst 0x38
                           .data:0000dd45 ff                               rst 0x38
                           .data:0000dd46 ff                               rst 0x38
                           .data:0000dd47 ff                               rst 0x38
                           .data:0000dd48 ff                               rst 0x38
                           .data:0000dd49 ff                               rst 0x38
                           .data:0000dd4a ff                               rst 0x38
                           .data:0000dd4b ff                               rst 0x38
                           .data:0000dd4c ff                               rst 0x38
                           .data:0000dd4d ff                               rst 0x38
                           .data:0000dd4e ff                               rst 0x38
                           .data:0000dd4f ff                               rst 0x38
                           .data:0000dd50 ff                               rst 0x38
                           .data:0000dd51 ff                               rst 0x38
                           .data:0000dd52 ff                               rst 0x38
                           .data:0000dd53 ff                               rst 0x38
                           .data:0000dd54 ff                               rst 0x38
                           .data:0000dd55 ff                               rst 0x38
                           .data:0000dd56 ff                               rst 0x38
                           .data:0000dd57 ff                               rst 0x38
                           .data:0000dd58 ff                               rst 0x38
                           .data:0000dd59 ff                               rst 0x38
                           .data:0000dd5a ff                               rst 0x38
                           .data:0000dd5b ff                               rst 0x38
                           .data:0000dd5c ff                               rst 0x38
                           .data:0000dd5d ff                               rst 0x38
                           .data:0000dd5e ff                               rst 0x38
                           .data:0000dd5f ff                               rst 0x38
                           .data:0000dd60 ff                               rst 0x38
                           .data:0000dd61 ff                               rst 0x38
                           .data:0000dd62 ff                               rst 0x38
                           .data:0000dd63 ff                               rst 0x38
                           .data:0000dd64 ff                               rst 0x38
                           .data:0000dd65 ff                               rst 0x38
                           .data:0000dd66 ff                               rst 0x38
                           .data:0000dd67 ff                               rst 0x38
                           .data:0000dd68 ff                               rst 0x38
                           .data:0000dd69 ff                               rst 0x38
                           .data:0000dd6a ff                               rst 0x38
                           .data:0000dd6b ff                               rst 0x38
                           .data:0000dd6c ff                               rst 0x38
                           .data:0000dd6d ff                               rst 0x38
                           .data:0000dd6e ff                               rst 0x38
                           .data:0000dd6f ff                               rst 0x38
                           .data:0000dd70 ff                               rst 0x38
                           .data:0000dd71 ff                               rst 0x38
                           .data:0000dd72 ff                               rst 0x38
                           .data:0000dd73 ff                               rst 0x38
                           .data:0000dd74 ff                               rst 0x38
                           .data:0000dd75 ff                               rst 0x38
                           .data:0000dd76 ff                               rst 0x38
                           .data:0000dd77 ff                               rst 0x38
                           .data:0000dd78 ff                               rst 0x38
                           .data:0000dd79 ff                               rst 0x38
                           .data:0000dd7a ff                               rst 0x38
                           .data:0000dd7b ff                               rst 0x38
                           .data:0000dd7c ff                               rst 0x38
                           .data:0000dd7d ff                               rst 0x38
                           .data:0000dd7e ff                               rst 0x38
                           .data:0000dd7f ff                               rst 0x38
                           .data:0000dd80 ff                               rst 0x38
                           .data:0000dd81 ff                               rst 0x38
                           .data:0000dd82 ff                               rst 0x38
                           .data:0000dd83 ff                               rst 0x38
                           .data:0000dd84 ff                               rst 0x38
                           .data:0000dd85 ff                               rst 0x38
                           .data:0000dd86 ff                               rst 0x38
                           .data:0000dd87 ff                               rst 0x38
                           .data:0000dd88 ff                               rst 0x38
                           .data:0000dd89 ff                               rst 0x38
                           .data:0000dd8a ff                               rst 0x38
                           .data:0000dd8b ff                               rst 0x38
                           .data:0000dd8c ff                               rst 0x38
                           .data:0000dd8d ff                               rst 0x38
                           .data:0000dd8e ff                               rst 0x38
                           .data:0000dd8f ff                               rst 0x38
                           .data:0000dd90 ff                               rst 0x38
                           .data:0000dd91 ff                               rst 0x38
                           .data:0000dd92 ff                               rst 0x38
                           .data:0000dd93 ff                               rst 0x38
                           .data:0000dd94 ff                               rst 0x38
                           .data:0000dd95 ff                               rst 0x38
                           .data:0000dd96 ff                               rst 0x38
                           .data:0000dd97 ff                               rst 0x38
                           .data:0000dd98 ff                               rst 0x38
                           .data:0000dd99 ff                               rst 0x38
                           .data:0000dd9a ff                               rst 0x38
                           .data:0000dd9b ff                               rst 0x38
                           .data:0000dd9c ff                               rst 0x38
                           .data:0000dd9d ff                               rst 0x38
                           .data:0000dd9e ff                               rst 0x38
                           .data:0000dd9f ff                               rst 0x38
                           .data:0000dda0 ff                               rst 0x38
                           .data:0000dda1 ff                               rst 0x38
                           .data:0000dda2 ff                               rst 0x38
                           .data:0000dda3 ff                               rst 0x38
                           .data:0000dda4 ff                               rst 0x38
                           .data:0000dda5 ff                               rst 0x38
                           .data:0000dda6 ff                               rst 0x38
                           .data:0000dda7 ff                               rst 0x38
                           .data:0000dda8 ff                               rst 0x38
                           .data:0000dda9 ff                               rst 0x38
                           .data:0000ddaa ff                               rst 0x38
                           .data:0000ddab ff                               rst 0x38
                           .data:0000ddac ff                               rst 0x38
                           .data:0000ddad ff                               rst 0x38
                           .data:0000ddae ff                               rst 0x38
                           .data:0000ddaf ff                               rst 0x38
                           .data:0000ddb0 ff                               rst 0x38
                           .data:0000ddb1 ff                               rst 0x38
                           .data:0000ddb2 ff                               rst 0x38
                           .data:0000ddb3 ff                               rst 0x38
                           .data:0000ddb4 ff                               rst 0x38
                           .data:0000ddb5 ff                               rst 0x38
                           .data:0000ddb6 ff                               rst 0x38
                           .data:0000ddb7 ff                               rst 0x38
                           .data:0000ddb8 ff                               rst 0x38
                           .data:0000ddb9 ff                               rst 0x38
                           .data:0000ddba ff                               rst 0x38
                           .data:0000ddbb ff                               rst 0x38
                           .data:0000ddbc ff                               rst 0x38
                           .data:0000ddbd ff                               rst 0x38
                           .data:0000ddbe ff                               rst 0x38
                           .data:0000ddbf ff                               rst 0x38
                           .data:0000ddc0 ff                               rst 0x38
                           .data:0000ddc1 ff                               rst 0x38
                           .data:0000ddc2 ff                               rst 0x38
                           .data:0000ddc3 ff                               rst 0x38
                           .data:0000ddc4 ff                               rst 0x38
                           .data:0000ddc5 ff                               rst 0x38
                           .data:0000ddc6 ff                               rst 0x38
                           .data:0000ddc7 ff                               rst 0x38
                           .data:0000ddc8 ff                               rst 0x38
                           .data:0000ddc9 ff                               rst 0x38
                           .data:0000ddca ff                               rst 0x38
                           .data:0000ddcb ff                               rst 0x38
                           .data:0000ddcc ff                               rst 0x38
                           .data:0000ddcd ff                               rst 0x38
                           .data:0000ddce ff                               rst 0x38
                           .data:0000ddcf ff                               rst 0x38
                           .data:0000ddd0 ff                               rst 0x38
                           .data:0000ddd1 ff                               rst 0x38
                           .data:0000ddd2 ff                               rst 0x38
                           .data:0000ddd3 ff                               rst 0x38
                           .data:0000ddd4 ff                               rst 0x38
                           .data:0000ddd5 ff                               rst 0x38
                           .data:0000ddd6 ff                               rst 0x38
                           .data:0000ddd7 ff                               rst 0x38
                           .data:0000ddd8 ff                               rst 0x38
                           .data:0000ddd9 ff                               rst 0x38
                           .data:0000ddda ff                               rst 0x38
                           .data:0000dddb ff                               rst 0x38
                           .data:0000dddc ff                               rst 0x38
                           .data:0000dddd ff                               rst 0x38
                           .data:0000ddde ff                               rst 0x38
                           .data:0000dddf ff                               rst 0x38
                           .data:0000dde0 ff                               rst 0x38
                           .data:0000dde1 ff                               rst 0x38
                           .data:0000dde2 ff                               rst 0x38
                           .data:0000dde3 ff                               rst 0x38
                           .data:0000dde4 ff                               rst 0x38
                           .data:0000dde5 ff                               rst 0x38
                           .data:0000dde6 ff                               rst 0x38
                           .data:0000dde7 ff                               rst 0x38
                           .data:0000dde8 ff                               rst 0x38
                           .data:0000dde9 ff                               rst 0x38
                           .data:0000ddea ff                               rst 0x38
                           .data:0000ddeb ff                               rst 0x38
                           .data:0000ddec ff                               rst 0x38
                           .data:0000dded ff                               rst 0x38
                           .data:0000ddee ff                               rst 0x38
                           .data:0000ddef ff                               rst 0x38
                           .data:0000ddf0 ff                               rst 0x38
                           .data:0000ddf1 ff                               rst 0x38
                           .data:0000ddf2 ff                               rst 0x38
                           .data:0000ddf3 ff                               rst 0x38
                           .data:0000ddf4 ff                               rst 0x38
                           .data:0000ddf5 ff                               rst 0x38
                           .data:0000ddf6 ff                               rst 0x38
                           .data:0000ddf7 ff                               rst 0x38
                           .data:0000ddf8 ff                               rst 0x38
                           .data:0000ddf9 ff                               rst 0x38
                           .data:0000ddfa ff                               rst 0x38
                           .data:0000ddfb ff                               rst 0x38
                           .data:0000ddfc ff                               rst 0x38
                           .data:0000ddfd ff                               rst 0x38
                           .data:0000ddfe ff                               rst 0x38
                           .data:0000ddff ff                               rst 0x38
                           .data:0000de00 ff                               rst 0x38
                           .data:0000de01 ff                               rst 0x38
                           .data:0000de02 ff                               rst 0x38
                           .data:0000de03 ff                               rst 0x38
                           .data:0000de04 ff                               rst 0x38
                           .data:0000de05 ff                               rst 0x38
                           .data:0000de06 ff                               rst 0x38
                           .data:0000de07 ff                               rst 0x38
                           .data:0000de08 ff                               rst 0x38
                           .data:0000de09 ff                               rst 0x38
                           .data:0000de0a ff                               rst 0x38
                           .data:0000de0b ff                               rst 0x38
                           .data:0000de0c ff                               rst 0x38
                           .data:0000de0d ff                               rst 0x38
                           .data:0000de0e ff                               rst 0x38
                           .data:0000de0f ff                               rst 0x38
                           .data:0000de10 ff                               rst 0x38
                           .data:0000de11 ff                               rst 0x38
                           .data:0000de12 ff                               rst 0x38
                           .data:0000de13 ff                               rst 0x38
                           .data:0000de14 ff                               rst 0x38
                           .data:0000de15 ff                               rst 0x38
                           .data:0000de16 ff                               rst 0x38
                           .data:0000de17 ff                               rst 0x38
                           .data:0000de18 ff                               rst 0x38
                           .data:0000de19 ff                               rst 0x38
                           .data:0000de1a ff                               rst 0x38
                           .data:0000de1b ff                               rst 0x38
                           .data:0000de1c ff                               rst 0x38
                           .data:0000de1d ff                               rst 0x38
                           .data:0000de1e ff                               rst 0x38
                           .data:0000de1f ff                               rst 0x38
                           .data:0000de20 ff                               rst 0x38
                           .data:0000de21 ff                               rst 0x38
                           .data:0000de22 ff                               rst 0x38
                           .data:0000de23 ff                               rst 0x38
                           .data:0000de24 ff                               rst 0x38
                           .data:0000de25 ff                               rst 0x38
                           .data:0000de26 ff                               rst 0x38
                           .data:0000de27 ff                               rst 0x38
                           .data:0000de28 ff                               rst 0x38
                           .data:0000de29 ff                               rst 0x38
                           .data:0000de2a ff                               rst 0x38
                           .data:0000de2b ff                               rst 0x38
                           .data:0000de2c ff                               rst 0x38
                           .data:0000de2d ff                               rst 0x38
                           .data:0000de2e ff                               rst 0x38
                           .data:0000de2f ff                               rst 0x38
                           .data:0000de30 ff                               rst 0x38
                           .data:0000de31 ff                               rst 0x38
                           .data:0000de32 ff                               rst 0x38
                           .data:0000de33 ff                               rst 0x38
                           .data:0000de34 ff                               rst 0x38
                           .data:0000de35 ff                               rst 0x38
                           .data:0000de36 ff                               rst 0x38
                           .data:0000de37 ff                               rst 0x38
                           .data:0000de38 ff                               rst 0x38
                           .data:0000de39 ff                               rst 0x38
                           .data:0000de3a ff                               rst 0x38
                           .data:0000de3b ff                               rst 0x38
                           .data:0000de3c ff                               rst 0x38
                           .data:0000de3d ff                               rst 0x38
                           .data:0000de3e ff                               rst 0x38
                           .data:0000de3f ff                               rst 0x38
                           .data:0000de40 ff                               rst 0x38
                           .data:0000de41 ff                               rst 0x38
                           .data:0000de42 ff                               rst 0x38
                           .data:0000de43 ff                               rst 0x38
                           .data:0000de44 ff                               rst 0x38
                           .data:0000de45 ff                               rst 0x38
                           .data:0000de46 ff                               rst 0x38
                           .data:0000de47 ff                               rst 0x38
                           .data:0000de48 ff                               rst 0x38
                           .data:0000de49 ff                               rst 0x38
                           .data:0000de4a ff                               rst 0x38
                           .data:0000de4b ff                               rst 0x38
                           .data:0000de4c ff                               rst 0x38
                           .data:0000de4d ff                               rst 0x38
                           .data:0000de4e ff                               rst 0x38
                           .data:0000de4f ff                               rst 0x38
                           .data:0000de50 ff                               rst 0x38
                           .data:0000de51 ff                               rst 0x38
                           .data:0000de52 ff                               rst 0x38
                           .data:0000de53 ff                               rst 0x38
                           .data:0000de54 ff                               rst 0x38
                           .data:0000de55 ff                               rst 0x38
                           .data:0000de56 ff                               rst 0x38
                           .data:0000de57 ff                               rst 0x38
                           .data:0000de58 ff                               rst 0x38
                           .data:0000de59 ff                               rst 0x38
                           .data:0000de5a ff                               rst 0x38
                           .data:0000de5b ff                               rst 0x38
                           .data:0000de5c ff                               rst 0x38
                           .data:0000de5d ff                               rst 0x38
                           .data:0000de5e ff                               rst 0x38
                           .data:0000de5f ff                               rst 0x38
                           .data:0000de60 ff                               rst 0x38
                           .data:0000de61 ff                               rst 0x38
                           .data:0000de62 ff                               rst 0x38
                           .data:0000de63 ff                               rst 0x38
                           .data:0000de64 ff                               rst 0x38
                           .data:0000de65 ff                               rst 0x38
                           .data:0000de66 ff                               rst 0x38
                           .data:0000de67 ff                               rst 0x38
                           .data:0000de68 ff                               rst 0x38
                           .data:0000de69 ff                               rst 0x38
                           .data:0000de6a ff                               rst 0x38
                           .data:0000de6b ff                               rst 0x38
                           .data:0000de6c ff                               rst 0x38
                           .data:0000de6d ff                               rst 0x38
                           .data:0000de6e ff                               rst 0x38
                           .data:0000de6f ff                               rst 0x38
                           .data:0000de70 ff                               rst 0x38
                           .data:0000de71 ff                               rst 0x38
                           .data:0000de72 ff                               rst 0x38
                           .data:0000de73 ff                               rst 0x38
                           .data:0000de74 ff                               rst 0x38
                           .data:0000de75 ff                               rst 0x38
                           .data:0000de76 ff                               rst 0x38
                           .data:0000de77 ff                               rst 0x38
                           .data:0000de78 ff                               rst 0x38
                           .data:0000de79 ff                               rst 0x38
                           .data:0000de7a ff                               rst 0x38
                           .data:0000de7b ff                               rst 0x38
                           .data:0000de7c ff                               rst 0x38
                           .data:0000de7d ff                               rst 0x38
                           .data:0000de7e ff                               rst 0x38
                           .data:0000de7f ff                               rst 0x38
                           .data:0000de80 ff                               rst 0x38
                           .data:0000de81 ff                               rst 0x38
                           .data:0000de82 ff                               rst 0x38
                           .data:0000de83 ff                               rst 0x38
                           .data:0000de84 ff                               rst 0x38
                           .data:0000de85 ff                               rst 0x38
                           .data:0000de86 ff                               rst 0x38
                           .data:0000de87 ff                               rst 0x38
                           .data:0000de88 ff                               rst 0x38
                           .data:0000de89 ff                               rst 0x38
                           .data:0000de8a ff                               rst 0x38
                           .data:0000de8b ff                               rst 0x38
                           .data:0000de8c ff                               rst 0x38
                           .data:0000de8d ff                               rst 0x38
                           .data:0000de8e ff                               rst 0x38
                           .data:0000de8f ff                               rst 0x38
                           .data:0000de90 ff                               rst 0x38
                           .data:0000de91 ff                               rst 0x38
                           .data:0000de92 ff                               rst 0x38
                           .data:0000de93 ff                               rst 0x38
                           .data:0000de94 ff                               rst 0x38
                           .data:0000de95 ff                               rst 0x38
                           .data:0000de96 ff                               rst 0x38
                           .data:0000de97 ff                               rst 0x38
                           .data:0000de98 ff                               rst 0x38
                           .data:0000de99 ff                               rst 0x38
                           .data:0000de9a ff                               rst 0x38
                           .data:0000de9b ff                               rst 0x38
                           .data:0000de9c ff                               rst 0x38
                           .data:0000de9d ff                               rst 0x38
                           .data:0000de9e ff                               rst 0x38
                           .data:0000de9f ff                               rst 0x38
                           .data:0000dea0 ff                               rst 0x38
                           .data:0000dea1 ff                               rst 0x38
                           .data:0000dea2 ff                               rst 0x38
                           .data:0000dea3 ff                               rst 0x38
                           .data:0000dea4 ff                               rst 0x38
                           .data:0000dea5 ff                               rst 0x38
                           .data:0000dea6 ff                               rst 0x38
                           .data:0000dea7 ff                               rst 0x38
                           .data:0000dea8 ff                               rst 0x38
                           .data:0000dea9 ff                               rst 0x38
                           .data:0000deaa ff                               rst 0x38
                           .data:0000deab ff                               rst 0x38
                           .data:0000deac ff                               rst 0x38
                           .data:0000dead ff                               rst 0x38
                           .data:0000deae ff                               rst 0x38
                           .data:0000deaf ff                               rst 0x38
                           .data:0000deb0 ff                               rst 0x38
                           .data:0000deb1 ff                               rst 0x38
                           .data:0000deb2 ff                               rst 0x38
                           .data:0000deb3 ff                               rst 0x38
                           .data:0000deb4 ff                               rst 0x38
                           .data:0000deb5 ff                               rst 0x38
                           .data:0000deb6 ff                               rst 0x38
                           .data:0000deb7 ff                               rst 0x38
                           .data:0000deb8 ff                               rst 0x38
                           .data:0000deb9 ff                               rst 0x38
                           .data:0000deba ff                               rst 0x38
                           .data:0000debb ff                               rst 0x38
                           .data:0000debc ff                               rst 0x38
                           .data:0000debd ff                               rst 0x38
                           .data:0000debe ff                               rst 0x38
                           .data:0000debf ff                               rst 0x38
                           .data:0000dec0 ff                               rst 0x38
                           .data:0000dec1 ff                               rst 0x38
                           .data:0000dec2 ff                               rst 0x38
                           .data:0000dec3 ff                               rst 0x38
                           .data:0000dec4 ff                               rst 0x38
                           .data:0000dec5 ff                               rst 0x38
                           .data:0000dec6 ff                               rst 0x38
                           .data:0000dec7 ff                               rst 0x38
                           .data:0000dec8 ff                               rst 0x38
                           .data:0000dec9 ff                               rst 0x38
                           .data:0000deca ff                               rst 0x38
                           .data:0000decb ff                               rst 0x38
                           .data:0000decc ff                               rst 0x38
                           .data:0000decd ff                               rst 0x38
                           .data:0000dece ff                               rst 0x38
                           .data:0000decf ff                               rst 0x38
                           .data:0000ded0 ff                               rst 0x38
                           .data:0000ded1 ff                               rst 0x38
                           .data:0000ded2 ff                               rst 0x38
                           .data:0000ded3 ff                               rst 0x38
                           .data:0000ded4 ff                               rst 0x38
                           .data:0000ded5 ff                               rst 0x38
                           .data:0000ded6 ff                               rst 0x38
                           .data:0000ded7 ff                               rst 0x38
                           .data:0000ded8 ff                               rst 0x38
                           .data:0000ded9 ff                               rst 0x38
                           .data:0000deda ff                               rst 0x38
                           .data:0000dedb ff                               rst 0x38
                           .data:0000dedc ff                               rst 0x38
                           .data:0000dedd ff                               rst 0x38
                           .data:0000dede ff                               rst 0x38
                           .data:0000dedf ff                               rst 0x38
                           .data:0000dee0 ff                               rst 0x38
                           .data:0000dee1 ff                               rst 0x38
                           .data:0000dee2 ff                               rst 0x38
                           .data:0000dee3 ff                               rst 0x38
                           .data:0000dee4 ff                               rst 0x38
                           .data:0000dee5 ff                               rst 0x38
                           .data:0000dee6 ff                               rst 0x38
                           .data:0000dee7 ff                               rst 0x38
                           .data:0000dee8 ff                               rst 0x38
                           .data:0000dee9 ff                               rst 0x38
                           .data:0000deea ff                               rst 0x38
                           .data:0000deeb ff                               rst 0x38
                           .data:0000deec ff                               rst 0x38
                           .data:0000deed ff                               rst 0x38
                           .data:0000deee ff                               rst 0x38
                           .data:0000deef ff                               rst 0x38
                           .data:0000def0 ff                               rst 0x38
                           .data:0000def1 ff                               rst 0x38
                           .data:0000def2 ff                               rst 0x38
                           .data:0000def3 ff                               rst 0x38
                           .data:0000def4 ff                               rst 0x38
                           .data:0000def5 ff                               rst 0x38
                           .data:0000def6 ff                               rst 0x38
                           .data:0000def7 ff                               rst 0x38
                           .data:0000def8 ff                               rst 0x38
                           .data:0000def9 ff                               rst 0x38
                           .data:0000defa ff                               rst 0x38
                           .data:0000defb ff                               rst 0x38
                           .data:0000defc ff                               rst 0x38
                           .data:0000defd ff                               rst 0x38
                           .data:0000defe ff                               rst 0x38
                           .data:0000deff ff                               rst 0x38
                           .data:0000df00 ff                               rst 0x38
                           .data:0000df01 ff                               rst 0x38
                           .data:0000df02 ff                               rst 0x38
                           .data:0000df03 ff                               rst 0x38
                           .data:0000df04 ff                               rst 0x38
                           .data:0000df05 ff                               rst 0x38
                           .data:0000df06 ff                               rst 0x38
                           .data:0000df07 ff                               rst 0x38
                           .data:0000df08 ff                               rst 0x38
                           .data:0000df09 ff                               rst 0x38
                           .data:0000df0a ff                               rst 0x38
                           .data:0000df0b ff                               rst 0x38
                           .data:0000df0c ff                               rst 0x38
                           .data:0000df0d ff                               rst 0x38
                           .data:0000df0e ff                               rst 0x38
                           .data:0000df0f ff                               rst 0x38
                           .data:0000df10 ff                               rst 0x38
                           .data:0000df11 ff                               rst 0x38
                           .data:0000df12 ff                               rst 0x38
                           .data:0000df13 ff                               rst 0x38
                           .data:0000df14 ff                               rst 0x38
                           .data:0000df15 ff                               rst 0x38
                           .data:0000df16 ff                               rst 0x38
                           .data:0000df17 ff                               rst 0x38
                           .data:0000df18 ff                               rst 0x38
                           .data:0000df19 ff                               rst 0x38
                           .data:0000df1a ff                               rst 0x38
                           .data:0000df1b ff                               rst 0x38
                           .data:0000df1c ff                               rst 0x38
                           .data:0000df1d ff                               rst 0x38
                           .data:0000df1e ff                               rst 0x38
                           .data:0000df1f ff                               rst 0x38
                           .data:0000df20 ff                               rst 0x38
                           .data:0000df21 ff                               rst 0x38
                           .data:0000df22 ff                               rst 0x38
                           .data:0000df23 ff                               rst 0x38
                           .data:0000df24 ff                               rst 0x38
                           .data:0000df25 ff                               rst 0x38
                           .data:0000df26 ff                               rst 0x38
                           .data:0000df27 ff                               rst 0x38
                           .data:0000df28 ff                               rst 0x38
                           .data:0000df29 ff                               rst 0x38
                           .data:0000df2a ff                               rst 0x38
                           .data:0000df2b ff                               rst 0x38
                           .data:0000df2c ff                               rst 0x38
                           .data:0000df2d ff                               rst 0x38
                           .data:0000df2e ff                               rst 0x38
                           .data:0000df2f ff                               rst 0x38
                           .data:0000df30 ff                               rst 0x38
                           .data:0000df31 ff                               rst 0x38
                           .data:0000df32 ff                               rst 0x38
                           .data:0000df33 ff                               rst 0x38
                           .data:0000df34 ff                               rst 0x38
                           .data:0000df35 ff                               rst 0x38
                           .data:0000df36 ff                               rst 0x38
                           .data:0000df37 ff                               rst 0x38
                           .data:0000df38 ff                               rst 0x38
                           .data:0000df39 ff                               rst 0x38
                           .data:0000df3a ff                               rst 0x38
                           .data:0000df3b ff                               rst 0x38
                           .data:0000df3c ff                               rst 0x38
                           .data:0000df3d ff                               rst 0x38
                           .data:0000df3e ff                               rst 0x38
                           .data:0000df3f ff                               rst 0x38
                           .data:0000df40 ff                               rst 0x38
                           .data:0000df41 ff                               rst 0x38
                           .data:0000df42 ff                               rst 0x38
                           .data:0000df43 ff                               rst 0x38
                           .data:0000df44 ff                               rst 0x38
                           .data:0000df45 ff                               rst 0x38
                           .data:0000df46 ff                               rst 0x38
                           .data:0000df47 ff                               rst 0x38
                           .data:0000df48 ff                               rst 0x38
                           .data:0000df49 ff                               rst 0x38
                           .data:0000df4a ff                               rst 0x38
                           .data:0000df4b ff                               rst 0x38
                           .data:0000df4c ff                               rst 0x38
                           .data:0000df4d ff                               rst 0x38
                           .data:0000df4e ff                               rst 0x38
                           .data:0000df4f ff                               rst 0x38
                           .data:0000df50 ff                               rst 0x38
                           .data:0000df51 ff                               rst 0x38
                           .data:0000df52 ff                               rst 0x38
                           .data:0000df53 ff                               rst 0x38
                           .data:0000df54 ff                               rst 0x38
                           .data:0000df55 ff                               rst 0x38
                           .data:0000df56 ff                               rst 0x38
                           .data:0000df57 ff                               rst 0x38
                           .data:0000df58 ff                               rst 0x38
                           .data:0000df59 ff                               rst 0x38
                           .data:0000df5a ff                               rst 0x38
                           .data:0000df5b ff                               rst 0x38
                           .data:0000df5c ff                               rst 0x38
                           .data:0000df5d ff                               rst 0x38
                           .data:0000df5e ff                               rst 0x38
                           .data:0000df5f ff                               rst 0x38
                           .data:0000df60 ff                               rst 0x38
                           .data:0000df61 ff                               rst 0x38
                           .data:0000df62 ff                               rst 0x38
                           .data:0000df63 ff                               rst 0x38
                           .data:0000df64 ff                               rst 0x38
                           .data:0000df65 ff                               rst 0x38
                           .data:0000df66 ff                               rst 0x38
                           .data:0000df67 ff                               rst 0x38
                           .data:0000df68 ff                               rst 0x38
                           .data:0000df69 ff                               rst 0x38
                           .data:0000df6a ff                               rst 0x38
                           .data:0000df6b ff                               rst 0x38
                           .data:0000df6c ff                               rst 0x38
                           .data:0000df6d ff                               rst 0x38
                           .data:0000df6e ff                               rst 0x38
                           .data:0000df6f ff                               rst 0x38
                           .data:0000df70 ff                               rst 0x38
                           .data:0000df71 ff                               rst 0x38
                           .data:0000df72 ff                               rst 0x38
                           .data:0000df73 ff                               rst 0x38
                           .data:0000df74 ff                               rst 0x38
                           .data:0000df75 ff                               rst 0x38
                           .data:0000df76 ff                               rst 0x38
                           .data:0000df77 ff                               rst 0x38
                           .data:0000df78 ff                               rst 0x38
                           .data:0000df79 ff                               rst 0x38
                           .data:0000df7a ff                               rst 0x38
                           .data:0000df7b ff                               rst 0x38
                           .data:0000df7c ff                               rst 0x38
                           .data:0000df7d ff                               rst 0x38
                           .data:0000df7e ff                               rst 0x38
                           .data:0000df7f ff                               rst 0x38
                           .data:0000df80 ff                               rst 0x38
                           .data:0000df81 ff                               rst 0x38
                           .data:0000df82 ff                               rst 0x38
                           .data:0000df83 ff                               rst 0x38
                           .data:0000df84 ff                               rst 0x38
                           .data:0000df85 ff                               rst 0x38
                           .data:0000df86 ff                               rst 0x38
                           .data:0000df87 ff                               rst 0x38
                           .data:0000df88 ff                               rst 0x38
                           .data:0000df89 ff                               rst 0x38
                           .data:0000df8a ff                               rst 0x38
                           .data:0000df8b ff                               rst 0x38
                           .data:0000df8c ff                               rst 0x38
                           .data:0000df8d ff                               rst 0x38
                           .data:0000df8e ff                               rst 0x38
                           .data:0000df8f ff                               rst 0x38
                           .data:0000df90 ff                               rst 0x38
                           .data:0000df91 ff                               rst 0x38
                           .data:0000df92 ff                               rst 0x38
                           .data:0000df93 ff                               rst 0x38
                           .data:0000df94 ff                               rst 0x38
                           .data:0000df95 ff                               rst 0x38
                           .data:0000df96 ff                               rst 0x38
                           .data:0000df97 ff                               rst 0x38
                           .data:0000df98 ff                               rst 0x38
                           .data:0000df99 ff                               rst 0x38
                           .data:0000df9a ff                               rst 0x38
                           .data:0000df9b ff                               rst 0x38
                           .data:0000df9c ff                               rst 0x38
                           .data:0000df9d ff                               rst 0x38
                           .data:0000df9e ff                               rst 0x38
                           .data:0000df9f ff                               rst 0x38
                           .data:0000dfa0 ff                               rst 0x38
                           .data:0000dfa1 ff                               rst 0x38
                           .data:0000dfa2 ff                               rst 0x38
                           .data:0000dfa3 ff                               rst 0x38
                           .data:0000dfa4 ff                               rst 0x38
                           .data:0000dfa5 ff                               rst 0x38
                           .data:0000dfa6 ff                               rst 0x38
                           .data:0000dfa7 ff                               rst 0x38
                           .data:0000dfa8 ff                               rst 0x38
                           .data:0000dfa9 ff                               rst 0x38
                           .data:0000dfaa ff                               rst 0x38
                           .data:0000dfab ff                               rst 0x38
                           .data:0000dfac ff                               rst 0x38
                           .data:0000dfad ff                               rst 0x38
                           .data:0000dfae ff                               rst 0x38
                           .data:0000dfaf ff                               rst 0x38
                           .data:0000dfb0 ff                               rst 0x38
                           .data:0000dfb1 ff                               rst 0x38
                           .data:0000dfb2 ff                               rst 0x38
                           .data:0000dfb3 ff                               rst 0x38
                           .data:0000dfb4 ff                               rst 0x38
                           .data:0000dfb5 ff                               rst 0x38
                           .data:0000dfb6 ff                               rst 0x38
                           .data:0000dfb7 ff                               rst 0x38
                           .data:0000dfb8 ff                               rst 0x38
                           .data:0000dfb9 ff                               rst 0x38
                           .data:0000dfba ff                               rst 0x38
                           .data:0000dfbb ff                               rst 0x38
                           .data:0000dfbc ff                               rst 0x38
                           .data:0000dfbd ff                               rst 0x38
                           .data:0000dfbe ff                               rst 0x38
                           .data:0000dfbf ff                               rst 0x38
                           .data:0000dfc0 ff                               rst 0x38
                           .data:0000dfc1 ff                               rst 0x38
                           .data:0000dfc2 ff                               rst 0x38
                           .data:0000dfc3 ff                               rst 0x38
                           .data:0000dfc4 ff                               rst 0x38
                           .data:0000dfc5 ff                               rst 0x38
                           .data:0000dfc6 ff                               rst 0x38
                           .data:0000dfc7 ff                               rst 0x38
                           .data:0000dfc8 ff                               rst 0x38
                           .data:0000dfc9 ff                               rst 0x38
                           .data:0000dfca ff                               rst 0x38
                           .data:0000dfcb ff                               rst 0x38
                           .data:0000dfcc ff                               rst 0x38
                           .data:0000dfcd ff                               rst 0x38
                           .data:0000dfce ff                               rst 0x38
                           .data:0000dfcf ff                               rst 0x38
                           .data:0000dfd0 ff                               rst 0x38
                           .data:0000dfd1 ff                               rst 0x38
                           .data:0000dfd2 ff                               rst 0x38
                           .data:0000dfd3 ff                               rst 0x38
                           .data:0000dfd4 ff                               rst 0x38
                           .data:0000dfd5 ff                               rst 0x38
                           .data:0000dfd6 ff                               rst 0x38
                           .data:0000dfd7 ff                               rst 0x38
                           .data:0000dfd8 ff                               rst 0x38
                           .data:0000dfd9 ff                               rst 0x38
                           .data:0000dfda ff                               rst 0x38
                           .data:0000dfdb ff                               rst 0x38
                           .data:0000dfdc ff                               rst 0x38
                           .data:0000dfdd ff                               rst 0x38
                           .data:0000dfde ff                               rst 0x38
                           .data:0000dfdf ff                               rst 0x38
                           .data:0000dfe0 ff                               rst 0x38
                           .data:0000dfe1 ff                               rst 0x38
                           .data:0000dfe2 ff                               rst 0x38
                           .data:0000dfe3 ff                               rst 0x38
                           .data:0000dfe4 ff                               rst 0x38
                           .data:0000dfe5 ff                               rst 0x38
                           .data:0000dfe6 ff                               rst 0x38
                           .data:0000dfe7 ff                               rst 0x38
                           .data:0000dfe8 ff                               rst 0x38
                           .data:0000dfe9 ff                               rst 0x38
                           .data:0000dfea ff                               rst 0x38
                           .data:0000dfeb ff                               rst 0x38
                           .data:0000dfec ff                               rst 0x38
                           .data:0000dfed ff                               rst 0x38
                           .data:0000dfee ff                               rst 0x38
                           .data:0000dfef ff                               rst 0x38
                           .data:0000dff0 ff                               rst 0x38
                           .data:0000dff1 ff                               rst 0x38
                           .data:0000dff2 ff                               rst 0x38
                           .data:0000dff3 ff                               rst 0x38
                           .data:0000dff4 ff                               rst 0x38
                           .data:0000dff5 ff                               rst 0x38
                           .data:0000dff6 ff                               rst 0x38
                           .data:0000dff7 ff                               rst 0x38
                           .data:0000dff8 ff                               rst 0x38
                           .data:0000dff9 ff                               rst 0x38
                           .data:0000dffa ff                               rst 0x38
                           .data:0000dffb ff                               rst 0x38
                           .data:0000dffc ff                               rst 0x38
                           .data:0000dffd ff                               rst 0x38
                           .data:0000dffe ff                               rst 0x38
                           .data:0000dfff ff                               rst 0x38
                           .data:0000e000 c3 fb ec                         jp 0xecfb
                           .data:0000e003 c3 ea ec                         jp 0xecea
                           .data:0000e006 c3 26 ed                         jp 0xed26
                           .data:0000e009 c3 15 ed                         jp 0xed15
                           .data:0000e00c c3 a9 ef                         jp 0xefa9
                           .data:0000e00f c3 bb ef                         jp 0xefbb
                           .data:0000e012 c3 b5 ef                         jp 0xefb5
                           .data:0000e015 c3 d3 ed                         jp 0xedd3
                           .data:0000e018 c3 b0 ed                         jp 0xedb0
                           .data:0000e01b c3 1d ee                         jp 0xee1d
                           .data:0000e01e c3 66 ee                         jp 0xee66
                           .data:0000e021 c3 7d ee                         jp 0xee7d
                           .data:0000e024 c3 45 ef                         jp 0xef45
                           .data:0000e027 c3 a2 ee                         jp 0xeea2
                           .data:0000e02a c3 c6 ef                         jp 0xefc6
                           .data:0000e02d c3 ce ef                         jp 0xefce
                           .data:0000e030 c3 d8 ef                         jp 0xefd8
                           .data:0000e033 c3 d2 ef                         jp 0xefd2
                           .data:0000e036 c3 f2 ef                         jp 0xeff2
                           .data:0000e039 c3 16 f0                         jp 0xf016
                           .data:0000e03c c3 32 f0                         jp 0xf032
                           .data:0000e03f c3 5c f0                         jp 0xf05c
                           .data:0000e042 c3 85 f0                         jp 0xf085
                           .data:0000e045 c3 ae f0                         jp 0xf0ae
                           .data:0000e048 c3 13 ef                         jp 0xef13
                           .data:0000e04b c3 46 ef                         jp 0xef46
                           .data:0000e04e c3 76 ef                         jp 0xef76
                           .data:0000e051 c3 b0 ec                         jp 0xecb0
                           .data:0000e054 c3 b6 e1                         jp 0xe1b6
                           .data:0000e057 c3 f1 e1                         jp 0xe1f1
                           .data:0000e05a c3 70 e2                         jp 0xe270
                           .data:0000e05d c3 77 e2                         jp 0xe277
                           .data:0000e060 c3 83 e2                         jp 0xe283
                           .data:0000e063 c3 8f e2                         jp 0xe28f
                           .data:0000e066 c3 98 e2                         jp 0xe298
                           .data:0000e069 c3 9f e2                         jp 0xe29f
                           .data:0000e06c c3 ae e2                         jp 0xe2ae
                           .data:0000e06f c3 b5 e2                         jp 0xe2b5
                           .data:0000e072 c3 bc e2                         jp 0xe2bc
                           .data:0000e075 c3 d5 e2                         jp 0xe2d5
                           .data:0000e078 c3 e4 e2                         jp 0xe2e4
                           .data:0000e07b c3 ec e2                         jp 0xe2ec
                           .data:0000e07e c3 fb e2                         jp 0xe2fb
                           .data:0000e081 c3 0a e3                         jp 0xe30a
                           .data:0000e084 c3 13 e3                         jp 0xe313
                           .data:0000e087 c3 39 e3                         jp 0xe339
                           .data:0000e08a c3 40 e3                         jp 0xe340
                           .data:0000e08d c3 97 e3                         jp 0xe397
                           .data:0000e090 c3 b8 e3                         jp 0xe3b8
                           .data:0000e093 c3 d6 e3                         jp 0xe3d6
                           .data:0000e096 c3 fd e3                         jp 0xe3fd
                           .data:0000e099 c3 03 e4                         jp 0xe403
                           .data:0000e09c c3 9a e4                         jp 0xe49a
                           .data:0000e09f c3 cd e4                         jp 0xe4cd
                           .data:0000e0a2 c3 d3 e4                         jp 0xe4d3
                           .data:0000e0a5 c3 d9 e4                         jp 0xe4d9
                           .data:0000e0a8 c3 3c e5                         jp 0xe53c
                           .data:0000e0ab c3 64 e5                         jp 0xe564
                           .data:0000e0ae c3 72 e5                         jp 0xe572
                           .data:0000e0b1 c3 80 e5                         jp 0xe580
                           .data:0000e0b4 c3 87 e5                         jp 0xe587
                           .data:0000e0b7 c3 8e e5                         jp 0xe58e
                           .data:0000e0ba c3 a4 e5                         jp 0xe5a4
                           .data:0000e0bd c3 bd e5                         jp 0xe5bd
                           .data:0000e0c0 c3 73 e7                         jp 0xe773
                           .data:0000e0c3 c3 8d e8                         jp 0xe88d
                           .data:0000e0c6 c3 45 ec                         jp 0xec45
                           .data:0000e0c9 c3 9a ff                         jp 0xff9a
                           .data:0000e0cc c3 aa ff                         jp 0xffaa
                           .data:0000e0cf c3 b2 ff                         jp 0xffb2
                           .data:0000e0d2 c3 c6 ff                         jp 0xffc6
                           .data:0000e0d5 c3 d0 ff                         jp 0xffd0
                           .data:0000e0d8 c3 a1 ff                         jp 0xffa1
                           .data:0000e0db c3 b2 ff                         jp 0xffb2
                           .data:0000e0de c3 eb ff                         jp 0xffeb
                           .data:0000e0e1 c3 24 fb                         jp 0xfb24
                           .data:0000e0e4 c3 90 ff                         jp 0xff90
                           .data:0000e0e7 c3 74 fc                         jp 0xfc74
                           .data:0000e0ea c3 57 fc                         jp 0xfc57
                           .data:0000e0ed c3 dc fa                         jp 0xfadc
                           .data:0000e0f0 c3 ec fa                         jp 0xfaec
                           .data:0000e0f3 c3 0f f3                         jp 0xf30f
                           .data:0000e0f6 c3 e4 f3                         jp 0xf3e4
                           .data:0000e0f9 c3 83 f5                         jp 0xf583
                           .data:0000e0fc c3 b3 f5                         jp 0xf5b3
                           .data:0000e0ff c3 09 f2                         jp 0xf209
                           .data:0000e102 c3 fb f1                         jp 0xf1fb
                           .data:0000e105 c3 f4 f1                         jp 0xf1f4
                           .data:0000e108 c3 95 f5                         jp 0xf595
                           .data:0000e10b c3 e4 f4                         jp 0xf4e4
                           .data:0000e10e c3 b5 f2                         jp 0xf2b5
                           .data:0000e111 c3 7b f2                         jp 0xf27b
                           .data:0000e114 c3 4a f2                         jp 0xf24a
                           .data:0000e117 c3 08 f3                         jp 0xf308
                           .data:0000e11a c3 28 f2                         jp 0xf228
                           .data:0000e11d c3 89 f7                         jp 0xf789
                           .data:0000e120 c3 55 fb                         jp 0xfb55
                           .data:0000e123 c3 cb f5                         jp 0xf5cb
                           .data:0000e126 c3 54 fb                         jp 0xfb54
                           .data:0000e129 c3 bc fb                         jp 0xfbbc
                           .data:0000e12c c3 ed fa                         jp 0xfaed
                           .data:0000e12f c3 83 f7                         jp 0xf783
                           .data:0000e132 c3 de fb                         jp 0xfbde
                           .data:0000e135 c3 4e fc                         jp 0xfc4e
                           .data:0000e138 c3 aa f5                         jp 0xf5aa
                           .data:0000e13b c3 9e f5                         jp 0xf59e
                           .data:0000e13e c3 8c f5                         jp 0xf58c
                           .data:0000e141 c3 30 fb                         jp 0xfb30
                           .data:0000e144 c3 18 fc                         jp 0xfc18
                           .data:0000e147 c3 bf f5                         jp 0xf5bf
                           .data:0000e14a c3 0a fb                         jp 0xfb0a
                           .data:0000e14d c3 3d fb                         jp 0xfb3d
                           .data:0000e150 c3 3e fb                         jp 0xfb3e
                           .data:0000e153 c3 08 fc                         jp 0xfc08
                           .data:0000e156 c3 6a fb                         jp 0xfb6a
                           .data:0000e159 c3 44 fc                         jp 0xfc44
                           .data:0000e15c c3 4f fc                         jp 0xfc4f
                           .data:0000e15f c3 bd f2                         jp 0xf2bd
                           .data:0000e162 c3 61 ff                         jp 0xff61
                           .data:0000e165 c3 74 ff                         jp 0xff74
                           .data:0000e168 c3 d4 fb                         jp 0xfbd4
                           .data:0000e16b c3 1d fc                         jp 0xfc1d
                           .data:0000e16e c3 e4 fa                         jp 0xfae4
                           .data:0000e171 c3 e6 fb                         jp 0xfbe6
                           .data:0000e174 c3 45 fc                         jp 0xfc45
                           .data:0000e177 c3 02 f2                         jp 0xf202
                           .data:0000e17a c3 a6 f2                         jp 0xf2a6
                           .data:0000e17d c3 ee fb                         jp 0xfbee
                           .data:0000e180 c3 6c fc                         jp 0xfc6c
                           .data:0000e183 c3 1d fb                         jp 0xfb1d
                           .data:0000e186 c3 a4 f1                         jp 0xf1a4
                           .data:0000e189 c3 b9 f1                         jp 0xf1b9
                           .data:0000e18c c3 87 ff                         jp 0xff87
                           .data:0000e18f c3 8e fc                         jp 0xfc8e
                           .data:0000e192 c3 8c fc                         jp 0xfc8c
                           .data:0000e195 c3 f2 f0                         jp 0xf0f2
                           .data:0000e198 c3 7f f1                         jp 0xf17f
                           .data:0000e19b c3 2b f1                         jp 0xf12b
                           .data:0000e19e c3 50 f1                         jp 0xf150
                           .data:0000e1a1 c3 ac fa                         jp 0xfaac
                           .data:0000e1a4 c3 58 fa                         jp 0xfa58
                           .data:0000e1a7 c3 82 fa                         jp 0xfa82
                           .data:0000e1aa c3 94 fc                         jp 0xfc94
                           .data:0000e1ad c3 66 f8                         jp 0xf866
                           .data:0000e1b0 c3 b0 fd                         jp 0xfdb0
                           .data:0000e1b3 c3 e2 fd                         jp 0xfde2
                           .data:0000e1b6 21 00 00                         ld hl,0x0000
                           .data:0000e1b9 22 64 04                         ld (0x0464),hl
                           .data:0000e1bc 22 60 04                         ld (0x0460),hl
                           .data:0000e1bf 2a 97 01                         ld hl,(0x0197)
                           .data:0000e1c2 2b                               dec hl
                           .data:0000e1c3 22 62 04                         ld (0x0462),hl
                           .data:0000e1c6 2a 97 01                         ld hl,(0x0197)
                           .data:0000e1c9 22 68 04                         ld (0x0468),hl
                           .data:0000e1cc cd d4 fb                         call 0xfbd4
                           .data:0000e1cf 22 6a 04                         ld (0x046a),hl
                           .data:0000e1d2 2a 97 01                         ld hl,(0x0197)
                           .data:0000e1d5 11 01 00                         ld de,0x0001
                           .data:0000e1d8 eb                               ex de,hl
                           .data:0000e1d9 cd ee fb                         call 0xfbee
                           .data:0000e1dc 22 70 04                         ld (0x0470),hl
                           .data:0000e1df 2a 99 01                         ld hl,(0x0199)
                           .data:0000e1e2 22 78 04                         ld (0x0478),hl
                           .data:0000e1e5 2b                               dec hl
                           .data:0000e1e6 11 01 00                         ld de,0x0001
                           .data:0000e1e9 eb                               ex de,hl
                           .data:0000e1ea cd ee fb                         call 0xfbee
                           .data:0000e1ed 22 72 04                         ld (0x0472),hl
                           .data:0000e1f0 c9                               ret
                           .data:0000e1f1 cd 98 e2                         call 0xe298
                           .data:0000e1f4 11 01 00                         ld de,0x0001
                           .data:0000e1f7 cd 24 fb                         call 0xfb24
                           .data:0000e1fa 28 15                            jr z,0xe211
                           .data:0000e1fc 2a 99 01                         ld hl,(0x0199)
                           .data:0000e1ff eb                               ex de,hl
                           .data:0000e200 2a 8d 01                         ld hl,(0x018d)
                           .data:0000e203 cd 0a fb                         call 0xfb0a
                           .data:0000e206 e5                               push hl
                           .data:0000e207 cd ae e2                         call 0xe2ae
                           .data:0000e20a 2b                               dec hl
                           .data:0000e20b d1                               pop de
                           .data:0000e20c cd bc fb                         call 0xfbbc
                           .data:0000e20f 18 03                            jr 0xe214
                           .data:0000e211 2a 99 01                         ld hl,(0x0199)
                           .data:0000e214 22 78 04                         ld (0x0478),hl
                           .data:0000e217 2a 5e 04                         ld hl,(0x045e)
                           .data:0000e21a 11 01 00                         ld de,0x0001
                           .data:0000e21d cd 24 fb                         call 0xfb24
                           .data:0000e220 28 0d                            jr z,0xe22f
                           .data:0000e222 2a 8d 01                         ld hl,(0x018d)
                           .data:0000e225 e5                               push hl
                           .data:0000e226 cd ae e2                         call 0xe2ae
                           .data:0000e229 e5                               push hl
                           .data:0000e22a cd b0 ed                         call 0xedb0
                           .data:0000e22d d1                               pop de
                           .data:0000e22e d1                               pop de
                           .data:0000e22f cd 98 e2                         call 0xe298
                           .data:0000e232 11 01 00                         ld de,0x0001
                           .data:0000e235 cd 24 fb                         call 0xfb24
                           .data:0000e238 28 05                            jr z,0xe23f
                           .data:0000e23a cd ae e2                         call 0xe2ae
                           .data:0000e23d 18 03                            jr 0xe242
                           .data:0000e23f 21 00 00                         ld hl,0x0000
                           .data:0000e242 e5                               push hl
                           .data:0000e243 cd 26 ed                         call 0xed26
                           .data:0000e246 d1                               pop de
                           .data:0000e247 2a 78 04                         ld hl,(0x0478)
                           .data:0000e24a 2b                               dec hl
                           .data:0000e24b 22 66 04                         ld (0x0466),hl
                           .data:0000e24e 2a 78 04                         ld hl,(0x0478)
                           .data:0000e251 22 6c 04                         ld (0x046c),hl
                           .data:0000e254 cd d4 fb                         call 0xfbd4
                           .data:0000e257 22 6e 04                         ld (0x046e),hl
                           .data:0000e25a cd ff eb                         call 0xebff
                           .data:0000e25d 28 03                            jr z,0xe262
                           .data:0000e25f cd c4 e5                         call 0xe5c4
                           .data:0000e262 2a 5e 04                         ld hl,(0x045e)
                           .data:0000e265 11 02 00                         ld de,0x0002
                           .data:0000e268 cd 24 fb                         call 0xfb24
                           .data:0000e26b c8                               ret z
                           .data:0000e26c cd 03 e4                         call 0xe403
                           .data:0000e26f c9                               ret
                           .data:0000e270 2a 5c 04                         ld hl,(0x045c)
                           .data:0000e273 22 5e 04                         ld (0x045e),hl
                           .data:0000e276 c9                               ret
                           .data:0000e277 cd 70 e2                         call 0xe270
                           .data:0000e27a 21 00 00                         ld hl,0x0000
                           .data:0000e27d 22 5c 04                         ld (0x045c),hl
                           .data:0000e280 c3 f1 e1                         jp 0xe1f1
                           .data:0000e283 cd 70 e2                         call 0xe270
                           .data:0000e286 21 01 00                         ld hl,0x0001
                           .data:0000e289 22 5c 04                         ld (0x045c),hl
                           .data:0000e28c c3 f1 e1                         jp 0xe1f1
                           .data:0000e28f 21 02 00                         ld hl,0x0002
                           .data:0000e292 22 5c 04                         ld (0x045c),hl
                           .data:0000e295 c3 15 ed                         jp 0xed15
                           .data:0000e298 cd 90 ff                         call 0xff90
                           .data:0000e29b 2a 5c 04                         ld hl,(0x045c)
                           .data:0000e29e c9                               ret
                           .data:0000e29f cd 90 ff                         call 0xff90
                           .data:0000e2a2 21 08 00                         ld hl,0x0008
                           .data:0000e2a5 39                               add hl,sp
                           .data:0000e2a6 5e                               ld e,(hl)
                           .data:0000e2a7 23                               inc hl
                           .data:0000e2a8 56                               ld d,(hl)
                           .data:0000e2a9 eb                               ex de,hl
                           .data:0000e2aa 22 5a 04                         ld (0x045a),hl
                           .data:0000e2ad c9                               ret
                           .data:0000e2ae cd 90 ff                         call 0xff90
                           .data:0000e2b1 2a 5a 04                         ld hl,(0x045a)
                           .data:0000e2b4 c9                               ret
                           .data:0000e2b5 cd 90 ff                         call 0xff90
                           .data:0000e2b8 2a 58 04                         ld hl,(0x0458)
                           .data:0000e2bb c9                               ret
                           .data:0000e2bc cd 90 ff                         call 0xff90
                           .data:0000e2bf cd 73 e7                         call 0xe773
                           .data:0000e2c2 21 08 00                         ld hl,0x0008
                           .data:0000e2c5 39                               add hl,sp
                           .data:0000e2c6 5e                               ld e,(hl)
                           .data:0000e2c7 23                               inc hl
                           .data:0000e2c8 56                               ld d,(hl)
                           .data:0000e2c9 eb                               ex de,hl
                           .data:0000e2ca 22 58 04                         ld (0x0458),hl
                           .data:0000e2cd e5                               push hl
                           .data:0000e2ce cd 45 ef                         call 0xef45
                           .data:0000e2d1 d1                               pop de
                           .data:0000e2d2 c3 73 e7                         jp 0xe773
                           .data:0000e2d5 21 01 00                         ld hl,0x0001
                           .data:0000e2d8 7d                               ld a,l
                           .data:0000e2d9 32 55 04                         ld (0x0455),a
                           .data:0000e2dc cd 39 e3                         call 0xe339
                           .data:0000e2df 7d                               ld a,l
                           .data:0000e2e0 32 54 04                         ld (0x0454),a
                           .data:0000e2e3 c9                               ret
                           .data:0000e2e4 21 00 00                         ld hl,0x0000
                           .data:0000e2e7 7d                               ld a,l
                           .data:0000e2e8 32 55 04                         ld (0x0455),a
                           .data:0000e2eb c9                               ret
                           .data:0000e2ec 21 03 00                         ld hl,0x0003
                           .data:0000e2ef 7d                               ld a,l
                           .data:0000e2f0 32 55 04                         ld (0x0455),a
                           .data:0000e2f3 cd b5 e2                         call 0xe2b5
                           .data:0000e2f6 7d                               ld a,l
                           .data:0000e2f7 32 54 04                         ld (0x0454),a
                           .data:0000e2fa c9                               ret
                           .data:0000e2fb 21 02 00                         ld hl,0x0002
                           .data:0000e2fe 7d                               ld a,l
                           .data:0000e2ff 32 55 04                         ld (0x0455),a
                           .data:0000e302 cd 39 e3                         call 0xe339
                           .data:0000e305 7d                               ld a,l
                           .data:0000e306 32 54 04                         ld (0x0454),a
                           .data:0000e309 c9                               ret
                           .data:0000e30a cd 90 ff                         call 0xff90
                           .data:0000e30d 2a 55 04                         ld hl,(0x0455)
                           .data:0000e310 26 00                            ld h,0x00
                           .data:0000e312 c9                               ret
                           .data:0000e313 cd 90 ff                         call 0xff90
                           .data:0000e316 cd 73 e7                         call 0xe773
                           .data:0000e319 21 08 00                         ld hl,0x0008
                           .data:0000e31c 39                               add hl,sp
                           .data:0000e31d 5e                               ld e,(hl)
                           .data:0000e31e 23                               inc hl
                           .data:0000e31f 56                               ld d,(hl)
                           .data:0000e320 eb                               ex de,hl
                           .data:0000e321 22 56 04                         ld (0x0456),hl
                           .data:0000e324 cd 73 e7                         call 0xe773
                           .data:0000e327 cd 0a e3                         call 0xe30a
                           .data:0000e32a 11 03 00                         ld de,0x0003
                           .data:0000e32d cd 24 fb                         call 0xfb24
                           .data:0000e330 c0                               ret nz
                           .data:0000e331 2a 56 04                         ld hl,(0x0456)
                           .data:0000e334 7d                               ld a,l
                           .data:0000e335 32 54 04                         ld (0x0454),a
                           .data:0000e338 c9                               ret
                           .data:0000e339 cd 90 ff                         call 0xff90
                           .data:0000e33c 2a 56 04                         ld hl,(0x0456)
                           .data:0000e33f c9                               ret
                           .data:0000e340 cd 90 ff                         call 0xff90
                           .data:0000e343 21 f3 01                         ld hl,0x01f3
                           .data:0000e346 cd 09 f2                         call 0xf209
                           .data:0000e349 cd 4a f2                         call 0xf24a
                           .data:0000e34c cd b0 fd                         call 0xfdb0
                           .data:0000e34f cd 87 ff                         call 0xff87
                           .data:0000e352 21 08 00                         ld hl,0x0008
                           .data:0000e355 39                               add hl,sp
                           .data:0000e356 cd fb f1                         call 0xf1fb
                           .data:0000e359 cd e4 f4                         call 0xf4e4
                           .data:0000e35c 21 e7 01                         ld hl,0x01e7
                           .data:0000e35f cd fb f1                         call 0xf1fb
                           .data:0000e362 cd 0f f3                         call 0xf30f
                           .data:0000e365 cd 4a f2                         call 0xf24a
                           .data:0000e368 21 f3 01                         ld hl,0x01f3
                           .data:0000e36b cd 09 f2                         call 0xf209
                           .data:0000e36e cd 4a f2                         call 0xf24a
                           .data:0000e371 cd e2 fd                         call 0xfde2
                           .data:0000e374 cd 87 ff                         call 0xff87
                           .data:0000e377 21 10 00                         ld hl,0x0010
                           .data:0000e37a 39                               add hl,sp
                           .data:0000e37b cd fb f1                         call 0xf1fb
                           .data:0000e37e cd e4 f4                         call 0xf4e4
                           .data:0000e381 21 df 01                         ld hl,0x01df
                           .data:0000e384 cd fb f1                         call 0xf1fb
                           .data:0000e387 cd 0f f3                         call 0xf30f
                           .data:0000e38a cd 4a f2                         call 0xf24a
                           .data:0000e38d cd 8d e8                         call 0xe88d
                           .data:0000e390 eb                               ex de,hl
                           .data:0000e391 21 10 00                         ld hl,0x0010
                           .data:0000e394 39                               add hl,sp
                           .data:0000e395 f9                               ld sp,hl
                           .data:0000e396 c9                               ret
                           .data:0000e397 cd 90 ff                         call 0xff90
                           .data:0000e39a 21 10 00                         ld hl,0x0010
                           .data:0000e39d 39                               add hl,sp
                           .data:0000e39e cd 09 f2                         call 0xf209
                           .data:0000e3a1 cd 4a f2                         call 0xf24a
                           .data:0000e3a4 21 10 00                         ld hl,0x0010
                           .data:0000e3a7 39                               add hl,sp
                           .data:0000e3a8 cd 09 f2                         call 0xf209
                           .data:0000e3ab cd 4a f2                         call 0xf24a
                           .data:0000e3ae cd 8d e8                         call 0xe88d
                           .data:0000e3b1 eb                               ex de,hl
                           .data:0000e3b2 21 10 00                         ld hl,0x0010
                           .data:0000e3b5 39                               add hl,sp
                           .data:0000e3b6 f9                               ld sp,hl
                           .data:0000e3b7 c9                               ret
                           .data:0000e3b8 cd 90 ff                         call 0xff90
                           .data:0000e3bb 21 f3 01                         ld hl,0x01f3
                           .data:0000e3be cd 09 f2                         call 0xf209
                           .data:0000e3c1 21 08 00                         ld hl,0x0008
                           .data:0000e3c4 39                               add hl,sp
                           .data:0000e3c5 cd fb f1                         call 0xf1fb
                           .data:0000e3c8 cd 0f f3                         call 0xf30f
                           .data:0000e3cb cd 4a f2                         call 0xf24a
                           .data:0000e3ce cd d6 e3                         call 0xe3d6
                           .data:0000e3d1 eb                               ex de,hl
                           .data:0000e3d2 cd 87 ff                         call 0xff87
                           .data:0000e3d5 c9                               ret
                           .data:0000e3d6 cd 90 ff                         call 0xff90
                           .data:0000e3d9 cd 73 e7                         call 0xe773
                           .data:0000e3dc 21 f3 01                         ld hl,0x01f3
                           .data:0000e3df e5                               push hl
                           .data:0000e3e0 21 0a 00                         ld hl,0x000a
                           .data:0000e3e3 39                               add hl,sp
                           .data:0000e3e4 cd 09 f2                         call 0xf209
                           .data:0000e3e7 cd 4a f2                         call 0xf24a
                           .data:0000e3ea cd 45 ec                         call 0xec45
                           .data:0000e3ed cd 87 ff                         call 0xff87
                           .data:0000e3f0 e1                               pop hl
                           .data:0000e3f1 cd 28 f2                         call 0xf228
                           .data:0000e3f4 cd 89 f7                         call 0xf789
                           .data:0000e3f7 22 fb 01                         ld (0x01fb),hl
                           .data:0000e3fa c3 73 e7                         jp 0xe773
                           .data:0000e3fd 21 f3 01                         ld hl,0x01f3
                           .data:0000e400 c3 09 f2                         jp 0xf209
                           .data:0000e403 2a 66 04                         ld hl,(0x0466)
                           .data:0000e406 e5                               push hl
                           .data:0000e407 2a 58 04                         ld hl,(0x0458)
                           .data:0000e40a e5                               push hl
                           .data:0000e40b cd a2 ee                         call 0xeea2
                           .data:0000e40e d1                               pop de
                           .data:0000e40f d1                               pop de
                           .data:0000e410 c3 73 e7                         jp 0xe773
                           .data:0000e413 cd 90 ff                         call 0xff90
                           .data:0000e416 21 10 00                         ld hl,0x0010
                           .data:0000e419 39                               add hl,sp
                           .data:0000e41a cd 09 f2                         call 0xf209
                           .data:0000e41d cd 4a f2                         call 0xf24a
                           .data:0000e420 cd 02 e7                         call 0xe702
                           .data:0000e423 cd 87 ff                         call 0xff87
                           .data:0000e426 cd 4a f2                         call 0xf24a
                           .data:0000e429 21 10 00                         ld hl,0x0010
                           .data:0000e42c 39                               add hl,sp
                           .data:0000e42d cd 09 f2                         call 0xf209
                           .data:0000e430 cd 4a f2                         call 0xf24a
                           .data:0000e433 cd 02 e7                         call 0xe702
                           .data:0000e436 cd 87 ff                         call 0xff87
                           .data:0000e439 cd 4a f2                         call 0xf24a
                           .data:0000e43c cd d9 e4                         call 0xe4d9
                           .data:0000e43f eb                               ex de,hl
                           .data:0000e440 21 10 00                         ld hl,0x0010
                           .data:0000e443 39                               add hl,sp
                           .data:0000e444 f9                               ld sp,hl
                           .data:0000e445 2a 76 04                         ld hl,(0x0476)
                           .data:0000e448 e5                               push hl
                           .data:0000e449 2a 74 04                         ld hl,(0x0474)
                           .data:0000e44c e5                               push hl
                           .data:0000e44d cd 1c ec                         call 0xec1c
                           .data:0000e450 d1                               pop de
                           .data:0000e451 d1                               pop de
                           .data:0000e452 cd ff eb                         call 0xebff
                           .data:0000e455 28 42                            jr z,0xe499
                           .data:0000e457 2a 97 01                         ld hl,(0x0197)
                           .data:0000e45a ed 5b 74 04                      ld de,(0x0474)
                           .data:0000e45e cd 6a fb                         call 0xfb6a
                           .data:0000e461 22 74 04                         ld (0x0474),hl
                           .data:0000e464 11 00 00                         ld de,0x0000
                           .data:0000e467 eb                               ex de,hl
                           .data:0000e468 cd 54 fb                         call 0xfb54
                           .data:0000e46b 28 0b                            jr z,0xe478
                           .data:0000e46d 2a 97 01                         ld hl,(0x0197)
                           .data:0000e470 eb                               ex de,hl
                           .data:0000e471 2a 74 04                         ld hl,(0x0474)
                           .data:0000e474 19                               add hl,de
                           .data:0000e475 22 74 04                         ld (0x0474),hl
                           .data:0000e478 2a 78 04                         ld hl,(0x0478)
                           .data:0000e47b ed 5b 76 04                      ld de,(0x0476)
                           .data:0000e47f cd 6a fb                         call 0xfb6a
                           .data:0000e482 22 76 04                         ld (0x0476),hl
                           .data:0000e485 11 00 00                         ld de,0x0000
                           .data:0000e488 eb                               ex de,hl
                           .data:0000e489 cd 54 fb                         call 0xfb54
                           .data:0000e48c 28 0b                            jr z,0xe499
                           .data:0000e48e 2a 78 04                         ld hl,(0x0478)
                           .data:0000e491 eb                               ex de,hl
                           .data:0000e492 2a 76 04                         ld hl,(0x0476)
                           .data:0000e495 19                               add hl,de
                           .data:0000e496 22 76 04                         ld (0x0476),hl
                           .data:0000e499 c9                               ret
                           .data:0000e49a cd 90 ff                         call 0xff90
                           .data:0000e49d 21 10 00                         ld hl,0x0010
                           .data:0000e4a0 39                               add hl,sp
                           .data:0000e4a1 cd 09 f2                         call 0xf209
                           .data:0000e4a4 cd 4a f2                         call 0xf24a
                           .data:0000e4a7 21 10 00                         ld hl,0x0010
                           .data:0000e4aa 39                               add hl,sp
                           .data:0000e4ab cd 09 f2                         call 0xf209
                           .data:0000e4ae cd 4a f2                         call 0xf24a
                           .data:0000e4b1 cd 13 e4                         call 0xe413
                           .data:0000e4b4 eb                               ex de,hl
                           .data:0000e4b5 21 10 00                         ld hl,0x0010
                           .data:0000e4b8 39                               add hl,sp
                           .data:0000e4b9 f9                               ld sp,hl
                           .data:0000e4ba 2a 54 04                         ld hl,(0x0454)
                           .data:0000e4bd e5                               push hl
                           .data:0000e4be 2a 76 04                         ld hl,(0x0476)
                           .data:0000e4c1 e5                               push hl
                           .data:0000e4c2 2a 74 04                         ld hl,(0x0474)
                           .data:0000e4c5 e5                               push hl
                           .data:0000e4c6 cd da ec                         call 0xecda
                           .data:0000e4c9 d1                               pop de
                           .data:0000e4ca d1                               pop de
                           .data:0000e4cb d1                               pop de
                           .data:0000e4cc c9                               ret
                           .data:0000e4cd 21 df 01                         ld hl,0x01df
                           .data:0000e4d0 c3 09 f2                         jp 0xf209
                           .data:0000e4d3 21 e7 01                         ld hl,0x01e7
                           .data:0000e4d6 c3 09 f2                         jp 0xf209
                           .data:0000e4d9 cd ed fa                         call 0xfaed
                           .data:0000e4dc f0                               ret p
                           .data:0000e4dd ff                               rst 0x38
                           .data:0000e4de 21 18 00                         ld hl,0x0018
                           .data:0000e4e1 39                               add hl,sp
                           .data:0000e4e2 e5                               push hl
                           .data:0000e4e3 2a 70 04                         ld hl,(0x0470)
                           .data:0000e4e6 cd cb f5                         call 0xf5cb
                           .data:0000e4e9 cd a6 f2                         call 0xf2a6
                           .data:0000e4ec e1                               pop hl
                           .data:0000e4ed e5                               push hl
                           .data:0000e4ee cd 09 f2                         call 0xf209
                           .data:0000e4f1 cd 0f f3                         call 0xf30f
                           .data:0000e4f4 e1                               pop hl
                           .data:0000e4f5 cd 28 f2                         call 0xf228
                           .data:0000e4f8 21 20 00                         ld hl,0x0020
                           .data:0000e4fb 39                               add hl,sp
                           .data:0000e4fc e5                               push hl
                           .data:0000e4fd 2a 72 04                         ld hl,(0x0472)
                           .data:0000e500 cd cb f5                         call 0xf5cb
                           .data:0000e503 21 22 00                         ld hl,0x0022
                           .data:0000e506 39                               add hl,sp
                           .data:0000e507 cd fb f1                         call 0xf1fb
                           .data:0000e50a cd 08 f3                         call 0xf308
                           .data:0000e50d e1                               pop hl
                           .data:0000e50e cd 28 f2                         call 0xf228
                           .data:0000e511 21 18 00                         ld hl,0x0018
                           .data:0000e514 39                               add hl,sp
                           .data:0000e515 cd 09 f2                         call 0xf209
                           .data:0000e518 cd 4a f2                         call 0xf24a
                           .data:0000e51b cd c8 e6                         call 0xe6c8
                           .data:0000e51e eb                               ex de,hl
                           .data:0000e51f cd 87 ff                         call 0xff87
                           .data:0000e522 eb                               ex de,hl
                           .data:0000e523 22 74 04                         ld (0x0474),hl
                           .data:0000e526 21 20 00                         ld hl,0x0020
                           .data:0000e529 39                               add hl,sp
                           .data:0000e52a cd 09 f2                         call 0xf209
                           .data:0000e52d cd 4a f2                         call 0xf24a
                           .data:0000e530 cd c8 e6                         call 0xe6c8
                           .data:0000e533 eb                               ex de,hl
                           .data:0000e534 cd 87 ff                         call 0xff87
                           .data:0000e537 eb                               ex de,hl
                           .data:0000e538 22 76 04                         ld (0x0476),hl
                           .data:0000e53b c9                               ret
                           .data:0000e53c 21 e7 01                         ld hl,0x01e7
                           .data:0000e53f cd 09 f2                         call 0xf209
                           .data:0000e542 cd 4a f2                         call 0xf24a
                           .data:0000e545 21 df 01                         ld hl,0x01df
                           .data:0000e548 cd 09 f2                         call 0xf209
                           .data:0000e54b cd 4a f2                         call 0xf24a
                           .data:0000e54e cd d9 e4                         call 0xe4d9
                           .data:0000e551 eb                               ex de,hl
                           .data:0000e552 21 10 00                         ld hl,0x0010
                           .data:0000e555 39                               add hl,sp
                           .data:0000e556 f9                               ld sp,hl
                           .data:0000e557 2a 74 04                         ld hl,(0x0474)
                           .data:0000e55a 22 ef 01                         ld (0x01ef),hl
                           .data:0000e55d 2a 76 04                         ld hl,(0x0476)
                           .data:0000e560 22 f1 01                         ld (0x01f1),hl
                           .data:0000e563 c9                               ret
                           .data:0000e564 cd 80 e5                         call 0xe580
                           .data:0000e567 c8                               ret z
                           .data:0000e568 cd 73 e7                         call 0xe773
                           .data:0000e56b 21 00 00                         ld hl,0x0000
                           .data:0000e56e 22 fd 01                         ld (0x01fd),hl
                           .data:0000e571 c9                               ret
                           .data:0000e572 cd 80 e5                         call 0xe580
                           .data:0000e575 c0                               ret nz
                           .data:0000e576 21 01 00                         ld hl,0x0001
                           .data:0000e579 22 fd 01                         ld (0x01fd),hl
                           .data:0000e57c cd 73 e7                         call 0xe773
                           .data:0000e57f c9                               ret
                           .data:0000e580 cd 90 ff                         call 0xff90
                           .data:0000e583 2a fd 01                         ld hl,(0x01fd)
                           .data:0000e586 c9                               ret
                           .data:0000e587 21 00 00                         ld hl,0x0000
                           .data:0000e58a 22 ff 01                         ld (0x01ff),hl
                           .data:0000e58d c9                               ret
                           .data:0000e58e cd 73 e7                         call 0xe773
                           .data:0000e591 21 01 00                         ld hl,0x0001
                           .data:0000e594 22 ff 01                         ld (0x01ff),hl
                           .data:0000e597 2a 78 04                         ld hl,(0x0478)
                           .data:0000e59a 7c                               ld a,h
                           .data:0000e59b b5                               or l
                           .data:0000e59c 28 03                            jr z,0xe5a1
                           .data:0000e59e cd c4 e5                         call 0xe5c4
                           .data:0000e5a1 c3 73 e7                         jp 0xe773
                           .data:0000e5a4 2a f1 01                         ld hl,(0x01f1)
                           .data:0000e5a7 e5                               push hl
                           .data:0000e5a8 2a ef 01                         ld hl,(0x01ef)
                           .data:0000e5ab e5                               push hl
                           .data:0000e5ac cd b0 ec                         call 0xecb0
                           .data:0000e5af d1                               pop de
                           .data:0000e5b0 d1                               pop de
                           .data:0000e5b1 28 03                            jr z,0xe5b6
                           .data:0000e5b3 cd 51 04                         call 0x0451
                           .data:0000e5b6 21 02 00                         ld hl,0x0002
                           .data:0000e5b9 22 ff 01                         ld (0x01ff),hl
                           .data:0000e5bc c9                               ret
                           .data:0000e5bd cd 90 ff                         call 0xff90
                           .data:0000e5c0 2a ff 01                         ld hl,(0x01ff)
                           .data:0000e5c3 c9                               ret
                           .data:0000e5c4 18 03                            jr 0xe5c9
                           .data:0000e5c6 cd 3c e5                         call 0xe53c
                           .data:0000e5c9 2a ef 01                         ld hl,(0x01ef)
                           .data:0000e5cc 11 00 00                         ld de,0x0000
                           .data:0000e5cf eb                               ex de,hl
                           .data:0000e5d0 cd 54 fb                         call 0xfb54
                           .data:0000e5d3 28 30                            jr z,0xe605
                           .data:0000e5d5 21 e7 01                         ld hl,0x01e7
                           .data:0000e5d8 cd 09 f2                         call 0xf209
                           .data:0000e5db cd 4a f2                         call 0xf24a
                           .data:0000e5de 21 df 01                         ld hl,0x01df
                           .data:0000e5e1 e5                               push hl
                           .data:0000e5e2 2a 97 01                         ld hl,(0x0197)
                           .data:0000e5e5 cd cb f5                         call 0xf5cb
                           .data:0000e5e8 cd a6 f2                         call 0xf2a6
                           .data:0000e5eb e1                               pop hl
                           .data:0000e5ec e5                               push hl
                           .data:0000e5ed cd 09 f2                         call 0xf209
                           .data:0000e5f0 cd 0f f3                         call 0xf30f
                           .data:0000e5f3 e1                               pop hl
                           .data:0000e5f4 cd 28 f2                         call 0xf228
                           .data:0000e5f7 cd 4a f2                         call 0xf24a
                           .data:0000e5fa cd d9 e4                         call 0xe4d9
                           .data:0000e5fd eb                               ex de,hl
                           .data:0000e5fe 21 10 00                         ld hl,0x0010
                           .data:0000e601 39                               add hl,sp
                           .data:0000e602 f9                               ld sp,hl
                           .data:0000e603 18 c1                            jr 0xe5c6
                           .data:0000e605 18 03                            jr 0xe60a
                           .data:0000e607 cd 3c e5                         call 0xe53c
                           .data:0000e60a 2a ef 01                         ld hl,(0x01ef)
                           .data:0000e60d eb                               ex de,hl
                           .data:0000e60e 2a 97 01                         ld hl,(0x0197)
                           .data:0000e611 cd 3d fb                         call 0xfb3d
                           .data:0000e614 28 30                            jr z,0xe646
                           .data:0000e616 21 e7 01                         ld hl,0x01e7
                           .data:0000e619 cd 09 f2                         call 0xf209
                           .data:0000e61c cd 4a f2                         call 0xf24a
                           .data:0000e61f 21 df 01                         ld hl,0x01df
                           .data:0000e622 e5                               push hl
                           .data:0000e623 2a 97 01                         ld hl,(0x0197)
                           .data:0000e626 cd cb f5                         call 0xf5cb
                           .data:0000e629 cd a6 f2                         call 0xf2a6
                           .data:0000e62c e1                               pop hl
                           .data:0000e62d e5                               push hl
                           .data:0000e62e cd 09 f2                         call 0xf209
                           .data:0000e631 cd 08 f3                         call 0xf308
                           .data:0000e634 e1                               pop hl
                           .data:0000e635 cd 28 f2                         call 0xf228
                           .data:0000e638 cd 4a f2                         call 0xf24a
                           .data:0000e63b cd d9 e4                         call 0xe4d9
                           .data:0000e63e eb                               ex de,hl
                           .data:0000e63f 21 10 00                         ld hl,0x0010
                           .data:0000e642 39                               add hl,sp
                           .data:0000e643 f9                               ld sp,hl
                           .data:0000e644 18 c1                            jr 0xe607
                           .data:0000e646 18 03                            jr 0xe64b
                           .data:0000e648 cd 3c e5                         call 0xe53c
                           .data:0000e64b 2a f1 01                         ld hl,(0x01f1)
                           .data:0000e64e 11 00 00                         ld de,0x0000
                           .data:0000e651 eb                               ex de,hl
                           .data:0000e652 cd 54 fb                         call 0xfb54
                           .data:0000e655 28 30                            jr z,0xe687
                           .data:0000e657 21 e7 01                         ld hl,0x01e7
                           .data:0000e65a e5                               push hl
                           .data:0000e65b 2a 78 04                         ld hl,(0x0478)
                           .data:0000e65e cd cb f5                         call 0xf5cb
                           .data:0000e661 cd a6 f2                         call 0xf2a6
                           .data:0000e664 e1                               pop hl
                           .data:0000e665 e5                               push hl
                           .data:0000e666 cd 09 f2                         call 0xf209
                           .data:0000e669 cd 08 f3                         call 0xf308
                           .data:0000e66c e1                               pop hl
                           .data:0000e66d cd 28 f2                         call 0xf228
                           .data:0000e670 cd 4a f2                         call 0xf24a
                           .data:0000e673 21 df 01                         ld hl,0x01df
                           .data:0000e676 cd 09 f2                         call 0xf209
                           .data:0000e679 cd 4a f2                         call 0xf24a
                           .data:0000e67c cd d9 e4                         call 0xe4d9
                           .data:0000e67f eb                               ex de,hl
                           .data:0000e680 21 10 00                         ld hl,0x0010
                           .data:0000e683 39                               add hl,sp
                           .data:0000e684 f9                               ld sp,hl
                           .data:0000e685 18 c1                            jr 0xe648
                           .data:0000e687 18 03                            jr 0xe68c
                           .data:0000e689 cd 3c e5                         call 0xe53c
                           .data:0000e68c 2a f1 01                         ld hl,(0x01f1)
                           .data:0000e68f eb                               ex de,hl
                           .data:0000e690 2a 78 04                         ld hl,(0x0478)
                           .data:0000e693 cd 3d fb                         call 0xfb3d
                           .data:0000e696 c8                               ret z
                           .data:0000e697 21 e7 01                         ld hl,0x01e7
                           .data:0000e69a e5                               push hl
                           .data:0000e69b 2a 78 04                         ld hl,(0x0478)
                           .data:0000e69e cd cb f5                         call 0xf5cb
                           .data:0000e6a1 cd a6 f2                         call 0xf2a6
                           .data:0000e6a4 e1                               pop hl
                           .data:0000e6a5 e5                               push hl
                           .data:0000e6a6 cd 09 f2                         call 0xf209
                           .data:0000e6a9 cd 0f f3                         call 0xf30f
                           .data:0000e6ac e1                               pop hl
                           .data:0000e6ad cd 28 f2                         call 0xf228
                           .data:0000e6b0 cd 4a f2                         call 0xf24a
                           .data:0000e6b3 21 df 01                         ld hl,0x01df
                           .data:0000e6b6 cd 09 f2                         call 0xf209
                           .data:0000e6b9 cd 4a f2                         call 0xf24a
                           .data:0000e6bc cd d9 e4                         call 0xe4d9
                           .data:0000e6bf eb                               ex de,hl
                           .data:0000e6c0 21 10 00                         ld hl,0x0010
                           .data:0000e6c3 39                               add hl,sp
                           .data:0000e6c4 f9                               ld sp,hl
                           .data:0000e6c5 18 c2                            jr 0xe689
                           .data:0000e6c7 c9                               ret
                           .data:0000e6c8 cd 90 ff                         call 0xff90
                           .data:0000e6cb 21 08 00                         ld hl,0x0008
                           .data:0000e6ce 39                               add hl,sp
                           .data:0000e6cf cd 09 f2                         call 0xf209
                           .data:0000e6d2 21 cf 01                         ld hl,0x01cf
                           .data:0000e6d5 cd fb f1                         call 0xf1fb
                           .data:0000e6d8 cd aa f5                         call 0xf5aa
                           .data:0000e6db 28 0d                            jr z,0xe6ea
                           .data:0000e6dd cd 02 f2                         call 0xf202
                           .data:0000e6e0 40                               ld b,b
                           .data:0000e6e1 80                               add a,b
                           .data:0000e6e2 00                               nop
                           .data:0000e6e3 00                               nop
                           .data:0000e6e4 00                               nop
                           .data:0000e6e5 00                               nop
                           .data:0000e6e6 00                               nop
                           .data:0000e6e7 00                               nop
                           .data:0000e6e8 18 0b                            jr 0xe6f5
                           .data:0000e6ea cd 02 f2                         call 0xf202
                           .data:0000e6ed c0                               ret nz
                           .data:0000e6ee 7f                               ld a,a
                           .data:0000e6ef ff                               rst 0x38
                           .data:0000e6f0 ff                               rst 0x38
                           .data:0000e6f1 ff                               rst 0x38
                           .data:0000e6f2 ff                               rst 0x38
                           .data:0000e6f3 ff                               rst 0x38
                           .data:0000e6f4 b8                               cp b
                           .data:0000e6f5 21 08 00                         ld hl,0x0008
                           .data:0000e6f8 39                               add hl,sp
                           .data:0000e6f9 cd fb f1                         call 0xf1fb
                           .data:0000e6fc cd 0f f3                         call 0xf30f
                           .data:0000e6ff c3 89 f7                         jp 0xf789
                           .data:0000e702 cd 90 ff                         call 0xff90
                           .data:0000e705 21 08 00                         ld hl,0x0008
                           .data:0000e708 39                               add hl,sp
                           .data:0000e709 cd 09 f2                         call 0xf209
                           .data:0000e70c cd f4 f1                         call 0xf1f4
                           .data:0000e70f c2 3e 80                         jp nz,0x803e
                           .data:0000e712 00                               nop
                           .data:0000e713 00                               nop
                           .data:0000e714 00                               nop
                           .data:0000e715 00                               nop
                           .data:0000e716 00                               nop
                           .data:0000e717 cd aa f5                         call 0xf5aa
                           .data:0000e71a 28 1e                            jr z,0xe73a
                           .data:0000e71c 21 08 00                         ld hl,0x0008
                           .data:0000e71f 39                               add hl,sp
                           .data:0000e720 cd 09 f2                         call 0xf209
                           .data:0000e723 cd f4 f1                         call 0xf1f4
                           .data:0000e726 42                               ld b,d
                           .data:0000e727 3e 80                            ld a,0x80
                           .data:0000e729 00                               nop
                           .data:0000e72a 00                               nop
                           .data:0000e72b 00                               nop
                           .data:0000e72c 00                               nop
                           .data:0000e72d 00                               nop
                           .data:0000e72e cd 9e f5                         call 0xf59e
                           .data:0000e731 28 07                            jr z,0xe73a
                           .data:0000e733 21 08 00                         ld hl,0x0008
                           .data:0000e736 39                               add hl,sp
                           .data:0000e737 c3 09 f2                         jp 0xf209
                           .data:0000e73a c3 51 04                         jp 0x0451
                           .data:0000e73d cd 90 ff                         call 0xff90
                           .data:0000e740 21 08 00                         ld hl,0x0008
                           .data:0000e743 39                               add hl,sp
                           .data:0000e744 4e                               ld c,(hl)
                           .data:0000e745 23                               inc hl
                           .data:0000e746 46                               ld b,(hl)
                           .data:0000e747 60                               ld h,b
                           .data:0000e748 69                               ld l,c
                           .data:0000e749 11 80 00                         ld de,0x0080
                           .data:0000e74c cd dc fa                         call 0xfadc
                           .data:0000e74f 28 0a                            jr z,0xe75b
                           .data:0000e751 21 00 ff                         ld hl,0xff00
                           .data:0000e754 50                               ld d,b
                           .data:0000e755 59                               ld e,c
                           .data:0000e756 cd e6 fb                         call 0xfbe6
                           .data:0000e759 44                               ld b,h
                           .data:0000e75a 4d                               ld c,l
                           .data:0000e75b 21 0a 00                         ld hl,0x000a
                           .data:0000e75e 39                               add hl,sp
                           .data:0000e75f 5e                               ld e,(hl)
                           .data:0000e760 23                               inc hl
                           .data:0000e761 56                               ld d,(hl)
                           .data:0000e762 21 02 00                         ld hl,0x0002
                           .data:0000e765 cd dc fa                         call 0xfadc
                           .data:0000e768 28 06                            jr z,0xe770
                           .data:0000e76a 60                               ld h,b
                           .data:0000e76b 69                               ld l,c
                           .data:0000e76c cd d4 fb                         call 0xfbd4
                           .data:0000e76f c9                               ret
                           .data:0000e770 60                               ld h,b
                           .data:0000e771 69                               ld l,c
                           .data:0000e772 c9                               ret
                           .data:0000e773 cd 90 ff                         call 0xff90
                           .data:0000e776 cd 80 e5                         call 0xe580
                           .data:0000e779 c8                               ret z
                           .data:0000e77a cd 39 e3                         call 0xe339
                           .data:0000e77d 7d                               ld a,l
                           .data:0000e77e 32 01 02                         ld (0x0201),a
                           .data:0000e781 2a fb 01                         ld hl,(0x01fb)
                           .data:0000e784 23                               inc hl
                           .data:0000e785 23                               inc hl
                           .data:0000e786 23                               inc hl
                           .data:0000e787 11 06 00                         ld de,0x0006
                           .data:0000e78a eb                               ex de,hl
                           .data:0000e78b cd 0a fb                         call 0xfb0a
                           .data:0000e78e 11 0f 00                         ld de,0x000f
                           .data:0000e791 eb                               ex de,hl
                           .data:0000e792 cd 6a fb                         call 0xfb6a
                           .data:0000e795 22 9c 04                         ld (0x049c),hl
                           .data:0000e798 2a fb 01                         ld hl,(0x01fb)
                           .data:0000e79b 23                               inc hl
                           .data:0000e79c 23                               inc hl
                           .data:0000e79d 23                               inc hl
                           .data:0000e79e 11 5a 00                         ld de,0x005a
                           .data:0000e7a1 eb                               ex de,hl
                           .data:0000e7a2 cd 0a fb                         call 0xfb0a
                           .data:0000e7a5 22 9a 04                         ld (0x049a),hl
                           .data:0000e7a8 11 01 00                         ld de,0x0001
                           .data:0000e7ab cd dc fa                         call 0xfadc
                           .data:0000e7ae 28 0c                            jr z,0xe7bc
                           .data:0000e7b0 2a 9c 04                         ld hl,(0x049c)
                           .data:0000e7b3 11 0f 00                         ld de,0x000f
                           .data:0000e7b6 cd 6c fc                         call 0xfc6c
                           .data:0000e7b9 22 9c 04                         ld (0x049c),hl
                           .data:0000e7bc 2a 9c 04                         ld hl,(0x049c)
                           .data:0000e7bf 11 06 00                         ld de,0x0006
                           .data:0000e7c2 cd bc fb                         call 0xfbbc
                           .data:0000e7c5 11 28 01                         ld de,0x0128
                           .data:0000e7c8 19                               add hl,de
                           .data:0000e7c9 22 98 04                         ld (0x0498),hl
                           .data:0000e7cc 2a ef 01                         ld hl,(0x01ef)
                           .data:0000e7cf 22 88 04                         ld (0x0488),hl
                           .data:0000e7d2 2a f1 01                         ld hl,(0x01f1)
                           .data:0000e7d5 22 8a 04                         ld (0x048a),hl
                           .data:0000e7d8 21 8c 04                         ld hl,0x048c
                           .data:0000e7db 44                               ld b,h
                           .data:0000e7dc 4d                               ld c,l
                           .data:0000e7dd 18 08                            jr 0xe7e7
                           .data:0000e7df 21 04 00                         ld hl,0x0004
                           .data:0000e7e2 50                               ld d,b
                           .data:0000e7e3 59                               ld e,c
                           .data:0000e7e4 19                               add hl,de
                           .data:0000e7e5 44                               ld b,h
                           .data:0000e7e6 4d                               ld c,l
                           .data:0000e7e7 50                               ld d,b
                           .data:0000e7e8 59                               ld e,c
                           .data:0000e7e9 21 98 04                         ld hl,0x0498
                           .data:0000e7ec cd 4e fc                         call 0xfc4e
                           .data:0000e7ef 28 7b                            jr z,0xe86c
                           .data:0000e7f1 21 fc ff                         ld hl,0xfffc
                           .data:0000e7f4 09                               add hl,bc
                           .data:0000e7f5 5e                               ld e,(hl)
                           .data:0000e7f6 23                               inc hl
                           .data:0000e7f7 56                               ld d,(hl)
                           .data:0000e7f8 eb                               ex de,hl
                           .data:0000e7f9 22 7a 04                         ld (0x047a),hl
                           .data:0000e7fc 21 fe ff                         ld hl,0xfffe
                           .data:0000e7ff 09                               add hl,bc
                           .data:0000e800 5e                               ld e,(hl)
                           .data:0000e801 23                               inc hl
                           .data:0000e802 56                               ld d,(hl)
                           .data:0000e803 eb                               ex de,hl
                           .data:0000e804 22 7c 04                         ld (0x047c),hl
                           .data:0000e807 2a 9a 04                         ld hl,(0x049a)
                           .data:0000e80a e5                               push hl
                           .data:0000e80b 2a 98 04                         ld hl,(0x0498)
                           .data:0000e80e 23                               inc hl
                           .data:0000e80f 22 98 04                         ld (0x0498),hl
                           .data:0000e812 5e                               ld e,(hl)
                           .data:0000e813 16 00                            ld d,0x00
                           .data:0000e815 d5                               push de
                           .data:0000e816 cd 3d e7                         call 0xe73d
                           .data:0000e819 d1                               pop de
                           .data:0000e81a d1                               pop de
                           .data:0000e81b eb                               ex de,hl
                           .data:0000e81c 2a 88 04                         ld hl,(0x0488)
                           .data:0000e81f 19                               add hl,de
                           .data:0000e820 eb                               ex de,hl
                           .data:0000e821 60                               ld h,b
                           .data:0000e822 69                               ld l,c
                           .data:0000e823 73                               ld (hl),e
                           .data:0000e824 23                               inc hl
                           .data:0000e825 72                               ld (hl),d
                           .data:0000e826 eb                               ex de,hl
                           .data:0000e827 22 7e 04                         ld (0x047e),hl
                           .data:0000e82a 2a 9a 04                         ld hl,(0x049a)
                           .data:0000e82d 23                               inc hl
                           .data:0000e82e e5                               push hl
                           .data:0000e82f 2a 98 04                         ld hl,(0x0498)
                           .data:0000e832 23                               inc hl
                           .data:0000e833 22 98 04                         ld (0x0498),hl
                           .data:0000e836 5e                               ld e,(hl)
                           .data:0000e837 16 00                            ld d,0x00
                           .data:0000e839 d5                               push de
                           .data:0000e83a cd 3d e7                         call 0xe73d
                           .data:0000e83d d1                               pop de
                           .data:0000e83e d1                               pop de
                           .data:0000e83f eb                               ex de,hl
                           .data:0000e840 2a 8a 04                         ld hl,(0x048a)
                           .data:0000e843 19                               add hl,de
                           .data:0000e844 eb                               ex de,hl
                           .data:0000e845 21 02 00                         ld hl,0x0002
                           .data:0000e848 09                               add hl,bc
                           .data:0000e849 73                               ld (hl),e
                           .data:0000e84a 23                               inc hl
                           .data:0000e84b 72                               ld (hl),d
                           .data:0000e84c eb                               ex de,hl
                           .data:0000e84d 22 80 04                         ld (0x0480),hl
                           .data:0000e850 2a 01 02                         ld hl,(0x0201)
                           .data:0000e853 e5                               push hl
                           .data:0000e854 2a 80 04                         ld hl,(0x0480)
                           .data:0000e857 e5                               push hl
                           .data:0000e858 2a 7e 04                         ld hl,(0x047e)
                           .data:0000e85b e5                               push hl
                           .data:0000e85c cd da ec                         call 0xecda
                           .data:0000e85f d1                               pop de
                           .data:0000e860 d1                               pop de
                           .data:0000e861 2a 01 02                         ld hl,(0x0201)
                           .data:0000e864 e3                               ex (sp),hl
                           .data:0000e865 cd b0 e9                         call 0xe9b0
                           .data:0000e868 d1                               pop de
                           .data:0000e869 c3 df e7                         jp 0xe7df
                           .data:0000e86c 2a 94 04                         ld hl,(0x0494)
                           .data:0000e86f 22 7a 04                         ld (0x047a),hl
                           .data:0000e872 2a 96 04                         ld hl,(0x0496)
                           .data:0000e875 22 7c 04                         ld (0x047c),hl
                           .data:0000e878 2a 88 04                         ld hl,(0x0488)
                           .data:0000e87b 22 7e 04                         ld (0x047e),hl
                           .data:0000e87e 2a 8a 04                         ld hl,(0x048a)
                           .data:0000e881 22 80 04                         ld (0x0480),hl
                           .data:0000e884 2a 01 02                         ld hl,(0x0201)
                           .data:0000e887 e5                               push hl
                           .data:0000e888 cd b0 e9                         call 0xe9b0
                           .data:0000e88b d1                               pop de
                           .data:0000e88c c9                               ret
                           .data:0000e88d cd 90 ff                         call 0xff90
                           .data:0000e890 21 10 00                         ld hl,0x0010
                           .data:0000e893 39                               add hl,sp
                           .data:0000e894 cd 09 f2                         call 0xf209
                           .data:0000e897 cd 4a f2                         call 0xf24a
                           .data:0000e89a cd 02 e7                         call 0xe702
                           .data:0000e89d cd 87 ff                         call 0xff87
                           .data:0000e8a0 cd 4a f2                         call 0xf24a
                           .data:0000e8a3 21 10 00                         ld hl,0x0010
                           .data:0000e8a6 39                               add hl,sp
                           .data:0000e8a7 cd 09 f2                         call 0xf209
                           .data:0000e8aa cd 4a f2                         call 0xf24a
                           .data:0000e8ad cd 02 e7                         call 0xe702
                           .data:0000e8b0 cd 87 ff                         call 0xff87
                           .data:0000e8b3 cd 4a f2                         call 0xf24a
                           .data:0000e8b6 cd d9 e4                         call 0xe4d9
                           .data:0000e8b9 eb                               ex de,hl
                           .data:0000e8ba 21 10 00                         ld hl,0x0010
                           .data:0000e8bd 39                               add hl,sp
                           .data:0000e8be f9                               ld sp,hl
                           .data:0000e8bf 2a 76 04                         ld hl,(0x0476)
                           .data:0000e8c2 e5                               push hl
                           .data:0000e8c3 2a 74 04                         ld hl,(0x0474)
                           .data:0000e8c6 e5                               push hl
                           .data:0000e8c7 cd 1c ec                         call 0xec1c
                           .data:0000e8ca d1                               pop de
                           .data:0000e8cb d1                               pop de
                           .data:0000e8cc cd 73 e7                         call 0xe773
                           .data:0000e8cf 2a ef 01                         ld hl,(0x01ef)
                           .data:0000e8d2 22 7a 04                         ld (0x047a),hl
                           .data:0000e8d5 2a f1 01                         ld hl,(0x01f1)
                           .data:0000e8d8 22 7c 04                         ld (0x047c),hl
                           .data:0000e8db 2a 74 04                         ld hl,(0x0474)
                           .data:0000e8de 22 ef 01                         ld (0x01ef),hl
                           .data:0000e8e1 22 7e 04                         ld (0x047e),hl
                           .data:0000e8e4 2a 76 04                         ld hl,(0x0476)
                           .data:0000e8e7 22 f1 01                         ld (0x01f1),hl
                           .data:0000e8ea 22 80 04                         ld (0x0480),hl
                           .data:0000e8ed 21 df 01                         ld hl,0x01df
                           .data:0000e8f0 e5                               push hl
                           .data:0000e8f1 21 0a 00                         ld hl,0x000a
                           .data:0000e8f4 39                               add hl,sp
                           .data:0000e8f5 cd 09 f2                         call 0xf209
                           .data:0000e8f8 e1                               pop hl
                           .data:0000e8f9 cd 28 f2                         call 0xf228
                           .data:0000e8fc 21 e7 01                         ld hl,0x01e7
                           .data:0000e8ff e5                               push hl
                           .data:0000e900 21 12 00                         ld hl,0x0012
                           .data:0000e903 39                               add hl,sp
                           .data:0000e904 cd 09 f2                         call 0xf209
                           .data:0000e907 e1                               pop hl
                           .data:0000e908 cd 28 f2                         call 0xf228
                           .data:0000e90b cd ff eb                         call 0xebff
                           .data:0000e90e ca a0 e9                         jp z,0xe9a0
                           .data:0000e911 cd 0a e3                         call 0xe30a
                           .data:0000e914 ca 9b e9                         jp z,0xe99b
                           .data:0000e917 2a 54 04                         ld hl,(0x0454)
                           .data:0000e91a e5                               push hl
                           .data:0000e91b cd b0 e9                         call 0xe9b0
                           .data:0000e91e d1                               pop de
                           .data:0000e91f 22 9e 04                         ld (0x049e),hl
                           .data:0000e922 11 01 00                         ld de,0x0001
                           .data:0000e925 eb                               ex de,hl
                           .data:0000e926 cd ee fb                         call 0xfbee
                           .data:0000e929 44                               ld b,h
                           .data:0000e92a 4d                               ld c,l
                           .data:0000e92b 2a 76 04                         ld hl,(0x0476)
                           .data:0000e92e e5                               push hl
                           .data:0000e92f 2a 74 04                         ld hl,(0x0474)
                           .data:0000e932 e5                               push hl
                           .data:0000e933 cd b0 ec                         call 0xecb0
                           .data:0000e936 d1                               pop de
                           .data:0000e937 d1                               pop de
                           .data:0000e938 28 61                            jr z,0xe99b
                           .data:0000e93a 2a 7e 04                         ld hl,(0x047e)
                           .data:0000e93d 22 7a 04                         ld (0x047a),hl
                           .data:0000e940 2a 80 04                         ld hl,(0x0480)
                           .data:0000e943 22 7c 04                         ld (0x047c),hl
                           .data:0000e946 2a 9e 04                         ld hl,(0x049e)
                           .data:0000e949 29                               add hl,hl
                           .data:0000e94a 11 68 04                         ld de,0x0468
                           .data:0000e94d 19                               add hl,de
                           .data:0000e94e 5e                               ld e,(hl)
                           .data:0000e94f 23                               inc hl
                           .data:0000e950 56                               ld d,(hl)
                           .data:0000e951 d5                               push de
                           .data:0000e952 21 7a 04                         ld hl,0x047a
                           .data:0000e955 09                               add hl,bc
                           .data:0000e956 09                               add hl,bc
                           .data:0000e957 d1                               pop de
                           .data:0000e958 e5                               push hl
                           .data:0000e959 7e                               ld a,(hl)
                           .data:0000e95a 23                               inc hl
                           .data:0000e95b 66                               ld h,(hl)
                           .data:0000e95c 6f                               ld l,a
                           .data:0000e95d 19                               add hl,de
                           .data:0000e95e eb                               ex de,hl
                           .data:0000e95f e1                               pop hl
                           .data:0000e960 73                               ld (hl),e
                           .data:0000e961 23                               inc hl
                           .data:0000e962 72                               ld (hl),d
                           .data:0000e963 2a 74 04                         ld hl,(0x0474)
                           .data:0000e966 22 7e 04                         ld (0x047e),hl
                           .data:0000e969 2a 76 04                         ld hl,(0x0476)
                           .data:0000e96c 22 80 04                         ld (0x0480),hl
                           .data:0000e96f 2a 9e 04                         ld hl,(0x049e)
                           .data:0000e972 29                               add hl,hl
                           .data:0000e973 11 68 04                         ld de,0x0468
                           .data:0000e976 19                               add hl,de
                           .data:0000e977 5e                               ld e,(hl)
                           .data:0000e978 23                               inc hl
                           .data:0000e979 56                               ld d,(hl)
                           .data:0000e97a d5                               push de
                           .data:0000e97b 21 7e 04                         ld hl,0x047e
                           .data:0000e97e 09                               add hl,bc
                           .data:0000e97f 09                               add hl,bc
                           .data:0000e980 d1                               pop de
                           .data:0000e981 e5                               push hl
                           .data:0000e982 7e                               ld a,(hl)
                           .data:0000e983 23                               inc hl
                           .data:0000e984 66                               ld h,(hl)
                           .data:0000e985 6f                               ld l,a
                           .data:0000e986 19                               add hl,de
                           .data:0000e987 eb                               ex de,hl
                           .data:0000e988 e1                               pop hl
                           .data:0000e989 73                               ld (hl),e
                           .data:0000e98a 23                               inc hl
                           .data:0000e98b 72                               ld (hl),d
                           .data:0000e98c 2a 7e 04                         ld hl,(0x047e)
                           .data:0000e98f 22 74 04                         ld (0x0474),hl
                           .data:0000e992 2a 80 04                         ld hl,(0x0480)
                           .data:0000e995 22 76 04                         ld (0x0476),hl
                           .data:0000e998 c3 17 e9                         jp 0xe917
                           .data:0000e99b cd c4 e5                         call 0xe5c4
                           .data:0000e99e 18 0d                            jr 0xe9ad
                           .data:0000e9a0 cd 0a e3                         call 0xe30a
                           .data:0000e9a3 28 08                            jr z,0xe9ad
                           .data:0000e9a5 2a 54 04                         ld hl,(0x0454)
                           .data:0000e9a8 e5                               push hl
                           .data:0000e9a9 cd b0 e9                         call 0xe9b0
                           .data:0000e9ac d1                               pop de
                           .data:0000e9ad c3 73 e7                         jp 0xe773
                           .data:0000e9b0 cd 90 ff                         call 0xff90
                           .data:0000e9b3 21 ff ff                         ld hl,0xffff
                           .data:0000e9b6 22 86 04                         ld (0x0486),hl
                           .data:0000e9b9 22 84 04                         ld (0x0484),hl
                           .data:0000e9bc 22 a2 04                         ld (0x04a2),hl
                           .data:0000e9bf 21 00 00                         ld hl,0x0000
                           .data:0000e9c2 22 82 04                         ld (0x0482),hl
                           .data:0000e9c5 21 ff ff                         ld hl,0xffff
                           .data:0000e9c8 22 a0 04                         ld (0x04a0),hl
                           .data:0000e9cb 2a a0 04                         ld hl,(0x04a0)
                           .data:0000e9ce 23                               inc hl
                           .data:0000e9cf 22 a0 04                         ld (0x04a0),hl
                           .data:0000e9d2 11 02 00                         ld de,0x0002
                           .data:0000e9d5 eb                               ex de,hl
                           .data:0000e9d6 cd 54 fb                         call 0xfb54
                           .data:0000e9d9 ca 57 ea                         jp z,0xea57
                           .data:0000e9dc 21 ff ff                         ld hl,0xffff
                           .data:0000e9df 44                               ld b,h
                           .data:0000e9e0 4d                               ld c,l
                           .data:0000e9e1 03                               inc bc
                           .data:0000e9e2 60                               ld h,b
                           .data:0000e9e3 69                               ld l,c
                           .data:0000e9e4 11 02 00                         ld de,0x0002
                           .data:0000e9e7 eb                               ex de,hl
                           .data:0000e9e8 cd 54 fb                         call 0xfb54
                           .data:0000e9eb 28 67                            jr z,0xea54
                           .data:0000e9ed 60                               ld h,b
                           .data:0000e9ee 69                               ld l,c
                           .data:0000e9ef 29                               add hl,hl
                           .data:0000e9f0 29                               add hl,hl
                           .data:0000e9f1 11 60 04                         ld de,0x0460
                           .data:0000e9f4 19                               add hl,de
                           .data:0000e9f5 5e                               ld e,(hl)
                           .data:0000e9f6 23                               inc hl
                           .data:0000e9f7 56                               ld d,(hl)
                           .data:0000e9f8 d5                               push de
                           .data:0000e9f9 60                               ld h,b
                           .data:0000e9fa 69                               ld l,c
                           .data:0000e9fb 29                               add hl,hl
                           .data:0000e9fc e5                               push hl
                           .data:0000e9fd 2a a0 04                         ld hl,(0x04a0)
                           .data:0000ea00 29                               add hl,hl
                           .data:0000ea01 29                               add hl,hl
                           .data:0000ea02 d1                               pop de
                           .data:0000ea03 19                               add hl,de
                           .data:0000ea04 11 7a 04                         ld de,0x047a
                           .data:0000ea07 19                               add hl,de
                           .data:0000ea08 5e                               ld e,(hl)
                           .data:0000ea09 23                               inc hl
                           .data:0000ea0a 56                               ld d,(hl)
                           .data:0000ea0b eb                               ex de,hl
                           .data:0000ea0c 22 a4 04                         ld (0x04a4),hl
                           .data:0000ea0f d1                               pop de
                           .data:0000ea10 eb                               ex de,hl
                           .data:0000ea11 cd 54 fb                         call 0xfb54
                           .data:0000ea14 28 14                            jr z,0xea2a
                           .data:0000ea16 21 00 00                         ld hl,0x0000
                           .data:0000ea19 e5                               push hl
                           .data:0000ea1a c5                               push bc
                           .data:0000ea1b 2a a0 04                         ld hl,(0x04a0)
                           .data:0000ea1e e5                               push hl
                           .data:0000ea1f cd a1 ea                         call 0xeaa1
                           .data:0000ea22 d1                               pop de
                           .data:0000ea23 d1                               pop de
                           .data:0000ea24 d1                               pop de
                           .data:0000ea25 22 a2 04                         ld (0x04a2),hl
                           .data:0000ea28 18 28                            jr 0xea52
                           .data:0000ea2a 60                               ld h,b
                           .data:0000ea2b 69                               ld l,c
                           .data:0000ea2c 29                               add hl,hl
                           .data:0000ea2d 29                               add hl,hl
                           .data:0000ea2e 11 62 04                         ld de,0x0462
                           .data:0000ea31 19                               add hl,de
                           .data:0000ea32 5e                               ld e,(hl)
                           .data:0000ea33 23                               inc hl
                           .data:0000ea34 56                               ld d,(hl)
                           .data:0000ea35 d5                               push de
                           .data:0000ea36 2a a4 04                         ld hl,(0x04a4)
                           .data:0000ea39 d1                               pop de
                           .data:0000ea3a eb                               ex de,hl
                           .data:0000ea3b cd 55 fb                         call 0xfb55
                           .data:0000ea3e 28 12                            jr z,0xea52
                           .data:0000ea40 21 01 00                         ld hl,0x0001
                           .data:0000ea43 e5                               push hl
                           .data:0000ea44 c5                               push bc
                           .data:0000ea45 2a a0 04                         ld hl,(0x04a0)
                           .data:0000ea48 e5                               push hl
                           .data:0000ea49 cd a1 ea                         call 0xeaa1
                           .data:0000ea4c d1                               pop de
                           .data:0000ea4d d1                               pop de
                           .data:0000ea4e d1                               pop de
                           .data:0000ea4f 22 a2 04                         ld (0x04a2),hl
                           .data:0000ea52 18 8d                            jr 0xe9e1
                           .data:0000ea54 c3 cb e9                         jp 0xe9cb
                           .data:0000ea57 2a 82 04                         ld hl,(0x0482)
                           .data:0000ea5a 7c                               ld a,h
                           .data:0000ea5b b5                               or l
                           .data:0000ea5c 20 3f                            jr nz,0xea9d
                           .data:0000ea5e 2a 7c 04                         ld hl,(0x047c)
                           .data:0000ea61 e5                               push hl
                           .data:0000ea62 2a 7a 04                         ld hl,(0x047a)
                           .data:0000ea65 e5                               push hl
                           .data:0000ea66 cd b0 ec                         call 0xecb0
                           .data:0000ea69 d1                               pop de
                           .data:0000ea6a d1                               pop de
                           .data:0000ea6b 20 30                            jr nz,0xea9d
                           .data:0000ea6d 2a 80 04                         ld hl,(0x0480)
                           .data:0000ea70 e5                               push hl
                           .data:0000ea71 2a 7e 04                         ld hl,(0x047e)
                           .data:0000ea74 e5                               push hl
                           .data:0000ea75 cd b0 ec                         call 0xecb0
                           .data:0000ea78 d1                               pop de
                           .data:0000ea79 d1                               pop de
                           .data:0000ea7a 20 21                            jr nz,0xea9d
                           .data:0000ea7c 21 08 00                         ld hl,0x0008
                           .data:0000ea7f 39                               add hl,sp
                           .data:0000ea80 5e                               ld e,(hl)
                           .data:0000ea81 23                               inc hl
                           .data:0000ea82 56                               ld d,(hl)
                           .data:0000ea83 d5                               push de
                           .data:0000ea84 2a 80 04                         ld hl,(0x0480)
                           .data:0000ea87 e5                               push hl
                           .data:0000ea88 2a 7e 04                         ld hl,(0x047e)
                           .data:0000ea8b e5                               push hl
                           .data:0000ea8c 2a 7c 04                         ld hl,(0x047c)
                           .data:0000ea8f e5                               push hl
                           .data:0000ea90 2a 7a 04                         ld hl,(0x047a)
                           .data:0000ea93 e5                               push hl
                           .data:0000ea94 cd 7d ee                         call 0xee7d
                           .data:0000ea97 eb                               ex de,hl
                           .data:0000ea98 21 0a 00                         ld hl,0x000a
                           .data:0000ea9b 39                               add hl,sp
                           .data:0000ea9c f9                               ld sp,hl
                           .data:0000ea9d 2a a2 04                         ld hl,(0x04a2)
                           .data:0000eaa0 c9                               ret
                           .data:0000eaa1 cd 90 ff                         call 0xff90
                           .data:0000eaa4 21 0a 00                         ld hl,0x000a
                           .data:0000eaa7 39                               add hl,sp
                           .data:0000eaa8 4e                               ld c,(hl)
                           .data:0000eaa9 23                               inc hl
                           .data:0000eaaa 46                               ld b,(hl)
                           .data:0000eaab 21 08 00                         ld hl,0x0008
                           .data:0000eaae 39                               add hl,sp
                           .data:0000eaaf 5e                               ld e,(hl)
                           .data:0000eab0 23                               inc hl
                           .data:0000eab1 56                               ld d,(hl)
                           .data:0000eab2 eb                               ex de,hl
                           .data:0000eab3 22 a8 04                         ld (0x04a8),hl
                           .data:0000eab6 60                               ld h,b
                           .data:0000eab7 69                               ld l,c
                           .data:0000eab8 29                               add hl,hl
                           .data:0000eab9 e5                               push hl
                           .data:0000eaba 2a a8 04                         ld hl,(0x04a8)
                           .data:0000eabd 11 01 00                         ld de,0x0001
                           .data:0000eac0 cd 18 fc                         call 0xfc18
                           .data:0000eac3 29                               add hl,hl
                           .data:0000eac4 29                               add hl,hl
                           .data:0000eac5 d1                               pop de
                           .data:0000eac6 19                               add hl,de
                           .data:0000eac7 11 7a 04                         ld de,0x047a
                           .data:0000eaca 19                               add hl,de
                           .data:0000eacb 5e                               ld e,(hl)
                           .data:0000eacc 23                               inc hl
                           .data:0000eacd 56                               ld d,(hl)
                           .data:0000eace d5                               push de
                           .data:0000eacf 60                               ld h,b
                           .data:0000ead0 69                               ld l,c
                           .data:0000ead1 29                               add hl,hl
                           .data:0000ead2 e5                               push hl
                           .data:0000ead3 2a a8 04                         ld hl,(0x04a8)
                           .data:0000ead6 29                               add hl,hl
                           .data:0000ead7 29                               add hl,hl
                           .data:0000ead8 d1                               pop de
                           .data:0000ead9 19                               add hl,de
                           .data:0000eada 11 7a 04                         ld de,0x047a
                           .data:0000eadd 19                               add hl,de
                           .data:0000eade 5e                               ld e,(hl)
                           .data:0000eadf 23                               inc hl
                           .data:0000eae0 56                               ld d,(hl)
                           .data:0000eae1 e1                               pop hl
                           .data:0000eae2 cd 18 fc                         call 0xfc18
                           .data:0000eae5 22 aa 04                         ld (0x04aa),hl
                           .data:0000eae8 7c                               ld a,h
                           .data:0000eae9 b5                               or l
                           .data:0000eaea ca a7 eb                         jp z,0xeba7
                           .data:0000eaed 2a aa 04                         ld hl,(0x04aa)
                           .data:0000eaf0 cd cb f5                         call 0xf5cb
                           .data:0000eaf3 cd 4a f2                         call 0xf24a
                           .data:0000eaf6 21 01 00                         ld hl,0x0001
                           .data:0000eaf9 50                               ld d,b
                           .data:0000eafa 59                               ld e,c
                           .data:0000eafb eb                               ex de,hl
                           .data:0000eafc cd 18 fc                         call 0xfc18
                           .data:0000eaff 29                               add hl,hl
                           .data:0000eb00 e5                               push hl
                           .data:0000eb01 2a a8 04                         ld hl,(0x04a8)
                           .data:0000eb04 29                               add hl,hl
                           .data:0000eb05 29                               add hl,hl
                           .data:0000eb06 d1                               pop de
                           .data:0000eb07 19                               add hl,de
                           .data:0000eb08 11 7a 04                         ld de,0x047a
                           .data:0000eb0b 19                               add hl,de
                           .data:0000eb0c 5e                               ld e,(hl)
                           .data:0000eb0d 23                               inc hl
                           .data:0000eb0e 56                               ld d,(hl)
                           .data:0000eb0f d5                               push de
                           .data:0000eb10 21 01 00                         ld hl,0x0001
                           .data:0000eb13 50                               ld d,b
                           .data:0000eb14 59                               ld e,c
                           .data:0000eb15 eb                               ex de,hl
                           .data:0000eb16 cd 18 fc                         call 0xfc18
                           .data:0000eb19 29                               add hl,hl
                           .data:0000eb1a e5                               push hl
                           .data:0000eb1b 2a a8 04                         ld hl,(0x04a8)
                           .data:0000eb1e 11 01 00                         ld de,0x0001
                           .data:0000eb21 cd 18 fc                         call 0xfc18
                           .data:0000eb24 29                               add hl,hl
                           .data:0000eb25 29                               add hl,hl
                           .data:0000eb26 d1                               pop de
                           .data:0000eb27 19                               add hl,de
                           .data:0000eb28 11 7a 04                         ld de,0x047a
                           .data:0000eb2b 19                               add hl,de
                           .data:0000eb2c 5e                               ld e,(hl)
                           .data:0000eb2d 23                               inc hl
                           .data:0000eb2e 56                               ld d,(hl)
                           .data:0000eb2f e1                               pop hl
                           .data:0000eb30 cd 18 fc                         call 0xfc18
                           .data:0000eb33 cd cb f5                         call 0xf5cb
                           .data:0000eb36 cd 4a f2                         call 0xf24a
                           .data:0000eb39 21 1c 00                         ld hl,0x001c
                           .data:0000eb3c 39                               add hl,sp
                           .data:0000eb3d 5e                               ld e,(hl)
                           .data:0000eb3e 23                               inc hl
                           .data:0000eb3f 56                               ld d,(hl)
                           .data:0000eb40 eb                               ex de,hl
                           .data:0000eb41 29                               add hl,hl
                           .data:0000eb42 e5                               push hl
                           .data:0000eb43 60                               ld h,b
                           .data:0000eb44 69                               ld l,c
                           .data:0000eb45 29                               add hl,hl
                           .data:0000eb46 29                               add hl,hl
                           .data:0000eb47 d1                               pop de
                           .data:0000eb48 19                               add hl,de
                           .data:0000eb49 11 60 04                         ld de,0x0460
                           .data:0000eb4c 19                               add hl,de
                           .data:0000eb4d 5e                               ld e,(hl)
                           .data:0000eb4e 23                               inc hl
                           .data:0000eb4f 56                               ld d,(hl)
                           .data:0000eb50 eb                               ex de,hl
                           .data:0000eb51 22 ac 04                         ld (0x04ac),hl
                           .data:0000eb54 e5                               push hl
                           .data:0000eb55 60                               ld h,b
                           .data:0000eb56 69                               ld l,c
                           .data:0000eb57 29                               add hl,hl
                           .data:0000eb58 e5                               push hl
                           .data:0000eb59 2a a8 04                         ld hl,(0x04a8)
                           .data:0000eb5c 29                               add hl,hl
                           .data:0000eb5d 29                               add hl,hl
                           .data:0000eb5e d1                               pop de
                           .data:0000eb5f 19                               add hl,de
                           .data:0000eb60 11 7a 04                         ld de,0x047a
                           .data:0000eb63 19                               add hl,de
                           .data:0000eb64 5e                               ld e,(hl)
                           .data:0000eb65 23                               inc hl
                           .data:0000eb66 56                               ld d,(hl)
                           .data:0000eb67 e1                               pop hl
                           .data:0000eb68 cd 18 fc                         call 0xfc18
                           .data:0000eb6b cd cb f5                         call 0xf5cb
                           .data:0000eb6e cd 7b f2                         call 0xf27b
                           .data:0000eb71 cd e4 f4                         call 0xf4e4
                           .data:0000eb74 cd 7b f2                         call 0xf27b
                           .data:0000eb77 cd e4 f3                         call 0xf3e4
                           .data:0000eb7a cd 4a f2                         call 0xf24a
                           .data:0000eb7d cd c8 e6                         call 0xe6c8
                           .data:0000eb80 eb                               ex de,hl
                           .data:0000eb81 cd 87 ff                         call 0xff87
                           .data:0000eb84 d5                               push de
                           .data:0000eb85 21 01 00                         ld hl,0x0001
                           .data:0000eb88 50                               ld d,b
                           .data:0000eb89 59                               ld e,c
                           .data:0000eb8a eb                               ex de,hl
                           .data:0000eb8b cd 18 fc                         call 0xfc18
                           .data:0000eb8e 29                               add hl,hl
                           .data:0000eb8f e5                               push hl
                           .data:0000eb90 2a a8 04                         ld hl,(0x04a8)
                           .data:0000eb93 29                               add hl,hl
                           .data:0000eb94 29                               add hl,hl
                           .data:0000eb95 d1                               pop de
                           .data:0000eb96 19                               add hl,de
                           .data:0000eb97 11 7a 04                         ld de,0x047a
                           .data:0000eb9a 19                               add hl,de
                           .data:0000eb9b d1                               pop de
                           .data:0000eb9c e5                               push hl
                           .data:0000eb9d 7e                               ld a,(hl)
                           .data:0000eb9e 23                               inc hl
                           .data:0000eb9f 66                               ld h,(hl)
                           .data:0000eba0 6f                               ld l,a
                           .data:0000eba1 19                               add hl,de
                           .data:0000eba2 eb                               ex de,hl
                           .data:0000eba3 e1                               pop hl
                           .data:0000eba4 73                               ld (hl),e
                           .data:0000eba5 23                               inc hl
                           .data:0000eba6 72                               ld (hl),d
                           .data:0000eba7 2a ac 04                         ld hl,(0x04ac)
                           .data:0000ebaa e5                               push hl
                           .data:0000ebab 60                               ld h,b
                           .data:0000ebac 69                               ld l,c
                           .data:0000ebad 29                               add hl,hl
                           .data:0000ebae e5                               push hl
                           .data:0000ebaf 2a a8 04                         ld hl,(0x04a8)
                           .data:0000ebb2 29                               add hl,hl
                           .data:0000ebb3 29                               add hl,hl
                           .data:0000ebb4 d1                               pop de
                           .data:0000ebb5 19                               add hl,de
                           .data:0000ebb6 11 7a 04                         ld de,0x047a
                           .data:0000ebb9 19                               add hl,de
                           .data:0000ebba d1                               pop de
                           .data:0000ebbb 73                               ld (hl),e
                           .data:0000ebbc 23                               inc hl
                           .data:0000ebbd 72                               ld (hl),d
                           .data:0000ebbe 60                               ld h,b
                           .data:0000ebbf 69                               ld l,c
                           .data:0000ebc0 29                               add hl,hl
                           .data:0000ebc1 eb                               ex de,hl
                           .data:0000ebc2 21 0c 00                         ld hl,0x000c
                           .data:0000ebc5 39                               add hl,sp
                           .data:0000ebc6 7e                               ld a,(hl)
                           .data:0000ebc7 23                               inc hl
                           .data:0000ebc8 66                               ld h,(hl)
                           .data:0000ebc9 6f                               ld l,a
                           .data:0000ebca 19                               add hl,de
                           .data:0000ebcb 22 a6 04                         ld (0x04a6),hl
                           .data:0000ebce eb                               ex de,hl
                           .data:0000ebcf 2a 84 04                         ld hl,(0x0484)
                           .data:0000ebd2 cd 24 fb                         call 0xfb24
                           .data:0000ebd5 28 06                            jr z,0xebdd
                           .data:0000ebd7 21 01 00                         ld hl,0x0001
                           .data:0000ebda 22 82 04                         ld (0x0482),hl
                           .data:0000ebdd 2a a6 04                         ld hl,(0x04a6)
                           .data:0000ebe0 eb                               ex de,hl
                           .data:0000ebe1 2a 86 04                         ld hl,(0x0486)
                           .data:0000ebe4 cd 24 fb                         call 0xfb24
                           .data:0000ebe7 28 06                            jr z,0xebef
                           .data:0000ebe9 21 01 00                         ld hl,0x0001
                           .data:0000ebec 22 82 04                         ld (0x0482),hl
                           .data:0000ebef 2a 84 04                         ld hl,(0x0484)
                           .data:0000ebf2 22 86 04                         ld (0x0486),hl
                           .data:0000ebf5 2a a6 04                         ld hl,(0x04a6)
                           .data:0000ebf8 22 84 04                         ld (0x0484),hl
                           .data:0000ebfb 2a a6 04                         ld hl,(0x04a6)
                           .data:0000ebfe c9                               ret
                           .data:0000ebff cd 90 ff                         call 0xff90
                           .data:0000ec02 2a ff 01                         ld hl,(0x01ff)
                           .data:0000ec05 11 01 00                         ld de,0x0001
                           .data:0000ec08 cd 24 fb                         call 0xfb24
                           .data:0000ec0b 28 07                            jr z,0xec14
                           .data:0000ec0d 2a 78 04                         ld hl,(0x0478)
                           .data:0000ec10 7c                               ld a,h
                           .data:0000ec11 b5                               or l
                           .data:0000ec12 20 04                            jr nz,0xec18
                           .data:0000ec14 21 00 00                         ld hl,0x0000
                           .data:0000ec17 c9                               ret
                           .data:0000ec18 21 01 00                         ld hl,0x0001
                           .data:0000ec1b c9                               ret
                           .data:0000ec1c cd 90 ff                         call 0xff90
                           .data:0000ec1f 2a ff 01                         ld hl,(0x01ff)
                           .data:0000ec22 11 02 00                         ld de,0x0002
                           .data:0000ec25 cd 24 fb                         call 0xfb24
                           .data:0000ec28 28 1a                            jr z,0xec44
                           .data:0000ec2a 21 0a 00                         ld hl,0x000a
                           .data:0000ec2d 39                               add hl,sp
                           .data:0000ec2e 5e                               ld e,(hl)
                           .data:0000ec2f 23                               inc hl
                           .data:0000ec30 56                               ld d,(hl)
                           .data:0000ec31 d5                               push de
                           .data:0000ec32 21 0a 00                         ld hl,0x000a
                           .data:0000ec35 39                               add hl,sp
                           .data:0000ec36 5e                               ld e,(hl)
                           .data:0000ec37 23                               inc hl
                           .data:0000ec38 56                               ld d,(hl)
                           .data:0000ec39 d5                               push de
                           .data:0000ec3a cd b0 ec                         call 0xecb0
                           .data:0000ec3d d1                               pop de
                           .data:0000ec3e d1                               pop de
                           .data:0000ec3f 28 03                            jr z,0xec44
                           .data:0000ec41 cd 51 04                         call 0x0451
                           .data:0000ec44 c9                               ret
                           .data:0000ec45 cd 90 ff                         call 0xff90
                           .data:0000ec48 21 08 00                         ld hl,0x0008
                           .data:0000ec4b 39                               add hl,sp
                           .data:0000ec4c cd 09 f2                         call 0xf209
                           .data:0000ec4f 21 c7 01                         ld hl,0x01c7
                           .data:0000ec52 cd fb f1                         call 0xf1fb
                           .data:0000ec55 cd aa f5                         call 0xf5aa
                           .data:0000ec58 28 1c                            jr z,0xec76
                           .data:0000ec5a 21 08 00                         ld hl,0x0008
                           .data:0000ec5d 39                               add hl,sp
                           .data:0000ec5e e5                               push hl
                           .data:0000ec5f 21 c7 01                         ld hl,0x01c7
                           .data:0000ec62 cd 09 f2                         call 0xf209
                           .data:0000ec65 cd a6 f2                         call 0xf2a6
                           .data:0000ec68 e1                               pop hl
                           .data:0000ec69 e5                               push hl
                           .data:0000ec6a cd 09 f2                         call 0xf209
                           .data:0000ec6d cd 08 f3                         call 0xf308
                           .data:0000ec70 e1                               pop hl
                           .data:0000ec71 cd 28 f2                         call 0xf228
                           .data:0000ec74 18 d2                            jr 0xec48
                           .data:0000ec76 21 08 00                         ld hl,0x0008
                           .data:0000ec79 39                               add hl,sp
                           .data:0000ec7a cd 09 f2                         call 0xf209
                           .data:0000ec7d cd f4 f1                         call 0xf1f4
                           .data:0000ec80 00                               nop
                           .data:0000ec81 00                               nop
                           .data:0000ec82 00                               nop
                           .data:0000ec83 00                               nop
                           .data:0000ec84 00                               nop
                           .data:0000ec85 00                               nop
                           .data:0000ec86 00                               nop
                           .data:0000ec87 00                               nop
                           .data:0000ec88 cd 95 f5                         call 0xf595
                           .data:0000ec8b 28 1c                            jr z,0xeca9
                           .data:0000ec8d 21 08 00                         ld hl,0x0008
                           .data:0000ec90 39                               add hl,sp
                           .data:0000ec91 e5                               push hl
                           .data:0000ec92 21 c7 01                         ld hl,0x01c7
                           .data:0000ec95 cd 09 f2                         call 0xf209
                           .data:0000ec98 cd a6 f2                         call 0xf2a6
                           .data:0000ec9b e1                               pop hl
                           .data:0000ec9c e5                               push hl
                           .data:0000ec9d cd 09 f2                         call 0xf209
                           .data:0000eca0 cd 0f f3                         call 0xf30f
                           .data:0000eca3 e1                               pop hl
                           .data:0000eca4 cd 28 f2                         call 0xf228
                           .data:0000eca7 18 cd                            jr 0xec76
                           .data:0000eca9 21 08 00                         ld hl,0x0008
                           .data:0000ecac 39                               add hl,sp
                           .data:0000ecad c3 09 f2                         jp 0xf209
                           .data:0000ecb0 c5                               push bc
                           .data:0000ecb1 21 04 00                         ld hl,0x0004
                           .data:0000ecb4 39                               add hl,sp
                           .data:0000ecb5 5e                               ld e,(hl)
                           .data:0000ecb6 23                               inc hl
                           .data:0000ecb7 56                               ld d,(hl)
                           .data:0000ecb8 23                               inc hl
                           .data:0000ecb9 4e                               ld c,(hl)
                           .data:0000ecba 23                               inc hl
                           .data:0000ecbb 46                               ld b,(hl)
                           .data:0000ecbc 2a 66 04                         ld hl,(0x0466)
                           .data:0000ecbf 7c                               ld a,h
                           .data:0000ecc0 17                               rla
                           .data:0000ecc1 38 11                            jr c,0xecd4
                           .data:0000ecc3 ed 42                            sbc hl,bc
                           .data:0000ecc5 38 0d                            jr c,0xecd4
                           .data:0000ecc7 2a 62 04                         ld hl,(0x0462)
                           .data:0000ecca ed 52                            sbc hl,de
                           .data:0000eccc 38 06                            jr c,0xecd4
                           .data:0000ecce c1                               pop bc
                           .data:0000eccf 21 00 00                         ld hl,0x0000
                           .data:0000ecd2 a5                               and l
                           .data:0000ecd3 c9                               ret
                           .data:0000ecd4 c1                               pop bc
                           .data:0000ecd5 21 01 00                         ld hl,0x0001
                           .data:0000ecd8 b5                               or l
                           .data:0000ecd9 c9                               ret
                           .data:0000ecda e1                               pop hl
                           .data:0000ecdb 22 ae 04                         ld (0x04ae),hl
                           .data:0000ecde cd b0 ec                         call 0xecb0
                           .data:0000ece1 20 03                            jr nz,0xece6
                           .data:0000ece3 cd 66 ee                         call 0xee66
                           .data:0000ece6 2a ae 04                         ld hl,(0x04ae)
                           .data:0000ece9 e9                               jp (hl)
                           .data:0000ecea 3a 49 00                         ld a,(0x0049)
                           .data:0000eced cd 9b be                         call 0xbe9b
                           .data:0000ecf0 0e bc                            ld c,0xbc
                           .data:0000ecf2 3a 4a 00                         ld a,(0x004a)
                           .data:0000ecf5 cd 9b be                         call 0xbe9b
                           .data:0000ecf8 9e                               sbc a,(hl)
                           .data:0000ecf9 be                               cp (hl)
                           .data:0000ecfa c7                               rst 0x00
                           .data:0000ecfb af                               xor a
                           .data:0000ecfc 32 48 00                         ld (0x0048),a
                           .data:0000ecff 3d                               dec a
                           .data:0000ed00 32 40 00                         ld (0x0040),a
                           .data:0000ed03 cd 9b be                         call 0xbe9b
                           .data:0000ed06 11 bc 32                         ld de,0x32bc
                           .data:0000ed09 49                               ld c,c
                           .data:0000ed0a 00                               nop
                           .data:0000ed0b af                               xor a
                           .data:0000ed0c cd 9b be                         call 0xbe9b
                           .data:0000ed0f 9e                               sbc a,(hl)
                           .data:0000ed10 be                               cp (hl)
                           .data:0000ed11 32 4a 00                         ld (0x004a),a
                           .data:0000ed14 c9                               ret
                           .data:0000ed15 c5                               push bc
                           .data:0000ed16 3a b8 01                         ld a,(0x01b8)
                           .data:0000ed19 cd 34 ee                         call 0xee34
                           .data:0000ed1c 3e 81                            ld a,0x81
                           .data:0000ed1e 32 40 00                         ld (0x0040),a
                           .data:0000ed21 cd 50 ee                         call 0xee50
                           .data:0000ed24 c1                               pop bc
                           .data:0000ed25 c9                               ret
                           .data:0000ed26 c5                               push bc
                           .data:0000ed27 21 04 00                         ld hl,0x0004
                           .data:0000ed2a 39                               add hl,sp
                           .data:0000ed2b 3a 40 00                         ld a,(0x0040)
                           .data:0000ed2e be                               cp (hl)
                           .data:0000ed2f ca ae ed                         jp z,0xedae
                           .data:0000ed32 e5                               push hl
                           .data:0000ed33 f5                               push af
                           .data:0000ed34 cd 43 ee                         call 0xee43
                           .data:0000ed37 f1                               pop af
                           .data:0000ed38 b7                               or a
                           .data:0000ed39 3a b7 01                         ld a,(0x01b7)
                           .data:0000ed3c fc 34 ee                         call m,0xee34
                           .data:0000ed3f cd 9b be                         call 0xbe9b
                           .data:0000ed42 78                               ld a,b
                           .data:0000ed43 bb                               cp e
                           .data:0000ed44 3a 40 00                         ld a,(0x0040)
                           .data:0000ed47 e6 7f                            and 0x7f
                           .data:0000ed49 3d                               dec a
                           .data:0000ed4a 85                               add a,l
                           .data:0000ed4b 6f                               ld l,a
                           .data:0000ed4c e3                               ex (sp),hl
                           .data:0000ed4d 7e                               ld a,(hl)
                           .data:0000ed4e 32 40 00                         ld (0x0040),a
                           .data:0000ed51 d6 01                            sub 0x01
                           .data:0000ed53 6f                               ld l,a
                           .data:0000ed54 d2 59 ed                         jp nc,0xed59
                           .data:0000ed57 2e 18                            ld l,0x18
                           .data:0000ed59 3a 47 00                         ld a,(0x0047)
                           .data:0000ed5c 67                               ld h,a
                           .data:0000ed5d 16 00                            ld d,0x00
                           .data:0000ed5f 1e 18                            ld e,0x18
                           .data:0000ed61 cd 9b be                         call 0xbe9b
                           .data:0000ed64 66                               ld h,(hl)
                           .data:0000ed65 bb                               cp e
                           .data:0000ed66 e1                               pop hl
                           .data:0000ed67 cd 24 ee                         call 0xee24
                           .data:0000ed6a 21 c7 00                         ld hl,0x00c7
                           .data:0000ed6d 22 4b 00                         ld (0x004b),hl
                           .data:0000ed70 21 00 00                         ld hl,0x0000
                           .data:0000ed73 11 8f 01                         ld de,0x018f
                           .data:0000ed76 3a 40 00                         ld a,(0x0040)
                           .data:0000ed79 b7                               or a
                           .data:0000ed7a ca 8c ed                         jp z,0xed8c
                           .data:0000ed7d 6f                               ld l,a
                           .data:0000ed7e 29                               add hl,hl
                           .data:0000ed7f 29                               add hl,hl
                           .data:0000ed80 29                               add hl,hl
                           .data:0000ed81 2b                               dec hl
                           .data:0000ed82 22 4b 00                         ld (0x004b),hl
                           .data:0000ed85 23                               inc hl
                           .data:0000ed86 29                               add hl,hl
                           .data:0000ed87 2b                               dec hl
                           .data:0000ed88 cd c5 f0                         call 0xf0c5
                           .data:0000ed8b 19                               add hl,de
                           .data:0000ed8c 22 4d 00                         ld (0x004d),hl
                           .data:0000ed8f cd 9b be                         call 0xbe9b
                           .data:0000ed92 d2 bb 2a                         jp nc,0x2abb
                           .data:0000ed95 4b                               ld c,e
                           .data:0000ed96 00                               nop
                           .data:0000ed97 23                               inc hl
                           .data:0000ed98 cd c5 f0                         call 0xf0c5
                           .data:0000ed9b 11 90 01                         ld de,0x0190
                           .data:0000ed9e 19                               add hl,de
                           .data:0000ed9f 11 40 01                         ld de,0x0140
                           .data:0000eda2 cd 9b be                         call 0xbe9b
                           .data:0000eda5 c9                               ret
                           .data:0000eda6 bb                               cp e
                           .data:0000eda7 3a 40 00                         ld a,(0x0040)
                           .data:0000edaa b7                               or a
                           .data:0000edab c4 50 ee                         call nz,0xee50
                           .data:0000edae c1                               pop bc
                           .data:0000edaf c9                               ret
                           .data:0000edb0 c5                               push bc
                           .data:0000edb1 cd 43 ee                         call 0xee43
                           .data:0000edb4 cd 5d ee                         call 0xee5d
                           .data:0000edb7 cd fd ed                         call 0xedfd
                           .data:0000edba da ce ed                         jp c,0xedce
                           .data:0000edbd c2 c8 ed                         jp nz,0xedc8
                           .data:0000edc0 cd 9b be                         call 0xbe9b
                           .data:0000edc3 6c                               ld l,h
                           .data:0000edc4 bb                               cp e
                           .data:0000edc5 c3 ce ed                         jp 0xedce
                           .data:0000edc8 78                               ld a,b
                           .data:0000edc9 cd 9b be                         call 0xbe9b
                           .data:0000edcc 44                               ld b,h
                           .data:0000edcd bc                               cp h
                           .data:0000edce cd 50 ee                         call 0xee50
                           .data:0000edd1 c1                               pop bc
                           .data:0000edd2 c9                               ret
                           .data:0000edd3 3a 40 00                         ld a,(0x0040)
                           .data:0000edd6 b7                               or a
                           .data:0000edd7 c8                               ret z
                           .data:0000edd8 c5                               push bc
                           .data:0000edd9 cd 43 ee                         call 0xee43
                           .data:0000eddc cd 5d ee                         call 0xee5d
                           .data:0000eddf cd fd ed                         call 0xedfd
                           .data:0000ede2 da f8 ed                         jp c,0xedf8
                           .data:0000ede5 78                               ld a,b
                           .data:0000ede6 06 ff                            ld b,0xff
                           .data:0000ede8 c2 f3 ed                         jp nz,0xedf3
                           .data:0000edeb cd 9b be                         call 0xbe9b
                           .data:0000edee 4d                               ld c,l
                           .data:0000edef bc                               cp h
                           .data:0000edf0 c3 f8 ed                         jp 0xedf8
                           .data:0000edf3 cd 9b be                         call 0xbe9b
                           .data:0000edf6 50                               ld d,b
                           .data:0000edf7 bc                               cp h
                           .data:0000edf8 cd 50 ee                         call 0xee50
                           .data:0000edfb c1                               pop bc
                           .data:0000edfc c9                               ret
                           .data:0000edfd cd 9b be                         call 0xbe9b
                           .data:0000ee00 99                               sbc a,c
                           .data:0000ee01 bb                               cp e
                           .data:0000ee02 cd 9b be                         call 0xbe9b
                           .data:0000ee05 2c                               inc l
                           .data:0000ee06 bc                               cp h
                           .data:0000ee07 47                               ld b,a
                           .data:0000ee08 1d                               dec e
                           .data:0000ee09 2d                               dec l
                           .data:0000ee0a 26 00                            ld h,0x00
                           .data:0000ee0c 3a 47 00                         ld a,(0x0047)
                           .data:0000ee0f 57                               ld d,a
                           .data:0000ee10 3e 18                            ld a,0x18
                           .data:0000ee12 bd                               cp l
                           .data:0000ee13 d8                               ret c
                           .data:0000ee14 bb                               cp e
                           .data:0000ee15 d8                               ret c
                           .data:0000ee16 7b                               ld a,e
                           .data:0000ee17 ee 18                            xor 0x18
                           .data:0000ee19 c0                               ret nz
                           .data:0000ee1a 7d                               ld a,l
                           .data:0000ee1b b7                               or a
                           .data:0000ee1c c9                               ret
                           .data:0000ee1d c5                               push bc
                           .data:0000ee1e cd 5d ee                         call 0xee5d
                           .data:0000ee21 c1                               pop bc
                           .data:0000ee22 65                               ld h,l
                           .data:0000ee23 6b                               ld l,e
                           .data:0000ee24 3a 40 00                         ld a,(0x0040)
                           .data:0000ee27 e6 7f                            and 0x7f
                           .data:0000ee29 3d                               dec a
                           .data:0000ee2a 5f                               ld e,a
                           .data:0000ee2b 7d                               ld a,l
                           .data:0000ee2c 93                               sub e
                           .data:0000ee2d 6f                               ld l,a
                           .data:0000ee2e cd 9b be                         call 0xbe9b
                           .data:0000ee31 75                               ld (hl),l
                           .data:0000ee32 bb                               cp e
                           .data:0000ee33 c9                               ret
                           .data:0000ee34 cd 9b be                         call 0xbe9b
                           .data:0000ee37 0e bc                            ld c,0xbc
                           .data:0000ee39 cd 9b be                         call 0xbe9b
                           .data:0000ee3c 69                               ld l,c
                           .data:0000ee3d bb                               cp e
                           .data:0000ee3e 7a                               ld a,d
                           .data:0000ee3f 32 47 00                         ld (0x0047),a
                           .data:0000ee42 c9                               ret
                           .data:0000ee43 21 48 00                         ld hl,0x0048
                           .data:0000ee46 7e                               ld a,(hl)
                           .data:0000ee47 b7                               or a
                           .data:0000ee48 c0                               ret nz
                           .data:0000ee49 35                               dec (hl)
                           .data:0000ee4a cd 9b be                         call 0xbe9b
                           .data:0000ee4d 7e                               ld a,(hl)
                           .data:0000ee4e bb                               cp e
                           .data:0000ee4f c9                               ret
                           .data:0000ee50 21 48 00                         ld hl,0x0048
                           .data:0000ee53 7e                               ld a,(hl)
                           .data:0000ee54 b7                               or a
                           .data:0000ee55 c8                               ret z
                           .data:0000ee56 34                               inc (hl)
                           .data:0000ee57 cd 9b be                         call 0xbe9b
                           .data:0000ee5a 7b                               ld a,e
                           .data:0000ee5b bb                               cp e
                           .data:0000ee5c c9                               ret
                           .data:0000ee5d 21 08 00                         ld hl,0x0008
                           .data:0000ee60 39                               add hl,sp
                           .data:0000ee61 5e                               ld e,(hl)
                           .data:0000ee62 2b                               dec hl
                           .data:0000ee63 2b                               dec hl
                           .data:0000ee64 6e                               ld l,(hl)
                           .data:0000ee65 c9                               ret
                           .data:0000ee66 c5                               push bc
                           .data:0000ee67 21 09 00                         ld hl,0x0009
                           .data:0000ee6a 39                               add hl,sp
                           .data:0000ee6b cd d6 ee                         call 0xeed6
                           .data:0000ee6e cd f3 ee                         call 0xeef3
                           .data:0000ee71 d5                               push de
                           .data:0000ee72 cd 02 ef                         call 0xef02
                           .data:0000ee75 e1                               pop hl
                           .data:0000ee76 cd 9b be                         call 0xbe9b
                           .data:0000ee79 ea bb c1                         jp pe,0xc1bb
                           .data:0000ee7c c9                               ret
                           .data:0000ee7d c5                               push bc
                           .data:0000ee7e 21 0d 00                         ld hl,0x000d
                           .data:0000ee81 39                               add hl,sp
                           .data:0000ee82 cd d6 ee                         call 0xeed6
                           .data:0000ee85 cd f3 ee                         call 0xeef3
                           .data:0000ee88 d5                               push de
                           .data:0000ee89 cd 02 ef                         call 0xef02
                           .data:0000ee8c e3                               ex (sp),hl
                           .data:0000ee8d cd 9b be                         call 0xbe9b
                           .data:0000ee90 c0                               ret nz
                           .data:0000ee91 bb                               cp e
                           .data:0000ee92 e1                               pop hl
                           .data:0000ee93 cd f3 ee                         call 0xeef3
                           .data:0000ee96 d5                               push de
                           .data:0000ee97 cd 02 ef                         call 0xef02
                           .data:0000ee9a e1                               pop hl
                           .data:0000ee9b cd 9b be                         call 0xbe9b
                           .data:0000ee9e f6 bb                            or 0xbb
                           .data:0000eea0 c1                               pop bc
                           .data:0000eea1 c9                               ret
                           .data:0000eea2 c5                               push bc
                           .data:0000eea3 21 07 00                         ld hl,0x0007
                           .data:0000eea6 39                               add hl,sp
                           .data:0000eea7 cd 0d ef                         call 0xef0d
                           .data:0000eeaa eb                               ex de,hl
                           .data:0000eeab cd c5 f0                         call 0xf0c5
                           .data:0000eeae 11 8f 01                         ld de,0x018f
                           .data:0000eeb1 d5                               push de
                           .data:0000eeb2 19                               add hl,de
                           .data:0000eeb3 eb                               ex de,hl
                           .data:0000eeb4 2a 4d 00                         ld hl,(0x004d)
                           .data:0000eeb7 eb                               ex de,hl
                           .data:0000eeb8 cd b9 f0                         call 0xf0b9
                           .data:0000eebb d2 bf ee                         jp nc,0xeebf
                           .data:0000eebe eb                               ex de,hl
                           .data:0000eebf d1                               pop de
                           .data:0000eec0 d5                               push de
                           .data:0000eec1 cd 9b be                         call 0xbe9b
                           .data:0000eec4 d2 bb cd                         jp nc,0xcdbb
                           .data:0000eec7 9b                               sbc a,e
                           .data:0000eec8 be                               cp (hl)
                           .data:0000eec9 db bb                            in a,(0xbb)
                           .data:0000eecb d1                               pop de
                           .data:0000eecc 2a 4d 00                         ld hl,(0x004d)
                           .data:0000eecf cd 9b be                         call 0xbe9b
                           .data:0000eed2 d2 bb c1                         jp nc,0xc1bb
                           .data:0000eed5 c9                               ret
                           .data:0000eed6 cd 0d ef                         call 0xef0d
                           .data:0000eed9 7a                               ld a,d
                           .data:0000eeda 3d                               dec a
                           .data:0000eedb fe 01                            cp 0x01
                           .data:0000eedd ca e1 ee                         jp z,0xeee1
                           .data:0000eee0 af                               xor a
                           .data:0000eee1 e5                               push hl
                           .data:0000eee2 cd 9b be                         call 0xbe9b
                           .data:0000eee5 59                               ld e,c
                           .data:0000eee6 bc                               cp h
                           .data:0000eee7 e1                               pop hl
                           .data:0000eee8 7b                               ld a,e
                           .data:0000eee9 cd 9b be                         call 0xbe9b
                           .data:0000eeec de bb                            sbc a,0xbb
                           .data:0000eeee c9                               ret
                           .data:0000eeef 21 00 00                         ld hl,0x0000
                           .data:0000eef2 c9                               ret
                           .data:0000eef3 cd 0d ef                         call 0xef0d
                           .data:0000eef6 e5                               push hl
                           .data:0000eef7 2a 4b 00                         ld hl,(0x004b)
                           .data:0000eefa eb                               ex de,hl
                           .data:0000eefb cd c5 f0                         call 0xf0c5
                           .data:0000eefe 19                               add hl,de
                           .data:0000eeff eb                               ex de,hl
                           .data:0000ef00 e1                               pop hl
                           .data:0000ef01 c9                               ret
                           .data:0000ef02 cd 0d ef                         call 0xef0d
                           .data:0000ef05 e5                               push hl
                           .data:0000ef06 21 c0 fe                         ld hl,0xfec0
                           .data:0000ef09 19                               add hl,de
                           .data:0000ef0a eb                               ex de,hl
                           .data:0000ef0b e1                               pop hl
                           .data:0000ef0c c9                               ret
                           .data:0000ef0d 56                               ld d,(hl)
                           .data:0000ef0e 2b                               dec hl
                           .data:0000ef0f 5e                               ld e,(hl)
                           .data:0000ef10 2b                               dec hl
                           .data:0000ef11 7b                               ld a,e
                           .data:0000ef12 c9                               ret
                           .data:0000ef13 c5                               push bc
                           .data:0000ef14 21 0b 00                         ld hl,0x000b
                           .data:0000ef17 39                               add hl,sp
                           .data:0000ef18 cd d6 ee                         call 0xeed6
                           .data:0000ef1b cd 0d ef                         call 0xef0d
                           .data:0000ef1e d5                               push de
                           .data:0000ef1f cd f3 ee                         call 0xeef3
                           .data:0000ef22 d5                               push de
                           .data:0000ef23 cd 02 ef                         call 0xef02
                           .data:0000ef26 e1                               pop hl
                           .data:0000ef27 cd 9b be                         call 0xbe9b
                           .data:0000ef2a c0                               ret nz
                           .data:0000ef2b bb                               cp e
                           .data:0000ef2c 3e ff                            ld a,0xff
                           .data:0000ef2e cd 9b be                         call 0xbe9b
                           .data:0000ef31 63                               ld h,e
                           .data:0000ef32 bb                               cp e
                           .data:0000ef33 e1                               pop hl
                           .data:0000ef34 7e                               ld a,(hl)
                           .data:0000ef35 23                               inc hl
                           .data:0000ef36 b7                               or a
                           .data:0000ef37 c4 5a bb                         call nz,0xbb5a
                           .data:0000ef3a c2 34 ef                         jp nz,0xef34
                           .data:0000ef3d af                               xor a
                           .data:0000ef3e cd 9b be                         call 0xbe9b
                           .data:0000ef41 63                               ld h,e
                           .data:0000ef42 bb                               cp e
                           .data:0000ef43 c1                               pop bc
                           .data:0000ef44 c9                               ret
                           .data:0000ef45 c9                               ret
                           .data:0000ef46 c5                               push bc
                           .data:0000ef47 21 0b 00                         ld hl,0x000b
                           .data:0000ef4a 39                               add hl,sp
                           .data:0000ef4b cd 6c ef                         call 0xef6c
                           .data:0000ef4e 47                               ld b,a
                           .data:0000ef4f cd 6c ef                         call 0xef6c
                           .data:0000ef52 4f                               ld c,a
                           .data:0000ef53 87                               add a,a
                           .data:0000ef54 87                               add a,a
                           .data:0000ef55 87                               add a,a
                           .data:0000ef56 81                               add a,c
                           .data:0000ef57 80                               add a,b
                           .data:0000ef58 47                               ld b,a
                           .data:0000ef59 cd 6c ef                         call 0xef6c
                           .data:0000ef5c 4f                               ld c,a
                           .data:0000ef5d 87                               add a,a
                           .data:0000ef5e 81                               add a,c
                           .data:0000ef5f 80                               add a,b
                           .data:0000ef60 47                               ld b,a
                           .data:0000ef61 4f                               ld c,a
                           .data:0000ef62 cd 0d ef                         call 0xef0d
                           .data:0000ef65 cd 9b be                         call 0xbe9b
                           .data:0000ef68 32 bc c1                         ld (0xc1bc),a
                           .data:0000ef6b c9                               ret
                           .data:0000ef6c cd 0d ef                         call 0xef0d
                           .data:0000ef6f e6 03                            and 0x03
                           .data:0000ef71 fe 03                            cp 0x03
                           .data:0000ef73 d8                               ret c
                           .data:0000ef74 af                               xor a
                           .data:0000ef75 c9                               ret
                           .data:0000ef76 c5                               push bc
                           .data:0000ef77 21 05 00                         ld hl,0x0005
                           .data:0000ef7a 39                               add hl,sp
                           .data:0000ef7b cd 0d ef                         call 0xef0d
                           .data:0000ef7e cd 9b be                         call 0xbe9b
                           .data:0000ef81 35                               dec (hl)
                           .data:0000ef82 bc                               cp h
                           .data:0000ef83 21 41 00                         ld hl,0x0041
                           .data:0000ef86 3e 06                            ld a,0x06
                           .data:0000ef88 36 00                            ld (hl),0x00
                           .data:0000ef8a 23                               inc hl
                           .data:0000ef8b 3d                               dec a
                           .data:0000ef8c c2 88 ef                         jp nz,0xef88
                           .data:0000ef8f 68                               ld l,b
                           .data:0000ef90 26 09                            ld h,0x09
                           .data:0000ef92 cd cf f0                         call 0xf0cf
                           .data:0000ef95 32 43 00                         ld (0x0043),a
                           .data:0000ef98 26 03                            ld h,0x03
                           .data:0000ef9a cd cf f0                         call 0xf0cf
                           .data:0000ef9d 32 41 00                         ld (0x0041),a
                           .data:0000efa0 7d                               ld a,l
                           .data:0000efa1 32 45 00                         ld (0x0045),a
                           .data:0000efa4 21 41 00                         ld hl,0x0041
                           .data:0000efa7 c1                               pop bc
                           .data:0000efa8 c9                               ret
                           .data:0000efa9 11 07 00                         ld de,0x0007
                           .data:0000efac 21 04 00                         ld hl,0x0004
                           .data:0000efaf cd de ef                         call 0xefde
                           .data:0000efb2 26 00                            ld h,0x00
                           .data:0000efb4 c9                               ret
                           .data:0000efb5 11 04 00                         ld de,0x0004
                           .data:0000efb8 c3 db ef                         jp 0xefdb
                           .data:0000efbb 3a 40 00                         ld a,(0x0040)
                           .data:0000efbe b7                               or a
                           .data:0000efbf c8                               ret z
                           .data:0000efc0 11 0a 00                         ld de,0x000a
                           .data:0000efc3 c3 db ef                         jp 0xefdb
                           .data:0000efc6 2e 07                            ld l,0x07
                           .data:0000efc8 e5                               push hl
                           .data:0000efc9 cd bb ef                         call 0xefbb
                           .data:0000efcc e1                               pop hl
                           .data:0000efcd c9                               ret
                           .data:0000efce 2a 06 00                         ld hl,(0x0006)
                           .data:0000efd1 c9                               ret
                           .data:0000efd2 11 2b 00                         ld de,0x002b
                           .data:0000efd5 c3 db ef                         jp 0xefdb
                           .data:0000efd8 11 0d 00                         ld de,0x000d
                           .data:0000efdb 21 02 00                         ld hl,0x0002
                           .data:0000efde 39                               add hl,sp
                           .data:0000efdf c5                               push bc
                           .data:0000efe0 4e                               ld c,(hl)
                           .data:0000efe1 2a 01 00                         ld hl,(0x0001)
                           .data:0000efe4 19                               add hl,de
                           .data:0000efe5 7e                               ld a,(hl)
                           .data:0000efe6 23                               inc hl
                           .data:0000efe7 66                               ld h,(hl)
                           .data:0000efe8 6f                               ld l,a
                           .data:0000efe9 cd f1 ef                         call 0xeff1
                           .data:0000efec 6f                               ld l,a
                           .data:0000efed 67                               ld h,a
                           .data:0000efee b7                               or a
                           .data:0000efef c1                               pop bc
                           .data:0000eff0 c9                               ret
                           .data:0000eff1 e9                               jp (hl)
                           .data:0000eff2 21 02 00                         ld hl,0x0002
                           .data:0000eff5 39                               add hl,sp
                           .data:0000eff6 cd 25 f0                         call 0xf025
                           .data:0000eff9 21 06 f0                         ld hl,0xf006
                           .data:0000effc e6 0f                            and 0x0f
                           .data:0000effe 5f                               ld e,a
                           .data:0000efff 16 00                            ld d,0x00
                           .data:0000f001 19                               add hl,de
                           .data:0000f002 6e                               ld l,(hl)
                           .data:0000f003 26 00                            ld h,0x00
                           .data:0000f005 c9                               ret
                           .data:0000f006 ff                               rst 0x38
                           .data:0000f007 00                               nop
                           .data:0000f008 04                               inc b
                           .data:0000f009 ff                               rst 0x38
                           .data:0000f00a 06 07                            ld b,0x07
                           .data:0000f00c 05                               dec b
                           .data:0000f00d 06 02                            ld b,0x02
                           .data:0000f00f 01 03 02                         ld bc,0x0203
                           .data:0000f012 ff                               rst 0x38
                           .data:0000f013 00                               nop
                           .data:0000f014 04                               inc b
                           .data:0000f015 ff                               rst 0x38
                           .data:0000f016 21 02 00                         ld hl,0x0002
                           .data:0000f019 39                               add hl,sp
                           .data:0000f01a cd 25 f0                         call 0xf025
                           .data:0000f01d 21 00 00                         ld hl,0x0000
                           .data:0000f020 e6 30                            and 0x30
                           .data:0000f022 c8                               ret z
                           .data:0000f023 23                               inc hl
                           .data:0000f024 c9                               ret
                           .data:0000f025 eb                               ex de,hl
                           .data:0000f026 cd 9b be                         call 0xbe9b
                           .data:0000f029 24                               inc h
                           .data:0000f02a bb                               cp e
                           .data:0000f02b eb                               ex de,hl
                           .data:0000f02c 7e                               ld a,(hl)
                           .data:0000f02d b7                               or a
                           .data:0000f02e 7a                               ld a,d
                           .data:0000f02f c8                               ret z
                           .data:0000f030 7b                               ld a,e
                           .data:0000f031 c9                               ret
                           .data:0000f032 c5                               push bc
                           .data:0000f033 21 05 00                         ld hl,0x0005
                           .data:0000f036 39                               add hl,sp
                           .data:0000f037 cd 0d ef                         call 0xef0d
                           .data:0000f03a 21 f6 ff                         ld hl,0xfff6
                           .data:0000f03d 39                               add hl,sp
                           .data:0000f03e f9                               ld sp,hl
                           .data:0000f03f e5                               push hl
                           .data:0000f040 06 09                            ld b,0x09
                           .data:0000f042 1a                               ld a,(de)
                           .data:0000f043 77                               ld (hl),a
                           .data:0000f044 13                               inc de
                           .data:0000f045 23                               inc hl
                           .data:0000f046 05                               dec b
                           .data:0000f047 c2 42 f0                         jp nz,0xf042
                           .data:0000f04a e1                               pop hl
                           .data:0000f04b e5                               push hl
                           .data:0000f04c cd 9b be                         call 0xbe9b
                           .data:0000f04f aa                               xor d
                           .data:0000f050 bc                               cp h
                           .data:0000f051 d2 4c f0                         jp nc,0xf04c
                           .data:0000f054 e1                               pop hl
                           .data:0000f055 11 0a 00                         ld de,0x000a
                           .data:0000f058 19                               add hl,de
                           .data:0000f059 f9                               ld sp,hl
                           .data:0000f05a c1                               pop bc
                           .data:0000f05b c9                               ret
                           .data:0000f05c c5                               push bc
                           .data:0000f05d 21 05 00                         ld hl,0x0005
                           .data:0000f060 39                               add hl,sp
                           .data:0000f061 cd 0d ef                         call 0xef0d
                           .data:0000f064 21 ee ff                         ld hl,0xffee
                           .data:0000f067 39                               add hl,sp
                           .data:0000f068 f9                               ld sp,hl
                           .data:0000f069 e5                               push hl
                           .data:0000f06a 06 11                            ld b,0x11
                           .data:0000f06c 1a                               ld a,(de)
                           .data:0000f06d 77                               ld (hl),a
                           .data:0000f06e 13                               inc de
                           .data:0000f06f 23                               inc hl
                           .data:0000f070 05                               dec b
                           .data:0000f071 c2 6c f0                         jp nz,0xf06c
                           .data:0000f074 e1                               pop hl
                           .data:0000f075 e5                               push hl
                           .data:0000f076 7e                               ld a,(hl)
                           .data:0000f077 23                               inc hl
                           .data:0000f078 cd 9b be                         call 0xbe9b
                           .data:0000f07b bf                               cp a
                           .data:0000f07c bc                               cp h
                           .data:0000f07d e1                               pop hl
                           .data:0000f07e 11 12 00                         ld de,0x0012
                           .data:0000f081 19                               add hl,de
                           .data:0000f082 f9                               ld sp,hl
                           .data:0000f083 c1                               pop bc
                           .data:0000f084 c9                               ret
                           .data:0000f085 c5                               push bc
                           .data:0000f086 21 05 00                         ld hl,0x0005
                           .data:0000f089 39                               add hl,sp
                           .data:0000f08a cd 0d ef                         call 0xef0d
                           .data:0000f08d 21 ee ff                         ld hl,0xffee
                           .data:0000f090 39                               add hl,sp
                           .data:0000f091 f9                               ld sp,hl
                           .data:0000f092 e5                               push hl
                           .data:0000f093 06 11                            ld b,0x11
                           .data:0000f095 1a                               ld a,(de)
                           .data:0000f096 77                               ld (hl),a
                           .data:0000f097 13                               inc de
                           .data:0000f098 23                               inc hl
                           .data:0000f099 05                               dec b
                           .data:0000f09a c2 95 f0                         jp nz,0xf095
                           .data:0000f09d e1                               pop hl
                           .data:0000f09e e5                               push hl
                           .data:0000f09f 7e                               ld a,(hl)
                           .data:0000f0a0 23                               inc hl
                           .data:0000f0a1 cd 9b be                         call 0xbe9b
                           .data:0000f0a4 bc                               cp h
                           .data:0000f0a5 bc                               cp h
                           .data:0000f0a6 e1                               pop hl
                           .data:0000f0a7 11 12 00                         ld de,0x0012
                           .data:0000f0aa 19                               add hl,de
                           .data:0000f0ab f9                               ld sp,hl
                           .data:0000f0ac c1                               pop bc
                           .data:0000f0ad c9                               ret
                           .data:0000f0ae c5                               push bc
                           .data:0000f0af 21 04 00                         ld hl,0x0004
                           .data:0000f0b2 39                               add hl,sp
                           .data:0000f0b3 7e                               ld a,(hl)
                           .data:0000f0b4 cd b3 bc                         call 0xbcb3
                           .data:0000f0b7 c1                               pop bc
                           .data:0000f0b8 c9                               ret
                           .data:0000f0b9 c5                               push bc
                           .data:0000f0ba 47                               ld b,a
                           .data:0000f0bb 7c                               ld a,h
                           .data:0000f0bc ba                               cp d
                           .data:0000f0bd c2 c2 f0                         jp nz,0xf0c2
                           .data:0000f0c0 7d                               ld a,l
                           .data:0000f0c1 bb                               cp e
                           .data:0000f0c2 78                               ld a,b
                           .data:0000f0c3 c1                               pop bc
                           .data:0000f0c4 c9                               ret
                           .data:0000f0c5 f5                               push af
                           .data:0000f0c6 7c                               ld a,h
                           .data:0000f0c7 2f                               cpl
                           .data:0000f0c8 67                               ld h,a
                           .data:0000f0c9 7d                               ld a,l
                           .data:0000f0ca 2f                               cpl
                           .data:0000f0cb 6f                               ld l,a
                           .data:0000f0cc 23                               inc hl
                           .data:0000f0cd f1                               pop af
                           .data:0000f0ce c9                               ret
                           .data:0000f0cf 7c                               ld a,h
                           .data:0000f0d0 b7                               or a
                           .data:0000f0d1 c8                               ret z
                           .data:0000f0d2 c5                               push bc
                           .data:0000f0d3 06 08                            ld b,0x08
                           .data:0000f0d5 4c                               ld c,h
                           .data:0000f0d6 26 00                            ld h,0x00
                           .data:0000f0d8 b7                               or a
                           .data:0000f0d9 7d                               ld a,l
                           .data:0000f0da 17                               rla
                           .data:0000f0db 6f                               ld l,a
                           .data:0000f0dc 7c                               ld a,h
                           .data:0000f0dd 17                               rla
                           .data:0000f0de 67                               ld h,a
                           .data:0000f0df b9                               cp c
                           .data:0000f0e0 da e9 f0                         jp c,0xf0e9
                           .data:0000f0e3 91                               sub c
                           .data:0000f0e4 67                               ld h,a
                           .data:0000f0e5 7d                               ld a,l
                           .data:0000f0e6 f6 01                            or 0x01
                           .data:0000f0e8 6f                               ld l,a
                           .data:0000f0e9 05                               dec b
                           .data:0000f0ea c2 d8 f0                         jp nz,0xf0d8
                           .data:0000f0ed 7d                               ld a,l
                           .data:0000f0ee 6c                               ld l,h
                           .data:0000f0ef c1                               pop bc
                           .data:0000f0f0 37                               scf
                           .data:0000f0f1 c9                               ret
                           .data:0000f0f2 21 02 00                         ld hl,0x0002
                           .data:0000f0f5 39                               add hl,sp
                           .data:0000f0f6 c5                               push bc
                           .data:0000f0f7 01 ff 7f                         ld bc,0x7fff
                           .data:0000f0fa 5e                               ld e,(hl)
                           .data:0000f0fb 23                               inc hl
                           .data:0000f0fc 56                               ld d,(hl)
                           .data:0000f0fd 23                               inc hl
                           .data:0000f0fe 7e                               ld a,(hl)
                           .data:0000f0ff 23                               inc hl
                           .data:0000f100 66                               ld h,(hl)
                           .data:0000f101 6f                               ld l,a
                           .data:0000f102 78                               ld a,b
                           .data:0000f103 b1                               or c
                           .data:0000f104 ca 16 f1                         jp z,0xf116
                           .data:0000f107 1a                               ld a,(de)
                           .data:0000f108 be                               cp (hl)
                           .data:0000f109 c2 1b f1                         jp nz,0xf11b
                           .data:0000f10c b7                               or a
                           .data:0000f10d ca 16 f1                         jp z,0xf116
                           .data:0000f110 23                               inc hl
                           .data:0000f111 13                               inc de
                           .data:0000f112 0b                               dec bc
                           .data:0000f113 c3 02 f1                         jp 0xf102
                           .data:0000f116 c1                               pop bc
                           .data:0000f117 21 00 00                         ld hl,0x0000
                           .data:0000f11a c9                               ret
                           .data:0000f11b c1                               pop bc
                           .data:0000f11c da 25 f1                         jp c,0xf125
                           .data:0000f11f 21 01 00                         ld hl,0x0001
                           .data:0000f122 7d                               ld a,l
                           .data:0000f123 b4                               or h
                           .data:0000f124 c9                               ret
                           .data:0000f125 21 ff ff                         ld hl,0xffff
                           .data:0000f128 7d                               ld a,l
                           .data:0000f129 b4                               or h
                           .data:0000f12a c9                               ret
                           .data:0000f12b 21 02 00                         ld hl,0x0002
                           .data:0000f12e 39                               add hl,sp
                           .data:0000f12f c5                               push bc
                           .data:0000f130 01 ff 7f                         ld bc,0x7fff
                           .data:0000f133 5e                               ld e,(hl)
                           .data:0000f134 23                               inc hl
                           .data:0000f135 56                               ld d,(hl)
                           .data:0000f136 d5                               push de
                           .data:0000f137 23                               inc hl
                           .data:0000f138 7e                               ld a,(hl)
                           .data:0000f139 23                               inc hl
                           .data:0000f13a 66                               ld h,(hl)
                           .data:0000f13b 6f                               ld l,a
                           .data:0000f13c 78                               ld a,b
                           .data:0000f13d b1                               or c
                           .data:0000f13e ca 4d f1                         jp z,0xf14d
                           .data:0000f141 7e                               ld a,(hl)
                           .data:0000f142 12                               ld (de),a
                           .data:0000f143 b7                               or a
                           .data:0000f144 ca 4d f1                         jp z,0xf14d
                           .data:0000f147 23                               inc hl
                           .data:0000f148 13                               inc de
                           .data:0000f149 0b                               dec bc
                           .data:0000f14a c3 3c f1                         jp 0xf13c
                           .data:0000f14d e1                               pop hl
                           .data:0000f14e c1                               pop bc
                           .data:0000f14f c9                               ret
                           .data:0000f150 21 02 00                         ld hl,0x0002
                           .data:0000f153 39                               add hl,sp
                           .data:0000f154 7e                               ld a,(hl)
                           .data:0000f155 23                               inc hl
                           .data:0000f156 66                               ld h,(hl)
                           .data:0000f157 6f                               ld l,a
                           .data:0000f158 11 00 00                         ld de,0x0000
                           .data:0000f15b af                               xor a
                           .data:0000f15c be                               cp (hl)
                           .data:0000f15d ca 65 f1                         jp z,0xf165
                           .data:0000f160 13                               inc de
                           .data:0000f161 23                               inc hl
                           .data:0000f162 c3 5c f1                         jp 0xf15c
                           .data:0000f165 eb                               ex de,hl
                           .data:0000f166 7d                               ld a,l
                           .data:0000f167 b4                               or h
                           .data:0000f168 c9                               ret
                           .data:0000f169 21 02 00                         ld hl,0x0002
                           .data:0000f16c 39                               add hl,sp
                           .data:0000f16d c5                               push bc
                           .data:0000f16e 5e                               ld e,(hl)
                           .data:0000f16f 23                               inc hl
                           .data:0000f170 56                               ld d,(hl)
                           .data:0000f171 d5                               push de
                           .data:0000f172 23                               inc hl
                           .data:0000f173 5e                               ld e,(hl)
                           .data:0000f174 23                               inc hl
                           .data:0000f175 56                               ld d,(hl)
                           .data:0000f176 23                               inc hl
                           .data:0000f177 4e                               ld c,(hl)
                           .data:0000f178 23                               inc hl
                           .data:0000f179 46                               ld b,(hl)
                           .data:0000f17a eb                               ex de,hl
                           .data:0000f17b d1                               pop de
                           .data:0000f17c c3 02 f1                         jp 0xf102
                           .data:0000f17f c5                               push bc
                           .data:0000f180 01 ff 7f                         ld bc,0x7fff
                           .data:0000f183 21 04 00                         ld hl,0x0004
                           .data:0000f186 39                               add hl,sp
                           .data:0000f187 5e                               ld e,(hl)
                           .data:0000f188 23                               inc hl
                           .data:0000f189 56                               ld d,(hl)
                           .data:0000f18a d5                               push de
                           .data:0000f18b 23                               inc hl
                           .data:0000f18c 7e                               ld a,(hl)
                           .data:0000f18d 23                               inc hl
                           .data:0000f18e 66                               ld h,(hl)
                           .data:0000f18f 6f                               ld l,a
                           .data:0000f190 eb                               ex de,hl
                           .data:0000f191 78                               ld a,b
                           .data:0000f192 b1                               or c
                           .data:0000f193 ca 4d f1                         jp z,0xf14d
                           .data:0000f196 7e                               ld a,(hl)
                           .data:0000f197 b7                               or a
                           .data:0000f198 ca a0 f1                         jp z,0xf1a0
                           .data:0000f19b 23                               inc hl
                           .data:0000f19c 0b                               dec bc
                           .data:0000f19d c3 91 f1                         jp 0xf191
                           .data:0000f1a0 eb                               ex de,hl
                           .data:0000f1a1 c3 3c f1                         jp 0xf13c
                           .data:0000f1a4 d1                               pop de
                           .data:0000f1a5 e1                               pop hl
                           .data:0000f1a6 e5                               push hl
                           .data:0000f1a7 73                               ld (hl),e
                           .data:0000f1a8 23                               inc hl
                           .data:0000f1a9 72                               ld (hl),d
                           .data:0000f1aa 23                               inc hl
                           .data:0000f1ab d5                               push de
                           .data:0000f1ac eb                               ex de,hl
                           .data:0000f1ad 21 00 00                         ld hl,0x0000
                           .data:0000f1b0 39                               add hl,sp
                           .data:0000f1b1 eb                               ex de,hl
                           .data:0000f1b2 73                               ld (hl),e
                           .data:0000f1b3 23                               inc hl
                           .data:0000f1b4 72                               ld (hl),d
                           .data:0000f1b5 af                               xor a
                           .data:0000f1b6 67                               ld h,a
                           .data:0000f1b7 6f                               ld l,a
                           .data:0000f1b8 c9                               ret
                           .data:0000f1b9 e1                               pop hl
                           .data:0000f1ba e1                               pop hl
                           .data:0000f1bb 5e                               ld e,(hl)
                           .data:0000f1bc 23                               inc hl
                           .data:0000f1bd 56                               ld d,(hl)
                           .data:0000f1be 23                               inc hl
                           .data:0000f1bf d5                               push de
                           .data:0000f1c0 5e                               ld e,(hl)
                           .data:0000f1c1 23                               inc hl
                           .data:0000f1c2 56                               ld d,(hl)
                           .data:0000f1c3 e1                               pop hl
                           .data:0000f1c4 eb                               ex de,hl
                           .data:0000f1c5 73                               ld (hl),e
                           .data:0000f1c6 23                               inc hl
                           .data:0000f1c7 72                               ld (hl),d
                           .data:0000f1c8 2b                               dec hl
                           .data:0000f1c9 d1                               pop de
                           .data:0000f1ca f9                               ld sp,hl
                           .data:0000f1cb eb                               ex de,hl
                           .data:0000f1cc 7c                               ld a,h
                           .data:0000f1cd b5                               or l
                           .data:0000f1ce c9                               ret
                           .data:0000f1cf eb                               ex de,hl
                           .data:0000f1d0 2a 07 01                         ld hl,(0x0107)
                           .data:0000f1d3 c5                               push bc
                           .data:0000f1d4 1a                               ld a,(de)
                           .data:0000f1d5 77                               ld (hl),a
                           .data:0000f1d6 23                               inc hl
                           .data:0000f1d7 e6 7f                            and 0x7f
                           .data:0000f1d9 d6 40                            sub 0x40
                           .data:0000f1db 77                               ld (hl),a
                           .data:0000f1dc 23                               inc hl
                           .data:0000f1dd 36 00                            ld (hl),0x00
                           .data:0000f1df 06 03                            ld b,0x03
                           .data:0000f1e1 23                               inc hl
                           .data:0000f1e2 13                               inc de
                           .data:0000f1e3 1a                               ld a,(de)
                           .data:0000f1e4 77                               ld (hl),a
                           .data:0000f1e5 05                               dec b
                           .data:0000f1e6 c2 e1 f1                         jp nz,0xf1e1
                           .data:0000f1e9 06 05                            ld b,0x05
                           .data:0000f1eb af                               xor a
                           .data:0000f1ec 23                               inc hl
                           .data:0000f1ed 77                               ld (hl),a
                           .data:0000f1ee 05                               dec b
                           .data:0000f1ef c2 ec f1                         jp nz,0xf1ec
                           .data:0000f1f2 c1                               pop bc
                           .data:0000f1f3 c9                               ret
                           .data:0000f1f4 d1                               pop de
                           .data:0000f1f5 21 08 00                         ld hl,0x0008
                           .data:0000f1f8 19                               add hl,de
                           .data:0000f1f9 e5                               push hl
                           .data:0000f1fa eb                               ex de,hl
                           .data:0000f1fb eb                               ex de,hl
                           .data:0000f1fc 2a 09 01                         ld hl,(0x0109)
                           .data:0000f1ff c3 0d f2                         jp 0xf20d
                           .data:0000f202 d1                               pop de
                           .data:0000f203 21 08 00                         ld hl,0x0008
                           .data:0000f206 19                               add hl,de
                           .data:0000f207 e5                               push hl
                           .data:0000f208 eb                               ex de,hl
                           .data:0000f209 eb                               ex de,hl
                           .data:0000f20a 2a 07 01                         ld hl,(0x0107)
                           .data:0000f20d c5                               push bc
                           .data:0000f20e 1a                               ld a,(de)
                           .data:0000f20f 77                               ld (hl),a
                           .data:0000f210 23                               inc hl
                           .data:0000f211 e6 7f                            and 0x7f
                           .data:0000f213 d6 40                            sub 0x40
                           .data:0000f215 77                               ld (hl),a
                           .data:0000f216 23                               inc hl
                           .data:0000f217 36 00                            ld (hl),0x00
                           .data:0000f219 06 07                            ld b,0x07
                           .data:0000f21b 23                               inc hl
                           .data:0000f21c 13                               inc de
                           .data:0000f21d 1a                               ld a,(de)
                           .data:0000f21e 77                               ld (hl),a
                           .data:0000f21f 05                               dec b
                           .data:0000f220 c2 1b f2                         jp nz,0xf21b
                           .data:0000f223 23                               inc hl
                           .data:0000f224 36 00                            ld (hl),0x00
                           .data:0000f226 c1                               pop bc
                           .data:0000f227 c9                               ret
                           .data:0000f228 c5                               push bc
                           .data:0000f229 e5                               push hl
                           .data:0000f22a cd 0d f6                         call 0xf60d
                           .data:0000f22d d1                               pop de
                           .data:0000f22e 2a 07 01                         ld hl,(0x0107)
                           .data:0000f231 7e                               ld a,(hl)
                           .data:0000f232 e6 80                            and 0x80
                           .data:0000f234 47                               ld b,a
                           .data:0000f235 23                               inc hl
                           .data:0000f236 7e                               ld a,(hl)
                           .data:0000f237 c6 40                            add a,0x40
                           .data:0000f239 e6 7f                            and 0x7f
                           .data:0000f23b b0                               or b
                           .data:0000f23c 12                               ld (de),a
                           .data:0000f23d 23                               inc hl
                           .data:0000f23e 06 07                            ld b,0x07
                           .data:0000f240 13                               inc de
                           .data:0000f241 23                               inc hl
                           .data:0000f242 7e                               ld a,(hl)
                           .data:0000f243 12                               ld (de),a
                           .data:0000f244 05                               dec b
                           .data:0000f245 c2 40 f2                         jp nz,0xf240
                           .data:0000f248 c1                               pop bc
                           .data:0000f249 c9                               ret
                           .data:0000f24a e1                               pop hl
                           .data:0000f24b 22 b0 04                         ld (0x04b0),hl
                           .data:0000f24e cd 0d f6                         call 0xf60d
                           .data:0000f251 2a 07 01                         ld hl,(0x0107)
                           .data:0000f254 11 09 00                         ld de,0x0009
                           .data:0000f257 19                               add hl,de
                           .data:0000f258 56                               ld d,(hl)
                           .data:0000f259 2b                               dec hl
                           .data:0000f25a 5e                               ld e,(hl)
                           .data:0000f25b 2b                               dec hl
                           .data:0000f25c d5                               push de
                           .data:0000f25d 56                               ld d,(hl)
                           .data:0000f25e 2b                               dec hl
                           .data:0000f25f 5e                               ld e,(hl)
                           .data:0000f260 2b                               dec hl
                           .data:0000f261 d5                               push de
                           .data:0000f262 56                               ld d,(hl)
                           .data:0000f263 2b                               dec hl
                           .data:0000f264 5e                               ld e,(hl)
                           .data:0000f265 2b                               dec hl
                           .data:0000f266 d5                               push de
                           .data:0000f267 56                               ld d,(hl)
                           .data:0000f268 2b                               dec hl
                           .data:0000f269 2b                               dec hl
                           .data:0000f26a 7e                               ld a,(hl)
                           .data:0000f26b c6 40                            add a,0x40
                           .data:0000f26d e6 7f                            and 0x7f
                           .data:0000f26f 5f                               ld e,a
                           .data:0000f270 2b                               dec hl
                           .data:0000f271 7e                               ld a,(hl)
                           .data:0000f272 e6 80                            and 0x80
                           .data:0000f274 b3                               or e
                           .data:0000f275 5f                               ld e,a
                           .data:0000f276 d5                               push de
                           .data:0000f277 2a b0 04                         ld hl,(0x04b0)
                           .data:0000f27a e9                               jp (hl)
                           .data:0000f27b e1                               pop hl
                           .data:0000f27c 22 b0 04                         ld (0x04b0),hl
                           .data:0000f27f 2a 09 01                         ld hl,(0x0109)
                           .data:0000f282 d1                               pop de
                           .data:0000f283 73                               ld (hl),e
                           .data:0000f284 23                               inc hl
                           .data:0000f285 7b                               ld a,e
                           .data:0000f286 e6 7f                            and 0x7f
                           .data:0000f288 d6 40                            sub 0x40
                           .data:0000f28a 77                               ld (hl),a
                           .data:0000f28b 23                               inc hl
                           .data:0000f28c 36 00                            ld (hl),0x00
                           .data:0000f28e 23                               inc hl
                           .data:0000f28f 72                               ld (hl),d
                           .data:0000f290 23                               inc hl
                           .data:0000f291 d1                               pop de
                           .data:0000f292 73                               ld (hl),e
                           .data:0000f293 23                               inc hl
                           .data:0000f294 72                               ld (hl),d
                           .data:0000f295 23                               inc hl
                           .data:0000f296 d1                               pop de
                           .data:0000f297 73                               ld (hl),e
                           .data:0000f298 23                               inc hl
                           .data:0000f299 72                               ld (hl),d
                           .data:0000f29a 23                               inc hl
                           .data:0000f29b d1                               pop de
                           .data:0000f29c 73                               ld (hl),e
                           .data:0000f29d 23                               inc hl
                           .data:0000f29e 72                               ld (hl),d
                           .data:0000f29f 23                               inc hl
                           .data:0000f2a0 36 00                            ld (hl),0x00
                           .data:0000f2a2 2a b0 04                         ld hl,(0x04b0)
                           .data:0000f2a5 e9                               jp (hl)
                           .data:0000f2a6 2a 09 01                         ld hl,(0x0109)
                           .data:0000f2a9 eb                               ex de,hl
                           .data:0000f2aa 2a 07 01                         ld hl,(0x0107)
                           .data:0000f2ad 22 09 01                         ld (0x0109),hl
                           .data:0000f2b0 eb                               ex de,hl
                           .data:0000f2b1 22 07 01                         ld (0x0107),hl
                           .data:0000f2b4 c9                               ret
                           .data:0000f2b5 2a 07 01                         ld hl,(0x0107)
                           .data:0000f2b8 7e                               ld a,(hl)
                           .data:0000f2b9 ee 80                            xor 0x80
                           .data:0000f2bb 77                               ld (hl),a
                           .data:0000f2bc c9                               ret
                           .data:0000f2bd 2a 07 01                         ld hl,(0x0107)
                           .data:0000f2c0 23                               inc hl
                           .data:0000f2c1 7e                               ld a,(hl)
                           .data:0000f2c2 fe c0                            cp 0xc0
                           .data:0000f2c4 c2 35 fb                         jp nz,0xfb35
                           .data:0000f2c7 c3 29 fb                         jp 0xfb29
                           .data:0000f2ca af                               xor a
                           .data:0000f2cb 3d                               dec a
                           .data:0000f2cc c1                               pop bc
                           .data:0000f2cd c9                               ret
                           .data:0000f2ce af                               xor a
                           .data:0000f2cf 3c                               inc a
                           .data:0000f2d0 c1                               pop bc
                           .data:0000f2d1 c9                               ret
                           .data:0000f2d2 c5                               push bc
                           .data:0000f2d3 2a 07 01                         ld hl,(0x0107)
                           .data:0000f2d6 eb                               ex de,hl
                           .data:0000f2d7 2a 09 01                         ld hl,(0x0109)
                           .data:0000f2da 1a                               ld a,(de)
                           .data:0000f2db b7                               or a
                           .data:0000f2dc fa e6 f2                         jp m,0xf2e6
                           .data:0000f2df ae                               xor (hl)
                           .data:0000f2e0 fa ce f2                         jp m,0xf2ce
                           .data:0000f2e3 c3 eb f2                         jp 0xf2eb
                           .data:0000f2e6 ae                               xor (hl)
                           .data:0000f2e7 fa ca f2                         jp m,0xf2ca
                           .data:0000f2ea eb                               ex de,hl
                           .data:0000f2eb 23                               inc hl
                           .data:0000f2ec 13                               inc de
                           .data:0000f2ed 1a                               ld a,(de)
                           .data:0000f2ee be                               cp (hl)
                           .data:0000f2ef fa ca f2                         jp m,0xf2ca
                           .data:0000f2f2 c2 ce f2                         jp nz,0xf2ce
                           .data:0000f2f5 06 09                            ld b,0x09
                           .data:0000f2f7 23                               inc hl
                           .data:0000f2f8 13                               inc de
                           .data:0000f2f9 1a                               ld a,(de)
                           .data:0000f2fa be                               cp (hl)
                           .data:0000f2fb da ca f2                         jp c,0xf2ca
                           .data:0000f2fe c2 ce f2                         jp nz,0xf2ce
                           .data:0000f301 05                               dec b
                           .data:0000f302 c2 f7 f2                         jp nz,0xf2f7
                           .data:0000f305 af                               xor a
                           .data:0000f306 c1                               pop bc
                           .data:0000f307 c9                               ret
                           .data:0000f308 2a 09 01                         ld hl,(0x0109)
                           .data:0000f30b 7e                               ld a,(hl)
                           .data:0000f30c ee 80                            xor 0x80
                           .data:0000f30e 77                               ld (hl),a
                           .data:0000f30f c5                               push bc
                           .data:0000f310 2a 07 01                         ld hl,(0x0107)
                           .data:0000f313 11 0b 00                         ld de,0x000b
                           .data:0000f316 19                               add hl,de
                           .data:0000f317 06 07                            ld b,0x07
                           .data:0000f319 af                               xor a
                           .data:0000f31a 77                               ld (hl),a
                           .data:0000f31b 23                               inc hl
                           .data:0000f31c 05                               dec b
                           .data:0000f31d c2 1a f3                         jp nz,0xf31a
                           .data:0000f320 2a 09 01                         ld hl,(0x0109)
                           .data:0000f323 11 0b 00                         ld de,0x000b
                           .data:0000f326 19                               add hl,de
                           .data:0000f327 06 07                            ld b,0x07
                           .data:0000f329 77                               ld (hl),a
                           .data:0000f32a 23                               inc hl
                           .data:0000f32b 05                               dec b
                           .data:0000f32c c2 29 f3                         jp nz,0xf329
                           .data:0000f32f 2a 07 01                         ld hl,(0x0107)
                           .data:0000f332 eb                               ex de,hl
                           .data:0000f333 2a 09 01                         ld hl,(0x0109)
                           .data:0000f336 23                               inc hl
                           .data:0000f337 13                               inc de
                           .data:0000f338 1a                               ld a,(de)
                           .data:0000f339 96                               sub (hl)
                           .data:0000f33a f2 40 f3                         jp p,0xf340
                           .data:0000f33d eb                               ex de,hl
                           .data:0000f33e 2f                               cpl
                           .data:0000f33f 3c                               inc a
                           .data:0000f340 1b                               dec de
                           .data:0000f341 2b                               dec hl
                           .data:0000f342 22 09 01                         ld (0x0109),hl
                           .data:0000f345 eb                               ex de,hl
                           .data:0000f346 22 07 01                         ld (0x0107),hl
                           .data:0000f349 fe 09                            cp 0x09
                           .data:0000f34b d2 34 f6                         jp nc,0xf634
                           .data:0000f34e 4f                               ld c,a
                           .data:0000f34f e5                               push hl
                           .data:0000f350 d5                               push de
                           .data:0000f351 c6 09                            add a,0x09
                           .data:0000f353 5f                               ld e,a
                           .data:0000f354 16 00                            ld d,0x00
                           .data:0000f356 19                               add hl,de
                           .data:0000f357 22 b2 04                         ld (0x04b2),hl
                           .data:0000f35a d1                               pop de
                           .data:0000f35b 21 09 00                         ld hl,0x0009
                           .data:0000f35e 19                               add hl,de
                           .data:0000f35f 22 b4 04                         ld (0x04b4),hl
                           .data:0000f362 e1                               pop hl
                           .data:0000f363 eb                               ex de,hl
                           .data:0000f364 1a                               ld a,(de)
                           .data:0000f365 ae                               xor (hl)
                           .data:0000f366 f2 c6 f3                         jp p,0xf3c6
                           .data:0000f369 1a                               ld a,(de)
                           .data:0000f36a b7                               or a
                           .data:0000f36b fa 8d f3                         jp m,0xf38d
                           .data:0000f36e 06 07                            ld b,0x07
                           .data:0000f370 2a b2 04                         ld hl,(0x04b2)
                           .data:0000f373 eb                               ex de,hl
                           .data:0000f374 2a b4 04                         ld hl,(0x04b4)
                           .data:0000f377 1a                               ld a,(de)
                           .data:0000f378 9e                               sbc a,(hl)
                           .data:0000f379 12                               ld (de),a
                           .data:0000f37a 1b                               dec de
                           .data:0000f37b 2b                               dec hl
                           .data:0000f37c 05                               dec b
                           .data:0000f37d c2 77 f3                         jp nz,0xf377
                           .data:0000f380 1a                               ld a,(de)
                           .data:0000f381 de 00                            sbc a,0x00
                           .data:0000f383 12                               ld (de),a
                           .data:0000f384 1b                               dec de
                           .data:0000f385 0d                               dec c
                           .data:0000f386 f2 80 f3                         jp p,0xf380
                           .data:0000f389 eb                               ex de,hl
                           .data:0000f38a c3 a8 f3                         jp 0xf3a8
                           .data:0000f38d 06 07                            ld b,0x07
                           .data:0000f38f 2a b4 04                         ld hl,(0x04b4)
                           .data:0000f392 eb                               ex de,hl
                           .data:0000f393 2a b2 04                         ld hl,(0x04b2)
                           .data:0000f396 1a                               ld a,(de)
                           .data:0000f397 9e                               sbc a,(hl)
                           .data:0000f398 77                               ld (hl),a
                           .data:0000f399 1b                               dec de
                           .data:0000f39a 2b                               dec hl
                           .data:0000f39b 05                               dec b
                           .data:0000f39c c2 96 f3                         jp nz,0xf396
                           .data:0000f39f 3e 00                            ld a,0x00
                           .data:0000f3a1 9e                               sbc a,(hl)
                           .data:0000f3a2 77                               ld (hl),a
                           .data:0000f3a3 2b                               dec hl
                           .data:0000f3a4 0d                               dec c
                           .data:0000f3a5 f2 9f f3                         jp p,0xf39f
                           .data:0000f3a8 23                               inc hl
                           .data:0000f3a9 7e                               ld a,(hl)
                           .data:0000f3aa b7                               or a
                           .data:0000f3ab 3e 01                            ld a,0x01
                           .data:0000f3ad f2 bf f3                         jp p,0xf3bf
                           .data:0000f3b0 11 0f 00                         ld de,0x000f
                           .data:0000f3b3 19                               add hl,de
                           .data:0000f3b4 3e 00                            ld a,0x00
                           .data:0000f3b6 9e                               sbc a,(hl)
                           .data:0000f3b7 77                               ld (hl),a
                           .data:0000f3b8 2b                               dec hl
                           .data:0000f3b9 1d                               dec e
                           .data:0000f3ba f2 b4 f3                         jp p,0xf3b4
                           .data:0000f3bd 3e 81                            ld a,0x81
                           .data:0000f3bf 2a 07 01                         ld hl,(0x0107)
                           .data:0000f3c2 77                               ld (hl),a
                           .data:0000f3c3 c3 34 f6                         jp 0xf634
                           .data:0000f3c6 06 07                            ld b,0x07
                           .data:0000f3c8 2a b2 04                         ld hl,(0x04b2)
                           .data:0000f3cb eb                               ex de,hl
                           .data:0000f3cc 2a b4 04                         ld hl,(0x04b4)
                           .data:0000f3cf 1a                               ld a,(de)
                           .data:0000f3d0 8e                               adc a,(hl)
                           .data:0000f3d1 12                               ld (de),a
                           .data:0000f3d2 1b                               dec de
                           .data:0000f3d3 2b                               dec hl
                           .data:0000f3d4 05                               dec b
                           .data:0000f3d5 c2 cf f3                         jp nz,0xf3cf
                           .data:0000f3d8 1a                               ld a,(de)
                           .data:0000f3d9 ce 00                            adc a,0x00
                           .data:0000f3db 12                               ld (de),a
                           .data:0000f3dc 1b                               dec de
                           .data:0000f3dd 0d                               dec c
                           .data:0000f3de f2 d8 f3                         jp p,0xf3d8
                           .data:0000f3e1 c3 34 f6                         jp 0xf634
                           .data:0000f3e4 c5                               push bc
                           .data:0000f3e5 2a 07 01                         ld hl,(0x0107)
                           .data:0000f3e8 eb                               ex de,hl
                           .data:0000f3e9 2a 09 01                         ld hl,(0x0109)
                           .data:0000f3ec 1a                               ld a,(de)
                           .data:0000f3ed ae                               xor (hl)
                           .data:0000f3ee 12                               ld (de),a
                           .data:0000f3ef 23                               inc hl
                           .data:0000f3f0 13                               inc de
                           .data:0000f3f1 1a                               ld a,(de)
                           .data:0000f3f2 96                               sub (hl)
                           .data:0000f3f3 4f                               ld c,a
                           .data:0000f3f4 d5                               push de
                           .data:0000f3f5 e5                               push hl
                           .data:0000f3f6 7e                               ld a,(hl)
                           .data:0000f3f7 fe c0                            cp 0xc0
                           .data:0000f3f9 c2 01 f4                         jp nz,0xf401
                           .data:0000f3fc e1                               pop hl
                           .data:0000f3fd e1                               pop hl
                           .data:0000f3fe c3 bf f6                         jp 0xf6bf
                           .data:0000f401 13                               inc de
                           .data:0000f402 23                               inc hl
                           .data:0000f403 06 08                            ld b,0x08
                           .data:0000f405 13                               inc de
                           .data:0000f406 23                               inc hl
                           .data:0000f407 1a                               ld a,(de)
                           .data:0000f408 be                               cp (hl)
                           .data:0000f409 c2 20 f4                         jp nz,0xf420
                           .data:0000f40c 05                               dec b
                           .data:0000f40d c2 05 f4                         jp nz,0xf405
                           .data:0000f410 e1                               pop hl
                           .data:0000f411 e1                               pop hl
                           .data:0000f412 0c                               inc c
                           .data:0000f413 71                               ld (hl),c
                           .data:0000f414 23                               inc hl
                           .data:0000f415 36 00                            ld (hl),0x00
                           .data:0000f417 23                               inc hl
                           .data:0000f418 36 01                            ld (hl),0x01
                           .data:0000f41a 06 08                            ld b,0x08
                           .data:0000f41c af                               xor a
                           .data:0000f41d c3 58 f6                         jp 0xf658
                           .data:0000f420 d1                               pop de
                           .data:0000f421 e1                               pop hl
                           .data:0000f422 71                               ld (hl),c
                           .data:0000f423 da 29 f4                         jp c,0xf429
                           .data:0000f426 0c                               inc c
                           .data:0000f427 71                               ld (hl),c
                           .data:0000f428 2b                               dec hl
                           .data:0000f429 d5                               push de
                           .data:0000f42a 11 09 00                         ld de,0x0009
                           .data:0000f42d 19                               add hl,de
                           .data:0000f42e 06 08                            ld b,0x08
                           .data:0000f430 11 e4 04                         ld de,0x04e4
                           .data:0000f433 7e                               ld a,(hl)
                           .data:0000f434 12                               ld (de),a
                           .data:0000f435 2b                               dec hl
                           .data:0000f436 13                               inc de
                           .data:0000f437 05                               dec b
                           .data:0000f438 c2 33 f4                         jp nz,0xf433
                           .data:0000f43b e1                               pop hl
                           .data:0000f43c 11 09 00                         ld de,0x0009
                           .data:0000f43f 19                               add hl,de
                           .data:0000f440 06 08                            ld b,0x08
                           .data:0000f442 11 ec 04                         ld de,0x04ec
                           .data:0000f445 7e                               ld a,(hl)
                           .data:0000f446 12                               ld (de),a
                           .data:0000f447 2b                               dec hl
                           .data:0000f448 13                               inc de
                           .data:0000f449 05                               dec b
                           .data:0000f44a c2 45 f4                         jp nz,0xf445
                           .data:0000f44d 06 08                            ld b,0x08
                           .data:0000f44f 21 dc 04                         ld hl,0x04dc
                           .data:0000f452 af                               xor a
                           .data:0000f453 77                               ld (hl),a
                           .data:0000f454 23                               inc hl
                           .data:0000f455 05                               dec b
                           .data:0000f456 c2 53 f4                         jp nz,0xf453
                           .data:0000f459 3e 40                            ld a,0x40
                           .data:0000f45b 32 db 04                         ld (0x04db),a
                           .data:0000f45e 21 dc 04                         ld hl,0x04dc
                           .data:0000f461 06 10                            ld b,0x10
                           .data:0000f463 b7                               or a
                           .data:0000f464 7e                               ld a,(hl)
                           .data:0000f465 8f                               adc a,a
                           .data:0000f466 77                               ld (hl),a
                           .data:0000f467 23                               inc hl
                           .data:0000f468 05                               dec b
                           .data:0000f469 c2 64 f4                         jp nz,0xf464
                           .data:0000f46c 9f                               sbc a,a
                           .data:0000f46d e6 01                            and 0x01
                           .data:0000f46f 4f                               ld c,a
                           .data:0000f470 06 08                            ld b,0x08
                           .data:0000f472 11 e4 04                         ld de,0x04e4
                           .data:0000f475 21 ec 04                         ld hl,0x04ec
                           .data:0000f478 b7                               or a
                           .data:0000f479 1a                               ld a,(de)
                           .data:0000f47a 9e                               sbc a,(hl)
                           .data:0000f47b 12                               ld (de),a
                           .data:0000f47c 13                               inc de
                           .data:0000f47d 23                               inc hl
                           .data:0000f47e 05                               dec b
                           .data:0000f47f c2 79 f4                         jp nz,0xf479
                           .data:0000f482 79                               ld a,c
                           .data:0000f483 de 00                            sbc a,0x00
                           .data:0000f485 c2 96 f4                         jp nz,0xf496
                           .data:0000f488 21 dc 04                         ld hl,0x04dc
                           .data:0000f48b 34                               inc (hl)
                           .data:0000f48c 21 db 04                         ld hl,0x04db
                           .data:0000f48f 35                               dec (hl)
                           .data:0000f490 c2 5e f4                         jp nz,0xf45e
                           .data:0000f493 c3 c8 f4                         jp 0xf4c8
                           .data:0000f496 21 db 04                         ld hl,0x04db
                           .data:0000f499 35                               dec (hl)
                           .data:0000f49a ca c8 f4                         jp z,0xf4c8
                           .data:0000f49d 21 dc 04                         ld hl,0x04dc
                           .data:0000f4a0 06 10                            ld b,0x10
                           .data:0000f4a2 b7                               or a
                           .data:0000f4a3 7e                               ld a,(hl)
                           .data:0000f4a4 8f                               adc a,a
                           .data:0000f4a5 77                               ld (hl),a
                           .data:0000f4a6 23                               inc hl
                           .data:0000f4a7 05                               dec b
                           .data:0000f4a8 c2 a3 f4                         jp nz,0xf4a3
                           .data:0000f4ab 9f                               sbc a,a
                           .data:0000f4ac 4f                               ld c,a
                           .data:0000f4ad 06 08                            ld b,0x08
                           .data:0000f4af 11 e4 04                         ld de,0x04e4
                           .data:0000f4b2 21 ec 04                         ld hl,0x04ec
                           .data:0000f4b5 b7                               or a
                           .data:0000f4b6 1a                               ld a,(de)
                           .data:0000f4b7 8e                               adc a,(hl)
                           .data:0000f4b8 12                               ld (de),a
                           .data:0000f4b9 13                               inc de
                           .data:0000f4ba 23                               inc hl
                           .data:0000f4bb 05                               dec b
                           .data:0000f4bc c2 b6 f4                         jp nz,0xf4b6
                           .data:0000f4bf 79                               ld a,c
                           .data:0000f4c0 ce 00                            adc a,0x00
                           .data:0000f4c2 c2 96 f4                         jp nz,0xf496
                           .data:0000f4c5 c3 88 f4                         jp 0xf488
                           .data:0000f4c8 2a 07 01                         ld hl,(0x0107)
                           .data:0000f4cb 11 0c 00                         ld de,0x000c
                           .data:0000f4ce 19                               add hl,de
                           .data:0000f4cf 36 00                            ld (hl),0x00
                           .data:0000f4d1 2b                               dec hl
                           .data:0000f4d2 36 00                            ld (hl),0x00
                           .data:0000f4d4 11 dc 04                         ld de,0x04dc
                           .data:0000f4d7 06 08                            ld b,0x08
                           .data:0000f4d9 2b                               dec hl
                           .data:0000f4da 1a                               ld a,(de)
                           .data:0000f4db 77                               ld (hl),a
                           .data:0000f4dc 13                               inc de
                           .data:0000f4dd 05                               dec b
                           .data:0000f4de c2 d9 f4                         jp nz,0xf4d9
                           .data:0000f4e1 c3 34 f6                         jp 0xf634
                           .data:0000f4e4 c5                               push bc
                           .data:0000f4e5 2a 07 01                         ld hl,(0x0107)
                           .data:0000f4e8 eb                               ex de,hl
                           .data:0000f4e9 2a 09 01                         ld hl,(0x0109)
                           .data:0000f4ec 1a                               ld a,(de)
                           .data:0000f4ed ae                               xor (hl)
                           .data:0000f4ee 12                               ld (de),a
                           .data:0000f4ef 23                               inc hl
                           .data:0000f4f0 13                               inc de
                           .data:0000f4f1 1a                               ld a,(de)
                           .data:0000f4f2 fe c0                            cp 0xc0
                           .data:0000f4f4 ca 4e f6                         jp z,0xf64e
                           .data:0000f4f7 86                               add a,(hl)
                           .data:0000f4f8 12                               ld (de),a
                           .data:0000f4f9 7e                               ld a,(hl)
                           .data:0000f4fa fe c0                            cp 0xc0
                           .data:0000f4fc ca 4e f6                         jp z,0xf64e
                           .data:0000f4ff d5                               push de
                           .data:0000f500 11 09 00                         ld de,0x0009
                           .data:0000f503 19                               add hl,de
                           .data:0000f504 06 08                            ld b,0x08
                           .data:0000f506 11 ec 04                         ld de,0x04ec
                           .data:0000f509 7e                               ld a,(hl)
                           .data:0000f50a 12                               ld (de),a
                           .data:0000f50b 2b                               dec hl
                           .data:0000f50c 13                               inc de
                           .data:0000f50d 05                               dec b
                           .data:0000f50e c2 09 f5                         jp nz,0xf509
                           .data:0000f511 e1                               pop hl
                           .data:0000f512 11 09 00                         ld de,0x0009
                           .data:0000f515 19                               add hl,de
                           .data:0000f516 06 08                            ld b,0x08
                           .data:0000f518 11 e4 04                         ld de,0x04e4
                           .data:0000f51b 7e                               ld a,(hl)
                           .data:0000f51c 12                               ld (de),a
                           .data:0000f51d 2b                               dec hl
                           .data:0000f51e 13                               inc de
                           .data:0000f51f 05                               dec b
                           .data:0000f520 c2 1b f5                         jp nz,0xf51b
                           .data:0000f523 06 08                            ld b,0x08
                           .data:0000f525 21 dc 04                         ld hl,0x04dc
                           .data:0000f528 af                               xor a
                           .data:0000f529 77                               ld (hl),a
                           .data:0000f52a 23                               inc hl
                           .data:0000f52b 05                               dec b
                           .data:0000f52c c2 29 f5                         jp nz,0xf529
                           .data:0000f52f 3e 40                            ld a,0x40
                           .data:0000f531 32 db 04                         ld (0x04db),a
                           .data:0000f534 21 dc 04                         ld hl,0x04dc
                           .data:0000f537 06 10                            ld b,0x10
                           .data:0000f539 b7                               or a
                           .data:0000f53a 7e                               ld a,(hl)
                           .data:0000f53b 8f                               adc a,a
                           .data:0000f53c 77                               ld (hl),a
                           .data:0000f53d 23                               inc hl
                           .data:0000f53e 05                               dec b
                           .data:0000f53f c2 3a f5                         jp nz,0xf53a
                           .data:0000f542 d2 65 f5                         jp nc,0xf565
                           .data:0000f545 06 08                            ld b,0x08
                           .data:0000f547 11 dc 04                         ld de,0x04dc
                           .data:0000f54a 21 ec 04                         ld hl,0x04ec
                           .data:0000f54d b7                               or a
                           .data:0000f54e 1a                               ld a,(de)
                           .data:0000f54f 8e                               adc a,(hl)
                           .data:0000f550 12                               ld (de),a
                           .data:0000f551 13                               inc de
                           .data:0000f552 23                               inc hl
                           .data:0000f553 05                               dec b
                           .data:0000f554 c2 4e f5                         jp nz,0xf54e
                           .data:0000f557 06 08                            ld b,0x08
                           .data:0000f559 1a                               ld a,(de)
                           .data:0000f55a ce 00                            adc a,0x00
                           .data:0000f55c 12                               ld (de),a
                           .data:0000f55d d2 65 f5                         jp nc,0xf565
                           .data:0000f560 13                               inc de
                           .data:0000f561 05                               dec b
                           .data:0000f562 c2 59 f5                         jp nz,0xf559
                           .data:0000f565 21 db 04                         ld hl,0x04db
                           .data:0000f568 35                               dec (hl)
                           .data:0000f569 c2 34 f5                         jp nz,0xf534
                           .data:0000f56c 2a 07 01                         ld hl,(0x0107)
                           .data:0000f56f 11 0c 00                         ld de,0x000c
                           .data:0000f572 19                               add hl,de
                           .data:0000f573 11 e2 04                         ld de,0x04e2
                           .data:0000f576 06 0a                            ld b,0x0a
                           .data:0000f578 1a                               ld a,(de)
                           .data:0000f579 77                               ld (hl),a
                           .data:0000f57a 13                               inc de
                           .data:0000f57b 2b                               dec hl
                           .data:0000f57c 05                               dec b
                           .data:0000f57d c2 78 f5                         jp nz,0xf578
                           .data:0000f580 c3 34 f6                         jp 0xf634
                           .data:0000f583 cd d2 f2                         call 0xf2d2
                           .data:0000f586 ca 35 fb                         jp z,0xfb35
                           .data:0000f589 c3 29 fb                         jp 0xfb29
                           .data:0000f58c cd d2 f2                         call 0xf2d2
                           .data:0000f58f ca 29 fb                         jp z,0xfb29
                           .data:0000f592 c3 35 fb                         jp 0xfb35
                           .data:0000f595 cd d2 f2                         call 0xf2d2
                           .data:0000f598 fa 35 fb                         jp m,0xfb35
                           .data:0000f59b c3 29 fb                         jp 0xfb29
                           .data:0000f59e cd d2 f2                         call 0xf2d2
                           .data:0000f5a1 fa 35 fb                         jp m,0xfb35
                           .data:0000f5a4 ca 35 fb                         jp z,0xfb35
                           .data:0000f5a7 c3 29 fb                         jp 0xfb29
                           .data:0000f5aa cd d2 f2                         call 0xf2d2
                           .data:0000f5ad fa 29 fb                         jp m,0xfb29
                           .data:0000f5b0 c3 35 fb                         jp 0xfb35
                           .data:0000f5b3 cd d2 f2                         call 0xf2d2
                           .data:0000f5b6 fa 29 fb                         jp m,0xfb29
                           .data:0000f5b9 ca 29 fb                         jp z,0xfb29
                           .data:0000f5bc c3 35 fb                         jp 0xfb35
                           .data:0000f5bf c5                               push bc
                           .data:0000f5c0 7c                               ld a,h
                           .data:0000f5c1 b5                               or l
                           .data:0000f5c2 ca 4e f6                         jp z,0xf64e
                           .data:0000f5c5 eb                               ex de,hl
                           .data:0000f5c6 06 00                            ld b,0x00
                           .data:0000f5c8 c3 e1 f5                         jp 0xf5e1
                           .data:0000f5cb c5                               push bc
                           .data:0000f5cc 7c                               ld a,h
                           .data:0000f5cd b5                               or l
                           .data:0000f5ce ca 4e f6                         jp z,0xf64e
                           .data:0000f5d1 eb                               ex de,hl
                           .data:0000f5d2 06 00                            ld b,0x00
                           .data:0000f5d4 7a                               ld a,d
                           .data:0000f5d5 b7                               or a
                           .data:0000f5d6 f2 e1 f5                         jp p,0xf5e1
                           .data:0000f5d9 2f                               cpl
                           .data:0000f5da 57                               ld d,a
                           .data:0000f5db 7b                               ld a,e
                           .data:0000f5dc 2f                               cpl
                           .data:0000f5dd 5f                               ld e,a
                           .data:0000f5de 13                               inc de
                           .data:0000f5df 06 80                            ld b,0x80
                           .data:0000f5e1 2a 07 01                         ld hl,(0x0107)
                           .data:0000f5e4 70                               ld (hl),b
                           .data:0000f5e5 23                               inc hl
                           .data:0000f5e6 7a                               ld a,d
                           .data:0000f5e7 b7                               or a
                           .data:0000f5e8 c2 f8 f5                         jp nz,0xf5f8
                           .data:0000f5eb 36 01                            ld (hl),0x01
                           .data:0000f5ed 23                               inc hl
                           .data:0000f5ee 36 00                            ld (hl),0x00
                           .data:0000f5f0 23                               inc hl
                           .data:0000f5f1 73                               ld (hl),e
                           .data:0000f5f2 06 07                            ld b,0x07
                           .data:0000f5f4 af                               xor a
                           .data:0000f5f5 c3 04 f6                         jp 0xf604
                           .data:0000f5f8 36 02                            ld (hl),0x02
                           .data:0000f5fa 23                               inc hl
                           .data:0000f5fb 36 00                            ld (hl),0x00
                           .data:0000f5fd 23                               inc hl
                           .data:0000f5fe 72                               ld (hl),d
                           .data:0000f5ff 23                               inc hl
                           .data:0000f600 73                               ld (hl),e
                           .data:0000f601 06 06                            ld b,0x06
                           .data:0000f603 af                               xor a
                           .data:0000f604 23                               inc hl
                           .data:0000f605 77                               ld (hl),a
                           .data:0000f606 05                               dec b
                           .data:0000f607 c2 04 f6                         jp nz,0xf604
                           .data:0000f60a c3 92 f6                         jp 0xf692
                           .data:0000f60d 2a 07 01                         ld hl,(0x0107)
                           .data:0000f610 11 0a 00                         ld de,0x000a
                           .data:0000f613 19                               add hl,de
                           .data:0000f614 7e                               ld a,(hl)
                           .data:0000f615 fe 80                            cp 0x80
                           .data:0000f617 d8                               ret c
                           .data:0000f618 c2 21 f6                         jp nz,0xf621
                           .data:0000f61b 2b                               dec hl
                           .data:0000f61c 7e                               ld a,(hl)
                           .data:0000f61d f6 01                            or 0x01
                           .data:0000f61f 77                               ld (hl),a
                           .data:0000f620 c9                               ret
                           .data:0000f621 c5                               push bc
                           .data:0000f622 01 00 08                         ld bc,0x0800
                           .data:0000f625 37                               scf
                           .data:0000f626 2b                               dec hl
                           .data:0000f627 7e                               ld a,(hl)
                           .data:0000f628 89                               adc a,c
                           .data:0000f629 77                               ld (hl),a
                           .data:0000f62a 05                               dec b
                           .data:0000f62b c2 26 f6                         jp nz,0xf626
                           .data:0000f62e b7                               or a
                           .data:0000f62f c2 34 f6                         jp nz,0xf634
                           .data:0000f632 c1                               pop bc
                           .data:0000f633 c9                               ret
                           .data:0000f634 2a 07 01                         ld hl,(0x0107)
                           .data:0000f637 23                               inc hl
                           .data:0000f638 7e                               ld a,(hl)
                           .data:0000f639 54                               ld d,h
                           .data:0000f63a 5d                               ld e,l
                           .data:0000f63b 23                               inc hl
                           .data:0000f63c 4f                               ld c,a
                           .data:0000f63d af                               xor a
                           .data:0000f63e be                               cp (hl)
                           .data:0000f63f c2 94 f6                         jp nz,0xf694
                           .data:0000f642 06 08                            ld b,0x08
                           .data:0000f644 23                               inc hl
                           .data:0000f645 be                               cp (hl)
                           .data:0000f646 c2 60 f6                         jp nz,0xf660
                           .data:0000f649 0d                               dec c
                           .data:0000f64a 05                               dec b
                           .data:0000f64b c2 44 f6                         jp nz,0xf644
                           .data:0000f64e af                               xor a
                           .data:0000f64f 2a 07 01                         ld hl,(0x0107)
                           .data:0000f652 06 0a                            ld b,0x0a
                           .data:0000f654 77                               ld (hl),a
                           .data:0000f655 23                               inc hl
                           .data:0000f656 36 c0                            ld (hl),0xc0
                           .data:0000f658 23                               inc hl
                           .data:0000f659 77                               ld (hl),a
                           .data:0000f65a 05                               dec b
                           .data:0000f65b c2 58 f6                         jp nz,0xf658
                           .data:0000f65e c1                               pop bc
                           .data:0000f65f c9                               ret
                           .data:0000f660 3e 08                            ld a,0x08
                           .data:0000f662 90                               sub b
                           .data:0000f663 47                               ld b,a
                           .data:0000f664 ca 80 f6                         jp z,0xf680
                           .data:0000f667 2b                               dec hl
                           .data:0000f668 79                               ld a,c
                           .data:0000f669 12                               ld (de),a
                           .data:0000f66a d5                               push de
                           .data:0000f66b 13                               inc de
                           .data:0000f66c 3e 0f                            ld a,0x0f
                           .data:0000f66e 90                               sub b
                           .data:0000f66f 4f                               ld c,a
                           .data:0000f670 7e                               ld a,(hl)
                           .data:0000f671 12                               ld (de),a
                           .data:0000f672 13                               inc de
                           .data:0000f673 23                               inc hl
                           .data:0000f674 0d                               dec c
                           .data:0000f675 c2 70 f6                         jp nz,0xf670
                           .data:0000f678 af                               xor a
                           .data:0000f679 12                               ld (de),a
                           .data:0000f67a 13                               inc de
                           .data:0000f67b 05                               dec b
                           .data:0000f67c c2 79 f6                         jp nz,0xf679
                           .data:0000f67f d1                               pop de
                           .data:0000f680 1a                               ld a,(de)
                           .data:0000f681 b7                               or a
                           .data:0000f682 fa 8d f6                         jp m,0xf68d
                           .data:0000f685 fe 40                            cp 0x40
                           .data:0000f687 da 92 f6                         jp c,0xf692
                           .data:0000f68a c3 bf f6                         jp 0xf6bf
                           .data:0000f68d fe c1                            cp 0xc1
                           .data:0000f68f da ae f6                         jp c,0xf6ae
                           .data:0000f692 c1                               pop bc
                           .data:0000f693 c9                               ret
                           .data:0000f694 0c                               inc c
                           .data:0000f695 79                               ld a,c
                           .data:0000f696 12                               ld (de),a
                           .data:0000f697 06 0f                            ld b,0x0f
                           .data:0000f699 d5                               push de
                           .data:0000f69a 21 10 00                         ld hl,0x0010
                           .data:0000f69d 19                               add hl,de
                           .data:0000f69e 54                               ld d,h
                           .data:0000f69f 5d                               ld e,l
                           .data:0000f6a0 1b                               dec de
                           .data:0000f6a1 1a                               ld a,(de)
                           .data:0000f6a2 77                               ld (hl),a
                           .data:0000f6a3 2b                               dec hl
                           .data:0000f6a4 05                               dec b
                           .data:0000f6a5 c2 a0 f6                         jp nz,0xf6a0
                           .data:0000f6a8 36 00                            ld (hl),0x00
                           .data:0000f6aa d1                               pop de
                           .data:0000f6ab c3 80 f6                         jp 0xf680
                           .data:0000f6ae af                               xor a
                           .data:0000f6af 2a 07 01                         ld hl,(0x0107)
                           .data:0000f6b2 23                               inc hl
                           .data:0000f6b3 36 c1                            ld (hl),0xc1
                           .data:0000f6b5 23                               inc hl
                           .data:0000f6b6 77                               ld (hl),a
                           .data:0000f6b7 23                               inc hl
                           .data:0000f6b8 36 01                            ld (hl),0x01
                           .data:0000f6ba 06 08                            ld b,0x08
                           .data:0000f6bc c3 58 f6                         jp 0xf658
                           .data:0000f6bf 2a 07 01                         ld hl,(0x0107)
                           .data:0000f6c2 23                               inc hl
                           .data:0000f6c3 36 3f                            ld (hl),0x3f
                           .data:0000f6c5 23                               inc hl
                           .data:0000f6c6 36 00                            ld (hl),0x00
                           .data:0000f6c8 3e ff                            ld a,0xff
                           .data:0000f6ca 06 07                            ld b,0x07
                           .data:0000f6cc 23                               inc hl
                           .data:0000f6cd 77                               ld (hl),a
                           .data:0000f6ce 05                               dec b
                           .data:0000f6cf c2 cc f6                         jp nz,0xf6cc
                           .data:0000f6d2 23                               inc hl
                           .data:0000f6d3 36 00                            ld (hl),0x00
                           .data:0000f6d5 c1                               pop bc
                           .data:0000f6d6 c9                               ret
                           .data:0000f6d7 c5                               push bc
                           .data:0000f6d8 2a 07 01                         ld hl,(0x0107)
                           .data:0000f6db 36 00                            ld (hl),0x00
                           .data:0000f6dd 23                               inc hl
                           .data:0000f6de 36 03                            ld (hl),0x03
                           .data:0000f6e0 11 04 00                         ld de,0x0004
                           .data:0000f6e3 19                               add hl,de
                           .data:0000f6e4 5d                               ld e,l
                           .data:0000f6e5 54                               ld d,h
                           .data:0000f6e6 06 05                            ld b,0x05
                           .data:0000f6e8 af                               xor a
                           .data:0000f6e9 23                               inc hl
                           .data:0000f6ea 77                               ld (hl),a
                           .data:0000f6eb 05                               dec b
                           .data:0000f6ec c2 e9 f6                         jp nz,0xf6e9
                           .data:0000f6ef 06 04                            ld b,0x04
                           .data:0000f6f1 2a 03 01                         ld hl,(0x0103)
                           .data:0000f6f4 23                               inc hl
                           .data:0000f6f5 23                               inc hl
                           .data:0000f6f6 23                               inc hl
                           .data:0000f6f7 7e                               ld a,(hl)
                           .data:0000f6f8 2a 03 01                         ld hl,(0x0103)
                           .data:0000f6fb b7                               or a
                           .data:0000f6fc f2 10 f7                         jp p,0xf710
                           .data:0000f6ff 3e 00                            ld a,0x00
                           .data:0000f701 9e                               sbc a,(hl)
                           .data:0000f702 12                               ld (de),a
                           .data:0000f703 23                               inc hl
                           .data:0000f704 1b                               dec de
                           .data:0000f705 05                               dec b
                           .data:0000f706 c2 ff f6                         jp nz,0xf6ff
                           .data:0000f709 1b                               dec de
                           .data:0000f70a 3e 80                            ld a,0x80
                           .data:0000f70c 12                               ld (de),a
                           .data:0000f70d c3 34 f6                         jp 0xf634
                           .data:0000f710 7e                               ld a,(hl)
                           .data:0000f711 12                               ld (de),a
                           .data:0000f712 23                               inc hl
                           .data:0000f713 1b                               dec de
                           .data:0000f714 05                               dec b
                           .data:0000f715 c2 10 f7                         jp nz,0xf710
                           .data:0000f718 c3 34 f6                         jp 0xf634
                           .data:0000f71b c5                               push bc
                           .data:0000f71c 2a 03 01                         ld hl,(0x0103)
                           .data:0000f71f 54                               ld d,h
                           .data:0000f720 5d                               ld e,l
                           .data:0000f721 af                               xor a
                           .data:0000f722 77                               ld (hl),a
                           .data:0000f723 23                               inc hl
                           .data:0000f724 77                               ld (hl),a
                           .data:0000f725 23                               inc hl
                           .data:0000f726 77                               ld (hl),a
                           .data:0000f727 23                               inc hl
                           .data:0000f728 77                               ld (hl),a
                           .data:0000f729 2a 07 01                         ld hl,(0x0107)
                           .data:0000f72c 4e                               ld c,(hl)
                           .data:0000f72d 23                               inc hl
                           .data:0000f72e 7e                               ld a,(hl)
                           .data:0000f72f b7                               or a
                           .data:0000f730 ca 92 f6                         jp z,0xf692
                           .data:0000f733 fa 92 f6                         jp m,0xf692
                           .data:0000f736 fe 05                            cp 0x05
                           .data:0000f738 d2 61 f7                         jp nc,0xf761
                           .data:0000f73b 47                               ld b,a
                           .data:0000f73c 23                               inc hl
                           .data:0000f73d 85                               add a,l
                           .data:0000f73e 6f                               ld l,a
                           .data:0000f73f d2 43 f7                         jp nc,0xf743
                           .data:0000f742 24                               inc h
                           .data:0000f743 7e                               ld a,(hl)
                           .data:0000f744 12                               ld (de),a
                           .data:0000f745 13                               inc de
                           .data:0000f746 2b                               dec hl
                           .data:0000f747 05                               dec b
                           .data:0000f748 c2 43 f7                         jp nz,0xf743
                           .data:0000f74b 79                               ld a,c
                           .data:0000f74c b7                               or a
                           .data:0000f74d f2 92 f6                         jp p,0xf692
                           .data:0000f750 06 04                            ld b,0x04
                           .data:0000f752 2a 03 01                         ld hl,(0x0103)
                           .data:0000f755 3e 00                            ld a,0x00
                           .data:0000f757 9e                               sbc a,(hl)
                           .data:0000f758 77                               ld (hl),a
                           .data:0000f759 23                               inc hl
                           .data:0000f75a 05                               dec b
                           .data:0000f75b c2 55 f7                         jp nz,0xf755
                           .data:0000f75e c3 92 f6                         jp 0xf692
                           .data:0000f761 eb                               ex de,hl
                           .data:0000f762 79                               ld a,c
                           .data:0000f763 b7                               or a
                           .data:0000f764 fa 75 f7                         jp m,0xf775
                           .data:0000f767 36 7f                            ld (hl),0x7f
                           .data:0000f769 23                               inc hl
                           .data:0000f76a 36 ff                            ld (hl),0xff
                           .data:0000f76c 23                               inc hl
                           .data:0000f76d 36 ff                            ld (hl),0xff
                           .data:0000f76f 23                               inc hl
                           .data:0000f770 36 ff                            ld (hl),0xff
                           .data:0000f772 c3 dc f7                         jp 0xf7dc
                           .data:0000f775 36 80                            ld (hl),0x80
                           .data:0000f777 23                               inc hl
                           .data:0000f778 36 00                            ld (hl),0x00
                           .data:0000f77a 23                               inc hl
                           .data:0000f77b 36 00                            ld (hl),0x00
                           .data:0000f77d 23                               inc hl
                           .data:0000f77e 36 00                            ld (hl),0x00
                           .data:0000f780 c3 dc f7                         jp 0xf7dc
                           .data:0000f783 c5                               push bc
                           .data:0000f784 0e 00                            ld c,0x00
                           .data:0000f786 c3 8c f7                         jp 0xf78c
                           .data:0000f789 c5                               push bc
                           .data:0000f78a 0e 01                            ld c,0x01
                           .data:0000f78c 2a 07 01                         ld hl,(0x0107)
                           .data:0000f78f 46                               ld b,(hl)
                           .data:0000f790 23                               inc hl
                           .data:0000f791 7e                               ld a,(hl)
                           .data:0000f792 b7                               or a
                           .data:0000f793 ca 99 f7                         jp z,0xf799
                           .data:0000f796 f2 9f f7                         jp p,0xf79f
                           .data:0000f799 21 00 00                         ld hl,0x0000
                           .data:0000f79c c3 92 f6                         jp 0xf692
                           .data:0000f79f fe 03                            cp 0x03
                           .data:0000f7a1 d2 c3 f7                         jp nc,0xf7c3
                           .data:0000f7a4 23                               inc hl
                           .data:0000f7a5 85                               add a,l
                           .data:0000f7a6 6f                               ld l,a
                           .data:0000f7a7 d2 ab f7                         jp nc,0xf7ab
                           .data:0000f7aa 24                               inc h
                           .data:0000f7ab 5e                               ld e,(hl)
                           .data:0000f7ac 2b                               dec hl
                           .data:0000f7ad 56                               ld d,(hl)
                           .data:0000f7ae eb                               ex de,hl
                           .data:0000f7af 79                               ld a,c
                           .data:0000f7b0 b7                               or a
                           .data:0000f7b1 ca 92 f6                         jp z,0xf692
                           .data:0000f7b4 78                               ld a,b
                           .data:0000f7b5 b7                               or a
                           .data:0000f7b6 f2 92 f6                         jp p,0xf692
                           .data:0000f7b9 7c                               ld a,h
                           .data:0000f7ba 2f                               cpl
                           .data:0000f7bb 67                               ld h,a
                           .data:0000f7bc 7d                               ld a,l
                           .data:0000f7bd 2f                               cpl
                           .data:0000f7be 6f                               ld l,a
                           .data:0000f7bf 23                               inc hl
                           .data:0000f7c0 c3 92 f6                         jp 0xf692
                           .data:0000f7c3 79                               ld a,c
                           .data:0000f7c4 b7                               or a
                           .data:0000f7c5 c2 ce f7                         jp nz,0xf7ce
                           .data:0000f7c8 21 ff ff                         ld hl,0xffff
                           .data:0000f7cb c3 dc f7                         jp 0xf7dc
                           .data:0000f7ce 78                               ld a,b
                           .data:0000f7cf b7                               or a
                           .data:0000f7d0 fa d9 f7                         jp m,0xf7d9
                           .data:0000f7d3 21 ff 7f                         ld hl,0x7fff
                           .data:0000f7d6 c3 dc f7                         jp 0xf7dc
                           .data:0000f7d9 21 00 80                         ld hl,0x8000
                           .data:0000f7dc c1                               pop bc
                           .data:0000f7dd c9                               ret
                           .data:0000f7de 40                               ld b,b
                           .data:0000f7df 80                               add a,b
                           .data:0000f7e0 00                               nop
                           .data:0000f7e1 00                               nop
                           .data:0000f7e2 00                               nop
                           .data:0000f7e3 00                               nop
                           .data:0000f7e4 00                               nop
                           .data:0000f7e5 00                               nop
                           .data:0000f7e6 40                               ld b,b
                           .data:0000f7e7 0c                               inc c
                           .data:0000f7e8 cc cc cc                         call z,0xcccc
                           .data:0000f7eb cc cc cd                         call z,0xcdcc
                           .data:0000f7ee 40                               ld b,b
                           .data:0000f7ef 01 47 ae                         ld bc,0xae47
                           .data:0000f7f2 14                               inc d
                           .data:0000f7f3 7a                               ld a,d
                           .data:0000f7f4 e1                               pop hl
                           .data:0000f7f5 48                               ld c,b
                           .data:0000f7f6 3f                               ccf
                           .data:0000f7f7 20 c4                            jr nz,0xf7bd
                           .data:0000f7f9 9b                               sbc a,e
                           .data:0000f7fa a5                               and l
                           .data:0000f7fb e3                               ex (sp),hl
                           .data:0000f7fc 54                               ld d,h
                           .data:0000f7fd 00                               nop
                           .data:0000f7fe 3f                               ccf
                           .data:0000f7ff 03                               inc bc
                           .data:0000f800 46                               ld b,(hl)
                           .data:0000f801 dc 5d 63                         call c,0x635d
                           .data:0000f804 88                               adc a,b
                           .data:0000f805 66                               ld h,(hl)
                           .data:0000f806 3e 53                            ld a,0x53
                           .data:0000f808 e2 d6 23                         jp po,0x23d6
                           .data:0000f80b 8d                               adc a,l
                           .data:0000f80c a3                               and e
                           .data:0000f80d cd 3e 08                         call 0x083e
                           .data:0000f810 63                               ld h,e
                           .data:0000f811 7b                               ld a,e
                           .data:0000f812 d0                               ret nc
                           .data:0000f813 5a                               ld e,d
                           .data:0000f814 f6 c8                            or 0xc8
                           .data:0000f816 3d                               dec a
                           .data:0000f817 d6 bf                            sub 0xbf
                           .data:0000f819 94                               sub h
                           .data:0000f81a d5                               push de
                           .data:0000f81b e5                               push hl
                           .data:0000f81c 7a                               ld a,d
                           .data:0000f81d 66                               ld h,(hl)
                           .data:0000f81e 3d                               dec a
                           .data:0000f81f 15                               dec d
                           .data:0000f820 79                               ld a,c
                           .data:0000f821 8e                               adc a,(hl)
                           .data:0000f822 e2 30 8c                         jp po,0x8c30
                           .data:0000f825 3d                               dec a
                           .data:0000f826 3d                               dec a
                           .data:0000f827 02                               ld (bc),a
                           .data:0000f828 25                               dec h
                           .data:0000f829 c1                               pop bc
                           .data:0000f82a 7d                               ld a,l
                           .data:0000f82b 04                               inc b
                           .data:0000f82c da d3 3c                         jp c,0x3cd3
                           .data:0000f82f 36 f9                            ld (hl),0xf9
                           .data:0000f831 bf                               cp a
                           .data:0000f832 b3                               or e
                           .data:0000f833 af                               xor a
                           .data:0000f834 7b                               ld a,e
                           .data:0000f835 80                               add a,b
                           .data:0000f836 3c                               inc a
                           .data:0000f837 05                               dec b
                           .data:0000f838 7f                               ld a,a
                           .data:0000f839 5f                               ld e,a
                           .data:0000f83a f8                               ret m
                           .data:0000f83b 5e                               ld e,(hl)
                           .data:0000f83c 59                               ld e,c
                           .data:0000f83d 26 3b                            ld h,0x3b
                           .data:0000f83f 8c                               adc a,h
                           .data:0000f840 bc                               cp h
                           .data:0000f841 cc 09 6f                         call z,0x6f09
                           .data:0000f844 50                               ld d,b
                           .data:0000f845 9a                               sbc a,d
                           .data:0000f846 3b                               dec sp
                           .data:0000f847 0e 12                            ld c,0x12
                           .data:0000f849 e1                               pop hl
                           .data:0000f84a 34                               inc (hl)
                           .data:0000f84b 24                               inc h
                           .data:0000f84c bb                               cp e
                           .data:0000f84d 43                               ld b,e
                           .data:0000f84e 3b                               dec sp
                           .data:0000f84f 01 68 49                         ld bc,0x4968
                           .data:0000f852 b8                               cp b
                           .data:0000f853 6a                               ld l,d
                           .data:0000f854 12                               ld (de),a
                           .data:0000f855 ba                               cp d
                           .data:0000f856 3a 24 07                         ld a,(0x0724)
                           .data:0000f859 5f                               ld e,a
                           .data:0000f85a 3d                               dec a
                           .data:0000f85b ce ac                            adc a,0xac
                           .data:0000f85d 33                               inc sp
                           .data:0000f85e 3a 03 9a                         ld a,(0x9a03)
                           .data:0000f861 56                               ld d,(hl)
                           .data:0000f862 52                               ld d,d
                           .data:0000f863 fb                               ei
                           .data:0000f864 11 38 c5                         ld de,0xc538
                           .data:0000f867 21 0c 00                         ld hl,0x000c
                           .data:0000f86a 39                               add hl,sp
                           .data:0000f86b 5e                               ld e,(hl)
                           .data:0000f86c 23                               inc hl
                           .data:0000f86d 56                               ld d,(hl)
                           .data:0000f86e eb                               ex de,hl
                           .data:0000f86f 22 f4 04                         ld (0x04f4),hl
                           .data:0000f872 21 04 00                         ld hl,0x0004
                           .data:0000f875 39                               add hl,sp
                           .data:0000f876 cd 09 f2                         call 0xf209
                           .data:0000f879 21 0f 00                         ld hl,0x000f
                           .data:0000f87c 22 f6 04                         ld (0x04f6),hl
                           .data:0000f87f 2a 07 01                         ld hl,(0x0107)
                           .data:0000f882 7e                               ld a,(hl)
                           .data:0000f883 b7                               or a
                           .data:0000f884 f2 93 f8                         jp p,0xf893
                           .data:0000f887 cd b5 f2                         call 0xf2b5
                           .data:0000f88a 2a f4 04                         ld hl,(0x04f4)
                           .data:0000f88d 36 2d                            ld (hl),0x2d
                           .data:0000f88f 23                               inc hl
                           .data:0000f890 22 f4 04                         ld (0x04f4),hl
                           .data:0000f893 01 00 00                         ld bc,0x0000
                           .data:0000f896 cd bd f2                         call 0xf2bd
                           .data:0000f899 ca 19 f9                         jp z,0xf919
                           .data:0000f89c cd f4 f1                         call 0xf1f4
                           .data:0000f89f 41                               ld b,c
                           .data:0000f8a0 0a                               ld a,(bc)
                           .data:0000f8a1 00                               nop
                           .data:0000f8a2 00                               nop
                           .data:0000f8a3 00                               nop
                           .data:0000f8a4 00                               nop
                           .data:0000f8a5 00                               nop
                           .data:0000f8a6 00                               nop
                           .data:0000f8a7 2a 07 01                         ld hl,(0x0107)
                           .data:0000f8aa 23                               inc hl
                           .data:0000f8ab 7e                               ld a,(hl)
                           .data:0000f8ac fe 01                            cp 0x01
                           .data:0000f8ae fa 12 f9                         jp m,0xf912
                           .data:0000f8b1 ca f9 f8                         jp z,0xf8f9
                           .data:0000f8b4 fe 02                            cp 0x02
                           .data:0000f8b6 c2 c1 f8                         jp nz,0xf8c1
                           .data:0000f8b9 23                               inc hl
                           .data:0000f8ba 23                               inc hl
                           .data:0000f8bb 7e                               ld a,(hl)
                           .data:0000f8bc fe 27                            cp 0x27
                           .data:0000f8be da 01 f9                         jp c,0xf901
                           .data:0000f8c1 cd 12 fa                         call 0xfa12
                           .data:0000f8c4 cd f4 f1                         call 0xf1f4
                           .data:0000f8c7 40                               ld b,b
                           .data:0000f8c8 19                               add hl,de
                           .data:0000f8c9 99                               sbc a,c
                           .data:0000f8ca 99                               sbc a,c
                           .data:0000f8cb 99                               sbc a,c
                           .data:0000f8cc 99                               sbc a,c
                           .data:0000f8cd 99                               sbc a,c
                           .data:0000f8ce 9a                               sbc a,d
                           .data:0000f8cf cd 1e fa                         call 0xfa1e
                           .data:0000f8d2 03                               inc bc
                           .data:0000f8d3 cd 95 f5                         call 0xf595
                           .data:0000f8d6 c2 cf f8                         jp nz,0xf8cf
                           .data:0000f8d9 cd 12 fa                         call 0xfa12
                           .data:0000f8dc 2a 07 01                         ld hl,(0x0107)
                           .data:0000f8df 23                               inc hl
                           .data:0000f8e0 23                               inc hl
                           .data:0000f8e1 23                               inc hl
                           .data:0000f8e2 7e                               ld a,(hl)
                           .data:0000f8e3 fe 0a                            cp 0x0a
                           .data:0000f8e5 da 19 f9                         jp c,0xf919
                           .data:0000f8e8 0b                               dec bc
                           .data:0000f8e9 cd 1e fa                         call 0xfa1e
                           .data:0000f8ec c3 19 f9                         jp 0xf919
                           .data:0000f8ef 2a 07 01                         ld hl,(0x0107)
                           .data:0000f8f2 23                               inc hl
                           .data:0000f8f3 7e                               ld a,(hl)
                           .data:0000f8f4 fe 01                            cp 0x01
                           .data:0000f8f6 c2 01 f9                         jp nz,0xf901
                           .data:0000f8f9 23                               inc hl
                           .data:0000f8fa 23                               inc hl
                           .data:0000f8fb 7e                               ld a,(hl)
                           .data:0000f8fc fe 0a                            cp 0x0a
                           .data:0000f8fe da 19 f9                         jp c,0xf919
                           .data:0000f901 cd e4 f3                         call 0xf3e4
                           .data:0000f904 03                               inc bc
                           .data:0000f905 c3 ef f8                         jp 0xf8ef
                           .data:0000f908 2a 07 01                         ld hl,(0x0107)
                           .data:0000f90b 23                               inc hl
                           .data:0000f90c 7e                               ld a,(hl)
                           .data:0000f90d fe 01                            cp 0x01
                           .data:0000f90f f2 19 f9                         jp p,0xf919
                           .data:0000f912 cd 1e fa                         call 0xfa1e
                           .data:0000f915 0b                               dec bc
                           .data:0000f916 c3 08 f9                         jp 0xf908
                           .data:0000f919 2a f6 04                         ld hl,(0x04f6)
                           .data:0000f91c 2b                               dec hl
                           .data:0000f91d 29                               add hl,hl
                           .data:0000f91e 29                               add hl,hl
                           .data:0000f91f 29                               add hl,hl
                           .data:0000f920 11 de f7                         ld de,0xf7de
                           .data:0000f923 19                               add hl,de
                           .data:0000f924 cd fb f1                         call 0xf1fb
                           .data:0000f927 cd 0f f3                         call 0xf30f
                           .data:0000f92a cd f4 f1                         call 0xf1f4
                           .data:0000f92d 41                               ld b,c
                           .data:0000f92e 0a                               ld a,(bc)
                           .data:0000f92f 00                               nop
                           .data:0000f930 00                               nop
                           .data:0000f931 00                               nop
                           .data:0000f932 00                               nop
                           .data:0000f933 00                               nop
                           .data:0000f934 00                               nop
                           .data:0000f935 cd aa f5                         call 0xf5aa
                           .data:0000f938 ca 49 f9                         jp z,0xf949
                           .data:0000f93b 21 01 00                         ld hl,0x0001
                           .data:0000f93e cd bf f5                         call 0xf5bf
                           .data:0000f941 03                               inc bc
                           .data:0000f942 2a f6 04                         ld hl,(0x04f6)
                           .data:0000f945 23                               inc hl
                           .data:0000f946 22 f6 04                         ld (0x04f6),hl
                           .data:0000f949 60                               ld h,b
                           .data:0000f94a 69                               ld l,c
                           .data:0000f94b 22 f8 04                         ld (0x04f8),hl
                           .data:0000f94e 3e ff                            ld a,0xff
                           .data:0000f950 32 fa 04                         ld (0x04fa),a
                           .data:0000f953 78                               ld a,b
                           .data:0000f954 b7                               or a
                           .data:0000f955 fa 61 f9                         jp m,0xf961
                           .data:0000f958 79                               ld a,c
                           .data:0000f959 fe 0f                            cp 0x0f
                           .data:0000f95b d2 68 f9                         jp nc,0xf968
                           .data:0000f95e c3 88 f9                         jp 0xf988
                           .data:0000f961 79                               ld a,c
                           .data:0000f962 2f                               cpl
                           .data:0000f963 fe 01                            cp 0x01
                           .data:0000f965 da 6f f9                         jp c,0xf96f
                           .data:0000f968 af                               xor a
                           .data:0000f969 32 fa 04                         ld (0x04fa),a
                           .data:0000f96c c3 88 f9                         jp 0xf988
                           .data:0000f96f 2a f4 04                         ld hl,(0x04f4)
                           .data:0000f972 36 30                            ld (hl),0x30
                           .data:0000f974 23                               inc hl
                           .data:0000f975 36 2e                            ld (hl),0x2e
                           .data:0000f977 23                               inc hl
                           .data:0000f978 b7                               or a
                           .data:0000f979 ca 83 f9                         jp z,0xf983
                           .data:0000f97c 36 30                            ld (hl),0x30
                           .data:0000f97e 23                               inc hl
                           .data:0000f97f 3d                               dec a
                           .data:0000f980 c2 7c f9                         jp nz,0xf97c
                           .data:0000f983 22 f4 04                         ld (0x04f4),hl
                           .data:0000f986 3e ff                            ld a,0xff
                           .data:0000f988 4f                               ld c,a
                           .data:0000f989 06 ff                            ld b,0xff
                           .data:0000f98b 04                               inc b
                           .data:0000f98c 3a f6 04                         ld a,(0x04f6)
                           .data:0000f98f b8                               cp b
                           .data:0000f990 da bd f9                         jp c,0xf9bd
                           .data:0000f993 ca bd f9                         jp z,0xf9bd
                           .data:0000f996 2a 07 01                         ld hl,(0x0107)
                           .data:0000f999 23                               inc hl
                           .data:0000f99a 7e                               ld a,(hl)
                           .data:0000f99b fe 01                            cp 0x01
                           .data:0000f99d 3e 30                            ld a,0x30
                           .data:0000f99f c2 a7 f9                         jp nz,0xf9a7
                           .data:0000f9a2 23                               inc hl
                           .data:0000f9a3 23                               inc hl
                           .data:0000f9a4 86                               add a,(hl)
                           .data:0000f9a5 36 00                            ld (hl),0x00
                           .data:0000f9a7 2a f4 04                         ld hl,(0x04f4)
                           .data:0000f9aa 77                               ld (hl),a
                           .data:0000f9ab 23                               inc hl
                           .data:0000f9ac 78                               ld a,b
                           .data:0000f9ad b9                               cp c
                           .data:0000f9ae c2 b4 f9                         jp nz,0xf9b4
                           .data:0000f9b1 36 2e                            ld (hl),0x2e
                           .data:0000f9b3 23                               inc hl
                           .data:0000f9b4 22 f4 04                         ld (0x04f4),hl
                           .data:0000f9b7 cd 1e fa                         call 0xfa1e
                           .data:0000f9ba c3 8b f9                         jp 0xf98b
                           .data:0000f9bd 2a f4 04                         ld hl,(0x04f4)
                           .data:0000f9c0 2b                               dec hl
                           .data:0000f9c1 7e                               ld a,(hl)
                           .data:0000f9c2 fe 30                            cp 0x30
                           .data:0000f9c4 ca c0 f9                         jp z,0xf9c0
                           .data:0000f9c7 3a fa 04                         ld a,(0x04fa)
                           .data:0000f9ca b7                               or a
                           .data:0000f9cb ca d8 f9                         jp z,0xf9d8
                           .data:0000f9ce 7e                               ld a,(hl)
                           .data:0000f9cf fe 2e                            cp 0x2e
                           .data:0000f9d1 ca 0e fa                         jp z,0xfa0e
                           .data:0000f9d4 23                               inc hl
                           .data:0000f9d5 c3 0e fa                         jp 0xfa0e
                           .data:0000f9d8 23                               inc hl
                           .data:0000f9d9 36 65                            ld (hl),0x65
                           .data:0000f9db 23                               inc hl
                           .data:0000f9dc 36 2b                            ld (hl),0x2b
                           .data:0000f9de 3a f9 04                         ld a,(0x04f9)
                           .data:0000f9e1 b7                               or a
                           .data:0000f9e2 3a f8 04                         ld a,(0x04f8)
                           .data:0000f9e5 f2 ec f9                         jp p,0xf9ec
                           .data:0000f9e8 36 2d                            ld (hl),0x2d
                           .data:0000f9ea 2f                               cpl
                           .data:0000f9eb 3c                               inc a
                           .data:0000f9ec 23                               inc hl
                           .data:0000f9ed fe 64                            cp 0x64
                           .data:0000f9ef da f7 f9                         jp c,0xf9f7
                           .data:0000f9f2 36 31                            ld (hl),0x31
                           .data:0000f9f4 23                               inc hl
                           .data:0000f9f5 d6 64                            sub 0x64
                           .data:0000f9f7 06 00                            ld b,0x00
                           .data:0000f9f9 fe 0a                            cp 0x0a
                           .data:0000f9fb da 04 fa                         jp c,0xfa04
                           .data:0000f9fe 04                               inc b
                           .data:0000f9ff d6 0a                            sub 0x0a
                           .data:0000fa01 c3 f9 f9                         jp 0xf9f9
                           .data:0000fa04 c6 30                            add a,0x30
                           .data:0000fa06 5f                               ld e,a
                           .data:0000fa07 3e 30                            ld a,0x30
                           .data:0000fa09 80                               add a,b
                           .data:0000fa0a 77                               ld (hl),a
                           .data:0000fa0b 23                               inc hl
                           .data:0000fa0c 73                               ld (hl),e
                           .data:0000fa0d 23                               inc hl
                           .data:0000fa0e 36 00                            ld (hl),0x00
                           .data:0000fa10 c1                               pop bc
                           .data:0000fa11 c9                               ret
                           .data:0000fa12 cd a6 f2                         call 0xf2a6
                           .data:0000fa15 21 01 00                         ld hl,0x0001
                           .data:0000fa18 cd bf f5                         call 0xf5bf
                           .data:0000fa1b c3 e4 f3                         jp 0xf3e4
                           .data:0000fa1e c5                               push bc
                           .data:0000fa1f 2a 07 01                         ld hl,(0x0107)
                           .data:0000fa22 23                               inc hl
                           .data:0000fa23 34                               inc (hl)
                           .data:0000fa24 11 09 00                         ld de,0x0009
                           .data:0000fa27 19                               add hl,de
                           .data:0000fa28 af                               xor a
                           .data:0000fa29 06 08                            ld b,0x08
                           .data:0000fa2b c5                               push bc
                           .data:0000fa2c 5e                               ld e,(hl)
                           .data:0000fa2d eb                               ex de,hl
                           .data:0000fa2e 26 00                            ld h,0x00
                           .data:0000fa30 29                               add hl,hl
                           .data:0000fa31 44                               ld b,h
                           .data:0000fa32 4d                               ld c,l
                           .data:0000fa33 29                               add hl,hl
                           .data:0000fa34 29                               add hl,hl
                           .data:0000fa35 09                               add hl,bc
                           .data:0000fa36 eb                               ex de,hl
                           .data:0000fa37 83                               add a,e
                           .data:0000fa38 23                               inc hl
                           .data:0000fa39 77                               ld (hl),a
                           .data:0000fa3a 7a                               ld a,d
                           .data:0000fa3b ce 00                            adc a,0x00
                           .data:0000fa3d 2b                               dec hl
                           .data:0000fa3e 2b                               dec hl
                           .data:0000fa3f c1                               pop bc
                           .data:0000fa40 05                               dec b
                           .data:0000fa41 c2 2b fa                         jp nz,0xfa2b
                           .data:0000fa44 23                               inc hl
                           .data:0000fa45 77                               ld (hl),a
                           .data:0000fa46 b7                               or a
                           .data:0000fa47 ca 34 f6                         jp z,0xf634
                           .data:0000fa4a 2b                               dec hl
                           .data:0000fa4b 2b                               dec hl
                           .data:0000fa4c 7e                               ld a,(hl)
                           .data:0000fa4d b7                               or a
                           .data:0000fa4e fa 56 fa                         jp m,0xfa56
                           .data:0000fa51 fe 40                            cp 0x40
                           .data:0000fa53 d2 bf f6                         jp nc,0xf6bf
                           .data:0000fa56 c1                               pop bc
                           .data:0000fa57 c9                               ret
                           .data:0000fa58 cd 90 ff                         call 0xff90
                           .data:0000fa5b 21 08 00                         ld hl,0x0008
                           .data:0000fa5e 39                               add hl,sp
                           .data:0000fa5f 4e                               ld c,(hl)
                           .data:0000fa60 23                               inc hl
                           .data:0000fa61 46                               ld b,(hl)
                           .data:0000fa62 50                               ld d,b
                           .data:0000fa63 59                               ld e,c
                           .data:0000fa64 21 61 00                         ld hl,0x0061
                           .data:0000fa67 cd 3d fb                         call 0xfb3d
                           .data:0000fa6a 28 13                            jr z,0xfa7f
                           .data:0000fa6c 50                               ld d,b
                           .data:0000fa6d 59                               ld e,c
                           .data:0000fa6e 21 7a 00                         ld hl,0x007a
                           .data:0000fa71 cd 3e fb                         call 0xfb3e
                           .data:0000fa74 28 09                            jr z,0xfa7f
                           .data:0000fa76 60                               ld h,b
                           .data:0000fa77 69                               ld l,c
                           .data:0000fa78 11 20 00                         ld de,0x0020
                           .data:0000fa7b cd 6c fc                         call 0xfc6c
                           .data:0000fa7e c9                               ret
                           .data:0000fa7f 60                               ld h,b
                           .data:0000fa80 69                               ld l,c
                           .data:0000fa81 c9                               ret
                           .data:0000fa82 cd 90 ff                         call 0xff90
                           .data:0000fa85 21 08 00                         ld hl,0x0008
                           .data:0000fa88 39                               add hl,sp
                           .data:0000fa89 4e                               ld c,(hl)
                           .data:0000fa8a 23                               inc hl
                           .data:0000fa8b 46                               ld b,(hl)
                           .data:0000fa8c 50                               ld d,b
                           .data:0000fa8d 59                               ld e,c
                           .data:0000fa8e 21 41 00                         ld hl,0x0041
                           .data:0000fa91 cd 3d fb                         call 0xfb3d
                           .data:0000fa94 28 13                            jr z,0xfaa9
                           .data:0000fa96 50                               ld d,b
                           .data:0000fa97 59                               ld e,c
                           .data:0000fa98 21 5a 00                         ld hl,0x005a
                           .data:0000fa9b cd 3e fb                         call 0xfb3e
                           .data:0000fa9e 28 09                            jr z,0xfaa9
                           .data:0000faa0 60                               ld h,b
                           .data:0000faa1 69                               ld l,c
                           .data:0000faa2 11 20 00                         ld de,0x0020
                           .data:0000faa5 cd 6c fc                         call 0xfc6c
                           .data:0000faa8 c9                               ret
                           .data:0000faa9 60                               ld h,b
                           .data:0000faaa 69                               ld l,c
                           .data:0000faab c9                               ret
                           .data:0000faac cd 90 ff                         call 0xff90
                           .data:0000faaf 21 08 00                         ld hl,0x0008
                           .data:0000fab2 39                               add hl,sp
                           .data:0000fab3 4e                               ld c,(hl)
                           .data:0000fab4 23                               inc hl
                           .data:0000fab5 46                               ld b,(hl)
                           .data:0000fab6 60                               ld h,b
                           .data:0000fab7 69                               ld l,c
                           .data:0000fab8 11 20 00                         ld de,0x0020
                           .data:0000fabb cd 24 fb                         call 0xfb24
                           .data:0000fabe 20 18                            jr nz,0xfad8
                           .data:0000fac0 60                               ld h,b
                           .data:0000fac1 69                               ld l,c
                           .data:0000fac2 11 09 00                         ld de,0x0009
                           .data:0000fac5 cd 24 fb                         call 0xfb24
                           .data:0000fac8 20 0e                            jr nz,0xfad8
                           .data:0000faca 60                               ld h,b
                           .data:0000facb 69                               ld l,c
                           .data:0000facc 11 0a 00                         ld de,0x000a
                           .data:0000facf cd 24 fb                         call 0xfb24
                           .data:0000fad2 20 04                            jr nz,0xfad8
                           .data:0000fad4 21 00 00                         ld hl,0x0000
                           .data:0000fad7 c9                               ret
                           .data:0000fad8 21 01 00                         ld hl,0x0001
                           .data:0000fadb c9                               ret
                           .data:0000fadc 7c                               ld a,h
                           .data:0000fadd a2                               and d
                           .data:0000fade 67                               ld h,a
                           .data:0000fadf 7d                               ld a,l
                           .data:0000fae0 a3                               and e
                           .data:0000fae1 6f                               ld l,a
                           .data:0000fae2 b4                               or h
                           .data:0000fae3 c9                               ret
                           .data:0000fae4 7c                               ld a,h
                           .data:0000fae5 2f                               cpl
                           .data:0000fae6 67                               ld h,a
                           .data:0000fae7 7d                               ld a,l
                           .data:0000fae8 2f                               cpl
                           .data:0000fae9 6f                               ld l,a
                           .data:0000faea b4                               or h
                           .data:0000faeb c9                               ret
                           .data:0000faec e9                               jp (hl)
                           .data:0000faed e1                               pop hl
                           .data:0000faee c5                               push bc
                           .data:0000faef 5e                               ld e,(hl)
                           .data:0000faf0 23                               inc hl
                           .data:0000faf1 56                               ld d,(hl)
                           .data:0000faf2 23                               inc hl
                           .data:0000faf3 44                               ld b,h
                           .data:0000faf4 4d                               ld c,l
                           .data:0000faf5 21 00 00                         ld hl,0x0000
                           .data:0000faf8 39                               add hl,sp
                           .data:0000faf9 eb                               ex de,hl
                           .data:0000fafa 39                               add hl,sp
                           .data:0000fafb f9                               ld sp,hl
                           .data:0000fafc d5                               push de
                           .data:0000fafd 60                               ld h,b
                           .data:0000fafe 69                               ld l,c
                           .data:0000faff cd ec fa                         call 0xfaec
                           .data:0000fb02 eb                               ex de,hl
                           .data:0000fb03 e1                               pop hl
                           .data:0000fb04 f9                               ld sp,hl
                           .data:0000fb05 c1                               pop bc
                           .data:0000fb06 eb                               ex de,hl
                           .data:0000fb07 7c                               ld a,h
                           .data:0000fb08 b5                               or l
                           .data:0000fb09 c9                               ret
                           .data:0000fb0a 7a                               ld a,d
                           .data:0000fb0b ac                               xor h
                           .data:0000fb0c 32 fc 04                         ld (0x04fc),a
                           .data:0000fb0f cd 7b fb                         call 0xfb7b
                           .data:0000fb12 eb                               ex de,hl
                           .data:0000fb13 3a fc 04                         ld a,(0x04fc)
                           .data:0000fb16 b7                               or a
                           .data:0000fb17 fa d4 fb                         jp m,0xfbd4
                           .data:0000fb1a 7d                               ld a,l
                           .data:0000fb1b b4                               or h
                           .data:0000fb1c c9                               ret
                           .data:0000fb1d cd 94 fb                         call 0xfb94
                           .data:0000fb20 eb                               ex de,hl
                           .data:0000fb21 7d                               ld a,l
                           .data:0000fb22 b4                               or h
                           .data:0000fb23 c9                               ret
                           .data:0000fb24 a7                               and a
                           .data:0000fb25 ed 52                            sbc hl,de
                           .data:0000fb27 28 0c                            jr z,0xfb35
                           .data:0000fb29 21 00 00                         ld hl,0x0000
                           .data:0000fb2c af                               xor a
                           .data:0000fb2d 54                               ld d,h
                           .data:0000fb2e 5d                               ld e,l
                           .data:0000fb2f c9                               ret
                           .data:0000fb30 a7                               and a
                           .data:0000fb31 ed 52                            sbc hl,de
                           .data:0000fb33 28 f4                            jr z,0xfb29
                           .data:0000fb35 21 01 00                         ld hl,0x0001
                           .data:0000fb38 7d                               ld a,l
                           .data:0000fb39 b4                               or h
                           .data:0000fb3a 54                               ld d,h
                           .data:0000fb3b 5d                               ld e,l
                           .data:0000fb3c c9                               ret
                           .data:0000fb3d eb                               ex de,hl
                           .data:0000fb3e 7c                               ld a,h
                           .data:0000fb3f aa                               xor d
                           .data:0000fb40 fa 4c fb                         jp m,0xfb4c
                           .data:0000fb43 af                               xor a
                           .data:0000fb44 ed 52                            sbc hl,de
                           .data:0000fb46 67                               ld h,a
                           .data:0000fb47 3f                               ccf
                           .data:0000fb48 ce 00                            adc a,0x00
                           .data:0000fb4a 6f                               ld l,a
                           .data:0000fb4b c9                               ret
                           .data:0000fb4c 7a                               ld a,d
                           .data:0000fb4d 07                               rlca
                           .data:0000fb4e e6 01                            and 0x01
                           .data:0000fb50 6f                               ld l,a
                           .data:0000fb51 26 00                            ld h,0x00
                           .data:0000fb53 c9                               ret
                           .data:0000fb54 eb                               ex de,hl
                           .data:0000fb55 7c                               ld a,h
                           .data:0000fb56 aa                               xor d
                           .data:0000fb57 fa 62 fb                         jp m,0xfb62
                           .data:0000fb5a af                               xor a
                           .data:0000fb5b ed 52                            sbc hl,de
                           .data:0000fb5d 67                               ld h,a
                           .data:0000fb5e ce 00                            adc a,0x00
                           .data:0000fb60 6f                               ld l,a
                           .data:0000fb61 c9                               ret
                           .data:0000fb62 7c                               ld a,h
                           .data:0000fb63 07                               rlca
                           .data:0000fb64 e6 01                            and 0x01
                           .data:0000fb66 6f                               ld l,a
                           .data:0000fb67 26 00                            ld h,0x00
                           .data:0000fb69 c9                               ret
                           .data:0000fb6a 7a                               ld a,d
                           .data:0000fb6b 32 fc 04                         ld (0x04fc),a
                           .data:0000fb6e cd 7b fb                         call 0xfb7b
                           .data:0000fb71 3a fc 04                         ld a,(0x04fc)
                           .data:0000fb74 b7                               or a
                           .data:0000fb75 fa d4 fb                         jp m,0xfbd4
                           .data:0000fb78 7c                               ld a,h
                           .data:0000fb79 b5                               or l
                           .data:0000fb7a c9                               ret
                           .data:0000fb7b 7c                               ld a,h
                           .data:0000fb7c b7                               or a
                           .data:0000fb7d f2 86 fb                         jp p,0xfb86
                           .data:0000fb80 2f                               cpl
                           .data:0000fb81 67                               ld h,a
                           .data:0000fb82 7d                               ld a,l
                           .data:0000fb83 2f                               cpl
                           .data:0000fb84 6f                               ld l,a
                           .data:0000fb85 23                               inc hl
                           .data:0000fb86 7a                               ld a,d
                           .data:0000fb87 b7                               or a
                           .data:0000fb88 f2 94 fb                         jp p,0xfb94
                           .data:0000fb8b 2f                               cpl
                           .data:0000fb8c 57                               ld d,a
                           .data:0000fb8d 7b                               ld a,e
                           .data:0000fb8e 2f                               cpl
                           .data:0000fb8f 5f                               ld e,a
                           .data:0000fb90 13                               inc de
                           .data:0000fb91 c3 94 fb                         jp 0xfb94
                           .data:0000fb94 c5                               push bc
                           .data:0000fb95 44                               ld b,h
                           .data:0000fb96 4d                               ld c,l
                           .data:0000fb97 21 00 00                         ld hl,0x0000
                           .data:0000fb9a 3e 10                            ld a,0x10
                           .data:0000fb9c 29                               add hl,hl
                           .data:0000fb9d eb                               ex de,hl
                           .data:0000fb9e 29                               add hl,hl
                           .data:0000fb9f eb                               ex de,hl
                           .data:0000fba0 d2 a4 fb                         jp nc,0xfba4
                           .data:0000fba3 23                               inc hl
                           .data:0000fba4 a7                               and a
                           .data:0000fba5 ed 42                            sbc hl,bc
                           .data:0000fba7 d2 b3 fb                         jp nc,0xfbb3
                           .data:0000fbaa 09                               add hl,bc
                           .data:0000fbab 3d                               dec a
                           .data:0000fbac c2 9c fb                         jp nz,0xfb9c
                           .data:0000fbaf c1                               pop bc
                           .data:0000fbb0 7d                               ld a,l
                           .data:0000fbb1 b4                               or h
                           .data:0000fbb2 c9                               ret
                           .data:0000fbb3 13                               inc de
                           .data:0000fbb4 3d                               dec a
                           .data:0000fbb5 c2 9c fb                         jp nz,0xfb9c
                           .data:0000fbb8 c1                               pop bc
                           .data:0000fbb9 7d                               ld a,l
                           .data:0000fbba b4                               or h
                           .data:0000fbbb c9                               ret
                           .data:0000fbbc c5                               push bc
                           .data:0000fbbd 44                               ld b,h
                           .data:0000fbbe 4d                               ld c,l
                           .data:0000fbbf 21 00 00                         ld hl,0x0000
                           .data:0000fbc2 3e 10                            ld a,0x10
                           .data:0000fbc4 29                               add hl,hl
                           .data:0000fbc5 eb                               ex de,hl
                           .data:0000fbc6 29                               add hl,hl
                           .data:0000fbc7 eb                               ex de,hl
                           .data:0000fbc8 d2 cc fb                         jp nc,0xfbcc
                           .data:0000fbcb 09                               add hl,bc
                           .data:0000fbcc 3d                               dec a
                           .data:0000fbcd c2 c4 fb                         jp nz,0xfbc4
                           .data:0000fbd0 c1                               pop bc
                           .data:0000fbd1 7d                               ld a,l
                           .data:0000fbd2 b4                               or h
                           .data:0000fbd3 c9                               ret
                           .data:0000fbd4 7d                               ld a,l
                           .data:0000fbd5 2f                               cpl
                           .data:0000fbd6 6f                               ld l,a
                           .data:0000fbd7 7c                               ld a,h
                           .data:0000fbd8 2f                               cpl
                           .data:0000fbd9 67                               ld h,a
                           .data:0000fbda 23                               inc hl
                           .data:0000fbdb 7d                               ld a,l
                           .data:0000fbdc b4                               or h
                           .data:0000fbdd c9                               ret
                           .data:0000fbde 7c                               ld a,h
                           .data:0000fbdf b5                               or l
                           .data:0000fbe0 ca 35 fb                         jp z,0xfb35
                           .data:0000fbe3 c3 29 fb                         jp 0xfb29
                           .data:0000fbe6 7c                               ld a,h
                           .data:0000fbe7 b2                               or d
                           .data:0000fbe8 67                               ld h,a
                           .data:0000fbe9 7d                               ld a,l
                           .data:0000fbea b3                               or e
                           .data:0000fbeb 6f                               ld l,a
                           .data:0000fbec b4                               or h
                           .data:0000fbed c9                               ret
                           .data:0000fbee eb                               ex de,hl
                           .data:0000fbef 7b                               ld a,e
                           .data:0000fbf0 e6 1f                            and 0x1f
                           .data:0000fbf2 5f                               ld e,a
                           .data:0000fbf3 ca 15 fc                         jp z,0xfc15
                           .data:0000fbf6 7c                               ld a,h
                           .data:0000fbf7 b4                               or h
                           .data:0000fbf8 f2 5f fc                         jp p,0xfc5f
                           .data:0000fbfb 7c                               ld a,h
                           .data:0000fbfc 37                               scf
                           .data:0000fbfd 1f                               rra
                           .data:0000fbfe 67                               ld h,a
                           .data:0000fbff 7d                               ld a,l
                           .data:0000fc00 1f                               rra
                           .data:0000fc01 6f                               ld l,a
                           .data:0000fc02 1d                               dec e
                           .data:0000fc03 c2 fb fb                         jp nz,0xfbfb
                           .data:0000fc06 b4                               or h
                           .data:0000fc07 c9                               ret
                           .data:0000fc08 eb                               ex de,hl
                           .data:0000fc09 7b                               ld a,e
                           .data:0000fc0a e6 1f                            and 0x1f
                           .data:0000fc0c 5f                               ld e,a
                           .data:0000fc0d ca 15 fc                         jp z,0xfc15
                           .data:0000fc10 29                               add hl,hl
                           .data:0000fc11 1d                               dec e
                           .data:0000fc12 c2 10 fc                         jp nz,0xfc10
                           .data:0000fc15 7d                               ld a,l
                           .data:0000fc16 b4                               or h
                           .data:0000fc17 c9                               ret
                           .data:0000fc18 eb                               ex de,hl
                           .data:0000fc19 a7                               and a
                           .data:0000fc1a ed 52                            sbc hl,de
                           .data:0000fc1c c9                               ret
                           .data:0000fc1d eb                               ex de,hl
                           .data:0000fc1e e1                               pop hl
                           .data:0000fc1f c5                               push bc
                           .data:0000fc20 42                               ld b,d
                           .data:0000fc21 4b                               ld c,e
                           .data:0000fc22 5e                               ld e,(hl)
                           .data:0000fc23 23                               inc hl
                           .data:0000fc24 56                               ld d,(hl)
                           .data:0000fc25 1b                               dec de
                           .data:0000fc26 7a                               ld a,d
                           .data:0000fc27 b7                               or a
                           .data:0000fc28 fa 3d fc                         jp m,0xfc3d
                           .data:0000fc2b 23                               inc hl
                           .data:0000fc2c 79                               ld a,c
                           .data:0000fc2d be                               cp (hl)
                           .data:0000fc2e ca 37 fc                         jp z,0xfc37
                           .data:0000fc31 23                               inc hl
                           .data:0000fc32 23                               inc hl
                           .data:0000fc33 23                               inc hl
                           .data:0000fc34 c3 25 fc                         jp 0xfc25
                           .data:0000fc37 23                               inc hl
                           .data:0000fc38 78                               ld a,b
                           .data:0000fc39 be                               cp (hl)
                           .data:0000fc3a c2 32 fc                         jp nz,0xfc32
                           .data:0000fc3d 23                               inc hl
                           .data:0000fc3e 7e                               ld a,(hl)
                           .data:0000fc3f 23                               inc hl
                           .data:0000fc40 66                               ld h,(hl)
                           .data:0000fc41 6f                               ld l,a
                           .data:0000fc42 c1                               pop bc
                           .data:0000fc43 e9                               jp (hl)
                           .data:0000fc44 eb                               ex de,hl
                           .data:0000fc45 af                               xor a
                           .data:0000fc46 ed 52                            sbc hl,de
                           .data:0000fc48 67                               ld h,a
                           .data:0000fc49 3f                               ccf
                           .data:0000fc4a ce 00                            adc a,0x00
                           .data:0000fc4c 6f                               ld l,a
                           .data:0000fc4d c9                               ret
                           .data:0000fc4e eb                               ex de,hl
                           .data:0000fc4f af                               xor a
                           .data:0000fc50 ed 52                            sbc hl,de
                           .data:0000fc52 67                               ld h,a
                           .data:0000fc53 ce 00                            adc a,0x00
                           .data:0000fc55 6f                               ld l,a
                           .data:0000fc56 c9                               ret
                           .data:0000fc57 eb                               ex de,hl
                           .data:0000fc58 7b                               ld a,e
                           .data:0000fc59 e6 1f                            and 0x1f
                           .data:0000fc5b 5f                               ld e,a
                           .data:0000fc5c ca 15 fc                         jp z,0xfc15
                           .data:0000fc5f 7c                               ld a,h
                           .data:0000fc60 b7                               or a
                           .data:0000fc61 1f                               rra
                           .data:0000fc62 67                               ld h,a
                           .data:0000fc63 7d                               ld a,l
                           .data:0000fc64 1f                               rra
                           .data:0000fc65 6f                               ld l,a
                           .data:0000fc66 1d                               dec e
                           .data:0000fc67 c2 5f fc                         jp nz,0xfc5f
                           .data:0000fc6a b4                               or h
                           .data:0000fc6b c9                               ret
                           .data:0000fc6c 7c                               ld a,h
                           .data:0000fc6d aa                               xor d
                           .data:0000fc6e 67                               ld h,a
                           .data:0000fc6f 7d                               ld a,l
                           .data:0000fc70 ab                               xor e
                           .data:0000fc71 6f                               ld l,a
                           .data:0000fc72 b4                               or h
                           .data:0000fc73 c9                               ret
                           .data:0000fc74 d1                               pop de
                           .data:0000fc75 21 02 00                         ld hl,0x0002
                           .data:0000fc78 39                               add hl,sp
                           .data:0000fc79 c5                               push bc
                           .data:0000fc7a d5                               push de
                           .data:0000fc7b 11 fd 04                         ld de,0x04fd
                           .data:0000fc7e 01 06 00                         ld bc,0x0006
                           .data:0000fc81 ed b0                            ldir
                           .data:0000fc83 21 88 fc                         ld hl,0xfc88
                           .data:0000fc86 e3                               ex (sp),hl
                           .data:0000fc87 e9                               jp (hl)
                           .data:0000fc88 c1                               pop bc
                           .data:0000fc89 7c                               ld a,h
                           .data:0000fc8a b5                               or l
                           .data:0000fc8b c9                               ret
                           .data:0000fc8c 23                               inc hl
                           .data:0000fc8d 23                               inc hl
                           .data:0000fc8e 7e                               ld a,(hl)
                           .data:0000fc8f 23                               inc hl
                           .data:0000fc90 66                               ld h,(hl)
                           .data:0000fc91 6f                               ld l,a
                           .data:0000fc92 b4                               or h
                           .data:0000fc93 c9                               ret
                           .data:0000fc94 c5                               push bc
                           .data:0000fc95 af                               xor a
                           .data:0000fc96 32 03 05                         ld (0x0503),a
                           .data:0000fc99 32 04 05                         ld (0x0504),a
                           .data:0000fc9c 32 05 05                         ld (0x0505),a
                           .data:0000fc9f 6f                               ld l,a
                           .data:0000fca0 67                               ld h,a
                           .data:0000fca1 22 06 05                         ld (0x0506),hl
                           .data:0000fca4 cd bf f5                         call 0xf5bf
                           .data:0000fca7 21 04 00                         ld hl,0x0004
                           .data:0000fcaa 39                               add hl,sp
                           .data:0000fcab 4e                               ld c,(hl)
                           .data:0000fcac 23                               inc hl
                           .data:0000fcad 46                               ld b,(hl)
                           .data:0000fcae 0a                               ld a,(bc)
                           .data:0000fcaf fe 2d                            cp 0x2d
                           .data:0000fcb1 c2 ba fc                         jp nz,0xfcba
                           .data:0000fcb4 32 03 05                         ld (0x0503),a
                           .data:0000fcb7 c3 bf fc                         jp 0xfcbf
                           .data:0000fcba fe 2b                            cp 0x2b
                           .data:0000fcbc c2 c0 fc                         jp nz,0xfcc0
                           .data:0000fcbf 03                               inc bc
                           .data:0000fcc0 0a                               ld a,(bc)
                           .data:0000fcc1 fe 30                            cp 0x30
                           .data:0000fcc3 da ef fc                         jp c,0xfcef
                           .data:0000fcc6 fe 3a                            cp 0x3a
                           .data:0000fcc8 d2 ef fc                         jp nc,0xfcef
                           .data:0000fccb f5                               push af
                           .data:0000fccc cd 1e fa                         call 0xfa1e
                           .data:0000fccf cd a6 f2                         call 0xf2a6
                           .data:0000fcd2 f1                               pop af
                           .data:0000fcd3 d6 30                            sub 0x30
                           .data:0000fcd5 6f                               ld l,a
                           .data:0000fcd6 26 00                            ld h,0x00
                           .data:0000fcd8 cd bf f5                         call 0xf5bf
                           .data:0000fcdb cd 0f f3                         call 0xf30f
                           .data:0000fcde 3a 05 05                         ld a,(0x0505)
                           .data:0000fce1 b7                               or a
                           .data:0000fce2 ca bf fc                         jp z,0xfcbf
                           .data:0000fce5 2a 06 05                         ld hl,(0x0506)
                           .data:0000fce8 2b                               dec hl
                           .data:0000fce9 22 06 05                         ld (0x0506),hl
                           .data:0000fcec c3 bf fc                         jp 0xfcbf
                           .data:0000fcef fe 2e                            cp 0x2e
                           .data:0000fcf1 c2 01 fd                         jp nz,0xfd01
                           .data:0000fcf4 21 05 05                         ld hl,0x0505
                           .data:0000fcf7 7e                               ld a,(hl)
                           .data:0000fcf8 b7                               or a
                           .data:0000fcf9 c2 01 fd                         jp nz,0xfd01
                           .data:0000fcfc 36 01                            ld (hl),0x01
                           .data:0000fcfe c3 bf fc                         jp 0xfcbf
                           .data:0000fd01 21 00 00                         ld hl,0x0000
                           .data:0000fd04 f6 20                            or 0x20
                           .data:0000fd06 fe 65                            cp 0x65
                           .data:0000fd08 c2 4e fd                         jp nz,0xfd4e
                           .data:0000fd0b 03                               inc bc
                           .data:0000fd0c 0a                               ld a,(bc)
                           .data:0000fd0d fe 2d                            cp 0x2d
                           .data:0000fd0f c2 18 fd                         jp nz,0xfd18
                           .data:0000fd12 32 04 05                         ld (0x0504),a
                           .data:0000fd15 c3 1d fd                         jp 0xfd1d
                           .data:0000fd18 fe 2b                            cp 0x2b
                           .data:0000fd1a c2 1e fd                         jp nz,0xfd1e
                           .data:0000fd1d 03                               inc bc
                           .data:0000fd1e 0a                               ld a,(bc)
                           .data:0000fd1f fe 30                            cp 0x30
                           .data:0000fd21 da 38 fd                         jp c,0xfd38
                           .data:0000fd24 fe 3a                            cp 0x3a
                           .data:0000fd26 d2 38 fd                         jp nc,0xfd38
                           .data:0000fd29 d6 30                            sub 0x30
                           .data:0000fd2b 29                               add hl,hl
                           .data:0000fd2c 54                               ld d,h
                           .data:0000fd2d 5d                               ld e,l
                           .data:0000fd2e 29                               add hl,hl
                           .data:0000fd2f 29                               add hl,hl
                           .data:0000fd30 19                               add hl,de
                           .data:0000fd31 5f                               ld e,a
                           .data:0000fd32 16 00                            ld d,0x00
                           .data:0000fd34 19                               add hl,de
                           .data:0000fd35 c3 1d fd                         jp 0xfd1d
                           .data:0000fd38 3a 04 05                         ld a,(0x0504)
                           .data:0000fd3b b7                               or a
                           .data:0000fd3c ca 46 fd                         jp z,0xfd46
                           .data:0000fd3f 7c                               ld a,h
                           .data:0000fd40 2f                               cpl
                           .data:0000fd41 67                               ld h,a
                           .data:0000fd42 7d                               ld a,l
                           .data:0000fd43 2f                               cpl
                           .data:0000fd44 6f                               ld l,a
                           .data:0000fd45 23                               inc hl
                           .data:0000fd46 eb                               ex de,hl
                           .data:0000fd47 2a 06 05                         ld hl,(0x0506)
                           .data:0000fd4a 19                               add hl,de
                           .data:0000fd4b 22 06 05                         ld (0x0506),hl
                           .data:0000fd4e 2a 06 05                         ld hl,(0x0506)
                           .data:0000fd51 7c                               ld a,h
                           .data:0000fd52 b7                               or a
                           .data:0000fd53 f2 94 fd                         jp p,0xfd94
                           .data:0000fd56 fe ff                            cp 0xff
                           .data:0000fd58 c2 ae fd                         jp nz,0xfdae
                           .data:0000fd5b 7d                               ld a,l
                           .data:0000fd5c 2f                               cpl
                           .data:0000fd5d 3c                               inc a
                           .data:0000fd5e 4f                               ld c,a
                           .data:0000fd5f fe a6                            cp 0xa6
                           .data:0000fd61 d2 ae fd                         jp nc,0xfdae
                           .data:0000fd64 fe 96                            cp 0x96
                           .data:0000fd66 da 7b fd                         jp c,0xfd7b
                           .data:0000fd69 cd f4 f1                         call 0xf1f4
                           .data:0000fd6c 47                               ld b,a
                           .data:0000fd6d 23                               inc hl
                           .data:0000fd6e 86                               add a,(hl)
                           .data:0000fd6f f2 6f c1                         jp p,0xc16f
                           .data:0000fd72 00                               nop
                           .data:0000fd73 00                               nop
                           .data:0000fd74 cd e4 f3                         call 0xf3e4
                           .data:0000fd77 79                               ld a,c
                           .data:0000fd78 d6 10                            sub 0x10
                           .data:0000fd7a 4f                               ld c,a
                           .data:0000fd7b cd a6 f2                         call 0xf2a6
                           .data:0000fd7e 21 01 00                         ld hl,0x0001
                           .data:0000fd81 cd bf f5                         call 0xf5bf
                           .data:0000fd84 cd 1e fa                         call 0xfa1e
                           .data:0000fd87 0d                               dec c
                           .data:0000fd88 c2 84 fd                         jp nz,0xfd84
                           .data:0000fd8b cd a6 f2                         call 0xf2a6
                           .data:0000fd8e cd e4 f3                         call 0xf3e4
                           .data:0000fd91 c3 a4 fd                         jp 0xfda4
                           .data:0000fd94 c2 ae fd                         jp nz,0xfdae
                           .data:0000fd97 7d                               ld a,l
                           .data:0000fd98 b7                               or a
                           .data:0000fd99 ca a4 fd                         jp z,0xfda4
                           .data:0000fd9c 4f                               ld c,a
                           .data:0000fd9d cd 1e fa                         call 0xfa1e
                           .data:0000fda0 0d                               dec c
                           .data:0000fda1 c2 9d fd                         jp nz,0xfd9d
                           .data:0000fda4 3a 03 05                         ld a,(0x0503)
                           .data:0000fda7 b7                               or a
                           .data:0000fda8 ca ae fd                         jp z,0xfdae
                           .data:0000fdab cd b5 f2                         call 0xf2b5
                           .data:0000fdae c1                               pop bc
                           .data:0000fdaf c9                               ret
                           .data:0000fdb0 cd 90 ff                         call 0xff90
                           .data:0000fdb3 21 08 00                         ld hl,0x0008
                           .data:0000fdb6 39                               add hl,sp
                           .data:0000fdb7 e5                               push hl
                           .data:0000fdb8 21 0a 00                         ld hl,0x000a
                           .data:0000fdbb 39                               add hl,sp
                           .data:0000fdbc cd 09 f2                         call 0xf209
                           .data:0000fdbf cd f4 f1                         call 0xf1f4
                           .data:0000fdc2 41                               ld b,c
                           .data:0000fdc3 5a                               ld e,d
                           .data:0000fdc4 00                               nop
                           .data:0000fdc5 00                               nop
                           .data:0000fdc6 00                               nop
                           .data:0000fdc7 00                               nop
                           .data:0000fdc8 00                               nop
                           .data:0000fdc9 00                               nop
                           .data:0000fdca cd 0f f3                         call 0xf30f
                           .data:0000fdcd e1                               pop hl
                           .data:0000fdce cd 28 f2                         call 0xf228
                           .data:0000fdd1 21 08 00                         ld hl,0x0008
                           .data:0000fdd4 39                               add hl,sp
                           .data:0000fdd5 cd 09 f2                         call 0xf209
                           .data:0000fdd8 cd 4a f2                         call 0xf24a
                           .data:0000fddb cd e2 fd                         call 0xfde2
                           .data:0000fdde cd 87 ff                         call 0xff87
                           .data:0000fde1 c9                               ret
                           .data:0000fde2 cd ed fa                         call 0xfaed
                           .data:0000fde5 f8                               ret m
                           .data:0000fde6 ff                               rst 0x38
                           .data:0000fde7 21 10 00                         ld hl,0x0010
                           .data:0000fdea 39                               add hl,sp
                           .data:0000fdeb cd 09 f2                         call 0xf209
                           .data:0000fdee 21 cf 01                         ld hl,0x01cf
                           .data:0000fdf1 cd fb f1                         call 0xf1fb
                           .data:0000fdf4 cd 95 f5                         call 0xf595
                           .data:0000fdf7 28 1e                            jr z,0xfe17
                           .data:0000fdf9 21 10 00                         ld hl,0x0010
                           .data:0000fdfc 39                               add hl,sp
                           .data:0000fdfd e5                               push hl
                           .data:0000fdfe cd 02 f2                         call 0xf202
                           .data:0000fe01 41                               ld b,c
                           .data:0000fe02 b4                               or h
                           .data:0000fe03 00                               nop
                           .data:0000fe04 00                               nop
                           .data:0000fe05 00                               nop
                           .data:0000fe06 00                               nop
                           .data:0000fe07 00                               nop
                           .data:0000fe08 00                               nop
                           .data:0000fe09 21 12 00                         ld hl,0x0012
                           .data:0000fe0c 39                               add hl,sp
                           .data:0000fe0d cd fb f1                         call 0xf1fb
                           .data:0000fe10 cd 08 f3                         call 0xf308
                           .data:0000fe13 e1                               pop hl
                           .data:0000fe14 cd 28 f2                         call 0xf228
                           .data:0000fe17 21 10 00                         ld hl,0x0010
                           .data:0000fe1a 39                               add hl,sp
                           .data:0000fe1b cd 09 f2                         call 0xf209
                           .data:0000fe1e cd f4 f1                         call 0xf1f4
                           .data:0000fe21 44                               ld b,h
                           .data:0000fe22 3b                               dec sp
                           .data:0000fe23 9a                               sbc a,d
                           .data:0000fe24 ca 00 00                         jp z,0x0000
                           .data:0000fe27 00                               nop
                           .data:0000fe28 00                               nop
                           .data:0000fe29 cd b3 f5                         call 0xf5b3
                           .data:0000fe2c 28 0c                            jr z,0xfe3a
                           .data:0000fe2e cd 02 f2                         call 0xf202
                           .data:0000fe31 00                               nop
                           .data:0000fe32 00                               nop
                           .data:0000fe33 00                               nop
                           .data:0000fe34 00                               nop
                           .data:0000fe35 00                               nop
                           .data:0000fe36 00                               nop
                           .data:0000fe37 00                               nop
                           .data:0000fe38 00                               nop
                           .data:0000fe39 c9                               ret
                           .data:0000fe3a 21 10 00                         ld hl,0x0010
                           .data:0000fe3d 39                               add hl,sp
                           .data:0000fe3e e5                               push hl
                           .data:0000fe3f 21 12 00                         ld hl,0x0012
                           .data:0000fe42 39                               add hl,sp
                           .data:0000fe43 cd 09 f2                         call 0xf209
                           .data:0000fe46 cd 4a f2                         call 0xf24a
                           .data:0000fe49 cd 45 ec                         call 0xec45
                           .data:0000fe4c cd 87 ff                         call 0xff87
                           .data:0000fe4f e1                               pop hl
                           .data:0000fe50 cd 28 f2                         call 0xf228
                           .data:0000fe53 21 10 00                         ld hl,0x0010
                           .data:0000fe56 39                               add hl,sp
                           .data:0000fe57 cd 09 f2                         call 0xf209
                           .data:0000fe5a cd f4 f1                         call 0xf1f4
                           .data:0000fe5d 42                               ld b,d
                           .data:0000fe5e 01 0e 00                         ld bc,0x000e
                           .data:0000fe61 00                               nop
                           .data:0000fe62 00                               nop
                           .data:0000fe63 00                               nop
                           .data:0000fe64 00                               nop
                           .data:0000fe65 cd b3 f5                         call 0xf5b3
                           .data:0000fe68 28 1e                            jr z,0xfe88
                           .data:0000fe6a 21 10 00                         ld hl,0x0010
                           .data:0000fe6d 39                               add hl,sp
                           .data:0000fe6e e5                               push hl
                           .data:0000fe6f cd 02 f2                         call 0xf202
                           .data:0000fe72 42                               ld b,d
                           .data:0000fe73 02                               ld (bc),a
                           .data:0000fe74 1c                               inc e
                           .data:0000fe75 00                               nop
                           .data:0000fe76 00                               nop
                           .data:0000fe77 00                               nop
                           .data:0000fe78 00                               nop
                           .data:0000fe79 00                               nop
                           .data:0000fe7a 21 12 00                         ld hl,0x0012
                           .data:0000fe7d 39                               add hl,sp
                           .data:0000fe7e cd fb f1                         call 0xf1fb
                           .data:0000fe81 cd 08 f3                         call 0xf308
                           .data:0000fe84 e1                               pop hl
                           .data:0000fe85 cd 28 f2                         call 0xf228
                           .data:0000fe88 21 10 00                         ld hl,0x0010
                           .data:0000fe8b 39                               add hl,sp
                           .data:0000fe8c cd 09 f2                         call 0xf209
                           .data:0000fe8f cd f4 f1                         call 0xf1f4
                           .data:0000fe92 41                               ld b,c
                           .data:0000fe93 5a                               ld e,d
                           .data:0000fe94 00                               nop
                           .data:0000fe95 00                               nop
                           .data:0000fe96 00                               nop
                           .data:0000fe97 00                               nop
                           .data:0000fe98 00                               nop
                           .data:0000fe99 00                               nop
                           .data:0000fe9a cd b3 f5                         call 0xf5b3
                           .data:0000fe9d 28 1e                            jr z,0xfebd
                           .data:0000fe9f 21 10 00                         ld hl,0x0010
                           .data:0000fea2 39                               add hl,sp
                           .data:0000fea3 e5                               push hl
                           .data:0000fea4 cd 02 f2                         call 0xf202
                           .data:0000fea7 41                               ld b,c
                           .data:0000fea8 b4                               or h
                           .data:0000fea9 00                               nop
                           .data:0000feaa 00                               nop
                           .data:0000feab 00                               nop
                           .data:0000feac 00                               nop
                           .data:0000fead 00                               nop
                           .data:0000feae 00                               nop
                           .data:0000feaf 21 12 00                         ld hl,0x0012
                           .data:0000feb2 39                               add hl,sp
                           .data:0000feb3 cd fb f1                         call 0xf1fb
                           .data:0000feb6 cd 08 f3                         call 0xf308
                           .data:0000feb9 e1                               pop hl
                           .data:0000feba cd 28 f2                         call 0xf228
                           .data:0000febd 21 10 00                         ld hl,0x0010
                           .data:0000fec0 39                               add hl,sp
                           .data:0000fec1 e5                               push hl
                           .data:0000fec2 21 d7 01                         ld hl,0x01d7
                           .data:0000fec5 cd 09 f2                         call 0xf209
                           .data:0000fec8 cd a6 f2                         call 0xf2a6
                           .data:0000fecb e1                               pop hl
                           .data:0000fecc e5                               push hl
                           .data:0000fecd cd 09 f2                         call 0xf209
                           .data:0000fed0 cd e4 f3                         call 0xf3e4
                           .data:0000fed3 e1                               pop hl
                           .data:0000fed4 cd 28 f2                         call 0xf228
                           .data:0000fed7 21 04 00                         ld hl,0x0004
                           .data:0000feda 39                               add hl,sp
                           .data:0000fedb e5                               push hl
                           .data:0000fedc 21 12 00                         ld hl,0x0012
                           .data:0000fedf 39                               add hl,sp
                           .data:0000fee0 cd 09 f2                         call 0xf209
                           .data:0000fee3 21 12 00                         ld hl,0x0012
                           .data:0000fee6 39                               add hl,sp
                           .data:0000fee7 cd fb f1                         call 0xf1fb
                           .data:0000feea cd e4 f4                         call 0xf4e4
                           .data:0000feed e1                               pop hl
                           .data:0000feee cd 28 f2                         call 0xf228
                           .data:0000fef1 21 04 00                         ld hl,0x0004
                           .data:0000fef4 39                               add hl,sp
                           .data:0000fef5 cd 09 f2                         call 0xf209
                           .data:0000fef8 cd f4 f1                         call 0xf1f4
                           .data:0000fefb 3e 2e                            ld a,0x2e
                           .data:0000fefd 2e 53                            ld l,0x53
                           .data:0000feff b4                               or h
                           .data:0000ff00 e4 cc e9                         call po,0xe9cc
                           .data:0000ff03 cd e4 f4                         call 0xf4e4
                           .data:0000ff06 cd f4 f1                         call 0xf1f4
                           .data:0000ff09 bf                               cp a
                           .data:0000ff0a 0d                               dec c
                           .data:0000ff0b 00                               nop
                           .data:0000ff0c c0                               ret nz
                           .data:0000ff0d 2a 8e 2f                         ld hl,(0x2f8e)
                           .data:0000ff10 3e cd                            ld a,0xcd
                           .data:0000ff12 0f                               rrca
                           .data:0000ff13 f3                               di
                           .data:0000ff14 21 04 00                         ld hl,0x0004
                           .data:0000ff17 39                               add hl,sp
                           .data:0000ff18 cd fb f1                         call 0xf1fb
                           .data:0000ff1b cd e4 f4                         call 0xf4e4
                           .data:0000ff1e cd f4 f1                         call 0xf1f4
                           .data:0000ff21 40                               ld b,b
                           .data:0000ff22 02                               ld (bc),a
                           .data:0000ff23 22 22 1a                         ld (0x1a22),hl
                           .data:0000ff26 42                               ld b,d
                           .data:0000ff27 5c                               ld e,h
                           .data:0000ff28 ad                               xor l
                           .data:0000ff29 cd 0f f3                         call 0xf30f
                           .data:0000ff2c 21 04 00                         ld hl,0x0004
                           .data:0000ff2f 39                               add hl,sp
                           .data:0000ff30 cd fb f1                         call 0xf1fb
                           .data:0000ff33 cd e4 f4                         call 0xf4e4
                           .data:0000ff36 cd f4 f1                         call 0xf1f4
                           .data:0000ff39 c0                               ret nz
                           .data:0000ff3a 2a aa aa                         ld hl,(0xaaaa)
                           .data:0000ff3d a9                               xor c
                           .data:0000ff3e 85                               add a,l
                           .data:0000ff3f 76                               halt
                           .data:0000ff40 ac                               xor h
                           .data:0000ff41 cd 0f f3                         call 0xf30f
                           .data:0000ff44 21 04 00                         ld hl,0x0004
                           .data:0000ff47 39                               add hl,sp
                           .data:0000ff48 cd fb f1                         call 0xf1fb
                           .data:0000ff4b cd e4 f4                         call 0xf4e4
                           .data:0000ff4e 21 bf 01                         ld hl,0x01bf
                           .data:0000ff51 cd fb f1                         call 0xf1fb
                           .data:0000ff54 cd 0f f3                         call 0xf30f
                           .data:0000ff57 21 10 00                         ld hl,0x0010
                           .data:0000ff5a 39                               add hl,sp
                           .data:0000ff5b cd fb f1                         call 0xf1fb
                           .data:0000ff5e c3 e4 f4                         jp 0xf4e4
                           .data:0000ff61 eb                               ex de,hl
                           .data:0000ff62 2a 03 01                         ld hl,(0x0103)
                           .data:0000ff65 1a                               ld a,(de)
                           .data:0000ff66 77                               ld (hl),a
                           .data:0000ff67 23                               inc hl
                           .data:0000ff68 13                               inc de
                           .data:0000ff69 1a                               ld a,(de)
                           .data:0000ff6a 77                               ld (hl),a
                           .data:0000ff6b 23                               inc hl
                           .data:0000ff6c 13                               inc de
                           .data:0000ff6d 1a                               ld a,(de)
                           .data:0000ff6e 77                               ld (hl),a
                           .data:0000ff6f 23                               inc hl
                           .data:0000ff70 13                               inc de
                           .data:0000ff71 1a                               ld a,(de)
                           .data:0000ff72 77                               ld (hl),a
                           .data:0000ff73 c9                               ret
                           .data:0000ff74 eb                               ex de,hl
                           .data:0000ff75 2a 03 01                         ld hl,(0x0103)
                           .data:0000ff78 7e                               ld a,(hl)
                           .data:0000ff79 12                               ld (de),a
                           .data:0000ff7a 13                               inc de
                           .data:0000ff7b 23                               inc hl
                           .data:0000ff7c 7e                               ld a,(hl)
                           .data:0000ff7d 12                               ld (de),a
                           .data:0000ff7e 13                               inc de
                           .data:0000ff7f 23                               inc hl
                           .data:0000ff80 7e                               ld a,(hl)
                           .data:0000ff81 12                               ld (de),a
                           .data:0000ff82 13                               inc de
                           .data:0000ff83 23                               inc hl
                           .data:0000ff84 7e                               ld a,(hl)
                           .data:0000ff85 12                               ld (de),a
                           .data:0000ff86 c9                               ret
                           .data:0000ff87 e1                               pop hl
                           .data:0000ff88 d9                               exx
                           .data:0000ff89 21 08 00                         ld hl,0x0008
                           .data:0000ff8c 39                               add hl,sp
                           .data:0000ff8d f9                               ld sp,hl
                           .data:0000ff8e d9                               exx
                           .data:0000ff8f e9                               jp (hl)
                           .data:0000ff90 e3                               ex (sp),hl
                           .data:0000ff91 c5                               push bc
                           .data:0000ff92 cd ec fa                         call 0xfaec
                           .data:0000ff95 c1                               pop bc
                           .data:0000ff96 d1                               pop de
                           .data:0000ff97 7c                               ld a,h
                           .data:0000ff98 b5                               or l
                           .data:0000ff99 c9                               ret
                           .data:0000ff9a 2a 06 00                         ld hl,(0x0006)
                           .data:0000ff9d f9                               ld sp,hl
                           .data:0000ff9e cd eb ff                         call 0xffeb
                           .data:0000ffa1 01 00 00                         ld bc,0x0000
                           .data:0000ffa4 cd 05 00                         call 0x0005
                           .data:0000ffa7 c3 a1 ff                         jp 0xffa1
                           .data:0000ffaa cd 74 fc                         call 0xfc74
                           .data:0000ffad cd b5 ff                         call 0xffb5
                           .data:0000ffb0 eb                               ex de,hl
                           .data:0000ffb1 c9                               ret
                           .data:0000ffb2 cd 74 fc                         call 0xfc74
                           .data:0000ffb5 2a fd 04                         ld hl,(0x04fd)
                           .data:0000ffb8 44                               ld b,h
                           .data:0000ffb9 4d                               ld c,l
                           .data:0000ffba 2a ff 04                         ld hl,(0x04ff)
                           .data:0000ffbd eb                               ex de,hl
                           .data:0000ffbe cd 05 00                         call 0x0005
                           .data:0000ffc1 eb                               ex de,hl
                           .data:0000ffc2 6f                               ld l,a
                           .data:0000ffc3 26 00                            ld h,0x00
                           .data:0000ffc5 c9                               ret
                           .data:0000ffc6 cd 74 fc                         call 0xfc74
                           .data:0000ffc9 cd d3 ff                         call 0xffd3
                           .data:0000ffcc 6f                               ld l,a
                           .data:0000ffcd 26 00                            ld h,0x00
                           .data:0000ffcf c9                               ret
                           .data:0000ffd0 cd 74 fc                         call 0xfc74
                           .data:0000ffd3 2a fd 04                         ld hl,(0x04fd)
                           .data:0000ffd6 eb                               ex de,hl
                           .data:0000ffd7 2a 01 00                         ld hl,(0x0001)
                           .data:0000ffda 2b                               dec hl
                           .data:0000ffdb 2b                               dec hl
                           .data:0000ffdc 2b                               dec hl
                           .data:0000ffdd 19                               add hl,de
                           .data:0000ffde 19                               add hl,de
                           .data:0000ffdf 19                               add hl,de
                           .data:0000ffe0 eb                               ex de,hl
                           .data:0000ffe1 2a ff 04                         ld hl,(0x04ff)
                           .data:0000ffe4 44                               ld b,h
                           .data:0000ffe5 4d                               ld c,l
                           .data:0000ffe6 2a 01 05                         ld hl,(0x0501)
                           .data:0000ffe9 eb                               ex de,hl
                           .data:0000ffea e9                               jp (hl)
                           .data:0000ffeb c3 4e 04                         jp 0x044e
                           .data:0000ffee 1a                               ld a,(de)
                           .data:0000ffef 1a                               ld a,(de)
                           .data:0000fff0 1a                               ld a,(de)
                           .data:0000fff1 1a                               ld a,(de)
                           .data:0000fff2 1a                               ld a,(de)
                           .data:0000fff3 1a                               ld a,(de)
                           .data:0000fff4 1a                               ld a,(de)
                           .data:0000fff5 1a                               ld a,(de)
                           .data:0000fff6 1a                               ld a,(de)
                           .data:0000fff7 1a                               ld a,(de)
                           .data:0000fff8 1a                               ld a,(de)
                           .data:0000fff9 1a                               ld a,(de)
                           .data:0000fffa 1a                               ld a,(de)
                           .data:0000fffb 1a                               ld a,(de)
                           .data:0000fffc 1a                               ld a,(de)
                           .data:0000fffd 1a                               ld a,(de)
                           .data:0000fffe 1a                               ld a,(de)
                           .data:0000ffff 1a                               ld a,(de)
