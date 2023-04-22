ld bc,0x7f89
out (c),c
jp 0x0580
jp 0xb982
jp 0xb97c
push bc
ret
jp 0xba16
jp 0xba10
push de
ret
jp 0xb9bf
jp 0xb9b1
jp (hl)
nop
jp 0xbacb
jp 0xb9b9
nop
nop
jp 0xba2e
nop
out (c),c
exx
ei
di
exx
ld hl,0x002b
ld (hl),c
jr 0x0040
jp 0xb939
ret
nop
nop
nop
nop
set 2,c
jr 0x002c
ld hl,0x0040
dec l
ld a,(hl)
ld (hl),a
jr nz,0x0047
ld a,0xc7
ld (0x0030),a
ld hl,0x0391
ld de,0xb900
ld bc,0x01e9
ldir
di
ld a,(0xb1ab)
ld de,(0xb1a9)
ld b,0xc0
ld hl,0xb100
ld (hl),0x00
inc hl
djnz 0x0069
ld b,a
ld c,0xff
xor c
ret nz
ld c,a
ld e,a
ld d,a
ret
ld a,h
or l
ld a,c
jr nz,0x0080
ld a,l
ld hl,0xc006
ld (0xb1a8),a
ld (0xb1ab),a
ld (0xb1a9),hl
ld hl,0xabff
ld de,0x0040
ld bc,0xb0ff
ld sp,0xc000
rst 0x18
xor c
or c
rst 0x00
di
ld de,(0xb189)
ld hl,(0xb187)
ei
ret
di
xor a
ld (0xb18b),a
ld (0xb189),de
ld (0xb187),hl
ei
ret
ld hl,0xb187
inc (hl)
inc hl
jr z,0x00b4
ld b,0xf5
in a,(c)
rra
jr nc,0x00c7
ld hl,(0xb18c)
ld a,h
or a
call nz,0x0153
ld hl,(0xb18e)
ld a,h
or a
call nz,0x0153
call 0x1f61
ld hl,0xb192
dec (hl)
ret nz
ld (hl),0x06
call 0x1bb7
ld hl,(0xb190)
ld a,h
or a
ret z
ld hl,0xb104
set 0,(hl)
ret
dec hl
ld (hl),0x00
dec hl
ld a,(0xb101)
or a
jr nz,0x00fe
ld (0xb100),hl
ld (0xb102),hl
ld hl,0xb104
set 6,(hl)
ret
ld de,(0xb102)
ld (0xb102),hl
ex de,hl
ld (hl),e
inc hl
ld (hl),d
ret
ld (0xb105),sp
ld sp,0xb187
push hl
push de
push bc
ld hl,0xb104
bit 6,(hl)
jr z,0x0139
set 7,(hl)
ld hl,(0xb100)
ld a,h
or a
jr z,0x0132
ld e,(hl)
inc hl
ld d,(hl)
ld (0xb100),de
inc hl
call 0x020a
di
jr 0x011d
ld hl,0xb104
bit 0,(hl)
jr z,0x0149
ld (hl),0x00
scf
ex af,af'
call 0x0189
or a
ex af,af'
ld hl,0xb104
ld a,(hl)
or a
jr nz,0x011b
ld (hl),0x00
pop bc
pop de
pop hl
ld sp,(0xb105)
ret
ld e,(hl)
inc hl
ld a,(hl)
inc hl
or a
jp z,0x01e2
ld d,a
push de
call 0x01e2
pop hl
jr 0x0153
push hl
inc hl
inc hl
call 0x01d2
pop hl
ld de,0xb18c
jp 0x0373
ld de,0xb18c
jp 0x0382
push hl
inc hl
inc hl
call 0x01d2
pop hl
ld de,0xb18e
jp 0x0373
ld de,0xb18e
jp 0x0382
ld hl,(0xb190)
ld a,h
or a
ret z
ld e,(hl)
inc hl
ld d,(hl)
inc hl
ld c,(hl)
inc hl
ld b,(hl)
ld a,b
or c
jr z,0x01b0
dec bc
ld a,b
or c
jr nz,0x01ad
push de
inc hl
inc hl
push hl
inc hl
call 0x01e2
pop hl
ld b,(hl)
dec hl
ld c,(hl)
dec hl
pop de
ld (hl),b
dec hl
ld (hl),c
ex de,hl
jr 0x018c
push hl
inc hl
inc hl
di
ld (hl),e
inc hl
ld (hl),d
inc hl
ld (hl),c
inc hl
ld (hl),b
pop hl
ld de,0xb190
jp 0x0373
ld de,0xb190
call 0x0382
ret nc
ex de,hl
inc hl
ld e,(hl)
inc hl
ld d,(hl)
ret
di
inc hl
inc hl
ld (hl),0x00
inc hl
ld (hl),b
inc hl
ld (hl),e
inc hl
ld (hl),d
inc hl
ld (hl),c
inc hl
ei
ret
inc hl
inc hl
di
ld a,(hl)
inc (hl)
jp m,0x0206
or a
jr nz,0x0200
inc hl
ld a,(hl)
dec hl
or a
jp p,0x022f
ex af,af'
jr nc,0x0209
ex af,af'
add a,a
jp p,0x00e8
inc hl
inc hl
jr 0x0223
ex af,af'
jr c,0x0204
ei
ex af,af'
ret
dec (hl)
jr 0x0200
ex af,af'
ei
ld a,(hl)
or a
ret m
push hl
call 0x021c
pop hl
dec (hl)
ret z
jp p,0x020e
inc (hl)
ret
inc hl
inc hl
inc hl
ld a,(hl)
inc hl
rra
jp nc,0xb9b9
ld e,(hl)
inc hl
ld d,(hl)
ex de,hl
jp (hl)
ld hl,0x0000
ld (0xb194),hl
ret
push hl
ld b,a
ld de,0xb196
ex de,hl
dec hl
dec hl
ld d,(hl)
dec hl
ld e,(hl)
ld a,d
or a
jr z,0x0245
inc de
inc de
inc de
ld a,(de)
cp b
jr nc,0x0234
pop de
dec de
inc hl
ld a,(hl)
ld (de),a
dec de
ld (hl),d
dec hl
ld a,(hl)
ld (de),a
ld (hl),e
ex af,af'
jr c,0x0254
ei
ex af,af'
ret
di
ld hl,(0xb193)
ld a,h
or a
jr z,0x0275
push hl
ld e,(hl)
inc hl
ld d,(hl)
inc hl
inc hl
ld a,(0xb195)
cp (hl)
jr nc,0x0274
push af
ld a,(hl)
ld (0xb195),a
ld (0xb193),de
pop af
pop hl
ei
ret
ld (0xb195),a
inc hl
inc hl
dec (hl)
ret z
di
jp p,0x022f
inc (hl)
ei
ret
call 0x028e
ld de,0xb193
jp 0x0382
inc hl
inc hl
ld (hl),0xc0
dec hl
dec hl
ret
ld hl,0xb195
set 5,(hl)
ret
ld hl,0xb195
res 5,(hl)
ret
push hl
ld de,(0xb1a6)
ld (0xb1a6),hl
ld (hl),e
inc hl
ld (hl),d
inc hl
ld (hl),c
inc hl
ld (hl),b
pop hl
ret
ld de,0xb196
ld bc,0x0010
call 0xbaa6
ex de,hl
dec hl
set 7,(hl)
ld hl,(0xb1a6)
ld a,l
jr 0x02d5
push hl
inc hl
inc hl
ld c,(hl)
inc hl
ld b,(hl)
call 0x02f4
pop de
ret c
ex de,hl
ld a,(hl)
inc hl
ld h,(hl)
ld l,a
or h
jr nz,0x02c5
ld c,0xff
inc c
call 0xba83
push af
and 0x03
ld b,a
call z,0x02f4
jr c,0x02f0
pop af
add a,a
jr nc,0x02da
ld a,c
or a
jr z,0x02da
ret
pop af
jp 0x060b
ld hl,0xc004
ld a,b
or a
jr z,0x02ff
ld h,b
ld l,c
ld c,0xff
call 0xba7e
push bc
ld e,(hl)
inc hl
ld d,(hl)
inc hl
ex de,hl
jr 0x0321
ld bc,0xb196
ld a,(bc)
cp (hl)
jr nz,0x0319
inc hl
inc bc
add a,a
jr nc,0x030d
ex de,hl
jr 0x0325
ld a,(hl)
inc hl
add a,a
jr nc,0x0319
inc de
inc de
inc de
ld a,(hl)
or a
jr nz,0x030a
pop bc
jp 0xba8c
ld c,0x07
call 0x0332
dec c
jr nz,0x032b
ret
ld a,c
cp 0x08
ret nc
call 0xba7e
ld a,(0xc000)
and 0x03
dec a
jr nz,0x0360
push bc
call 0xc006
push de
inc hl
ex de,hl
ld hl,0xb1aa
ld bc,(0xb1a8)
ld b,0x00
add hl,bc
add hl,bc
ld (hl),e
inc hl
ld (hl),d
ld hl,0xfffc
add hl,de
call 0x02a1
dec hl
pop de
pop bc
jp 0xba8c
ld a,(hl)
cp e
inc hl
ld a,(hl)
dec hl
jr nz,0x036d
cp d
scf
ret z
or a
ret z
ld l,(hl)
ld h,a
jr 0x0363
ex de,hl
di
call 0x0363
jr c,0x0380
ld (hl),e
inc hl
ld (hl),d
inc de
xor a
ld (de),a
ei
ret
ex de,hl
di
call 0x0363
jr nc,0x038f
ld a,(de)
ld (hl),a
inc de
inc hl
ld a,(de)
ld (hl),a
ei
ret
jp 0xba5e
jp 0xba68
jp 0xba4a
jp 0xba54
jp 0xba72
jp 0xba7e
jp 0xbaa2
jp 0xba83
jp 0xba8c
jp 0xbaa6
jp 0xbaac
ld a,(0xb194)
or a
ret z
push hl
di
ld hl,(0xb193)
ld a,h
or a
jr z,0x03c7
inc hl
inc hl
inc hl
ld a,(0xb195)
cp (hl)
pop hl
ei
ret
di
ex af,af'
jr c,0x0401
exx
ld a,c
scf
ei
ex af,af'
di
push af
res 2,c
out (c),c
call 0x00b1
or a
ex af,af'
ld c,a
ld b,0x7f
ld a,(0xb104)
or a
jr z,0x03fb
jp m,0xb96a
ld a,c
and 0x0c
push af
res 2,c
exx
call 0x010a
exx
pop hl
ld a,c
and 0xf3
or h
ld c,a
out (c),c
exx
pop af
ei
ret
ex af,af'
pop hl
push af
set 2,c
out (c),c
call 0x003b
jr 0x03dc
di
push hl
exx
pop de
jr 0x0419
di
exx
pop hl
ld e,(hl)
inc hl
ld d,(hl)
ex af,af'
ld a,d
res 7,d
res 6,d
rlca
rlca
rlca
rlca
xor c
and 0x0c
xor c
push bc
call 0xb9a8
di
exx
ex af,af'
ld a,c
pop bc
and 0x03
res 1,c
res 0,c
or c
jr 0x043a
push de
ld c,a
out (c),c
or a
ex af,af'
exx
ei
ret
di
ex af,af'
ld a,c
push hl
exx
pop de
jr 0x045f
di
push hl
exx
pop hl
jr 0x0459
di
exx
pop hl
ld e,(hl)
inc hl
ld d,(hl)
inc hl
push hl
ex de,hl
ld e,(hl)
inc hl
ld d,(hl)
inc hl
ex af,af'
ld a,(hl)
cp 0xfc
jr nc,0x0421
ld b,0xdf
out (c),a
ld hl,0xb1a8
ld b,(hl)
ld (hl),a
push bc
push iy
dec a
cp 0x07
jr nc,0x0483
add a,a
add a,0xac
ld l,a
adc a,0xb1
sub l
ld h,a
ld a,(hl)
inc hl
ld h,(hl)
ld l,a
push hl
pop iy
ld b,0x7f
ld a,c
set 2,a
res 3,a
call 0xb9a8
pop iy
di
exx
ex af,af'
ld e,c
pop bc
ld a,b
ld b,0xdf
out (c),a
ld (0xb1a8),a
ld b,0x7f
ld a,e
jr 0x0430
di
push hl
exx
pop de
jr 0x04af
di
exx
pop hl
ld e,(hl)
inc hl
ld d,(hl)
inc hl
push hl
ex af,af'
ld a,d
set 7,d
set 6,d
and 0xc0
rlca
rlca
ld hl,0xb1ab
add a,(hl)
jr 0x0463
di
exx
pop hl
ld e,(hl)
inc hl
ld d,(hl)
res 2,c
out (c),c
ld (0xba3f),de
exx
ei
call 0xba3e
di
exx
set 2,c
out (c),c
exx
ei
ret
di
exx
ld a,c
res 2,c
out (c),c
exx
ei
ret
di
exx
ld a,c
set 2,c
out (c),c
exx
ei
ret
di
exx
ld a,c
res 3,c
out (c),c
exx
ei
ret
di
exx
ld a,c
set 3,c
out (c),c
exx
ei
ret
di
exx
xor c
and 0x0c
xor c
ld c,a
out (c),c
exx
ei
ret
call 0xba5e
jr 0x0523
call 0xba7e
ld a,(0xc000)
ld hl,(0xc001)
push af
ld a,b
call 0xba72
pop af
push hl
di
ld b,0xdf
out (c),c
ld hl,0xb1a8
ld b,(hl)
ld (hl),c
ld c,b
ld b,a
ei
pop hl
ret
ld a,(0xb1a8)
ret
call 0xbab2
ldir
ret
call 0xbab2
lddr
ret
di
exx
pop hl
push bc
set 2,c
set 3,c
out (c),c
call 0xbac7
di
exx
pop bc
out (c),c
exx
ei
ret
push hl
exx
ei
ret
di
exx
ld e,c
set 2,e
set 3,e
out (c),e
exx
ld a,(hl)
exx
out (c),c
exx
ei
ret
exx
ld a,c
or 0x0c
out (c),a
ld a,(ix+0)
out (c),c
exx
ret
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
di
ld bc,0xf782
out (c),c
ld bc,0xf400
out (c),c
ld bc,0xf600
out (c),c
ld bc,0xef7f
out (c),c
ld b,0xf5
in a,(c)
and 0x10
ld hl,0x05c4
jr nz,0x05a3
ld hl,0x05d4
ld bc,0xbc0f
out (c),c
dec hl
ld a,(hl)
inc b
out (c),a
dec b
dec c
jp p,0x05a6
jr 0x05d4
ccf
jr z,0x05e5
adc a,(hl)
ld h,0x00
add hl,de
ld e,0x00
rlca
nop
nop
jr nc,0x05c2
ret nz
nop
ccf
jr z,0x05f5
adc a,(hl)
rra
ld b,0x19
dec de
nop
rlca
nop
nop
jr nc,0x05d2
ret nz
nop
ld de,0x065c
ld hl,0x0000
jr 0x060e
ld sp,0xc000
push hl
call 0x1e68
di
ld bc,0xf8ff
out (c),c
call 0x005c
pop hl
push de
push bc
push hl
call 0x1a1e
call 0x1088
call 0x0ab1
call 0xba5e
pop hl
call 0x0775
pop bc
pop de
jr c,0x060b
ex de,hl
ld c,b
ld de,0x06e8
jr 0x060e
ld de,0x0726
di
im 1
exx
ld bc,0xdf00
out (c),c
ld bc,0xf8ff
out (c),c
ld hl,0xb100
ld de,0xb101
ld bc,0x07ff
ld (hl),0x00
ldir
ld bc,0x7f89
out (c),c
exx
xor a
ex af,af'
ld sp,0xc000
push hl
push bc
push de
call 0x0044
call 0x0888
call 0x19e0
call 0x1e68
call 0x0aa0
call 0x1078
call 0x15b0
call 0x2370
call 0x07e6
ei
pop hl
call 0x0775
pop bc
pop hl
jp 0x0077
call 0x0712
call 0x06eb
ld hl,0x066d
call 0x06eb
ld hl,0x0693
jr 0x06eb
jr nz,0x06a5
inc (hl)
ld c,e
jr nz,0x06c0
ld l,c
ld h,e
ld (hl),d
ld l,a
ld h,e
ld l,a
ld l,l
ld (hl),b
ld (hl),l
ld (hl),h
ld h,l
ld (hl),d
jr nz,0x06a1
jr z,0x06f9
ld sp,0x0d29
ld a,(bc)
dec c
ld a,(bc)
nop
ld b,e
ld l,a
ld (hl),b
ld a,c
ld (hl),d
ld l,c
ld h,a
ld l,b
ld (hl),h
jr nz,0x0639
ld sp,0x3839
inc (hl)
jr nz,0x06dc
ld l,l
ld (hl),e
ld (hl),h
ld (hl),d
ld h,c
ld h,h
jr nz,0x06e6
ld l,a
ld l,(hl)
ld (hl),e
ld (hl),l
ld l,l
ld h,l
ld (hl),d
jr nz,0x06f1
ld l,h
ld h,l
ld h,e
ld (hl),h
ld (hl),d
ld l,a
ld l,(hl)
ld l,c
ld h,e
ld (hl),e
jr nz,0x0728
ld l,h
ld h,e
dec c
ld a,(bc)
jr nz,0x06de
jr nz,0x06e0
jr nz,0x06e2
jr nz,0x06e4
jr nz,0x06e6
jr nz,0x0729
ld l,(hl)
ld h,h
jr nz,0x0718
ld l,a
ld h,e
ld l,a
ld l,l
ld l,a
ld (hl),h
ld l,c
halt
ld h,l
jr nz,0x072a
ld l,a
ld h,(hl)
ld (hl),h
ld (hl),a
ld h,c
ld (hl),d
ld h,l
jr nz,0x072c
ld (hl),h
ld h,h
ld l,0x0d
ld a,(bc)
dec c
ld a,(bc)
nop
ld hl,0x06f4
ld a,(hl)
inc hl
or a
ret z
call 0x1400
jr 0x06eb
ld hl,(0x2a2a)
jr nz,0x0749
ld d,d
ld c,a
ld b,a
ld d,d
ld b,c
ld c,l
jr nz,0x074d
ld c,a
ld b,c
ld b,h
jr nz,0x074c
ld b,c
ld c,c
ld c,h
ld b,l
ld b,h
jr nz,0x0737
ld hl,(0x0d2a)
ld a,(bc)
nop
ld b,0xf5
in a,(c)
cpl
and 0x0e
rrca
ld hl,0x0727
inc a
ld b,a
ld a,(hl)
inc hl
or a
jr nz,0x071f
djnz 0x071f
ret
ld b,c
ld (hl),d
ld l,(hl)
ld l,a
ld l,h
ld h,h
nop
ld a,(bc)
jr nz,0x0772
ld l,l
ld (hl),e
ld (hl),h
ld (hl),d
ld h,c
ld h,h
nop
ld a,(bc)
jr nz,0x078a
ld (hl),d
ld l,c
ld l,a
ld l,(hl)
nop
ld a,(bc)
jr nz,0x0796
ld h,e
ld l,b
ld l,(hl)
ld h,l
ld l,c
ld h,h
ld h,l
ld (hl),d
nop
ld a,(bc)
jr nz,0x0790
ld (hl),a
ld h,c
nop
ld a,(bc)
jr nz,0x07a8
ld l,a
ld l,h
ld h,c
halt
ld l,a
ld a,b
nop
ld a,(bc)
jr nz,0x07b2
ld h,c
ld l,c
ld (hl),e
ld l,b
ld l,a
nop
ld a,(bc)
jr nz,0x07bc
ld (hl),d
ld l,c
ld (hl),l
ld l,l
ld (hl),b
ld l,b
nop
ld a,(bc)
jr nz,0x07bb
ld (hl),e
ld (hl),b
nop
jp (hl)
cp 0x03
ret nc
di
exx
res 1,c
res 0,c
or c
ld c,a
out (c),c
ei
exx
ret
push bc
push de
ld bc,0x7f10
call 0x07ab
ld c,0x00
call 0x07ab
dec de
jr nz,0x0790
pop de
pop bc
ret
push bc
push de
ld bc,0x7f10
call 0x07ab
ld c,0x00
call 0x07ab
jr nz,0x07a3
pop de
pop bc
ret
out (c),c
ld a,(de)
inc de
and 0x1f
or 0x40
out (c),a
inc c
ld a,c
cp 0x10
ret
push af
push bc
ld b,0xf5
in a,(c)
rra
jr nc,0x07be
pop bc
pop af
ret
push bc
rrca
rrca
and 0x30
ld c,a
ld a,h
rra
and 0x03
or c
ld bc,0xbc0c
out (c),c
inc b
out (c),a
dec b
inc c
out (c),c
inc b
ld a,h
rra
ld a,l
rra
out (c),a
pop bc
ret
ld hl,0x07ec
jp 0x0a8a
inc bc
pop af
cp l
jp 0x07f8
push bc
call 0xbdf1
pop bc
ret
ld bc,0x0032
call 0x081b
jr nc,0x0807
djnz 0x07fb
dec c
jr nz,0x07fb
or a
ret
push bc
ld b,0xef
and 0x7f
out (c),a
or 0x80
di
out (c),a
and 0x7f
ei
out (c),a
pop bc
scf
ret
push bc
ld c,a
ld b,0xf5
in a,(c)
rla
rla
ld a,c
pop bc
ret
di
ld b,0xf4
out (c),a
ld b,0xf6
in a,(c)
or 0xc0
out (c),a
and 0x3f
out (c),a
ld b,0xf4
out (c),c
ld b,0xf6
ld c,a
or 0x80
out (c),a
out (c),c
ei
ret
ld bc,0xf40e
out (c),c
ld b,0xf6
in a,(c)
and 0x30
ld c,a
or 0xc0
out (c),a
out (c),c
inc b
ld a,0x92
out (c),a
push bc
set 6,c
ld b,0xf6
out (c),c
ld b,0xf4
in a,(c)
ld b,(hl)
ld (hl),a
and b
cpl
ld (de),a
inc hl
inc de
inc c
ld a,c
and 0x0f
cp 0x0a
jr nz,0x0860
pop bc
ld a,0x82
out (c),a
dec b
out (c),c
ret
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
ld de,0x08ac
ld hl,0xbb00
ld bc,0xbfcf
call 0x0897
ld bc,0x30ef
ld (hl),c
inc hl
ld a,(de)
ld (hl),a
inc de
inc hl
ex de,hl
ld a,c
cpl
rlca
rlca
and 0x80
or (hl)
ex de,hl
ld (hl),a
inc de
inc hl
djnz 0x0897
ret
ret po
add hl,de
ld e,0x1a
inc a
ld a,(de)
ld b,d
ld a,(de)
ld (hl),a
ld a,(de)
cp l
ld a,(de)
ld l,0x1b
ld a,e
ld a,(de)
ld d,(hl)
dec de
ld e,h
dec de
cp l
inc e
or e
dec de
ld e,h
inc e
ld d,d
dec e
ld a,0x1d
ld d,a
dec e
ld b,e
dec e
ld e,h
dec e
ld c,b
dec e
xor e
inc e
and (hl)
inc e
ld l,l
inc e
ld l,c
inc e
ld (hl),c
inc e
add a,d
inc e
sub b
inc e
ld a,b
djnz 0x086b
djnz 0x0936
inc d
ld c,e
inc d
nop
inc d
inc (hl)
inc de
xor e
inc de
and a
inc de
inc c
ld (de),a
ld d,(hl)
ld (de),a
ld b,b
dec d
ld e,(hl)
ld de,0x1169
ld (hl),h
ld de,0x1180
adc a,c
ld (de),a
sbc a,d
ld (de),a
ld a,c
ld (de),a
add a,c
ld (de),a
adc a,0x11
ld l,b
ld (de),a
ld l,b
ld (de),a
xor c
ld (de),a
cp l
ld (de),a
xor (hl)
ld (de),a
jp 0xc912
ld (de),a
ld a,d
inc de
add a,a
inc de
out (0x12),a
pop af
ld (de),a
defb 0xfd
ld (de),a
ld hl,(0xcb13)
inc d
ret pe
djnz 0x092e
ld de,0x15b0
rst 0x18
dec d
call p,0xf115
dec d
call m,0x0415
ld d,0x12
ld d,0x34
rla
ld a,c
rla
and (hl)
rla
cp h
rla
push bc
rla
or 0x17
inc b
jr 0x0942
rla
ld a,(bc)
jr 0x095c
jr 0x095b
jr 0x0974
jr 0x0973
jr 0x098a
jr 0x0989
jr 0x099a
add hl,de
and b
ld a,(bc)
or c
ld a,(bc)
inc a
dec bc
ld b,l
dec bc
ld d,b
dec bc
jp z,0xec0a
ld a,(bc)
rst 0x30
ld a,(bc)
ld d,a
dec bc
ld h,h
dec bc
xor c
dec bc
ld sp,hl
dec bc
dec b
inc c
inc de
inc c
dec l
inc c
add a,(hl)
inc c
and b
inc c
call pe,0x140c
dec c
pop af
inc c
add hl,de
dec c
call po,0xe80c
inc c
or e
dec c
or a
dec c
rst 0x18
dec c
jp m,0x3e0d
ld c,0xf3
ld c,0x49
rrca
ld c,c
inc c
ld l,e
inc c
call nz,0x2f0f
djnz 0x0a0b
inc hl
ld a,a
inc hl
adc a,(hl)
inc hl
ld c,e
ld hl,(0x2a4f)
ld d,c
ld hl,(0x2392)
call m,0x0123
inc h
dec (hl)
inc h
xor e
inc h
sbc a,d
inc h
sub (hl)
inc h
xor e
inc hl
dec d
inc h
ld l,0x24
ld e,e
inc h
jp pe,0x2824
dec h
ccf
jr z,0x09f9
jr z,0x0a16
jr z,0x0a2f
ld e,0x9f
rra
ld l,h
jr nz,0x0956
jr nz,0x0a19
jr nz,0x099c
ld e,0xe6
ld e,0x38
inc hl
dec a
inc hl
ld c,c
inc hl
ld c,(hl)
inc hl
ld e,h
nop
add hl,hl
inc bc
ld (0xa103),a
ld (bc),a
or d
ld (bc),a
ld h,e
ld bc,0x016a
ld (hl),b
ld bc,0x0176
ld a,l
ld bc,0x0183
or e
ld bc,0x01c5
jp nc,0xe201
ld bc,0x0228
add a,l
ld (bc),a
ld d,(hl)
ld (bc),a
ld a,(de)
ld (bc),a
ld (hl),a
ld (bc),a
sub l
ld (bc),a
sbc a,e
ld (bc),a
adc a,(hl)
ld (bc),a
sbc a,c
nop
and e
nop
call c,0x0b05
ld b,0xba
rlca
halt
rlca
add a,0x07
add a,(hl)
rlca
sbc a,c
rlca
and 0x07
jp p,0x1b07
ex af,af'
rlca
ex af,af'
ld h,0x08
adc a,b
ex af,af'
sbc a,b
ld hl,(0x2e18)
add hl,hl
ld l,0x55
ld l,0x66
ld l,0x8e
ld l,0xa1
ld l,0xac
ld l,0xb6
ld l,0x1d
cpl
ccf
inc sp
scf
inc sp
dec sp
inc sp
dec d
inc (hl)
sbc a,(hl)
inc (hl)
ld a,b
dec (hl)
sbc a,d
dec (hl)
ret m
dec (hl)
ret pe
dec (hl)
xor (hl)
ld sp,0x31a3
ld a,(bc)
ld sp,0x310d
inc d
jr nc,0x0a68
jr nc,0x09eb
jr nc,0x0a19
ld sp,0x31b2
ld sp,0x4132
ld (0x2e5e),a
sub h
cpl
and c
cpl
or a
cpl
and 0x2f
ex af,af'
scf
ld c,0x37
dec d
scf
jr z,0x0aad
ld sp,0x3037
scf
add hl,sp
scf
ld a,d
scf
add a,c
scf
ld d,b
scf
adc a,h
scf
jp (hl)
scf
call nc,0xe037
scf
ld c,(hl)
ld b,0x00
inc hl
ld e,(hl)
inc hl
ld d,(hl)
inc hl
ldir
ret
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
ld de,0x104d
call 0x0786
ld a,0xc0
ld (0xb1cb),a
call 0x0ab1
jp 0x0af2
xor a
call 0x0c49
ld hl,0x0abe
call 0x0a8a
jp 0x0cd2
add hl,bc
push hl
cp l
jp 0x0c82
jp 0x0c68
jp 0x0af7
and 0x03
cp 0x03
ret nc
push af
call 0x0d4f
pop af
ld e,a
call 0x10b7
push af
call 0x15d6
push hl
ld a,e
call 0x0b11
call 0xbdeb
pop hl
call 0x15b6
pop af
jp 0x10d5
ld a,(0xb1c8)
cp 0x01
ret
ld a,0x01
call 0x0b11
call 0x0d4f
ld hl,0x0000
call 0x0b3c
ld hl,(0xb1ca)
ld l,0x00
ld d,h
ld e,0x01
ld bc,0x3fff
ld (hl),l
ldir
jp 0x0d3c
ld hl,0x0b3a
cp 0x01
jr c,0x0b20
ld hl,0x0b36
jr z,0x0b20
ld hl,0x0b2e
ld de,0xb1cf
ld bc,0x0008
ldir
ld (0xb1c8),a
jp 0x0776
add a,b
ld b,b
jr nz,0x0b42
ex af,af'
inc b
ld (bc),a
ld bc,0x4488
ld (0xaa11),hl
ld d,l
ld a,h
and 0x07
ld h,a
ld (0xb1c9),hl
jr 0x0b4a
and 0xc0
ld (0xb1cb),a
call 0x0b50
jp 0x07c6
ld hl,(0xb1c9)
ld a,(0xb1cb)
ret
call 0x0aec
ld bc,0x1318
ret c
ld b,0x27
ret z
ld b,0x4f
ret
push de
call 0x0aec
ld b,0x04
jr c,0x0b71
ld b,0x02
jr z,0x0b71
dec b
push bc
ld e,h
ld d,0x00
ld h,d
push de
ld d,h
ld e,l
add hl,hl
add hl,hl
add hl,de
add hl,hl
add hl,hl
add hl,hl
add hl,hl
pop de
add hl,de
djnz 0x0b81
ld de,(0xb1c9)
add hl,de
ld a,h
and 0x07
ld h,a
ld a,(0xb1cb)
add a,h
ld h,a
pop bc
pop de
ret
ld a,e
sub l
inc a
add a,a
add a,a
add a,a
ld e,a
ld a,d
sub h
inc a
ld d,a
call 0x0b64
xor a
add a,d
djnz 0x0ba4
ld d,a
ret
push de
ex de,hl
ld hl,0x00c7
or a
sbc hl,de
ld a,l
and 0x07
add a,a
add a,a
add a,a
ld c,a
ld a,l
and 0xf8
ld l,a
ld d,h
ld e,l
add hl,hl
add hl,hl
add hl,de
add hl,hl
pop de
call 0x0aec
ld b,0x01
jr c,0x0bd0
ld b,0x03
jr z,0x0bd0
ld b,0x07
ld a,b
and e
push af
ld a,b
rrca
srl d
rr e
rrca
jr c,0x0bd5
add hl,de
ld de,(0xb1c9)
add hl,de
ld a,h
and 0x07
ld h,a
ld a,(0xb1cb)
add a,h
add a,c
ld h,a
pop af
push hl
ld d,0x00
ld e,a
ld hl,0xb1cf
add hl,de
ld c,(hl)
ex de,hl
pop hl
ret
inc l
ret nz
inc h
ld a,h
and 0x07
ret nz
ld a,h
sub 0x08
ld h,a
ret
ld a,l
dec l
or a
ret nz
ld a,h
dec h
and 0x07
ret nz
ld a,h
add a,0x08
ld h,a
ret
ld a,h
add a,0x08
ld h,a
and 0x38
ret nz
ld a,h
sub 0x40
ld h,a
ld a,l
add a,0x50
ld l,a
ret nc
inc h
ld a,h
and 0x07
ret nz
ld a,h
sub 0x08
ld h,a
ret
ld a,h
sub 0x08
ld h,a
and 0x38
cp 0x38
ret nz
ld a,h
add a,0x40
ld h,a
ld a,l
sub 0x50
ld l,a
ret nc
ld a,h
dec h
and 0x07
ret nz
ld a,h
add a,0x08
ld h,a
ret
and 0x03
ld hl,0x0c6b
jr z,0x0c5f
cp 0x02
ld hl,0x0c72
jr c,0x0c5f
ld hl,0x0c77
jr z,0x0c5f
ld hl,0x0c7d
ld a,0xc3
ld (0xb1cc),a
ld (0xb1cd),hl
ret
jp 0xb1cc
ld a,(hl)
xor b
or c
xor c
xor b
ld (hl),a
ret
ld a,b
and c
xor (hl)
ld (hl),a
ret
ld a,c
cpl
or b
and (hl)
ld (hl),a
ret
ld a,b
and c
or (hl)
ld (hl),a
ret
ld a,(hl)
jp 0x0cac
push bc
push de
call 0x0cc2
ld e,a
ld b,0x08
ld a,(0xb1cf)
ld c,a
rrc e
rla
rrc c
jr c,0x0c9b
rlc e
djnz 0x0c92
pop de
pop bc
ret
push bc
ld b,a
ld a,(0xb1cf)
ld c,a
ld a,b
call 0x0cac
pop bc
ret
push de
ld de,0x0008
rrca
rl d
rrc c
jr c,0x0cb9
rr d
dec e
jr nz,0x0cb0
ld a,d
call 0x0cc2
pop de
ret
ld d,a
call 0x0aec
ld a,d
ret nc
rrca
rrca
adc a,0x00
rrca
sbc a,a
and 0x06
xor d
ret
ld hl,0x104d
ld de,0xb1d9
ld bc,0x0022
ldir
xor a
ld (0xb1fb),a
ld hl,0x0a0a
ld (0xb1d7),hl
ret
ld hl,(0xb1d7)
ret
and 0x0f
inc a
jr 0x0cf2
xor a
ld e,a
ld a,b
call 0x0d0a
ld b,(hl)
ld a,c
call 0x0d0a
ld c,(hl)
ld a,e
call 0x0d2f
ld (hl),c
ex de,hl
ld (hl),b
ld a,0xff
ld (0xb1fc),a
ret
and 0x1f
add a,0x93
ld l,a
adc a,0x0d
sub l
ld h,a
ret
and 0x0f
inc a
jr 0x0d1a
xor a
call 0x0d2f
ld a,(de)
ld e,(hl)
call 0x0d24
ld b,c
ld a,e
ld c,0x00
ld hl,0x0d93
cp (hl)
ret z
inc hl
inc c
jr 0x0d29
ld e,a
ld d,0x00
ld hl,0xb1ea
add hl,de
ex de,hl
ld hl,0xffef
add hl,de
ret
ld hl,0xb1fe
push hl
call 0x0170
call 0x0d6d
ld de,0x0d5b
ld b,0x81
pop hl
jp 0x0163
ld hl,0xb1fe
call 0x0170
call 0x0d81
jp 0x0786
ld hl,0xb1fd
dec (hl)
jr z,0x0d6d
dec hl
ld a,(hl)
or a
ret z
call 0x0d81
call 0x0799
jr 0x0d7c
call 0x0d81
ld (0xb1fd),a
call 0x0799
ld hl,0xb1fb
ld a,(hl)
cpl
ld (hl),a
xor a
ld (0xb1fc),a
ret
ld de,0xb1ea
ld a,(0xb1fb)
or a
ld a,(0xb1d8)
ret z
ld de,0xb1d9
ld a,(0xb1d7)
ret
inc d
inc b
dec d
inc e
jr 0x0db6
inc c
dec b
dec c
ld d,0x06
rla
ld e,0x00
rra
ld c,0x07
rrca
ld (de),a
ld (bc),a
inc de
ld a,(de)
add hl,de
dec de
ld a,(bc)
inc bc
dec bc
ld bc,0x0908
djnz 0x0dc4
ld c,a
call 0x0b95
push hl
ld a,d
call 0x0ee8
jr nc,0x0dc7
ld b,d
ld (hl),c
call 0x0bf9
djnz 0x0dbf
jr 0x0dd7
push bc
push de
ld (hl),c
dec d
jr z,0x0dd5
ld c,d
ld b,0x00
ld d,h
ld e,l
inc de
ldir
pop de
pop bc
pop hl
call 0x0c13
dec e
jr nz,0x0db7
ret
ld a,b
xor c
ld c,a
call 0x0b64
ld d,0x08
push hl
push bc
ld a,(hl)
xor c
ld (hl),a
call 0x0bf9
djnz 0x0de9
pop bc
pop hl
call 0x0c13
dec d
jr nz,0x0de7
ret
ld c,a
push bc
ld de,0xffd0
ld b,0x30
call 0x0e24
pop bc
call 0x07ba
ld a,b
or a
jr nz,0x0e19
ld de,0xffb0
call 0x0e37
ld de,0x0000
ld b,0x20
jr 0x0e24
ld de,0x0050
call 0x0e37
ld de,0xffb0
ld b,0x20
ld hl,(0xb1c9)
add hl,de
ld a,h
and 0x07
ld h,a
ld a,(0xb1cb)
add a,h
ld h,a
ld d,b
ld e,0x08
jp 0x0db7
ld hl,(0xb1c9)
add hl,de
jp 0x0b3c
push af
ld a,b
or a
jr z,0x0e73
push hl
call 0x0b95
ex (sp),hl
inc l
call 0x0b64
ld c,d
ld a,e
sub 0x08
ld b,a
jr z,0x0e6a
pop de
call 0x07ba
push bc
push hl
push de
call 0x0ea4
pop hl
call 0x0c13
ex de,hl
pop hl
call 0x0c13
pop bc
djnz 0x0e57
push de
pop hl
ld d,c
ld e,0x08
pop af
ld c,a
jp 0x0db7
push hl
push de
call 0x0b95
ld c,d
ld a,e
sub 0x08
ld b,a
pop de
ex (sp),hl
jr z,0x0e6a
push bc
ld l,e
ld d,h
inc e
call 0x0b64
ex de,hl
call 0x0b64
pop bc
call 0x07ba
call 0x0c2d
push hl
ex de,hl
call 0x0c2d
push hl
push bc
call 0x0ea4
pop bc
pop de
pop hl
djnz 0x0e90
jr 0x0e6a
ld b,0x00
call 0x0ee6
jr c,0x0ec1
call 0x0ee6
jr nc,0x0ed5
push bc
xor a
sub l
ld c,a
ldir
pop bc
cpl
inc a
add a,c
ld c,a
ld a,h
sub 0x08
ld h,a
jr 0x0ed5
call 0x0ee6
jr c,0x0ed8
push bc
xor a
sub e
ld c,a
ldir
pop bc
cpl
inc a
add a,c
ld c,a
ld a,d
sub 0x08
ld d,a
ldir
ret
ld b,c
ld a,(hl)
ld (de),a
call 0x0bf9
ex de,hl
call 0x0bf9
ex de,hl
djnz 0x0ed9
ret
ld a,c
ex de,hl
dec a
add a,l
ret nc
ld a,h
and 0x07
xor 0x07
ret nz
scf
ret
call 0x0aec
ld b,0x08
jr c,0x0f2b
jr z,0x0f02
ld bc,0x0008
ldir
ret
ld c,(hl)
inc hl
push hl
push bc
ld b,0x04
ld hl,0xb1cf
xor a
rlc c
jr nc,0x0f11
or (hl)
inc hl
djnz 0x0f0c
ld (de),a
inc de
ld b,0x04
ld hl,0xb1cf
xor a
rlc c
jr nc,0x0f21
or (hl)
inc hl
djnz 0x0f1c
ld (de),a
inc de
pop bc
pop hl
djnz 0x0f02
ret
ld c,(hl)
inc hl
push hl
push bc
ld b,0x04
xor a
ld hl,0xb1cf
rlc c
jr nc,0x0f3a
ld a,(hl)
inc hl
rlc c
jr nc,0x0f40
or (hl)
ld (de),a
inc de
djnz 0x0f31
pop bc
pop hl
djnz 0x0f2b
ret
ld c,a
call 0x0b64
call 0x0aec
ld b,0x08
jr c,0x0f99
jr z,0x0f61
ld a,(hl)
xor c
cpl
ld (de),a
inc de
call 0x0c13
djnz 0x0f56
ret
push hl
push de
push hl
ld a,(hl)
xor c
ld hl,0xb1cf
ld d,0x04
push af
and (hl)
jr nz,0x0f70
scf
rl e
inc hl
pop af
dec d
jr nz,0x0f6b
pop hl
call 0x0bf9
ld a,(hl)
xor c
ld hl,0xb1cf
ld d,0x04
push af
and (hl)
jr nz,0x0f87
scf
rl e
inc hl
pop af
dec d
jr nz,0x0f82
pop hl
ld (hl),e
ex de,hl
inc de
pop hl
call 0x0c13
djnz 0x0f61
ret
push hl
push de
ld d,0x04
ld a,(hl)
push hl
xor c
push af
ld hl,0xb1cf
and (hl)
jr nz,0x0fa8
scf
rl e
pop af
inc hl
and (hl)
jr nz,0x0fb0
scf
rl e
pop hl
call 0x0bf9
dec d
jr nz,0x0f9d
pop hl
ld (hl),e
ex de,hl
inc de
pop hl
call 0x0c13
djnz 0x0f99
ret
push af
push hl
ld a,d
cpl
ld h,a
ld a,e
cpl
ld l,a
inc hl
add hl,bc
inc hl
ex (sp),hl
xor a
sub e
push af
call 0x0ba9
push hl
ld a,b
cpl
ld l,a
ld h,0xff
ld (0xb207),hl
pop hl
pop af
and b
ld b,a
jr z,0x102a
ex (sp),hl
jr 0x0feb
ld a,(de)
or c
ld c,a
dec hl
ld a,h
or l
jr z,0x1024
inc de
djnz 0x0fe8
ex de,hl
pop hl
pop af
ld b,a
call 0xbde8
call 0x0bf9
push hl
ld hl,(0xb207)
add hl,de
jr nc,0x1010
ex de,hl
pop hl
ld c,0xff
call 0xbde8
call 0x0bf9
jr 0x0ffd
ld a,e
or a
jr z,0x1022
xor a
ld hl,0xb1cf
or (hl)
inc hl
dec e
jr nz,0x1018
ld c,a
pop hl
jp 0xbde8
pop hl
ret
pop hl
pop af
ld b,a
jp 0xbde8
pop de
pop af
ld b,a
jr 0x0ffd
push af
push hl
ld a,h
cpl
ld h,a
ld a,l
cpl
ld l,a
inc hl
add hl,bc
inc hl
ex (sp),hl
call 0x0ba9
pop de
pop af
ld b,a
call 0xbde8
call 0x0c2d
dec de
ld a,d
or e
jr nz,0x1041
ret
inc b
inc b
ld a,(bc)
inc de
inc c
dec bc
inc d
dec d
dec c
ld b,0x1e
rra
rlca
ld (de),a
add hl,de
inc b
rla
inc b
inc b
ld a,(bc)
inc de
inc c
dec bc
inc d
dec d
dec c
ld b,0x1e
rra
rlca
ld (de),a
add hl,de
ld a,(bc)
rlca
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
call 0x1088
xor a
ld (0xb295),a
ld hl,0x0001
call 0x113d
jp 0x10a3
ld hl,0x1091
call 0x0a8a
jp 0x145b
rrca
call 0xc3bd
ld h,e
ld (de),a
jp 0x1263
jp 0x134a
jp 0x13c0
jp 0x140c
ld a,0x08
ld de,0xb20d
ld hl,0xb285
ld bc,0x000f
ldir
dec a
jr nz,0x10a8
ld (0xb20c),a
ret
ld a,(0xb20c)
ld c,a
ld b,0x08
ld a,b
dec a
call 0x10e8
call 0xbdd0
call 0x12c3
ld (0xb290),a
call 0x12bd
ld (0xb28f),a
djnz 0x10bd
ld a,c
ret
ld c,a
ld b,0x08
ld a,b
dec a
call 0x10e8
push bc
ld hl,(0xb28f)
call 0x113d
pop bc
djnz 0x10d8
ld a,c
and 0x07
ld hl,0xb20c
cp (hl)
ret z
push bc
push de
ld c,(hl)
ld (hl),a
ld b,a
ld a,c
call 0x112a
call 0x1122
ld a,b
call 0x112a
ex de,hl
call 0x1122
ld a,c
pop de
pop bc
ret
ld a,(0xb20c)
push af
ld a,c
call 0x10e8
ld a,b
ld (0xb20c),a
call 0x112a
push de
ld a,c
call 0x112a
pop hl
call 0x1122
pop af
jr 0x10e8
push bc
ld bc,0x000f
ldir
pop bc
ret
and 0x07
ld e,a
add a,a
add a,a
add a,a
add a,a
sub e
add a,0x0d
ld e,a
adc a,0xb2
sub e
ld d,a
ld hl,0xb285
ret
ex de,hl
ld a,0x03
ld (0xb28d),a
ld a,d
call 0x12ae
ld a,e
call 0x12a9
xor a
call 0x13a7
call 0x137a
ld hl,0x0000
ld de,0x7f7f
call 0x120c
jp 0x1451
dec a
ld hl,0xb289
add a,(hl)
ld hl,(0xb285)
ld h,a
jr 0x1177
dec a
ld hl,0xb288
add a,(hl)
ld hl,(0xb285)
ld l,a
jr 0x1177
call 0x118a
call 0xbdd0
ld (0xb285),hl
jp 0xbdcd
ld hl,(0xb285)
call 0x1197
ld a,(0xb28c)
ret
ld a,(0xb288)
dec a
add a,l
ld l,a
ld a,(0xb289)
dec a
add a,h
ld h,a
ret
ld a,(0xb288)
sub l
cpl
inc a
inc a
ld l,a
ld a,(0xb289)
sub h
cpl
inc a
inc a
ld h,a
ret
call 0xbdd0
ld hl,(0xb285)
call 0x11da
ld (0xb285),hl
ret c
push hl
ld hl,0xb28c
ld a,b
add a,a
inc a
add a,(hl)
ld (hl),a
call 0x1256
ld a,(0xb290)
push af
call c,0x0e3e
pop af
call nc,0x0dfa
pop hl
ret
call 0x118a
call 0x11da
push af
call 0x1197
pop af
ret
ld a,(0xb28b)
cp h
jp p,0x11e6
ld a,(0xb289)
ld h,a
inc l
ld a,(0xb289)
dec a
cp h
jp m,0x11f3
ld a,(0xb28b)
ld h,a
dec l
ld a,(0xb288)
dec a
cp l
jp p,0x1206
ld a,(0xb28a)
cp l
scf
ret p
ld l,a
ld b,0xff
or a
ret
inc a
ld l,a
ld b,0x00
or a
ret
call 0x0b57
ld a,h
call 0x1244
ld h,a
ld a,d
call 0x1244
ld d,a
cp h
jr nc,0x121e
ld d,h
ld h,a
ld a,l
call 0x124d
ld l,a
ld a,e
call 0x124d
ld e,a
cp l
jr nc,0x122d
ld e,l
ld l,a
ld (0xb288),hl
ld (0xb28a),de
ld a,h
or l
jr nz,0x123e
ld a,d
xor b
jr nz,0x123e
ld a,e
xor c
ld (0xb287),a
jp 0x1177
or a
jp p,0x1249
xor a
cp b
ret c
ld a,b
ret
or a
jp p,0x1252
xor a
cp c
ret c
ld a,c
ret
ld hl,(0xb288)
ld de,(0xb28a)
ld a,(0xb287)
add a,0xff
ret
ld a,(0xb28d)
or a
ret nz
push bc
push de
push hl
call 0x11ab
ld bc,(0xb28f)
call 0x0ddf
pop hl
pop de
pop bc
ret
push af
ld a,0xfd
call 0x128b
pop af
ret
push af
ld a,0x02
call 0x129c
pop af
ret
ld a,0xfe
push af
call 0xbdd0
pop af
push hl
ld hl,0xb28d
and (hl)
ld (hl),a
pop hl
jp 0xbdcd
ld a,0x01
push af
call 0xbdd0
pop af
push hl
ld hl,0xb28d
or (hl)
ld (hl),a
pop hl
ret
ld hl,0xb28f
jr 0x12b1
ld hl,0xb290
push af
call 0xbdd0
pop af
call 0x0c86
ld (hl),a
jp 0xbdcd
ld a,(0xb28f)
jp 0x0ca0
ld a,(0xb290)
jp 0x0ca0
ld hl,(0xb28f)
ld a,h
ld h,l
ld l,a
ld (0xb28f),hl
ret
push de
ld e,a
call 0x132a
jr nc,0x12e3
ld d,a
ld a,e
sub d
ccf
jr nc,0x12e3
ld e,a
jr 0x12e6
ld hl,0x3800
push af
ld d,0x00
ex de,hl
add hl,hl
add hl,hl
add hl,hl
add hl,de
pop af
pop de
ret
ex de,hl
call 0x12d3
ret nc
ex de,hl
ld bc,0x0008
ldir
ret
push hl
ld a,d
or a
ld d,0x00
jr nz,0x131d
dec d
push de
ld c,e
ex de,hl
ld a,c
call 0x12d3
ld a,h
xor d
jr nz,0x1314
ld a,l
xor e
jr z,0x131c
push bc
call 0x12f7
pop bc
inc c
jr nz,0x1308
pop de
call 0x132a
ld (0xb294),de
pop de
ld (0xb296),de
ret
ld hl,(0xb294)
ld a,h
rrca
ld a,l
ld hl,(0xb296)
ret
ld b,a
ld a,(0xb28e)
or a
ret z
push bc
call 0x11a8
inc h
ld (0xb285),hl
dec h
pop af
call 0xbdd3
jp 0xbdcd
push hl
call 0x12d3
ld de,0xb298
push de
call 0x0ef3
pop de
pop hl
call 0x0b64
ld c,0x08
push bc
push hl
push bc
push de
ex de,hl
ld c,(hl)
call 0x1376
call 0x0bf9
pop de
inc de
pop bc
djnz 0x135e
pop hl
call 0x0c13
pop bc
dec c
jr nz,0x135c
ret
ld hl,(0xb291)
jp (hl)
ld hl,0x1391
or a
jr z,0x1383
ld hl,0x139f
ld (0xb291),hl
ret
ld hl,(0xb291)
ld de,0xec6f
add hl,de
ld a,h
or l
ret
ld hl,(0xb28f)
ld a,c
cpl
and h
ld b,a
ld a,c
and l
or b
ld c,0xff
jr 0x13a2
ld a,(0xb28f)
ld b,a
ex de,hl
jp 0x0c6b
ld (0xb293),a
ret
push hl
push de
push bc
call 0xbdd0
ld hl,(0xb285)
call 0xbdd6
push af
call 0xbdcd
pop af
pop bc
pop de
pop hl
ret
ld a,(0xb28f)
ld de,0xb298
push hl
push de
call 0x0f49
call 0x13e3
pop de
pop hl
jr nc,0x13d3
ret nz
ld a,(0xb290)
push de
call 0x0f49
pop de
ld b,0x08
ld a,(de)
cpl
ld (de),a
inc de
djnz 0x13dd
ld c,0x00
ld a,c
call 0x12d3
ld de,0xb298
ld b,0x08
ld a,(de)
cp (hl)
jr nz,0x13fb
inc hl
inc de
djnz 0x13ee
ld a,c
cp 0x20
scf
ret
inc c
jr nz,0x13e5
xor a
ret
push af
push bc
push de
push hl
call 0xbdd9
pop hl
pop de
pop bc
pop af
ret
ld c,a
ld a,(0xb293)
or a
ld a,c
jp nz,0x1945
ld hl,0xb2b8
ld b,(hl)
ld a,b
cp 0x0a
jr nc,0x1446
or a
jr nz,0x1427
ld a,c
cp 0x20
jp nc,0x1334
inc b
ld (hl),b
ld e,b
ld d,0x00
add hl,de
ld (hl),c
ld a,(0xb2b9)
ld e,a
ld hl,0xb2c3
add hl,de
add hl,de
add hl,de
ld a,(hl)
cp b
ret nc
inc hl
ld e,(hl)
inc hl
ld d,(hl)
ld hl,0xb2b9
ld a,c
call 0x0016
xor a
ld (0xb2b8),a
ret
call 0x129a
xor a
jr 0x1456
call 0x1289
ld a,0xff
ld (0xb28e),a
jr 0x1446
xor a
ld (0xb2b8),a
ld hl,0x146b
ld de,0xb2c3
ld bc,0x0060
ldir
ret
nop
jp po,0x0114
inc (hl)
inc de
nop
sbc a,d
ld (de),a
nop
adc a,c
ld (de),a
ld bc,0x0aca
ld bc,0x1945
nop
ld d,c
inc d
nop
ret c
inc d
nop
ld a,(bc)
dec d
nop
rrca
dec d
nop
inc d
dec d
nop
add hl,de
dec d
nop
ld b,b
dec d
nop
jr nc,0x14aa
ld bc,0x12ae
ld bc,0x12a9
nop
ld c,a
dec d
nop
adc a,(hl)
dec d
nop
add a,h
dec d
nop
ld l,l
dec d
nop
ld d,(hl)
dec d
nop
ld c,e
inc d
ld bc,0x14e3
ld bc,0x0c49
nop
ret
ld (de),a
add hl,bc
inc b
dec d
inc b
ret m
inc d
nop
jp po,0x0314
ret pe
inc d
ld (bc),a
pop af
inc d
nop
ld hl,(0x0215)
jr c,0x14e0
ld hl,0xb2c3
ret
add a,a
nop
nop
ld e,d
nop
nop
dec bc
inc d
nop
push ix
ld hl,0x14cf
call 0x1f9f
pop ix
ret
rrca
sbc a,a
jp 0x137a
inc hl
ld a,(hl)
inc hl
ld b,(hl)
inc hl
ld c,(hl)
jp 0x0cec
inc hl
ld b,(hl)
inc hl
ld c,(hl)
jp 0x0cf1
inc hl
ld d,(hl)
inc hl
ld a,(hl)
inc hl
ld e,(hl)
inc hl
ld l,(hl)
ld h,a
jp 0x120c
inc hl
ld a,(hl)
inc hl
jp 0x12f1
ld de,0xff00
jr 0x151c
ld de,0x0100
jr 0x151c
ld de,0x0001
jr 0x151c
ld de,0x00ff
push de
call 0x11a8
pop de
ld a,l
add a,e
ld l,a
ld a,h
add a,d
ld h,a
jp 0x117a
ld hl,(0xb288)
jp 0x1177
call 0x11a8
ld a,(0xb289)
jr 0x1526
inc hl
ld d,(hl)
inc hl
ld e,(hl)
ex de,hl
jp 0x1174
call 0xbdd0
ld hl,(0xb288)
ld (0xb285),hl
ld de,(0xb28a)
jr 0x1597
call 0x11a8
ld d,h
ld e,l
jr 0x1597
call 0x1584
ld hl,(0xb288)
ld de,(0xb28a)
ld a,(0xb285)
ld l,a
inc l
cp e
ld a,(0xb290)
call c,0x0db3
ret
call 0x158e
ld hl,(0xb288)
ld a,(0xb28b)
ld d,a
ld a,(0xb285)
dec a
ld e,a
cp l
ld a,(0xb290)
call nc,0x0db3
ret
call 0x11a8
ld e,l
ld a,(0xb28b)
ld d,a
jr 0x1597
call 0x11a8
ex de,hl
ld l,e
ld a,(0xb289)
ld h,a
ld a,(0xb290)
call 0x0db3
call 0xbdcd
ret
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
call 0x15df
ld hl,0x0001
ld a,h
call 0x17fd
ld a,l
call 0x17f6
ld hl,0x0000
ld d,h
ld e,l
call 0x1604
ld de,0x8000
ld hl,0x7fff
push hl
push de
call 0x1734
pop hl
pop de
jp 0x1779
call 0x180a
ld h,a
call 0x1804
ld l,a
ret
ld hl,0x15e5
jp 0x0a8a
add hl,bc
call c,0xc3bd
ld d,0x18
jp 0x182a
jp 0x183c
call 0x1657
ld (0xb32c),de
ld (0xb32e),hl
ret
ld de,(0xb32c)
ld hl,(0xb32e)
ret
ld (0xb328),de
ld (0xb32a),hl
ld de,0x0000
ld h,d
ld l,e
jr 0x15f4
ld de,(0xb328)
ld hl,(0xb32a)
ret
call 0x15fc
call 0x15f4
push hl
call 0x0aec
cpl
add a,0x01
adc a,0x02
ld h,0x00
ld l,a
bit 7,d
jr z,0x1633
ex de,hl
add hl,de
ex de,hl
cpl
and e
ld e,a
ld a,l
ld hl,(0xb328)
add hl,de
rrca
call c,0x1774
rrca
call c,0x1774
pop de
push hl
ld a,d
rlca
jr nc,0x164a
inc de
ld a,e
and 0xfe
ld e,a
ld hl,(0xb32a)
add hl,de
call 0x1774
pop de
ret
push hl
ld hl,(0xb32c)
add hl,de
pop de
push hl
ld hl,(0xb32e)
add hl,de
pop de
ret
push de
push hl
ld hl,(0xb330)
dec hl
or a
sbc hl,de
jp p,0x16ac
ld hl,(0xb332)
or a
sbc hl,de
jp m,0x16ac
pop de
ld hl,(0xb334)
or a
sbc hl,de
jp m,0x16ad
ld hl,(0xb336)
dec hl
or a
sbc hl,de
jp m,0x1691
ld de,(0xb336)
ld hl,(0xb336)
dec hl
or a
sbc hl,bc
jp p,0x16ad
ld hl,(0xb334)
or a
sbc hl,bc
jp p,0x16a8
ld bc,(0xb334)
ex de,hl
pop de
scf
ret
pop hl
pop de
or a
ret
push hl
push de
ex de,hl
ld hl,(0xb336)
dec hl
or a
sbc hl,de
jp p,0x16f8
ld hl,(0xb334)
or a
sbc hl,de
jp m,0x16f8
pop de
ld hl,(0xb332)
or a
sbc hl,de
jp m,0x16f9
ld hl,(0xb330)
dec hl
or a
sbc hl,de
jp m,0x16de
ld de,(0xb330)
ld hl,(0xb330)
dec hl
or a
sbc hl,bc
jp p,0x16f9
ld hl,(0xb332)
or a
sbc hl,bc
jp p,0x16f5
ld bc,(0xb332)
pop hl
scf
ret
pop de
pop hl
or a
ret
call 0x161d
push hl
ld hl,(0xb330)
dec hl
or a
sbc hl,de
jp p,0x172d
ld hl,(0xb332)
or a
sbc hl,de
jp m,0x172d
pop hl
push de
ex de,hl
ld hl,(0xb336)
dec hl
or a
sbc hl,de
jp p,0x1730
ld hl,(0xb334)
or a
sbc hl,de
jp m,0x1730
ex de,hl
pop de
scf
ret
pop hl
or a
ret
ex de,hl
pop de
or a
ret
push hl
call 0x1760
pop de
push hl
call 0x1760
pop de
ld a,e
sub l
ld a,d
sbc a,h
jr c,0x1745
ex de,hl
ld a,e
and 0xf8
ld e,a
ld a,l
or 0x07
ld l,a
call 0x0aec
dec a
call m,0x1770
dec a
call m,0x1770
ld (0xb330),de
ld (0xb332),hl
ret
ld a,d
or a
ld hl,0x0000
ret m
ld hl,0x027f
ld a,e
sub l
ld a,d
sbc a,h
ret nc
ex de,hl
ret
sra d
rr e
sra h
rr l
ret
push hl
call 0x1792
pop de
push hl
call 0x1792
pop de
ld a,l
sub e
ld a,h
sbc a,d
jr c,0x178a
ex de,hl
ld (0xb334),de
ld (0xb336),hl
ret
ld a,d
or a
ld hl,0x0000
ret m
srl d
rr e
ld hl,0x00c7
ld a,e
sub l
ld a,d
sbc a,h
ret nc
ex de,hl
ret
ld de,(0xb330)
ld hl,(0xb332)
call 0x0aec
dec a
call m,0x17b6
dec a
ret p
add hl,hl
inc hl
ex de,hl
add hl,hl
ex de,hl
ret
ld de,(0xb334)
ld hl,(0xb336)
jr 0x17b6
call 0x17a6
or a
sbc hl,de
inc hl
call 0x1774
call 0x1774
srl l
ld b,l
ld de,(0xb336)
ld hl,(0xb334)
push hl
or a
sbc hl,de
inc hl
ld c,l
ld de,(0xb330)
pop hl
push bc
call 0x0ba9
pop de
ld a,(0xb339)
ld c,a
call 0x0db7
jp 0x160b
call 0x0c86
ld (0xb338),a
ret
call 0x0c86
ld (0xb339),a
ret
ld a,(0xb338)
jp 0x0ca0
ld a,(0xb339)
jp 0x0ca0
call 0x1657
jp 0xbddc
call 0x16fc
ret nc
call 0x0ba9
ld a,(0xb338)
ld b,a
jp 0xbde8
call 0x1657
jp 0xbddf
call 0x16fc
jp nc,0x180a
call 0x0ba9
jp 0xbde5
call 0x1657
jp 0xbde2
push hl
push de
call 0x161a
ld (0xb342),de
ld (0xb344),hl
pop de
pop hl
call 0x161d
push hl
ld hl,(0xb342)
or a
sbc hl,de
ld b,h
ld c,l
jp m,0x1869
ld hl,(0xb342)
ex de,hl
ld (0xb342),hl
ld hl,(0xb344)
ex (sp),hl
ld (0xb344),hl
jr 0x1871
ld hl,0x0000
or a
sbc hl,bc
ld b,h
ld c,l
pop de
ld hl,(0xb344)
or a
sbc hl,de
ex de,hl
jp p,0x188e
ld hl,0x0000
or a
sbc hl,de
ld d,h
ld e,l
or a
sbc hl,bc
ld hl,0x0001
jr nc,0x18b3
jr 0x1898
ld h,d
ld l,e
or a
sbc hl,bc
ld hl,0xffff
jr nc,0x18a1
ld (0xb33a),hl
ld h,b
ld l,c
ld a,0xff
jr 0x18ba
push hl
ld hl,(0xb342)
add hl,bc
ld (0xb342),hl
ld hl,(0xb344)
or a
sbc hl,de
ld (0xb344),hl
pop hl
ld (0xb33a),hl
ld h,b
ld l,c
ex de,hl
xor a
ld (0xb346),a
inc de
ld (0xb340),de
inc hl
call 0x378c
ld (0xb33c),hl
ld (0xb33e),de
ld bc,(0xb340)
ld d,b
ld e,c
srl d
rr e
push bc
ld bc,(0xb33c)
ld hl,(0xb33e)
add hl,de
ex de,hl
ld hl,(0xb340)
or a
sbc hl,de
jr nc,0x18f0
add hl,de
ex de,hl
or a
sbc hl,de
ex de,hl
inc bc
push de
ld a,(0xb346)
or a
jr z,0x191a
ld hl,(0xb342)
ld d,h
ld e,l
add hl,bc
ld (0xb342),hl
ld b,h
ld c,l
dec bc
ld hl,(0xb344)
push hl
call 0x16b0
ld a,(0xb338)
call c,0x0fc4
pop de
ld hl,(0xb33a)
add hl,de
ld (0xb344),hl
jr 0x193d
ld hl,(0xb344)
ld d,h
ld e,l
add hl,bc
ld (0xb344),hl
ld b,h
ld c,l
dec bc
ex de,hl
ld de,(0xb342)
push de
call 0x1664
ld a,(0xb338)
call c,0x102f
pop de
ld hl,(0xb33a)
add hl,de
ld (0xb342),hl
pop de
pop bc
dec bc
ld a,b
or c
jr nz,0x18d7
ret
push ix
call 0x12d3
ld de,0xb33a
push de
pop ix
ld bc,0x0008
ldir
call 0x161a
call 0x16ff
jr nc,0x19a9
push hl
push de
ld bc,0x0007
ex de,hl
add hl,bc
ex de,hl
or a
sbc hl,bc
call 0x16ff
pop de
pop hl
jr nc,0x19a9
call 0x0ba9
ld d,0x08
push hl
ld e,0x08
call 0x19cf
rrc c
call c,0x0bf9
rlc (ix+0)
dec e
jr nz,0x1977
pop hl
call 0x0c13
inc ix
dec d
jr nz,0x1974
pop ix
call 0x15fc
ex de,hl
call 0x0aec
ld bc,0x0008
cp 0x01
jr z,0x19a3
jr nc,0x19a4
add hl,bc
add hl,bc
add hl,bc
add hl,bc
ex de,hl
jp 0x15f4
ld c,0x08
push de
ld b,0x08
call 0x16ff
jr nc,0x19bf
push hl
push de
push bc
call 0x0ba9
call 0x19cf
pop bc
pop de
pop hl
rlc (ix+0)
inc de
djnz 0x19ae
pop de
dec hl
inc ix
dec c
jr nz,0x19ab
jr 0x198f
bit 7,(ix+0)
ld a,(0xb338)
jr nz,0x19db
ld a,(0xb339)
ld b,a
jp 0xbde8
rst 0x00
ld hl,0x1e02
call 0x1c6d
xor a
ld (0xb50b),a
ld h,a
ld l,a
ld (0xb4e7),hl
ld hl,0xb43c
ld de,0xffb0
ld (0xb547),hl
add hl,de
ld (0xb545),hl
add hl,de
ld (0xb543),hl
add hl,de
ld (0xb541),hl
ex de,hl
ld hl,0x1d69
ld bc,0x00fa
ldir
ld b,0x0a
ld hl,0xb4eb
ld (hl),0x00
inc hl
djnz 0x1a12
ld b,0x0a
ld (hl),0xff
inc hl
djnz 0x1a19
call 0x1ced
call 0x1a75
ld de,0xb446
ld hl,0x0098
call 0x1a81
ld hl,0x1a36
call 0x0a8a
jp 0x1c82
inc bc
xor 0xbd
jp 0x1c2f
call 0x1a42
jr nc,0x1a3c
ret
push hl
ld hl,0xb4e0
ld a,(hl)
ld (hl),0xff
cp (hl)
jr c,0x1a73
ld hl,(0xb4de)
ld a,h
or a
jr nz,0x1a64
call 0x1b5c
jr nc,0x1a73
cp 0x80
jr c,0x1a73
cp 0xa0
ccf
jr c,0x1a73
ld h,a
ld l,0x00
push de
call 0x1b2e
jr c,0x1a6c
ld h,0x00
inc l
ld (0xb4de),hl
pop de
jr nc,0x1a53
pop hl
ret
ld a,0xff
ld (0xb4e0),a
ret
call 0x1a81
ccf
ei
ret
di
ld a,l
sub 0x31
ld a,h
sbc a,0x00
ret c
add hl,de
ld (0xb4e3),hl
ex de,hl
ld (0xb4e1),hl
ld bc,0x0a30
ld (hl),0x01
inc hl
ld (hl),c
inc hl
inc c
djnz 0x1a94
ex de,hl
ld hl,0x1ab3
ld c,0x0a
ldir
ex de,hl
ld b,0x13
xor a
ld (hl),a
inc hl
djnz 0x1aa8
ld (0xb4e5),hl
ld (0xb4df),a
ret
ld bc,0x012e
dec c
dec b
ld d,d
ld d,l
ld c,(hl)
ld (0xf30d),hl
ld a,b
call 0x1b3e
jr nc,0x1ae3
push bc
push de
push hl
call 0x1ae5
ccf
pop hl
pop de
pop bc
jr nc,0x1ae3
dec de
ld a,c
inc c
ld (de),a
inc de
rst 0x20
inc hl
dec c
jr nz,0x1ad3
ld hl,0xb4df
ld a,b
xor (hl)
jr nz,0x1ae2
ld (hl),a
scf
ei
ret
ld b,0x00
ld h,b
ld l,a
ld a,c
sub l
ret z
jr nc,0x1afd
ld a,l
ld l,c
ld c,a
add hl,de
ex de,hl
add hl,bc
call 0x1b22
jr z,0x1b1c
ldir
jr 0x1b1c
ld c,a
add hl,de
push hl
ld hl,(0xb4e5)
add hl,bc
ex de,hl
ld hl,(0xb4e3)
ld a,l
sub e
ld a,h
sbc a,d
pop hl
ret c
call 0x1b22
ld hl,(0xb4e5)
jr z,0x1b1c
push de
dec de
dec hl
lddr
pop de
ld (0xb4e5),de
or a
ret
ld a,(0xb4e5)
sub l
ld c,a
ld a,(0xb4e6)
sbc a,h
ld b,a
or c
ret
call 0x1b3e
ret nc
cp l
ret z
ccf
ret nc
push hl
ld h,0x00
add hl,de
ld a,(hl)
pop hl
scf
ret
and 0x7f
cp 0x20
ret nc
push hl
ld hl,(0xb4e1)
ld de,0x0000
inc a
add hl,de
ld e,(hl)
inc hl
dec a
jr nz,0x1b4b
ld a,e
ex de,hl
pop hl
scf
ret
call 0x1b5c
jr nc,0x1b56
ret
push hl
push bc
call 0x1d15
jr nc,0x1b9d
ld a,c
cp 0xef
jr z,0x1b9c
and 0x0f
add a,a
add a,a
add a,a
dec a
inc a
rrc b
jr nc,0x1b6e
call 0x1ba0
ld hl,0xb4e8
bit 7,(hl)
jr z,0x1b87
cp 0x61
jr c,0x1b87
cp 0x7b
jr nc,0x1b87
add a,0xe0
cp 0xff
jr z,0x1b5e
cp 0xfe
ld hl,0xb4e7
jr z,0x1b97
cp 0xfd
inc hl
jr nz,0x1b9c
ld a,(hl)
cpl
ld (hl),a
jr 0x1b5e
scf
pop bc
pop hl
ret
rl c
jp c,0x1d48
ld b,a
ld a,(0xb4e7)
or c
and 0x40
ld a,b
jp nz,0x1d43
jp 0x1d3e
ld hl,(0xb4e7)
ret
ld de,0xb4ff
ld hl,0xb4f5
call 0x0846
ld a,(0xb501)
and 0xa0
ld c,a
ld hl,0xb4ed
or (hl)
ld (hl),a
ld hl,0xb4ff
ld de,0xb4eb
ld b,0x00
ld a,(de)
xor (hl)
and (hl)
call nz,0x1c48
ld a,(hl)
ld (de),a
inc hl
inc de
inc c
ld a,c
and 0x0f
cp 0x0a
jr nz,0x1bd3
ld a,c
and 0xa0
bit 6,c
ld c,a
call nz,0xbdee
ld a,b
or a
ret nz
ld hl,0xb509
dec (hl)
ret nz
ld hl,(0xb50a)
ex de,hl
ld b,d
ld d,0x00
ld hl,0xb4eb
add hl,de
ld a,(hl)
ld hl,(0xb547)
add hl,de
and (hl)
and b
ret z
ld hl,0xb509
inc (hl)
ld a,(0xb540)
or a
ret nz
ld a,c
or e
ld c,a
ld a,(0xb4e9)
ld (0xb509),a
call 0x1cfe
ld a,c
and 0x0f
ld l,a
ld h,b
ld (0xb50a),hl
cp 0x08
ret nz
bit 4,b
ret nz
set 6,c
ret
ld hl,0xb4f3
bit 2,(hl)
ret z
ld a,c
xor 0xa0
jr nz,0x1c90
push bc
inc hl
ld b,0x0a
adc a,(hl)
dec hl
djnz 0x1c3e
pop bc
cp 0xa4
jr nz,0x1c90
rst 0x00
push hl
push de
ld e,a
cpl
inc a
and e
ld b,a
ld a,(0xb4ea)
call 0x1c18
ld a,b
xor e
jr nz,0x1c4a
pop de
pop hl
ret
ld a,(0xb4f1)
and 0x7f
ld l,a
ld a,(0xb4f4)
and 0x7f
ld h,a
ret
ld hl,(0xb4e9)
ret
ld (0xb4e9),hl
ret
call 0x1c82
ld hl,0xb50d
ld b,0x40
call 0x01d2
ld a,0xff
ld (0xb50c),a
ret
push bc
push de
ld hl,0xb50c
ld (hl),0x00
inc hl
call 0x0285
pop de
pop bc
ret
ld hl,0xb50c
ld a,(hl)
ld (hl),0x00
cp (hl)
ret z
push bc
push de
inc hl
call 0x01e2
ld c,0xef
call 0x1cfe
pop de
pop bc
ret
ld hl,(0xb547)
jr 0x1cc8
cp 0x50
ret nc
ld hl,(0xb547)
call 0x1ccd
ld c,a
cpl
and (hl)
ld (hl),a
ld a,c
and b
or (hl)
ld (hl),a
ret
push af
ld a,(0xb4ed)
and 0xa0
ld c,a
pop af
ld hl,0xb4eb
call 0x1ccd
and (hl)
ret
push de
push af
and 0xf8
rrca
rrca
rrca
ld e,a
ld d,0x00
add hl,de
pop af
push hl
ld hl,0x1ce5
and 0x07
ld e,a
add hl,de
ld a,(hl)
pop hl
pop de
ret
ld bc,0x0402
ex af,af'
djnz 0x1d0b
ld b,b
add a,b
di
ld hl,0xb53c
ld (hl),0x15
inc hl
xor a
ld (hl),a
inc hl
ld (hl),0x01
inc hl
ld (hl),a
inc hl
ld (hl),a
ret
ld hl,0xb53c
or a
dec (hl)
jr z,0x1d13
call 0x1d2c
ld (hl),c
inc hl
ld (hl),b
ld hl,0xb540
inc (hl)
ld hl,0xb53e
scf
inc (hl)
ret
ld hl,0xb53e
or a
dec (hl)
jr z,0x1d2a
call 0x1d2c
ld c,(hl)
inc hl
ld b,(hl)
ld hl,0xb540
dec (hl)
ld hl,0xb53c
scf
inc (hl)
ret
inc hl
inc (hl)
ld a,(hl)
cp 0x14
jr nz,0x1d35
xor a
ld (hl),a
add a,a
adc a,0x14
ld l,a
adc a,0xb5
sub l
ld h,a
ret
ld hl,(0xb541)
jr 0x1d4b
ld hl,(0xb543)
jr 0x1d4b
ld hl,(0xb545)
add a,l
ld l,a
adc a,h
sub l
ld h,a
ld a,(hl)
ret
ld hl,(0xb541)
jr 0x1d5f
ld hl,(0xb543)
jr 0x1d5f
ld hl,(0xb545)
cp 0x50
ret nc
add a,l
ld l,a
adc a,h
sub l
ld h,a
ld (hl),b
ret
ret p
di
pop af
adc a,c
add a,(hl)
add a,e
adc a,e
adc a,d
jp p,0x87e0
adc a,b
add a,l
add a,c
add a,d
add a,b
djnz 0x1dd6
dec c
ld e,l
add a,h
rst 0x38
ld e,h
rst 0x38
ld e,(hl)
dec l
ld b,b
ld (hl),b
dec sp
ld a,(0x2e2f)
jr nc,0x1dc4
ld l,a
ld l,c
ld l,h
ld l,e
ld l,l
inc l
jr c,0x1dca
ld (hl),l
ld a,c
ld l,b
ld l,d
ld l,(hl)
jr nz,0x1dd0
dec (hl)
ld (hl),d
ld (hl),h
ld h,a
ld h,(hl)
ld h,d
halt
inc (hl)
inc sp
ld h,l
ld (hl),a
ld (hl),e
ld h,h
ld h,e
ld a,b
ld sp,0xfc32
ld (hl),c
add hl,bc
ld h,c
defb 0xfd
ld a,d
dec bc
ld a,(bc)
ex af,af'
add hl,bc
ld e,b
ld e,d
rst 0x38
ld a,a
call p,0xf5f7
adc a,c
add a,(hl)
add a,e
adc a,e
adc a,d
or 0xe0
add a,a
adc a,b
add a,l
add a,c
add a,d
add a,b
djnz 0x1e46
dec c
ld a,l
add a,h
rst 0x38
ld h,b
rst 0x38
and e
dec a
ld a,h
ld d,b
dec hl
ld hl,(0x3e3f)
ld e,a
add hl,hl
ld c,a
ld c,c
ld c,h
ld c,e
ld c,l
inc a
jr z,0x1e0a
ld d,l
ld e,c
ld c,b
ld c,d
ld c,(hl)
jr nz,0x1e10
dec h
ld d,d
ld d,h
ld b,a
ld b,(hl)
ld b,d
ld d,(hl)
inc h
inc hl
ld b,l
ld d,a
ld d,e
ld b,h
ld b,e
ld e,b
ld hl,0xfc22
ld d,c
add hl,bc
ld b,c
defb 0xfd
ld e,d
dec bc
ld a,(bc)
ex af,af'
add hl,bc
ld e,b
ld e,d
rst 0x38
ld a,a
ret m
ei
ld sp,hl
adc a,c
add a,(hl)
add a,e
adc a,h
adc a,d
jp m,0x87e0
adc a,b
add a,l
add a,c
add a,d
add a,b
djnz 0x1e36
dec c
dec e
add a,h
rst 0x38
inc e
rst 0x38
ld e,0xff
nop
djnz 0x1e25
rst 0x38
rst 0x38
rst 0x38
rra
rst 0x38
rrca
add hl,bc
inc c
dec bc
dec c
rst 0x38
rst 0x38
rst 0x38
dec d
add hl,de
ex af,af'
ld a,(bc)
ld c,0xff
rst 0x38
rst 0x38
ld (de),a
inc d
rlca
ld b,0x02
ld d,0xff
rst 0x38
dec b
rla
inc de
inc b
inc bc
jr 0x1e49
ld a,(hl)
call m,0xe111
ld bc,0x1afe
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
ld a,a
rlca
inc bc
ld c,e
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
xor e
adc a,a
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
xor a
di
ld (0xb552),a
ld (0xb551),a
ld hl,0xb555
ld de,0x1f03
ld b,0x81
call 0x01d2
ld a,0x3f
ld (0xb619),a
ld hl,0xb55c
ld bc,0x003d
ld de,0x0108
xor a
ld (hl),a
inc hl
ld (hl),d
inc hl
ld (hl),e
add hl,bc
inc a
ex de,hl
add hl,hl
ex de,hl
cp 0x03
jr nz,0x1e8a
ld c,0x07
push ix
push hl
ld hl,0xb51d
ld b,c
ld de,0x003f
add hl,de
srl b
jr nc,0x1ea1
push bc
push hl
pop ix
ex de,hl
call 0x227f
inc de
inc de
inc de
ld l,e
ld h,d
inc de
ld bc,0x003b
ld (hl),0x00
ldir
ld (ix+28),0x04
pop bc
ex de,hl
inc b
djnz 0x1ea5
pop hl
pop ix
ret
ld hl,0xb552
di
ld a,(hl)
ld (hl),0x00
ei
or a
ret z
dec hl
ld (hl),a
ld l,0x03
ld c,0x00
ld a,0x07
add a,l
call 0x0826
dec l
jr nz,0x1edb
scf
ret
ld a,(0xb551)
or a
ret z
ld ix,0xb51d
ld de,0x003f
add ix,de
srl a
push af
ld a,(ix+15)
call c,0x2276
pop af
jr nz,0x1ef2
jp 0x201e
push ix
ld hl,0xb550
push hl
xor a
ld (hl),a
inc hl
ld b,(hl)
push bc
inc hl
or (hl)
jr z,0x1f34
ld ix,0xb51d
ld bc,0x003f
add ix,bc
srl a
jr nc,0x1f19
push af
ld a,(ix+4)
rra
call c,0x22c2
ld a,(ix+7)
rra
call c,0x21b6
call c,0x20a8
pop af
jr nz,0x1f16
pop bc
pop hl
ld a,(hl)
or a
jr z,0x1f5a
ld c,a
inc hl
ld a,(hl)
ld (hl),b
xor b
ld b,a
inc hl
or (hl)
ld (hl),a
ld a,b
cpl
and c
jr z,0x1f5a
ld ix,0xb51d
ld de,0x003f
add ix,de
srl a
push af
call c,0x227f
pop af
jr nz,0x1f4f
xor a
ld (0xb554),a
pop ix
ret
ld hl,0xb552
ld a,(hl)
or a
ret z
inc hl
dec (hl)
ret nz
inc (hl)
inc hl
ld a,(hl)
or a
ret nz
dec hl
ld (hl),0x03
dec hl
ld b,(hl)
ld hl,0xb522
ld de,0x003f
xor a
add hl,de
srl b
jr nc,0x1f7b
dec (hl)
jr nz,0x1f88
dec hl
rlc (hl)
adc a,d
inc hl
inc hl
dec (hl)
jr nz,0x1f91
inc hl
rlc (hl)
adc a,d
dec hl
dec hl
inc b
djnz 0x1f7b
or a
ret z
ld hl,0xb554
ld (hl),a
inc hl
jp 0x01e2
call 0x1ee6
ld a,(hl)
and 0x07
scf
ret z
ld c,a
or (hl)
call m,0x1e9a
ld b,c
ld ix,0xb51d
ld de,0x003f
xor a
add ix,de
srl b
jr nc,0x1fb5
ld (ix+30),d
cp (ix+28)
ccf
sbc a,a
inc b
djnz 0x1fb5
or a
ret nz
ld b,c
ld a,(hl)
rra
rra
rra
or b
and 0x0f
ld c,a
inc hl
ld ix,0xb51d
ld de,0x003f
add ix,de
srl b
jr nc,0x1fd9
push hl
push bc
ld a,(ix+27)
inc (ix+27)
dec (ix+28)
ex de,hl
call 0x203a
push hl
ex de,hl
ld a,(ix+1)
cpl
and c
ld (de),a
inc de
ld a,(hl)
inc hl
add a,a
add a,a
add a,a
add a,a
ld b,a
ld a,(hl)
inc hl
and 0x0f
or b
ld (de),a
inc de
ld bc,0x0006
ldir
pop hl
di
ld a,(ix+26)
inc (ix+26)
or (ix+3)
ei
call z,0x20bd
pop bc
pop hl
inc b
djnz 0x1fd6
push hl
ld hl,0xb551
ld a,(hl)
or a
jr z,0x2037
ld (hl),0x00
di
inc hl
ld b,(hl)
or b
ld (hl),a
ld a,b
or a
jr nz,0x2036
inc hl
ld (hl),0x03
inc hl
ld (hl),a
ei
pop hl
scf
ret
and 0x03
add a,a
add a,a
add a,a
add a,0x1f
push ix
pop hl
add a,l
ld l,a
adc a,h
sub l
ld h,a
ret
ld l,a
call 0x1ee6
ld a,l
and 0x07
ret z
ld ix,0xb51d
ld de,0x003f
add ix,de
srl a
jr nc,0x2059
push af
bit 3,(ix+3)
call nz,0x20b7
pop af
jr nz,0x2056
jr 0x201e
and 0x07
ret z
ld hl,0xb520
ld de,0x003f
add hl,de
rra
jr nc,0x2075
di
ld a,(hl)
add a,a
add a,a
add a,a
ld de,0x0019
add hl,de
or (hl)
inc hl
inc hl
ld (hl),0x00
ei
ret
and 0x07
ret z
ex de,hl
ld hl,0xb539
ld bc,0x003f
add hl,bc
rra
jr nc,0x2093
xor a
di
cp (hl)
inc hl
ld (hl),e
inc hl
jr nz,0x20a2
ld (hl),d
ei
ret
ld (hl),a
ei
ex de,hl
jp 0x01e2
ld a,(ix+26)
or a
jp z,0x227f
ld a,(ix+1)
ld hl,0xb550
or (hl)
ld (hl),a
ld a,(ix+25)
call 0x203a
ld a,(hl)
or a
jr z,0x20cd
bit 3,a
jr nz,0x2118
push hl
ld (hl),0x00
call 0x211f
pop hl
ret nc
ld (ix+3),0x10
inc hl
ld a,(hl)
and 0xf0
push af
xor (hl)
ld e,a
inc hl
ld c,(hl)
inc hl
ld d,(hl)
inc hl
or d
or c
jr z,0x20e9
push hl
call 0x22ab
ld d,(ix+1)
pop hl
ld c,(hl)
inc hl
ld e,(hl)
inc hl
ld a,(hl)
inc hl
ld h,(hl)
ld l,a
pop af
call 0x2175
ld hl,0xb551
ld a,(ix+1)
or (hl)
ld (hl),a
inc (ix+25)
dec (ix+26)
inc (ix+28)
di
ld a,(ix+30)
ld (ix+30),0x00
ei
or a
ret z
ld h,a
ld l,(ix+29)
jp 0x01e2
res 3,(hl)
ld (ix+3),0x08
ret
push ix
ld b,a
ld c,(ix+1)
ld ix,0xb55c
bit 0,a
jr nz,0x2139
ld ix,0xb59b
bit 1,a
jr nz,0x2139
ld ix,0xb5da
di
ld a,(ix+3)
and c
jr z,0x216d
ld a,b
cp (ix+1)
jr z,0x2160
push ix
ld ix,0xb5da
bit 2,a
jr nz,0x2154
ld ix,0xb59b
ld a,(ix+3)
and c
jr z,0x216c
ei
call 0x20b7
pop ix
ld (ix+3),0x00
ei
call 0x20b7
pop ix
scf
ret
pop hl
pop ix
ld (ix+3),b
ei
or a
ret
set 7,e
ld (ix+15),e
ld e,a
ld a,l
or h
jr nz,0x2180
dec hl
ld (ix+8),l
ld (ix+9),h
ld a,c
or a
jr z,0x2192
ld a,0x06
call 0x0826
ld a,(ix+2)
or d
call 0x228b
ld a,e
or a
jr z,0x21a4
ld hl,0xb60a
ld d,0x00
add hl,de
ld a,(hl)
or a
jr nz,0x21a7
ld hl,0x21b2
ld (ix+10),l
ld (ix+11),h
call 0x2265
jr 0x21bf
ld bc,0x0001
ret z
ld l,(ix+13)
ld h,(ix+14)
ld e,(ix+16)
ld a,e
cp 0xff
jr z,0x223a
add a,a
ld a,(hl)
inc hl
jr c,0x2213
jr z,0x21d8
dec e
or a
jr nz,0x21d5
or (ix+15)
jp p,0x21dd
add a,(ix+15)
and 0x0f
call 0x2273
ld c,(hl)
ld a,(ix+9)
ld b,a
add a,a
jr c,0x2200
xor a
sub c
add a,(ix+8)
jr c,0x21f8
dec b
jp p,0x21f5
ld c,(ix+8)
xor a
ld b,a
ld (ix+9),b
ld (ix+8),a
or b
jr nz,0x2200
ld e,0xff
ld a,e
or a
call z,0x2246
ld (ix+16),e
di
ld (ix+6),c
ld (ix+7),0x80
ei
or a
ret
ld d,a
ld c,e
ld a,0x0d
call 0x0826
ld c,d
ld a,0x0b
call 0x0826
ld c,(hl)
ld a,0x0c
call 0x0826
ld a,0x10
call 0x2273
call 0x2246
ld a,e
inc a
jr nz,0x21bf
ld hl,0x21b2
call 0x2265
jr 0x21bf
xor a
ld (ix+3),a
ld (ix+7),a
ld (ix+4),a
scf
ret
dec (ix+12)
jr nz,0x2269
ld a,(ix+9)
add a,a
ld hl,0x21b2
jr nc,0x2265
inc (ix+8)
jr nz,0x225f
inc (ix+9)
ld e,0xff
ret z
ld l,(ix+10)
ld h,(ix+11)
ld a,(hl)
ld (ix+12),a
inc hl
ld e,(hl)
inc hl
ld (ix+13),l
ld (ix+14),h
ret
ld (ix+15),a
ld c,a
ld a,(ix+0)
add a,0x08
jp 0x0826
ld a,(ix+1)
cpl
ld hl,0xb552
di
and (hl)
ld (hl),a
ei
xor a
ld b,a
ld a,(ix+1)
or (ix+2)
ld hl,0xb619
di
or (hl)
xor b
cp (hl)
ld (hl),a
ei
jr nz,0x22a0
ld a,b
or a
ret nz
xor a
call 0x2276
di
ld c,(hl)
ld a,0x07
jp 0x0826
call 0x2324
ld a,e
call 0x234e
ret nc
ld a,(hl)
and 0x7f
ret z
ld (ix+17),l
ld (ix+18),h
call 0x2313
jr 0x22cb
ld l,(ix+20)
ld h,(ix+21)
ld e,(ix+24)
ld c,(hl)
inc hl
ld a,e
sub 0xf0
jr c,0x22d6
ld e,0x00
jr 0x22e4
dec e
ld a,c
add a,a
sbc a,a
ld d,a
ld a,(ix+22)
add a,c
ld c,a
ld a,(ix+23)
adc a,d
ld d,a
call 0x2324
ld c,(hl)
ld a,e
or a
jr nz,0x2306
ld a,(ix+19)
dec a
jr nz,0x2303
ld l,(ix+17)
ld h,(ix+18)
ld a,(hl)
add a,0x80
jr c,0x2303
ld (ix+4),0x00
ret
call 0x2313
ld (ix+24),e
di
ld (ix+5),c
ld (ix+4),0x80
ei
ret
ld (ix+19),a
inc hl
ld e,(hl)
inc hl
ld (ix+20),l
ld (ix+21),h
ld a,e
or a
ret nz
inc e
ret
ld a,(ix+0)
add a,a
push af
ld (ix+22),c
call 0x0826
pop af
inc a
ld c,d
ld (ix+23),c
jp 0x0826
ld de,0xb60a
jr 0x2340
ld de,0xb6fa
ex de,hl
call 0x2351
ex de,hl
ret nc
ldir
ret
ld hl,0xb60a
jr 0x2351
ld hl,0xb6fa
or a
ret z
cp 0x10
ret nc
ld bc,0x0010
add a,a
add a,a
add a,a
add a,a
add a,l
ld l,a
adc a,h
sub l
ld h,a
scf
ret
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
call 0x2401
call 0x242e
xor a
call 0x238e
ld hl,0x014d
ld a,0x19
add hl,hl
add hl,hl
add hl,hl
add hl,hl
add hl,hl
add hl,hl
rrca
rrca
and 0x3f
ld l,a
ld (0xb8d1),hl
ret
ld (0xb800),a
ret
ld ix,0xb802
call 0x23af
ret nc
push hl
call 0x253f
ld de,(0xb81c)
ld bc,(0xb81f)
ld a,(0xb819)
pop hl
ret
ld ix,0xb847
ld a,(ix+0)
or a
ret nz
push ix
ex (sp),hl
ld (hl),0x01
inc hl
ld (hl),e
inc hl
ld (hl),d
inc hl
ld (hl),e
inc hl
ld (hl),d
inc hl
ex de,hl
pop hl
push de
ld c,0x40
ld (de),a
inc de
dec c
jr nz,0x23c7
pop de
push de
ld a,b
cp 0x10
jr c,0x23d5
ld b,0x10
inc b
ld c,b
jr 0x23e0
rst 0x20
inc hl
call 0x27b6
ld (de),a
inc de
djnz 0x23d9
dec c
jr z,0x23ee
dec de
ld a,(de)
xor 0x20
jr nz,0x23ee
ld (de),a
jr 0x23e2
pop hl
ld (ix+21),0x01
ld (ix+23),0x16
dec (ix+28)
scf
ret
ld a,(0xb802)
or a
ret z
ld hl,0xb802
ld a,0x01
ld (hl),0x00
inc hl
ld e,(hl)
inc hl
ld d,(hl)
ld hl,0xb8cc
xor (hl)
scf
ret nz
ld (hl),a
sbc a,a
ret
ld a,(0xb847)
cp 0x04
jr z,0x242e
add a,0xff
ret nc
ld hl,0xb85d
ld (hl),0xff
inc hl
inc hl
ld a,(hl)
inc hl
or (hl)
scf
call nz,0x2614
ret nc
ld hl,0xb847
ld a,0x02
jr 0x2406
push hl
push de
push bc
ld b,0x02
call 0x248b
jr nz,0x2459
ld hl,(0xb81a)
ld a,h
or l
scf
call z,0x253f
jr nc,0x2459
ld hl,(0xb81a)
dec hl
ld (0xb81a),hl
ld hl,(0xb805)
rst 0x20
inc hl
ld (0xb805),hl
jr 0x2487
push hl
push de
push bc
ld c,a
ld hl,0xb847
ld b,0x02
call 0x248e
jr nz,0x2487
ld hl,(0xb85f)
ld de,0x0800
sbc hl,de
push bc
call nc,0x2614
pop bc
jr nc,0x2487
ld hl,(0xb85f)
inc hl
ld (0xb85f),hl
ld hl,(0xb84a)
ld (hl),c
inc hl
ld (0xb84a),hl
pop bc
pop de
pop hl
ret
ld hl,0xb802
ld a,(hl)
cp b
ret z
xor 0x01
ret nz
ld (hl),b
ret
call 0x2435
ret nc
push hl
ld hl,(0xb81a)
inc hl
ld (0xb81a),hl
ld hl,(0xb805)
dec hl
ld (0xb805),hl
pop hl
ret
ex de,hl
ld b,0x03
call 0x248b
ret nz
ld (0xb81c),de
call 0x24cf
ld hl,(0xb81c)
ld de,(0xb81a)
add hl,de
ld (0xb81c),hl
call 0x253f
jr c,0x24b9
ret z
ld hl,(0xb8a6)
scf
ret
ld hl,(0xb803)
ld de,(0xb81c)
ld bc,(0xb81a)
ld a,e
sub l
ld a,d
sbc a,h
jp c,0xbaa6
add hl,bc
dec hl
ex de,hl
add hl,bc
dec hl
ex de,hl
jp 0xbaac
push hl
push bc
ld c,a
ld hl,0xb847
ld b,0x03
call 0x248e
ld a,c
pop bc
pop hl
ret nz
ld (0xb85e),a
ld (0xb864),de
ld (0xb866),bc
ld (0xb848),hl
ld (0xb85f),de
ld hl,0xf7ff
add hl,de
ccf
ret c
ld hl,0x0800
ld (0xb85f),hl
ex de,hl
sbc hl,de
push hl
ld hl,(0xb848)
add hl,de
push hl
call 0x2614
pop hl
pop de
ret nc
jr 0x2504
ld hl,0xb802
ld a,(hl)
or a
ret nz
ld (hl),0x05
ld (0xb803),de
call 0x238e
call 0x2544
jr c,0x2537
jp 0x2401
ld a,(0xb818)
or a
ret nz
ld bc,0x8301
call 0x2673
jr nc,0x25a8
ld hl,0xb88c
ld de,0x0040
ld a,0x2c
call 0x2836
jr nc,0x25a8
call 0x25c5
jr nz,0x25b5
ld b,0x8b
jr c,0x2564
ld b,0x89
call 0x2692
ld de,(0xb89f)
ld hl,(0xb81c)
ld a,(0xb802)
cp 0x03
jr z,0x2583
ld hl,0xf7ff
add hl,de
ld a,0x04
jr c,0x25a8
ld hl,(0xb803)
ld (0xb805),hl
ld a,0x16
call 0x2836
jr nc,0x25a8
ld hl,0xb817
inc (hl)
ld a,(0xb89d)
inc hl
ld (hl),a
xor a
ld (0xb81e),a
ld hl,(0xb89f)
ld (0xb81a),hl
call 0x27bf
ld a,0x8c
call z,0x270c
scf
jr 0x260d
or a
ld hl,0xb802
jr z,0x260b
ld b,0x85
call 0x2713
jr 0x254c
push af
ld b,0x88
call 0x2692
pop af
jr nc,0x254c
ld b,0x87
call 0x2711
jr 0x254c
call 0x27bf
scf
ret z
ld a,(0xb81e)
or a
jr z,0x25eb
ld a,(0xb8a3)
cpl
or a
ret nz
ld a,(0xb807)
or a
call nz,0x25f3
ret nz
ld hl,0xb88c
ld de,0xb807
ld bc,0x0040
ldir
xor a
ret
call 0x25f3
ret nz
ex de,hl
ld a,(de)
cp (hl)
ret
ld hl,0xb807
ld de,0xb88c
ld b,0x10
ld a,(de)
call 0x27b6
ld c,a
ld a,(hl)
call 0x27b6
xor c
ret nz
inc hl
inc de
djnz 0x25fb
ret
ld (hl),0x04
sbc a,a
push af
call 0x2a4f
pop af
ret
ld bc,0x8402
call 0x2673
jr nc,0x2666
ld b,0x8a
ld de,0xb84c
call 0x2695
ld hl,0xb863
call 0x2688
jr nc,0x2666
ld hl,(0xb848)
ld (0xb84a),hl
ld (0xb861),hl
push hl
ld hl,0xb84c
ld de,0x0040
ld a,0x2c
call 0x283f
pop hl
jr nc,0x2666
ld de,(0xb85f)
ld a,0x16
call 0x283f
ld hl,0xb85d
call c,0x2688
jr nc,0x2666
ld hl,0x0000
ld (0xb85f),hl
ld hl,0xb85c
inc (hl)
xor a
ld (0xb863),a
scf
jr 0x260d
or a
ld hl,0xb847
jr z,0x260b
ld b,0x86
call 0x2713
jr 0x262c
ld hl,0xb8cc
ld a,c
cp (hl)
ld (hl),0x00
scf
push hl
push bc
call nz,0x2760
pop bc
pop hl
sbc a,a
ret nc
ld (hl),c
jp 0x2a4b
ld a,(hl)
or a
scf
ret z
ld bc,0x012c
jp 0x2a72
ld de,0xb88c
ld a,(0xb800)
or a
ret nz
ld (0xb801),a
call 0x2783
call 0x2726
ld a,(de)
or a
jr nz,0x26b1
ld a,0x8e
call 0x2727
ld bc,0x0010
jr 0x26df
call 0x27bf
ld bc,0x1000
jr z,0x26c6
ld l,e
ld h,d
ld a,(hl)
or a
jr z,0x26c3
inc c
inc hl
djnz 0x26bb
ld a,b
ld b,c
ld c,a
call 0x278d
ld a,(de)
call 0x27b6
or a
jr nz,0x26d2
ld a,0x20
push bc
push de
call 0x1334
pop de
pop bc
inc de
djnz 0x26c9
call 0x275c
ex de,hl
add hl,bc
ex de,hl
ld a,0x8d
call 0x2727
ld b,0x02
call 0x278d
ld a,(de)
call 0x27a4
call 0x275c
inc de
call 0x27bf
jr nz,0x2704
inc de
ld a,(de)
and 0x0f
add a,0x24
call 0x2780
jr 0x275c
ld a,(de)
ld hl,0xb801
or (hl)
ret z
jr 0x277b
call 0x2727
jr 0x277b
ld a,0xff
push af
call 0x271f
pop af
add a,0x60
call nc,0x2780
jr 0x277b
call 0x1180
dec h
call nz,0x277b
ld a,b
push hl
and 0x7f
ld b,a
ld hl,0x27c5
jr z,0x2737
ld a,(hl)
inc hl
or a
jr nz,0x2730
djnz 0x2730
ld a,(hl)
or a
jr z,0x2740
call 0x2743
jr 0x2737
pop hl
inc hl
ret
jp m,0x2727
push hl
ld b,0x00
inc b
ld a,(hl)
inc hl
rlca
jr nc,0x2749
call 0x278d
pop hl
ld a,(hl)
inc hl
and 0x7f
call 0x2780
djnz 0x2753
ld a,0x20
jr 0x2780
ld a,(0xb800)
or a
scf
ret nz
call 0x271f
call 0x1a42
jr c,0x2769
call 0x1279
call 0x1b56
call 0x1281
cp 0x1b
ret z
scf
call 0x2783
ld a,0x0a
jp 0x1400
push af
push hl
ld a,0x01
call 0x115e
pop hl
pop af
ret
push de
call 0x1256
ld e,h
call 0x1180
ld a,h
dec a
add a,e
add a,b
dec a
cp d
pop de
ret c
ld a,0xff
ld (0xb801),a
jr 0x277b
ld b,0xff
inc b
sub 0x0a
jr nc,0x27a6
add a,0x3a
push af
ld a,b
or a
call nz,0x27a4
pop af
jr 0x2780
cp 0x61
ret c
cp 0x7b
ret nc
add a,0xe0
ret
ld a,(0xb802)
cp 0x05
ret
ld d,b
ld (hl),d
ld h,l
ld (hl),e
di
nop
ld d,b
ld c,h
ld b,c
exx
ld (hl),h
ld l,b
ld h,l
xor 0x61
ld l,(hl)
ld sp,hl
ld l,e
ld h,l
ld a,c
cp d
nop
ld h,l
ld (hl),d
ld (hl),d
ld l,a
jp p,0x8000
add a,c
nop
add a,b
ld d,d
ld b,l
jp 0x6e61
call po,0x0081
ld d,d
ld h,l
ld h,c
call po,0x0082
ld d,a
ld (hl),d
ld l,c
ld (hl),h
push hl
add a,d
nop
ld d,d
ld h,l
ld (hl),a
ld l,c
ld l,(hl)
call po,0x6174
ld (hl),b
push hl
nop
ld b,(hl)
ld l,a
ld (hl),l
ld l,(hl)
ld h,h
jr nz,0x27ac
nop
ld c,h
ld l,a
ld h,c
ld h,h
ld l,c
ld l,(hl)
rst 0x20
nop
ld d,e
ld h,c
halt
ld l,c
ld l,(hl)
rst 0x20
nop
nop
ld c,a
ex de,hl
nop
ld h,d
ld l,h
ld l,a
ld h,e
ex de,hl
nop
ld d,l
ld l,(hl)
ld l,(hl)
ld h,c
ld l,l
ld h,l
call po,0x6966
ld l,h
ld h,l
jr nz,0x2853
jr nz,0x27d5
nop
call 0x2873
push af
ld hl,0x28b8
jr 0x2858
call 0x2873
push af
call 0x2964
ld hl,0x28f7
call c,0x289d
call c,0x2979
jr 0x2860
call 0x2873
push af
ld hl,0x28c7
push hl
call 0x2919
pop hl
call c,0x289d
pop de
push af
ld bc,0xf782
out (c),c
ld bc,0xf610
out (c),c
ei
ld a,d
call 0x2a51
pop af
ret
ld (0xb8cd),a
dec de
inc e
push hl
push de
call 0x1e68
pop de
pop ix
call 0x2a4b
di
ld bc,0xf40e
out (c),c
ld bc,0xf6d0
out (c),c
ld c,0x10
out (c),c
ld bc,0xf792
out (c),c
ld bc,0xf658
out (c),c
ret
ld a,d
or a
jr z,0x28ae
push hl
push de
ld e,0x00
call 0x28ae
pop de
pop hl
ret nc
dec d
jr nz,0x28a1
ld bc,0xffff
ld (0xb8d3),bc
ld d,0x01
jp (hl)
call 0x29b0
ret nc
ld (ix+0),a
inc ix
dec d
dec e
jr nz,0x28b8
jr 0x28d9
call 0x29b0
ret nc
ld b,a
call 0xbadc
xor b
ld a,0x03
ret nz
inc ix
dec d
dec e
jr nz,0x28c7
dec d
jr z,0x28e2
call 0x29b0
ret nc
jr 0x28d9
call 0x29a6
call 0x29b0
ret nc
xor d
jr nz,0x28f3
call 0x29b0
ret nc
xor e
scf
ret z
ld a,0x02
or a
ret
call 0xbadc
call 0x29f8
ret nc
inc ix
dec d
dec e
jr nz,0x28f7
dec d
jr z,0x290e
xor a
call 0x29f8
ret nc
jr 0x2904
call 0x29a6
call 0x29f8
ret nc
ld a,e
jp 0x29f8
push de
call 0x2923
pop de
ret c
or a
ret z
jr 0x2919
ld l,0x55
call 0x29cd
ret nc
ld de,0x0000
ld h,d
call 0x29cd
ret nc
ex de,hl
ld b,0x00
add hl,bc
ex de,hl
dec h
jr nz,0x292d
ld h,c
ld a,c
sub d
ld c,a
sbc a,a
ld b,a
ex de,hl
add hl,bc
ex de,hl
call 0x29cd
ret nc
ld a,d
srl a
srl a
adc a,d
sub h
jr c,0x2939
sub c
jr c,0x2939
ld a,d
rra
adc a,d
ld h,a
ld (0xb8ce),hl
call 0x29b0
ret nc
ld hl,0xb8cd
xor (hl)
ret nz
scf
ret
call 0x2a89
ld hl,0x0801
call 0x297c
ret nc
or a
call 0x2a08
ret nc
ld a,(0xb8cd)
jp 0x29f8
ld hl,0x0021
ld b,0xf4
in a,(c)
and 0x04
ret z
push hl
scf
call 0x2a08
pop hl
dec hl
ld a,h
or l
jr nz,0x297c
scf
ret
ld hl,(0xb8d3)
xor h
jp p,0x29a0
ld a,h
xor 0x08
ld h,a
ld a,l
xor 0x10
ld l,a
scf
adc hl,hl
ld (0xb8d3),hl
ret
ld hl,(0xb8d3)
ld a,l
cpl
ld e,a
ld a,h
cpl
ld d,a
ret
push de
ld e,0x08
ld hl,(0xb8ce)
call 0x29d4
call c,0x29dd
jr nc,0x29cb
ld a,h
sub c
sbc a,a
rl d
call 0x2990
dec e
jr nz,0x29b3
ld a,d
scf
pop de
ret
ld b,0xf4
in a,(c)
and 0x04
ret z
ld a,r
add a,0x03
rrca
rrca
and 0x1f
ld c,a
ld b,0xf5
ld a,c
add a,0x02
ld c,a
jr c,0x29f3
in a,(c)
xor l
and 0x80
jr nz,0x29df
xor a
ld r,a
rrc l
scf
ret
xor a
ld r,a
inc a
ret
push de
ld e,0x08
ld d,a
rlc d
call 0x2a08
jr nc,0x2a06
dec e
jr nz,0x29fc
pop de
ret
ld bc,(0xb8d0)
ld hl,(0xb8d2)
sbc a,a
ld h,a
jr z,0x2a1a
ld a,l
add a,a
add a,b
ld l,a
ld a,c
sub b
ld c,a
ld a,l
ld (0xb8d0),a
ld l,0x0a
call 0x2a37
jr c,0x2a2b
sub c
jr nc,0x2a34
cpl
inc a
ld c,a
ld a,h
call 0x2990
ld l,0x0b
call 0x2a37
ld a,0x01
ret
ld a,r
srl a
sub c
jr nc,0x2a41
inc a
jr nz,0x2a3e
ld b,0xf7
out (c),l
push af
xor a
ld r,a
pop af
ret
ld a,0x10
jr 0x2a51
ld a,0xef
push bc
ld b,0xf6
in c,(c)
inc b
and 0x10
ld a,0x08
jr z,0x2a5e
inc a
out (c),a
scf
jr z,0x2a6f
ld a,c
and 0x10
push bc
ld bc,0x00c8
scf
call z,0x2a72
pop bc
ld a,c
pop bc
ret
push bc
push hl
call 0x2a89
ld a,0x42
call 0x1cbd
pop hl
pop bc
jr nz,0x2a87
dec bc
ld a,b
or c
jr nz,0x2a72
scf
ret
xor a
ret
ld bc,0x0682
dec bc
ld a,b
or c
jr nz,0x2a8c
ret
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
push bc
push de
push hl
push hl
ld bc,0x00ff
inc c
ld a,(hl)
inc hl
or a
jr nz,0x2a9f
ld (0xb8dd),a
call 0x2c6f
pop hl
call 0x2d67
push bc
push hl
call 0x2dd9
pop hl
pop bc
call 0x2ac6
jr nc,0x2aaf
push af
call 0x2cd2
pop af
pop hl
pop de
pop bc
cp 0xfc
ret
push hl
ld hl,0x2ae0
ld e,a
ld a,b
or c
ld a,e
jr nz,0x2adb
cp 0xf0
jr c,0x2adb
cp 0xf4
jr nc,0x2adb
ld hl,0x2b1c
call 0x2df6
ex (sp),hl
ret
inc de
ld bc,0xfc2c
ld b,d
dec hl
rst 0x28
ld b,b
dec hl
dec c
ld l,c
dec hl
ret p
or e
dec hl
pop af
ld a,(hl)
dec hl
jp p,0x2baa
di
ld (hl),l
dec hl
ret m
rst 0x00
dec hl
ld sp,hl
sub d
dec hl
jp m,0x2bbd
ei
adc a,c
dec hl
call p,0x2ca2
push af
and a
inc l
or 0x9d
inc l
rst 0x30
sbc a,b
inc l
ret po
jp pe,0x7f2c
dec a
inc l
djnz 0x2b62
inc l
pop hl
ld sp,hl
dec hl
inc b
dec hl
dec hl
ret p
cpl
dec hl
pop af
inc sp
dec hl
jp p,0x2b3b
di
scf
dec hl
ld a,0x07
jr 0x2b3d
ld a,0x0b
jr 0x2b3d
ld a,0x0a
jr 0x2b3d
ld a,0x09
jr 0x2b3d
ld a,0x08
call 0x1400
or a
ret
push af
call 0x2b49
pop af
scf
ret
call 0x2b69
ld hl,0x2b61
call 0x2b69
call 0x1180
dec h
ret z
ld a,0x0d
call 0x1400
ld a,0x0a
jp 0x1400
ld hl,(0x7242)
ld h,l
ld h,c
ld l,e
ld hl,(0xf500)
ld a,(hl)
inc hl
or a
call nz,0x2da8
jr nz,0x2b6a
pop af
scf
ret
ld d,0x01
call 0x2b93
jp z,0x2b2b
ret
call 0x2beb
ld a,c
sub b
cp d
jp c,0x2b2b
jr 0x2b93
call 0x2beb
ld a,d
sub e
ret z
ld d,a
jr 0x2b93
ld d,c
ld a,b
cp c
ret z
push de
call 0x2d50
ld a,(hl)
call nc,0x2da8
inc b
inc hl
call nc,0x2d67
pop de
dec d
jr nz,0x2b93
or 0xff
ret
ld d,0x01
call 0x2bc8
jp z,0x2b2b
ret
call 0x2beb
ld a,b
cp d
jp c,0x2b2b
jr 0x2bc8
call 0x2beb
ld a,e
sub 0x01
ret z
ld d,a
jr 0x2bc8
ld d,c
ld a,b
or a
ret z
call 0x2d4a
jr nc,0x2bd7
dec b
dec hl
dec d
jr nz,0x2bc8
jr 0x2be8
ld a,b
or a
jr z,0x2be5
dec b
dec hl
push de
call 0x2d29
pop de
dec d
jr nz,0x2bd7
call 0x2d67
or 0xff
ret
push hl
call 0x1256
ld a,d
sub h
inc a
ld d,a
call 0x1180
ld e,h
pop hl
ret
ld a,(0xb8dd)
cpl
ld (0xb8dd),a
ret
or a
ret z
ld e,a
ld a,(0xb8dd)
or a
jr z,0x2c17
ld a,b
cp c
jr z,0x2c17
ld (hl),e
ld a,e
call 0x2da8
inc hl
inc b
or a
ret
ld a,c
cp 0xff
jp z,0x2b2b
xor a
ld (0xb8dc),a
ld a,e
call 0x2da8
inc c
push hl
ld a,(hl)
ld (hl),e
ld e,a
inc hl
or a
jr nz,0x2c27
ld (hl),a
pop hl
inc b
inc hl
call 0x2d67
ld a,(0xb8dc)
or a
call nz,0x2d29
ret
ld a,b
or a
jp z,0x2b2b
call 0x2d4a
jp nc,0x2b2b
dec b
dec hl
ld a,b
cp c
jp z,0x2b2b
push hl
inc hl
ld a,(hl)
dec hl
ld (hl),a
inc hl
or a
jr nz,0x2c50
dec hl
ld (hl),0x20
ld (0xb8dc),a
ex (sp),hl
call 0x2d67
ex (sp),hl
ld (hl),0x00
pop hl
dec c
ld a,(0xb8dc)
or a
call nz,0x2d2d
ret
ld hl,0x0000
ld (0xb8de),hl
ret
ld de,(0xb8de)
ld a,h
xor d
ret nz
ld a,l
xor e
ret nz
scf
ret
ld c,a
ld hl,(0xb8de)
ld a,h
or l
ret z
ld a,l
add a,c
ld l,a
call 0x11ce
jr c,0x2c94
ld hl,0x0000
ld (0xb8de),hl
ret
ld de,0x0100
jr 0x2caa
ld de,0xff00
jr 0x2caa
ld de,0x00ff
jr 0x2caa
ld de,0x0001
push bc
push hl
ld hl,(0xb8de)
ld a,h
or l
call z,0x1180
ld a,h
add a,d
ld h,a
ld a,l
add a,e
ld l,a
call 0x11ce
jr nc,0x2cca
push hl
call 0x2cd2
pop hl
ld (0xb8de),hl
call 0x2ccd
pop hl
pop bc
ret
ld de,0x1268
jr 0x2cd5
ld de,0x1268
ld hl,(0xb8de)
ld a,h
or l
ret z
push hl
call 0x1180
ex (sp),hl
call 0x1174
call 0x0016
pop hl
jp 0x1174
push bc
push hl
call 0x1180
ex de,hl
ld hl,(0xb8de)
ld a,h
or l
jr nz,0x2d03
ld a,b
or c
jr nz,0x2d21
call 0x1180
ld (0xb8de),hl
jr 0x2d09
call 0x1174
call 0x1268
call 0x13ab
push af
ex de,hl
call 0x1174
ld hl,(0xb8de)
inc h
call 0x11ce
jr nc,0x2d1d
ld (0xb8de),hl
call 0x2ccd
pop af
pop hl
pop bc
jp c,0x2c01
jp 0x2b2b
ld d,0x01
jr 0x2d2f
ld d,0xff
push bc
push hl
push de
call 0x2cd2
pop de
ld hl,(0xb8de)
ld a,h
or l
jr z,0x2d46
ld a,h
add a,d
ld h,a
call 0x2c8c
call 0x2ccd
pop hl
pop bc
or a
ret
push de
ld de,0xff08
jr 0x2d54
push de
ld de,0x0109
push bc
push hl
call 0x1180
ld a,d
add a,h
ld h,a
call 0x11ce
ld a,e
call c,0x1400
pop hl
pop bc
pop de
ret
push bc
push hl
ex de,hl
call 0x1180
ld c,a
ex de,hl
ld a,(hl)
inc hl
or a
call nz,0x2d85
jr nz,0x2d6f
call 0x1180
sub c
ex de,hl
add a,l
ld l,a
call 0x1174
pop hl
pop bc
or a
ret
push af
push bc
push de
push hl
ld b,a
call 0x1180
sub c
add a,e
ld e,a
ld c,b
call 0x11ce
jr c,0x2d9b
ld a,b
add a,a
inc a
add a,e
ld e,a
ex de,hl
call 0x11ce
ld a,c
call c,0x2da8
pop hl
pop de
pop bc
pop af
ret
push af
push bc
push de
push hl
ld b,a
call 0x1180
ld c,a
push bc
call 0x11ce
pop bc
call c,0x2c76
push af
call c,0x2cd2
ld a,b
push bc
call 0x1334
pop bc
call 0x1180
sub c
call nz,0x2c82
pop af
jr nc,0x2dd4
sbc a,a
ld (0xb8dc),a
call 0x2ccd
pop hl
pop de
pop bc
pop af
ret
call 0x1180
ld c,a
call 0x11ce
call 0x2c76
jp c,0x1a3c
call 0x1279
call 0x1180
sub c
call nz,0x2c82
call 0x1a3c
jp 0x1281
push af
push bc
ld b,(hl)
inc hl
push hl
inc hl
inc hl
cp (hl)
inc hl
jr z,0x2e05
dec b
jr nz,0x2dfb
ex (sp),hl
pop af
ld a,(hl)
inc hl
ld h,(hl)
ld l,a
pop bc
pop af
ret
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
push hl
push de
push bc
ex de,hl
ld bc,0x0005
ldir
ex de,hl
dec hl
ld a,(hl)
pop bc
pop de
pop hl
scf
ret
push de
push bc
or 0x7f
ld b,a
xor a
ld (de),a
inc de
ld (de),a
inc de
ld c,0x90
ld a,h
or a
jr nz,0x2e41
ld c,a
ld h,l
ld l,a
or h
jr z,0x2e4c
ld c,0x88
jp m,0x2e4b
add hl,hl
dec c
or h
jp p,0x2e44
ld a,h
and b
ex de,hl
ld (hl),e
inc hl
ld (hl),a
inc hl
ld (hl),c
pop bc
pop hl
ret
push bc
ld bc,0xa000
call 0x2e60
pop bc
ret
ld b,0xa8
push de
call 0x36a1
pop de
ret
push hl
pop ix
xor a
sub (ix+4)
jr z,0x2e8a
add a,0x90
ret nc
push de
push bc
add a,0x10
call 0x363d
sla c
adc hl,de
jr z,0x2e87
ld a,(ix+3)
or a
ccf
pop bc
pop de
ret
sbc a,a
jr 0x2e83
ld l,a
ld h,a
scf
ret
call 0x2ea1
ret nc
ret p
push hl
ld a,c
inc (hl)
jr nz,0x2e9e
inc hl
dec a
jr nz,0x2e95
inc (hl)
inc c
pop hl
scf
ret
push hl
push de
push hl
pop ix
call 0x3604
pop de
pop hl
ret
call 0x2ea1
ret nc
ret z
bit 7,b
ret z
jr 0x2e93
call 0x35e8
ld b,a
jr z,0x2f0e
call m,0x35fb
push hl
ld a,(ix+4)
sub 0x80
ld e,a
sbc a,a
ld d,a
ld l,e
ld h,d
add hl,hl
add hl,hl
add hl,hl
add hl,de
add hl,hl
add hl,de
add hl,hl
add hl,hl
add hl,de
ld a,h
sub 0x09
ld e,a
pop hl
push bc
push de
call nz,0x2f1f
ld iy,0x2f13
call 0x35a0
jr z,0x2f01
jr nc,0x2ef0
call 0x3412
pop de
dec e
push de
jr 0x2edd
ld iy,0x2f18
call 0x35a0
jr c,0x2f01
call 0x349b
pop de
inc e
push de
jr 0x2ef0
call 0x2e8e
ld a,c
pop de
pop bc
ld c,a
dec a
add a,l
ld l,a
ret nc
inc h
ret
ld e,a
ld (hl),a
ld c,0x01
ret
ret p
rra
cp h
ld a,0x96
cp 0x27
ld l,e
ld l,(hl)
sbc a,(hl)
cpl
inc a
or a
scf
ret z
ld c,a
jp p,0x2f28
cpl
inc a
call 0x2f3e
jr z,0x2f36
push bc
push af
call 0x2f36
pop af
pop bc
jr 0x2f28
ld a,c
or a
jp p,0x349e
jp 0x3415
ld de,0x2f8f
sub 0x0d
ret nc
add a,0x0c
ld e,a
add a,a
add a,a
add a,e
add a,0x53
ld e,a
adc a,0x2f
sub e
ld d,a
xor a
ret
nop
nop
nop
jr nz,0x2edc
nop
nop
nop
ld c,b
add a,a
nop
nop
nop
ld a,d
adc a,d
nop
nop
ld b,b
inc e
adc a,(hl)
nop
nop
ld d,b
ld b,e
sub c
nop
nop
inc h
ld (hl),h
sub h
nop
add a,b
sub (hl)
jr 0x2f0e
nop
jr nz,0x2f35
ld a,0x9b
nop
jr z,0x2fe9
ld l,(hl)
sbc a,(hl)
nop
ld sp,hl
ld (bc),a
dec d
and d
ld b,b
or a
ld b,e
ld a,(0x10a5)
and l
call nc,0xa868
ld hl,(0x84e7)
ld de,0x21ac
ld h,l
adc a,c
ld (0xb8e6),hl
ld hl,0x6c07
ld (0xb8e4),hl
ret
ex de,hl
call 0x2f94
ex de,hl
call 0x35e8
ret z
ld de,0xb8e4
ld b,0x04
ld a,(de)
xor (hl)
ld (de),a
inc de
inc hl
djnz 0x2faf
ret
push hl
ld hl,(0xb8e6)
ld bc,0x6c07
call 0x2ffa
push hl
ld hl,(0xb8e4)
ld bc,0x8965
call 0x2ffa
push de
push hl
ld hl,(0xb8e6)
call 0x2ffa
ex (sp),hl
add hl,bc
ld (0xb8e4),hl
pop hl
ld bc,0x6c07
adc hl,bc
pop bc
add hl,bc
pop bc
add hl,bc
ld (0xb8e6),hl
pop hl
push hl
pop ix
ld hl,(0xb8e4)
ld de,(0xb8e6)
ld bc,0x0000
ld (ix+4),0x80
jp 0x36b1
ex de,hl
ld hl,0x0000
ld a,0x11
dec a
ret z
add hl,hl
rl e
rl d
jr nc,0x3000
add hl,bc
jr nc,0x3000
inc de
jr 0x3000
ld de,0x308b
jr 0x3017
ld de,0x3086
call 0x35e8
dec a
cp 0x01
ret nc
push de
call 0x356c
push af
ld (ix+4),0x80
ld de,0x3081
call 0x359a
jr nc,0x3035
inc (ix+4)
pop af
dec a
push af
call 0x3316
push de
ld de,0x3332
call 0x333f
ex de,hl
pop hl
push de
ld de,0x3332
call 0x3337
pop de
call 0x349e
call 0x32a9
inc b
ld c,h
ld c,e
ld d,a
ld e,(hl)
ld a,a
dec c
ex af,af'
sbc a,e
inc de
add a,b
inc hl
sub e
jr c,0x30d4
add a,b
jr nz,0x309c
xor d
jr c,0x2fe6
push de
call 0x3415
pop de
ex (sp),hl
ld a,h
or a
jp p,0x3071
cpl
inc a
ld l,a
ld a,h
ld h,0x00
call 0x2e29
ex de,hl
pop hl
call 0x333f
pop de
jp 0x3415
inc (hl)
di
inc b
dec (hl)
add a,b
ret m
rla
ld (hl),d
ld sp,0x8580
sbc a,d
jr nz,0x30a9
ld a,a
ld b,0xe1
call 0x3307
jp nc,0x3328
ld de,0x3100
call 0x359a
jp p,0x36ec
ld de,0x3105
call 0x359a
jp m,0x36e6
ld de,0x30fb
call 0x32d4
ld a,e
jp p,0x30b6
neg
push af
call 0x331d
call 0x330f
push de
call 0x32ac
inc bc
call p,0xeb32
rrca
ld (hl),e
ex af,af'
cp b
push de
ld d,d
ld a,e
nop
nop
nop
nop
add a,b
ex (sp),hl
call 0x32ac
ld (bc),a
add hl,bc
ld h,b
sbc a,0x01
ld a,b
ret m
rla
ld (hl),d
ld sp,0xcd7e
dec d
inc (hl)
pop de
push hl
ex de,hl
call 0x3337
ex de,hl
pop hl
call 0x349e
ld de,0x30cc
call 0x333f
inc (ix+4)
pop af
jp 0x357b
add hl,hl
dec sp
xor d
jr c,0x3081
rst 0x00
inc sp
rrca
jr nc,0x308c
ret m
rla
ld (hl),d
or c
add a,a
ld de,0x30cc
ex de,hl
call 0x35e8
ex de,hl
jp z,0x3328
push af
call 0x35e8
jr z,0x3140
ld b,a
call m,0x35fb
push hl
call 0x3182
pop hl
jr c,0x314b
ex (sp),hl
pop hl
jp m,0x3148
push bc
push de
call 0x3014
pop de
call c,0x3415
call c,0x3090
pop bc
ret nc
ld a,b
or a
call m,0x35fb
scf
ret
pop af
scf
ret p
call 0x36ec
xor a
ret
xor a
inc a
ret
ld c,a
pop af
push bc
push af
ld a,c
scf
adc a,a
jr nc,0x3151
ld b,a
call 0x330f
ex de,hl
ld a,b
add a,a
jr z,0x3172
push af
call 0x331d
jr nc,0x3179
pop af
jr nc,0x315a
push af
ld de,0xb8e8
call 0x3415
jr nc,0x3179
pop af
jr 0x315a
pop af
scf
call m,0x32fd
jr 0x3137
pop af
pop af
pop bc
jp m,0x36e6
jp 0x36ee
push bc
call 0x3317
call 0x2ea1
ld a,c
pop bc
jr nc,0x318f
jr z,0x3192
ld a,b
or a
ret
ld c,a
ld a,(hl)
rra
sbc a,a
and b
ld b,a
ld a,c
cp 0x02
sbc a,a
ret nc
ld a,(hl)
cp 0x27
ret c
xor a
ret
ld de,0x31a9
jp 0x2e18
and d
jp c,0x490f
add a,d
ld (0xb8f7),a
ret
call 0x35e8
call m,0x35fb
or 0x01
jr 0x31bd
xor a
push af
ld de,0x321d
ld b,0xf0
ld a,(0xb8f7)
or a
jr z,0x31ce
ld de,0x3222
ld b,0xf6
call 0x3307
jr nc,0x320d
pop af
call 0x32d5
ret nc
ld a,e
rra
call c,0x35fb
ld b,0xe8
call 0x3307
jp nc,0x36e6
inc (ix+4)
call 0x32a9
ld b,0x1b
dec l
ld a,(de)
and 0x6e
ret m
ei
rlca
jr z,0x326a
ld bc,0x6889
sbc a,c
ld a,c
pop hl
rst 0x18
dec (hl)
inc hl
ld a,l
jr z,0x31e9
ld e,l
and l
add a,b
and d
jp c,0x490f
add a,c
jp 0x3415
pop af
jp nz,0x3328
ld a,(0xb8f7)
cp 0x01
ret c
ld de,0x3227
jp 0x3415
ld l,(hl)
add a,e
ld sp,hl
ld (0xb67f),hl
ld h,b
dec bc
ld (hl),0x79
inc de
dec (hl)
jp m,0x7b0e
out (0xe0),a
ld l,0x65
add a,(hl)
call 0x330f
push de
call 0x31b2
ex (sp),hl
call c,0x31bc
pop de
jp c,0x349e
ret
call 0x35e8
push af
call m,0x35fb
ld b,0xf0
call 0x3307
jr nc,0x3299
dec a
push af
call p,0x32fd
call 0x32a9
dec bc
rst 0x38
pop bc
inc bc
rrca
ld (hl),a
add a,e
call m,0xebe8
ld a,c
ld l,a
jp z,0x3678
ld a,e
push de
ld a,0xb0
or l
ld a,h
or b
pop bc
adc a,e
add hl,bc
ld a,l
xor a
ret pe
ld (0x7db4),a
ld (hl),h
ld l,h
ld h,l
ld h,d
ld a,l
pop de
push af
scf
sub d
ld a,(hl)
ld a,d
jp 0x4ccb
ld a,(hl)
add a,e
and a
xor d
xor d
ld a,a
cp 0xff
rst 0x38
ld a,a
add a,b
call 0x3415
pop af
ld de,0x3205
call p,0x333b
ld a,(0xb8f7)
or a
ld de,0x322c
call nz,0x3415
pop af
call m,0x35fb
scf
ret
call 0x331d
call 0x3316
ex de,hl
pop de
ld a,(de)
inc de
ld b,a
call 0x2e18
inc de
inc de
inc de
inc de
inc de
push de
ld de,0xb8ed
dec b
ret z
push bc
ld de,0xb8f2
call 0x3415
pop bc
pop de
push de
push bc
call 0x333f
pop bc
pop de
jr 0x32b7
xor a
push af
call 0x3415
pop af
ld de,0x30cc
call nz,0x333f
push hl
call 0x2e66
jr nc,0x32f9
pop de
push hl
push af
push de
ld de,0xb8ed
call 0x2e29
ex de,hl
pop hl
call 0x3337
pop af
pop de
scf
ret
pop hl
xor a
inc a
ret
call 0x3316
ex de,hl
call 0x3328
jp 0x349e
call 0x356c
ret p
cp b
ret z
ccf
ret
ex de,hl
ld hl,0xb8e8
jp 0x2e18
ex de,hl
ld hl,0xb8f2
jp 0x2e18
ex de,hl
ld hl,0xb8ed
call 0x2e18
ex de,hl
jp 0x3415
push de
ld de,0x3332
call 0x2e18
pop de
scf
ret
nop
nop
nop
nop
add a,c
ld a,0x01
jr 0x3340
ld a,0x80
jr 0x3340
xor a
push hl
pop ix
push de
pop iy
ld b,(ix+3)
ld c,(iy+3)
or a
jr z,0x335a
jp m,0x3358
ld a,0x80
xor c
ld c,a
jr 0x335a
xor b
ld b,a
ld a,(ix+4)
cp (iy+4)
jr nc,0x3376
ld d,b
ld b,c
ld c,d
or a
ld d,a
ld a,(iy+4)
ld (ix+4),a
jr z,0x33c3
sub d
cp 0x21
jr nc,0x33c3
jr 0x3387
xor a
sub (iy+4)
jr z,0x33d5
add a,(ix+4)
cp 0x21
jr nc,0x33d5
push hl
pop iy
ex de,hl
ld e,a
ld a,b
xor c
push af
push bc
ld a,e
call 0x3643
ld a,c
pop bc
ld c,a
pop af
jp m,0x33da
ld a,(iy+0)
add a,l
ld l,a
ld a,(iy+1)
adc a,h
ld h,a
ld a,(iy+2)
adc a,e
ld e,a
ld a,(iy+3)
set 7,a
adc a,d
ld d,a
jp nc,0x36ba
rr d
rr e
rr h
rr l
rr c
inc (ix+4)
jp nz,0x36ba
jp 0x36ee
ld a,(iy+2)
ld (ix+2),a
ld a,(iy+1)
ld (ix+1),a
ld a,(iy+0)
ld (ix+0),a
ld (ix+3),b
scf
ret
xor a
sub c
ld c,a
ld a,(iy+0)
sbc a,l
ld l,a
ld a,(iy+1)
sbc a,h
ld h,a
ld a,(iy+2)
sbc a,e
ld e,a
ld a,(iy+3)
set 7,a
sbc a,d
ld d,a
jr nc,0x340b
ld a,b
cpl
ld b,a
xor a
sub c
ld c,a
ld a,0x00
sbc a,l
ld l,a
ld a,0x00
sbc a,h
ld h,a
ld a,0x00
sbc a,e
ld e,a
ld a,0x00
sbc a,d
ld d,a
add a,a
jp c,0x36ba
jp 0x36b1
ld de,0x2f53
push de
pop iy
push hl
pop ix
ld a,(iy+4)
or a
jr z,0x344d
dec a
call 0x3548
jr z,0x344d
jr nc,0x344a
push af
push bc
call 0x3450
ld a,c
pop bc
ld c,a
pop af
bit 7,d
jr nz,0x3443
dec a
jr z,0x344d
sla c
rl l
rl h
rl e
rl d
ld (ix+4),a
or a
jp nz,0x36ba
jp 0x36ee
jp 0x36e6
ld hl,0x0000
ld e,l
ld d,h
ld a,(iy+0)
call 0x3493
ld a,(iy+1)
call 0x3493
ld a,(iy+2)
call 0x3493
ld a,(iy+3)
or 0x80
ld b,0x08
rra
ld c,a
jr nc,0x3486
ld a,l
add a,(ix+0)
ld l,a
ld a,h
adc a,(ix+1)
ld h,a
ld a,e
adc a,(ix+2)
ld e,a
ld a,d
adc a,(ix+3)
ld d,a
rr d
rr e
rr h
rr l
rr c
djnz 0x3470
ret
or a
jr nz,0x346c
ld l,h
ld h,e
ld e,d
ld d,a
ret
ld de,0x2f53
push de
pop iy
push hl
pop ix
xor a
sub (iy+4)
jr z,0x3502
call 0x3548
jp z,0x36e6
jr nc,0x34ff
push bc
ld c,a
ld e,(hl)
inc hl
ld d,(hl)
inc hl
ld a,(hl)
inc hl
ld h,(hl)
ld l,a
ex de,hl
ld b,(iy+3)
set 7,b
call 0x3532
jr nc,0x34cd
ld a,c
or a
jr nz,0x34d3
jr 0x34fe
dec c
add hl,hl
rl e
rl d
ld (ix+4),c
call 0x3507
ld (ix+3),c
call 0x3507
ld (ix+2),c
call 0x3507
ld (ix+1),c
call 0x3507
call nc,0x3532
sbc a,a
ld l,c
ld h,(ix+1)
ld e,(ix+2)
ld d,(ix+3)
pop bc
ld c,a
jp 0x36ba
pop bc
jp 0x36ee
call 0x3594
xor a
ret
ld c,0x01
jr c,0x3513
ld a,d
cp b
ccf
call z,0x3536
jr nc,0x3526
ld a,l
sub (iy+0)
ld l,a
ld a,h
sbc a,(iy+1)
ld h,a
ld a,e
sbc a,(iy+2)
ld e,a
ld a,d
sbc a,b
ld d,a
scf
rl c
sbc a,a
add hl,hl
rl e
rl d
inc a
jr nz,0x3509
ret
ld a,d
cp b
ccf
ret nz
ld a,e
cp (iy+2)
ccf
ret nz
ld a,h
cp (iy+1)
ccf
ret nz
ld a,l
cp (iy+0)
ccf
ret
ld c,a
ld a,(ix+3)
xor (iy+3)
ld b,a
ld a,(ix+4)
or a
ret z
add a,c
ld c,a
rra
xor c
ld a,c
jp p,0x3568
set 7,(ix+3)
sub 0x7f
scf
ret nz
cp 0x01
ret
or a
ret m
xor a
ret
push hl
pop ix
ld a,(ix+4)
or a
ret z
sub 0x80
scf
ret
push hl
pop ix
or a
jp m,0x3589
add a,(ix+4)
ld (ix+4),a
ccf
ret c
jr 0x3594
add a,(ix+4)
jr c,0x3590
xor a
scf
ld (ix+4),a
ret
ld b,(ix+3)
call 0x36ee
push hl
pop ix
push de
pop iy
ld a,(ix+4)
cp (iy+4)
jr c,0x35e2
jr nz,0x35dd
or a
ret z
ld a,(ix+3)
xor (iy+3)
jp m,0x35dd
ld a,(ix+3)
sub (iy+3)
jr nz,0x35d4
ld a,(ix+2)
sub (iy+2)
jr nz,0x35d4
ld a,(ix+1)
sub (iy+1)
jr nz,0x35d4
ld a,(ix+0)
sub (iy+0)
ret z
sbc a,a
xor (iy+3)
add a,a
sbc a,a
ret c
inc a
ret
ld a,(ix+3)
jr 0x35d8
ld a,(iy+3)
cpl
jr 0x35d8
push hl
pop ix
ld a,(ix+4)
or a
ret z
ld a,(ix+3)
add a,a
sbc a,a
ret c
inc a
ret
push hl
pop ix
ld a,(ix+3)
xor 0x80
ld (ix+3),a
ret
xor a
sub (ix+4)
jr nz,0x3614
ld b,0x04
ld (hl),a
inc hl
djnz 0x360c
ld c,0x01
scf
ret
add a,0xa0
ret nc
push hl
call 0x363d
xor a
cp b
adc a,a
or c
ld c,l
ld b,h
pop hl
ld (hl),c
inc hl
ld (hl),b
inc hl
ld (hl),e
inc hl
ld e,a
ld a,(hl)
ld (hl),d
and 0x80
ld b,a
ld c,0x04
xor a
or (hl)
jr nz,0x3639
dec hl
dec c
jr nz,0x3631
inc c
ld a,e
or a
scf
ret
cp 0x21
jr c,0x3643
ld a,0x21
ld e,(hl)
inc hl
ld d,(hl)
inc hl
ld c,(hl)
inc hl
ld h,(hl)
ld l,c
ex de,hl
set 7,d
ld bc,0x0000
jr 0x365e
ld c,a
ld a,b
or l
ld b,a
ld a,c
ld c,l
ld l,h
ld h,e
ld e,d
ld d,0x00
sub 0x08
jr nc,0x3653
add a,0x08
ret z
srl d
rr e
rr h
rr l
rr c
dec a
jr nz,0x3665
ret
inc d
dec d
ret m
jr nz,0x368f
ld d,a
ld a,e
or h
or l
or c
ret z
ld a,d
sub 0x08
jr c,0x369f
ret z
ld d,e
ld e,h
ld h,l
ld l,c
ld c,0x00
inc d
dec d
jr z,0x367f
ret m
dec a
ret z
sla c
rl l
rl h
rl e
rl d
jp p,0x368f
ret
xor a
ret
push hl
pop ix
ld (ix+4),b
ld b,a
ld e,(hl)
inc hl
ld d,(hl)
inc hl
ld a,(hl)
inc hl
ld h,(hl)
ld l,a
ex de,hl
ld a,(ix+4)
call 0x3673
ld (ix+4),a
sla c
jr nc,0x36d1
inc l
jr nz,0x36d1
inc h
jr nz,0x36d1
inc e
jr nz,0x36d1
inc d
jr nz,0x36d1
inc (ix+4)
jr z,0x36ee
ld d,0x80
ld a,b
or 0x7f
and d
ld (ix+3),a
ld (ix+2),e
ld (ix+1),h
ld (ix+0),l
push ix
pop hl
scf
ret
xor a
ld (ix+4),a
jr 0x36e1
ld b,0x00
ld a,b
or 0x7f
ld (ix+3),a
or 0xff
ld (ix+4),a
ld (ix+0),a
ld (ix+1),a
ld (ix+2),a
ret
rst 0x00
rst 0x00
rst 0x00
rst 0x00
rst 0x00
ld b,h
call 0x37d1
jr 0x3710
ld b,0x00
ld e,0x00
ld c,0x02
ret
ld a,h
or a
jp m,0x3720
or b
jp m,0x37d4
scf
ret
xor 0x80
or l
ret nz
ld a,b
scf
adc a,a
ret
or a
adc hl,de
scf
ret po
or 0xff
ret
ex de,hl
or a
sbc hl,de
scf
ret po
or 0xff
ret
call 0x3745
call 0x3750
jp nc,0x3715
or 0xff
ret
ld a,h
xor d
ld b,a
ex de,hl
call 0x37d1
ex de,hl
jp 0x37d1
ld a,h
or a
jr z,0x3759
ld a,d
or a
scf
ret nz
ex de,hl
or l
ret z
ld a,d
or e
ld a,l
ld l,e
ld h,d
ret z
cp 0x03
jr c,0x3775
scf
adc a,a
jr nc,0x3766
add hl,hl
ret c
add a,a
jr nc,0x3770
add hl,de
ret c
cp 0x80
jr nz,0x3769
ret
cp 0x01
ret z
add hl,hl
ret
call 0x3789
jp c,0x3715
ret
ld c,h
call 0x3789
ex de,hl
ld b,c
jr 0x377d
call 0x3745
ld a,d
or e
ret z
push bc
ex de,hl
ld b,0x01
ld a,h
or a
jr nz,0x37a0
ld a,d
cp l
jr c,0x37a0
ld h,l
ld l,0x00
ld b,0x09
ld a,e
sub l
ld a,d
sbc a,h
jr c,0x37ab
inc b
add hl,hl
jr nc,0x37a0
ccf
ccf
ld a,b
ld b,h
ld c,l
ld hl,0x0000
dec a
jr nz,0x37b8
jr 0x37ce
add hl,hl
push af
ld a,b
rra
ld b,a
ld a,c
rra
ld c,a
ld a,e
sub c
ld a,d
sbc a,b
jr c,0x37ca
ld d,a
ld a,e
sub c
ld e,a
inc l
pop af
dec a
jr nz,0x37b7
scf
pop bc
ret
ld a,h
or a
ret p
xor a
sub l
ld l,a
sbc a,h
sub l
cp h
ld h,a
scf
ret nz
cp 0x01
ret
ld a,h
add a,a
sbc a,a
ret c
or l
ret z
xor a
inc a
ret
ld a,h
xor d
ld a,h
jp p,0x37f4
add a,a
sbc a,a
ret c
inc a
ret
cp d
jr nz,0x37f0
ld a,l
sub e
jr nz,0x37f0
ret
rst 0x00
rst 0x00
rst 0x00
ld bc,0xc3ff
jp 0xc3c3
jp 0xffc3
rst 0x38
ret nz
ret nz
ret nz
ret nz
ret nz
ret nz
ret nz
jr 0x382a
jr 0x382c
jr 0x382e
jr 0x3817
inc bc
inc bc
inc bc
inc bc
inc bc
inc bc
inc bc
rst 0x38
inc c
jr 0x3853
ld a,(hl)
inc c
jr 0x3857
nop
rst 0x38
jp 0xdbe7
in a,(0xe7)
jp 0x00ff
ld bc,0x0603
call z,0x3078
nop
inc a
ld h,(hl)
jp 0xffc3
inc h
rst 0x20
nop
nop
nop
jr nc,0x38a4
rst 0x38
ld h,b
jr nc,0x3848
nop
nop
inc c
ld b,0xff
ld b,0x0c
nop
jr 0x386a
jr 0x386c
in a,(0x7e)
inc a
jr 0x3871
inc a
ld a,(hl)
in a,(0x18)
jr 0x3877
jr 0x3879
ld e,d
inc a
sbc a,c
in a,(0x7e)
inc a
jr 0x3869
inc bc
inc sp
ld h,e
cp 0x60
jr nc,0x3870
inc a
ld h,(hl)
rst 0x38
in a,(0xdb)
rst 0x38
ld h,(hl)
inc a
inc a
ld h,(hl)
jp 0xdbdb
jp 0x3c66
rst 0x38
jp 0xffc3
jp 0xc3c3
rst 0x38
inc a
ld a,(hl)
in a,(0xdb)
rst 0x18
jp 0x3c66
inc a
ld h,(hl)
jp 0xdbdf
in a,(0x7e)
inc a
inc a
ld h,(hl)
jp 0xdbfb
in a,(0x7e)
inc a
inc a
ld a,(hl)
in a,(0xdb)
ei
jp 0x3c66
nop
ld bc,0x1e33
adc a,0x7b
ld sp,0x7e00
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
rst 0x20
inc bc
inc bc
inc bc
rst 0x38
inc bc
inc bc
inc bc
nop
rst 0x38
ld h,(hl)
inc a
jr 0x38dd
inc a
ld h,(hl)
rst 0x38
jr 0x38e2
inc a
inc a
inc a
inc a
jr 0x38e8
inc a
ld h,(hl)
ld h,(hl)
jr nc,0x38ed
nop
jr 0x38d8
inc a
ld h,(hl)
jp 0xc3ff
jp 0x3c66
rst 0x38
in a,(0xdb)
in a,(0xfb)
jp 0xffc3
rst 0x38
jp 0xfbc3
in a,(0xdb)
in a,(0xff)
rst 0x38
jp 0xdfc3
in a,(0xdb)
in a,(0xff)
rst 0x38
in a,(0xdb)
in a,(0xdf)
jp 0xffc3
nop
nop
nop
nop
nop
nop
nop
nop
jr 0x3922
jr 0x3924
jr 0x390e
jr 0x3910
ld l,h
ld l,h
ld l,h
nop
nop
nop
nop
nop
ld l,h
ld l,h
cp 0x6c
cp 0x6c
ld l,h
nop
jr 0x3960
ld e,b
inc a
ld a,(de)
ld a,h
jr 0x3928
nop
add a,0xcc
jr 0x395d
ld h,(hl)
add a,0x00
jr c,0x399e
jr c,0x39aa
call c,0x76cc
nop
jr 0x3952
jr nc,0x393c
nop
nop
nop
nop
inc c
jr 0x3973
jr nc,0x3975
jr 0x3953
nop
jr nc,0x3962
inc c
inc c
inc c
jr 0x397f
nop
nop
ld h,(hl)
inc a
rst 0x38
inc a
ld h,(hl)
nop
nop
nop
jr 0x3973
ld a,(hl)
jr 0x3976
nop
nop
nop
nop
nop
nop
nop
jr 0x397f
jr nc,0x3969
nop
nop
ld a,(hl)
nop
nop
nop
nop
nop
nop
nop
nop
nop
jr 0x398f
nop
ld b,0x0c
jr 0x39ac
ld h,b
ret nz
add a,b
nop
ld a,h
add a,0xce
sub 0xe6
add a,0x7c
nop
jr 0x39c2
jr 0x39a4
jr 0x39a6
ld a,(hl)
nop
inc a
ld h,(hl)
ld b,0x3c
ld h,b
ld h,(hl)
ld a,(hl)
nop
inc a
ld h,(hl)
ld b,0x1c
ld b,0x66
inc a
nop
inc e
inc a
ld l,h
call z,0x0cfe
ld e,0x00
ld a,(hl)
ld h,d
ld h,b
ld a,h
ld b,0x66
inc a
nop
inc a
ld h,(hl)
ld h,b
ld a,h
ld h,(hl)
ld h,(hl)
inc a
nop
ld a,(hl)
ld h,(hl)
ld b,0x0c
jr 0x39d6
jr 0x39c0
inc a
ld h,(hl)
ld h,(hl)
inc a
ld h,(hl)
ld h,(hl)
inc a
nop
inc a
ld h,(hl)
ld h,(hl)
ld a,0x06
ld h,(hl)
inc a
nop
nop
nop
jr 0x39ec
nop
jr 0x39ef
nop
nop
nop
jr 0x39f4
nop
jr 0x39f7
jr nc,0x39ed
jr 0x3a13
ld h,b
jr nc,0x39fe
inc c
nop
nop
nop
ld a,(hl)
nop
nop
ld a,(hl)
nop
nop
ld h,b
jr nc,0x3a0b
inc c
jr 0x3a26
ld h,b
nop
inc a
ld h,(hl)
ld h,(hl)
inc c
jr 0x39fe
jr 0x3a00
ld a,h
add a,0xde
sbc a,0xde
ret nz
ld a,h
nop
jr 0x3a46
ld h,(hl)
ld h,(hl)
ld a,(hl)
ld h,(hl)
ld h,(hl)
nop
call m,0x6666
ld a,h
ld h,(hl)
ld h,(hl)
call m,0x3c00
ld h,(hl)
ret nz
ret nz
ret nz
ld h,(hl)
inc a
nop
ret m
ld l,h
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld l,h
ret m
nop
cp 0x62
ld l,b
ld a,b
ld l,b
ld h,d
cp 0x00
cp 0x62
ld l,b
ld a,b
ld l,b
ld h,b
ret p
nop
inc a
ld h,(hl)
ret nz
ret nz
adc a,0x66
ld a,0x00
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld a,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
nop
ld a,(hl)
jr 0x3a63
jr 0x3a65
jr 0x3acd
nop
ld e,0x0c
inc c
inc c
call z,0x78cc
nop
and 0x66
ld l,h
ld a,b
ld l,h
ld h,(hl)
and 0x00
ret p
ld h,b
ld h,b
ld h,b
ld h,d
ld h,(hl)
cp 0x00
add a,0xee
cp 0xfe
sub 0xc6
add a,0x00
add a,0xe6
or 0xde
adc a,0xc6
add a,0x00
jr c,0x3ae6
add a,0xc6
add a,0x6c
jr c,0x3a80
call m,0x6666
ld a,h
ld h,b
ld h,b
ret p
nop
jr c,0x3af6
add a,0xc6
jp c,0x76cc
nop
call m,0x6666
ld a,h
ld l,h
ld h,(hl)
and 0x00
inc a
ld h,(hl)
ld h,b
inc a
ld b,0x66
inc a
nop
ld a,(hl)
ld e,d
jr 0x3abc
jr 0x3abe
inc a
nop
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
inc a
nop
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
inc a
jr 0x3ab8
add a,0xc6
add a,0xd6
cp 0xee
add a,0x00
add a,0x6c
jr c,0x3afc
ld l,h
add a,0xc6
nop
ld h,(hl)
ld h,(hl)
ld h,(hl)
inc a
jr 0x3ae6
inc a
nop
cp 0xc6
adc a,h
jr 0x3b07
ld h,(hl)
cp 0x00
inc a
jr nc,0x3b0b
jr nc,0x3b0d
jr nc,0x3b1b
nop
ret nz
ld h,b
jr nc,0x3afc
inc c
ld b,0x02
nop
inc a
inc c
inc c
inc c
inc c
inc c
inc a
nop
jr 0x3b2e
ld a,(hl)
jr 0x3b0d
jr 0x3b0f
nop
nop
nop
nop
nop
nop
nop
nop
rst 0x38
jr nc,0x3b1a
inc c
nop
nop
nop
nop
nop
nop
nop
ld a,b
inc c
ld a,h
call z,0x0076
ret po
ld h,b
ld a,h
ld h,(hl)
ld h,(hl)
ld h,(hl)
call c,0x0000
nop
inc a
ld h,(hl)
ld h,b
ld h,(hl)
inc a
nop
inc e
inc c
ld a,h
call z,0xcccc
halt
nop
nop
nop
inc a
ld h,(hl)
ld a,(hl)
ld h,b
inc a
nop
inc e
ld (hl),0x30
ld a,b
jr nc,0x3b66
ld a,b
nop
nop
nop
ld a,0x66
ld h,(hl)
ld a,0x06
ld a,h
ret po
ld h,b
ld l,h
halt
ld h,(hl)
ld h,(hl)
and 0x00
jr 0x3b4a
jr c,0x3b64
jr 0x3b66
inc a
nop
ld b,0x00
ld c,0x06
ld b,0x66
ld h,(hl)
inc a
ret po
ld h,b
ld h,(hl)
ld l,h
ld a,b
ld l,h
and 0x00
jr c,0x3b7a
jr 0x3b7c
jr 0x3b7e
inc a
nop
nop
nop
ld l,h
cp 0xd6
sub 0xc6
nop
nop
nop
call c,0x6666
ld h,(hl)
ld h,(hl)
nop
nop
nop
inc a
ld h,(hl)
ld h,(hl)
ld h,(hl)
inc a
nop
nop
nop
call c,0x6666
ld a,h
ld h,b
ret p
nop
nop
halt
call z,0x7ccc
inc c
ld e,0x00
nop
call c,0x6076
ld h,b
ret p
nop
nop
nop
inc a
ld h,b
inc a
ld b,0x7c
nop
jr nc,0x3bd2
ld a,h
jr nc,0x3bd5
ld (hl),0x1c
nop
nop
nop
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld a,0x00
nop
nop
ld h,(hl)
ld h,(hl)
ld h,(hl)
inc a
jr 0x3bb8
nop
nop
add a,0xd6
sub 0xfe
ld l,h
nop
nop
nop
add a,0x6c
jr c,0x3c32
add a,0x00
nop
nop
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld a,0x06
ld a,h
nop
nop
ld a,(hl)
ld c,h
jr 0x3c08
ld a,(hl)
nop
ld c,0x18
jr 0x3c4c
jr 0x3bf6
ld c,0x00
jr 0x3bfa
jr 0x3bfc
jr 0x3bfe
jr 0x3be8
ld (hl),b
jr 0x3c03
ld c,0x18
jr 0x3c5f
nop
halt
call c,0x0000
nop
nop
nop
nop
call z,0xcc33
inc sp
call z,0xcc33
inc sp
nop
nop
nop
nop
nop
nop
nop
nop
ret p
ret p
ret p
ret p
nop
nop
nop
nop
rrca
rrca
rrca
rrca
nop
nop
nop
nop
rst 0x38
rst 0x38
rst 0x38
rst 0x38
nop
nop
nop
nop
nop
nop
nop
nop
ret p
ret p
ret p
ret p
ret p
ret p
ret p
ret p
ret p
ret p
ret p
ret p
rrca
rrca
rrca
rrca
ret p
ret p
ret p
ret p
rst 0x38
rst 0x38
rst 0x38
rst 0x38
ret p
ret p
ret p
ret p
nop
nop
nop
nop
rrca
rrca
rrca
rrca
ret p
ret p
ret p
ret p
rrca
rrca
rrca
rrca
rrca
rrca
rrca
rrca
rrca
rrca
rrca
rrca
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rrca
rrca
rrca
rrca
nop
nop
nop
nop
rst 0x38
rst 0x38
rst 0x38
rst 0x38
ret p
ret p
ret p
ret p
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rrca
rrca
rrca
rrca
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
rst 0x38
nop
nop
nop
jr 0x3c9d
nop
nop
nop
jr 0x3ca2
jr 0x3ca4
jr 0x3c8e
nop
nop
nop
nop
nop
rra
rra
nop
nop
nop
jr 0x3cb2
jr 0x3cbb
rrca
nop
nop
nop
nop
nop
nop
jr 0x3cbd
jr 0x3cbf
jr 0x3cc1
jr 0x3cc3
jr 0x3cc5
jr 0x3cc7
jr 0x3cb1
nop
nop
rrca
rra
jr 0x3ccf
jr 0x3cd1
jr 0x3cd3
rra
rra
jr 0x3cd7
jr 0x3cc1
nop
nop
ret m
ret m
nop
nop
nop
jr 0x3ce2
jr 0x3cc4
ret p
nop
nop
nop
nop
nop
nop
rst 0x38
rst 0x38
nop
nop
nop
jr 0x3cf2
jr 0x3cdb
rst 0x38
nop
nop
nop
nop
nop
nop
ret p
ret m
jr 0x3cff
jr 0x3d01
jr 0x3d03
ret m
ret m
jr 0x3d07
jr 0x3cf1
nop
nop
rst 0x38
rst 0x38
jr 0x3d0f
jr 0x3d11
jr 0x3d13
rst 0x38
rst 0x38
jr 0x3d17
jr 0x3d11
jr c,0x3d6f
add a,0x00
nop
nop
nop
inc c
jr 0x3d3b
nop
nop
nop
nop
nop
ld h,(hl)
ld h,(hl)
nop
nop
nop
nop
nop
nop
inc a
ld h,(hl)
ld h,b
ret m
ld h,b
ld h,(hl)
cp 0x00
jr c,0x3d66
cp d
and d
cp d
ld b,h
jr c,0x3d28
ld a,(hl)
call p,0x74f4
inc (hl)
inc (hl)
inc (hl)
nop
ld e,0x30
jr c,0x3da0
jr c,0x3d4e
ret p
nop
jr 0x3d52
inc c
nop
nop
nop
nop
nop
ld b,b
ret nz
ld b,h
ld c,h
ld d,h
ld e,0x04
nop
ld b,b
ret nz
ld c,h
ld d,d
ld b,h
ex af,af'
ld e,0x00
ret po
djnz 0x3db5
ld d,0xea
rrca
ld (bc),a
nop
nop
jr 0x3d73
ld a,(hl)
jr 0x3d76
ld a,(hl)
nop
jr 0x3d7a
nop
ld a,(hl)
nop
jr 0x3d7f
nop
nop
nop
nop
ld a,(hl)
ld b,0x06
nop
nop
jr 0x3d72
jr 0x3da4
ld h,(hl)
ld h,(hl)
inc a
nop
jr 0x3d7a
jr 0x3d94
jr 0x3d96
jr 0x3d80
nop
nop
ld (hl),e
sbc a,0xcc
sbc a,0x73
nop
ld a,h
add a,0xc6
call m,0xc6c6
ret m
ret nz
nop
ld h,(hl)
ld h,(hl)
inc a
ld h,(hl)
ld h,(hl)
inc a
nop
inc a
ld h,b
ld h,b
inc a
ld h,(hl)
ld h,(hl)
inc a
nop
nop
nop
ld e,0x30
ld a,h
jr nc,0x3dc5
nop
jr c,0x3e16
add a,0xfe
add a,0x6c
jr c,0x3db0
nop
ret nz
ld h,b
jr nc,0x3ded
ld l,h
add a,0x00
nop
nop
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld a,h
ld h,b
ld h,b
nop
nop
nop
cp 0x6c
ld l,h
ld l,h
nop
nop
nop
nop
ld a,(hl)
ret c
ret c
ld (hl),b
nop
inc bc
ld b,0x0c
inc a
ld h,(hl)
inc a
ld h,b
ret nz
inc bc
ld b,0x0c
ld h,(hl)
ld h,(hl)
inc a
ld h,b
ret nz
nop
and 0x3c
jr 0x3e1d
ld l,h
rst 0x00
nop
nop
nop
ld h,(hl)
jp 0xdbdb
ld a,(hl)
nop
cp 0xc6
ld h,b
jr nc,0x3e55
add a,0xfe
nop
nop
ld a,h
add a,0xc6
add a,0x6c
xor 0x00
jr 0x3e32
ld h,b
ret nz
add a,b
nop
nop
nop
jr 0x3e16
ld b,0x03
ld bc,0x0000
nop
nop
nop
nop
ld bc,0x0603
inc c
jr 0x3e19
nop
nop
add a,b
ret nz
ld h,b
jr nc,0x3e38
jr 0x3e5e
ld h,(hl)
jp 0x0081
nop
nop
jr 0x3e36
ld b,0x03
inc bc
ld b,0x0c
jr 0x3e31
nop
nop
add a,c
jp 0x3c66
jr 0x3e51
jr nc,0x3e9b
ret nz
ret nz
ld h,b
jr nc,0x3e58
jr 0x3e72
ld h,b
pop bc
add a,e
ld b,0x0c
jr 0x3e61
inc c
ld b,0x83
pop bc
ld h,b
jr nc,0x3e68
jr 0x3e8e
ld h,(hl)
jp 0x66c3
inc a
jr 0x3e1c
rst 0x20
ld a,(hl)
inc a
inc a
ld a,(hl)
rst 0x20
jp 0x0703
ld c,0x1c
jr c,0x3ed6
ret po
ret nz
ret nz
ret po
ld (hl),b
jr c,0x3e89
ld c,0x07
inc bc
call z,0x33cc
inc sp
call z,0x33cc
inc sp
xor d
ld d,l
xor d
ld d,l
xor d
ld d,l
xor d
ld d,l
rst 0x38
rst 0x38
nop
nop
nop
nop
nop
nop
inc bc
inc bc
inc bc
inc bc
inc bc
inc bc
inc bc
inc bc
nop
nop
nop
nop
nop
nop
rst 0x38
rst 0x38
ret nz
ret nz
ret nz
ret nz
ret nz
ret nz
ret nz
ret nz
rst 0x38
cp 0xfc
ret m
ret p
ret po
ret nz
add a,b
rst 0x38
ld a,a
ccf
rra
rrca
rlca
inc bc
ld bc,0x0301
rlca
rrca
rra
ccf
ld a,a
rst 0x38
add a,b
ret nz
ret po
ret p
ret m
call m,0xfffe
xor d
ld d,l
xor d
ld d,l
nop
nop
nop
nop
ld a,(bc)
dec b
ld a,(bc)
dec b
ld a,(bc)
dec b
ld a,(bc)
dec b
nop
nop
nop
nop
xor d
ld d,l
xor d
ld d,l
and b
ld d,b
and b
ld d,b
and b
ld d,b
and b
ld d,b
xor d
ld d,h
xor b
ld d,b
and b
ld b,b
add a,b
nop
xor d
ld d,l
ld hl,(0x0a15)
dec b
ld (bc),a
ld bc,0x0201
dec b
ld a,(bc)
dec d
ld hl,(0xaa55)
nop
add a,b
ld b,b
and b
ld d,b
xor b
ld d,h
xor d
ld a,(hl)
rst 0x38
sbc a,c
rst 0x38
cp l
jp 0x7eff
ld a,(hl)
rst 0x38
sbc a,c
rst 0x38
jp 0xffbd
ld a,(hl)
jr c,0x3f4a
cp 0xfe
cp 0x10
jr c,0x3f18
djnz 0x3f52
ld a,h
cp 0x7c
jr c,0x3f2f
nop
ld l,h
cp 0xfe
cp 0x7c
jr c,0x3f37
nop
djnz 0x3f62
ld a,h
cp 0xfe
djnz 0x3f67
nop
nop
inc a
ld h,(hl)
jp 0x66c3
inc a
nop
nop
inc a
ld a,(hl)
rst 0x38
rst 0x38
ld a,(hl)
inc a
nop
nop
ld a,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld h,(hl)
ld a,(hl)
nop
nop
ld a,(hl)
ld a,(hl)
ld a,(hl)
ld a,(hl)
ld a,(hl)
ld a,(hl)
nop
rrca
rlca
dec c
ld a,b
call z,0xcccc
ld a,b
inc a
ld h,(hl)
ld h,(hl)
ld h,(hl)
inc a
jr 0x3fdd
jr 0x3f6d
inc c
inc c
inc c
inc c
inc a
ld a,h
jr c,0x3f81
inc e
ld e,0x1b
jr 0x3fe6
ret m
ld (hl),b
sbc a,c
ld e,d
inc h
jp 0x24c3
ld e,d
sbc a,c
djnz 0x3fb2
jr c,0x3fb4
jr c,0x3fb6
ld a,h
sub 0x18
inc a
ld a,(hl)
rst 0x38
jr 0x3f9e
jr 0x3fa0
jr 0x3fa2
jr 0x3fa4
rst 0x38
ld a,(hl)
inc a
jr 0x3fa1
jr nc,0x4003
rst 0x38
rst 0x38
ld (hl),b
jr nc,0x3fa8
ex af,af'
inc c
ld c,0xff
rst 0x38
ld c,0x0c
ex af,af'
nop
nop
jr 0x3fe0
ld a,(hl)
rst 0x38
rst 0x38
nop
nop
nop
rst 0x38
rst 0x38
ld a,(hl)
inc a
jr 0x3fb0
add a,b
ret po
ret m
cp 0xf8
ret po
add a,b
nop
ld (bc),a
ld c,0x3e
cp 0x3e
ld c,0x02
nop
jr c,0x3ffa
sub d
ld a,h
djnz 0x3fee
jr z,0x3ff0
jr c,0x4002
djnz 0x3fca
djnz 0x3ff6
ld b,h
add a,d
jr c,0x400a
ld (de),a
ld a,h
sub b
jr z,0x3ffb
ld (0x3838),hl
sub b
ld a,h
ld (de),a
jr z,0x4027
adc a,b
nop
inc a
jr 0x4020
inc a
inc a
jr 0x3fe8
inc a
rst 0x38
rst 0x38
jr 0x3ff9
jr 0x401f
jr 0x4009
inc a
ld a,(hl)
jr 0x400d
ld a,(hl)
inc a
jr 0x3ff9
inc h
ld h,(hl)
rst 0x38
ld h,(hl)
inc h
nop
nop
