	.text
	.syntax unified
	.eabi_attribute	67, "2.09"
	.eabi_attribute	6, 13
	.eabi_attribute	7, 77
	.eabi_attribute	8, 0
	.eabi_attribute	9, 2
	.fpu	fpv4-sp-d16
	.eabi_attribute	27, 1
	.eabi_attribute	36, 1
	.eabi_attribute	34, 1
	.eabi_attribute	17, 1
	.eabi_attribute	20, 1
	.eabi_attribute	21, 1
	.eabi_attribute	23, 3
	.eabi_attribute	24, 1
	.eabi_attribute	25, 1
	.eabi_attribute	28, 1
	.eabi_attribute	38, 1
	.eabi_attribute	14, 0
	.file	"flc_asm.d8eec276ff8a6799-cgu.0"
	.section	.analogsucks,"ax",%progbits
	.p2align	2
	.type	_ZN7flc_asm15FlashController17set_clock_divisor17hc5cf0d05434ae31dE,%function
	.code	16
	.thumb_func
_ZN7flc_asm15FlashController17set_clock_divisor17hc5cf0d05434ae31dE:
.Lfunc_begin0:
	.fnstart
	.cfi_sections .debug_frame
	.cfi_startproc
	.save	{r7, lr}
	push	{r7, lr}
	.cfi_def_cfa_offset 8
	.cfi_offset lr, -4
	.cfi_offset r7, -8
	.setfp	r7, sp
	mov	r7, sp
	.cfi_def_cfa_register r7
	ldr	r1, .LCPI0_0
	ldr	r2, .LCPI0_1
	udiv	r1, r0, r1
	muls	r2, r1, r2
	cmn	r2, r0
	itttt	eq
	ldreq	r0, .LCPI0_2
	ldreq	r2, [r0]
	bfieq	r2, r1, #0, #8
	streq	r2, [r0]
	it	eq
	popeq	{r7, pc}
.LBB0_1:
	@APP
.Ltmp0:
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	b	.Ltmp0
	@NO_APP
	.inst.n	0xdefe
	.p2align	2
.LCPI0_0:
	.long	1000000
.LCPI0_1:
	.long	4293967296
.LCPI0_2:
	.long	1073909764
.Lfunc_end0:
	.size	_ZN7flc_asm15FlashController17set_clock_divisor17hc5cf0d05434ae31dE, .Lfunc_end0-_ZN7flc_asm15FlashController17set_clock_divisor17hc5cf0d05434ae31dE
	.cfi_endproc
	.cantunwind
	.fnend

	.p2align	2
	.type	_ZN7flc_asm15FlashController16wait_until_ready17h3156387ccf27a694E,%function
	.code	16
	.thumb_func
_ZN7flc_asm15FlashController16wait_until_ready17h3156387ccf27a694E:
.Lfunc_begin1:
	.fnstart
	.cfi_startproc
	ldr	r0, .LCPI1_0
.LBB1_1:
	ldr	r1, [r0]
	lsls	r1, r1, #7
	bmi	.LBB1_1
	bx	lr
	.p2align	2
.LCPI1_0:
	.long	1073909768
.Lfunc_end1:
	.size	_ZN7flc_asm15FlashController16wait_until_ready17h3156387ccf27a694E, .Lfunc_end1-_ZN7flc_asm15FlashController16wait_until_ready17h3156387ccf27a694E
	.cfi_endproc
	.cantunwind
	.fnend

	.p2align	2
	.type	_ZN7flc_asm15FlashController9flush_icc17hdfc1ef73516cdd37E,%function
	.code	16
	.thumb_func
_ZN7flc_asm15FlashController9flush_icc17hdfc1ef73516cdd37E:
.Lfunc_begin2:
	.fnstart
	.cfi_startproc
	.pad	#8
	sub	sp, #8
	.cfi_def_cfa_offset 8
	mov.w	r0, #1073741824
	ldr	r1, [r0]
	orr	r1, r1, #64
	str	r1, [r0]
.LBB2_1:
	ldr	r1, [r0]
	lsls	r1, r1, #25
	bmi	.LBB2_1
	mov.w	r0, #268435456
	ldr	r0, [r0]
	str	r0, [sp]
	mov	r0, sp
	@APP
	@NO_APP
	ldr	r0, .LCPI2_0
	ldr	r0, [r0]
	str	r0, [sp, #4]
	add	r0, sp, #4
	@APP
	@NO_APP
	add	sp, #8
	bx	lr
	.p2align	2
.LCPI2_0:
	.long	268443648
.Lfunc_end2:
	.size	_ZN7flc_asm15FlashController9flush_icc17hdfc1ef73516cdd37E, .Lfunc_end2-_ZN7flc_asm15FlashController9flush_icc17hdfc1ef73516cdd37E
	.cfi_endproc
	.cantunwind
	.fnend

	.p2align	2
	.type	_ZN7flc_asm15FlashController11enable_icc017h626bac51309a41f0E,%function
	.code	16
	.thumb_func
_ZN7flc_asm15FlashController11enable_icc017h626bac51309a41f0E:
.Lfunc_begin3:
	.fnstart
	.cfi_startproc
	ldr	r0, .LCPI3_0
	ldr	r1, [r0]
	bic	r1, r1, #1
	str	r1, [r0]
	ldr.w	r1, [r0, #1536]
	movs	r1, #1
	str.w	r1, [r0, #1536]
.LBB3_1:
	ldr	r1, [r0]
	lsls	r1, r1, #15
	bpl	.LBB3_1
	ldr	r1, [r0]
	orr	r1, r1, #1
	str	r1, [r0]
.LBB3_3:
	ldr	r1, [r0]
	lsls	r1, r1, #15
	bpl	.LBB3_3
	bx	lr
	.p2align	2
.LCPI3_0:
	.long	1073914112
.Lfunc_end3:
	.size	_ZN7flc_asm15FlashController11enable_icc017h626bac51309a41f0E, .Lfunc_end3-_ZN7flc_asm15FlashController11enable_icc017h626bac51309a41f0E
	.cfi_endproc
	.cantunwind
	.fnend

	.globl	flc_read32_primitive
	.p2align	2
	.type	flc_read32_primitive,%function
	.code	16
	.thumb_func
flc_read32_primitive:
.Lfunc_begin4:
	.fnstart
	.cfi_startproc
	.save	{r7, lr}
	push	{r7, lr}
	.cfi_def_cfa_offset 8
	.cfi_offset lr, -4
	.cfi_offset r7, -8
	.setfp	r7, sp
	mov	r7, sp
	.cfi_def_cfa_register r7
	lsls	r1, r0, #30
	bne	.LBB4_4
	mov	r1, r0
	bfc	r1, #0, #19
	cmp.w	r1, #268435456
	bne	.LBB4_3
	ldr	r1, .LCPI4_0
	add	r1, r0
	cmp.w	r1, #524288
	itt	lo
	ldrlo	r0, [r0]
	poplo	{r7, pc}
.LBB4_3:
	@APP
.Ltmp1:
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	b	.Ltmp1
	@NO_APP
	.inst.n	0xdefe
.LBB4_4:
	@APP
.Ltmp2:
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	b	.Ltmp2
	@NO_APP
	.inst.n	0xdefe
	.p2align	2
.LCPI4_0:
	.long	4026531843
.Lfunc_end4:
	.size	flc_read32_primitive, .Lfunc_end4-flc_read32_primitive
	.cfi_endproc
	.cantunwind
	.fnend

	.globl	flc_write128_primitive
	.p2align	2
	.type	flc_write128_primitive,%function
	.code	16
	.thumb_func
flc_write128_primitive:
.Lfunc_begin5:
	.fnstart
	.cfi_startproc
	.save	{r4, r5, r6, r7, lr}
	push	{r4, r5, r6, r7, lr}
	.cfi_def_cfa_offset 20
	.cfi_offset lr, -4
	.cfi_offset r7, -8
	.cfi_offset r6, -12
	.cfi_offset r5, -16
	.cfi_offset r4, -20
	.setfp	r7, sp, #12
	add	r7, sp, #12
	.cfi_def_cfa r7, 8
	.save	{r8}
	str	r8, [sp, #-4]!
	.cfi_offset r8, -24
	mov	r5, r0
	bfc	r0, #0, #19
	cmp.w	r0, #268435456
	bne	.LBB5_6
	ldr	r0, .LCPI5_0
	add	r0, r5
	cmp.w	r0, #524288
	bhs	.LBB5_6
	lsls	r0, r5, #28
	bne	.LBB5_7
	mov	r8, r2
	mov	r4, r1
	bl	_ZN7flc_asm15FlashController16wait_until_ready17h3156387ccf27a694E
	ldr	r0, .LCPI5_1
	ldr	r6, .LCPI5_2
	ldr	r1, [r0]
	bic	r1, r1, #1
	str	r1, [r0]
	ldr	r0, [r6, #28]
	bic	r0, r0, #2
	str	r0, [r6, #28]
	mov	r0, r8
	bl	_ZN7flc_asm15FlashController17set_clock_divisor17hc5cf0d05434ae31dE
	ldr	r0, [r6]
	movs	r1, #2
	bfi	r0, r1, #28, #4
	str	r0, [r6]
	ldr	r0, [r6, #-8]
	str	r5, [r6, #-8]
	ldr	r0, [r6, #40]
	ldm	r4!, {r0, r1, r2, r3}
	str	r0, [r6, #40]
	ldr	r0, [r6, #44]
	str	r1, [r6, #44]
	ldr	r0, [r6, #48]
	str	r2, [r6, #48]
	ldr	r0, [r6, #52]
	str	r3, [r6, #52]
	ldr	r0, [r6]
	orr	r0, r0, #1
	str	r0, [r6]
.LBB5_4:
	ldr	r0, [r6]
	lsls	r0, r0, #31
	bne	.LBB5_4
	bl	_ZN7flc_asm15FlashController16wait_until_ready17h3156387ccf27a694E
	ldr	r0, [r6]
	movs	r1, #3
	bfi	r0, r1, #28, #4
	str	r0, [r6]
	bl	_ZN7flc_asm15FlashController9flush_icc17hdfc1ef73516cdd37E
	ldr	r8, [sp], #4
	pop.w	{r4, r5, r6, r7, lr}
	b	_ZN7flc_asm15FlashController11enable_icc017h626bac51309a41f0E
.LBB5_6:
	@APP
.Ltmp3:
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	b	.Ltmp3
	@NO_APP
	.inst.n	0xdefe
.LBB5_7:
	@APP
.Ltmp4:
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	b	.Ltmp4
	@NO_APP
	.inst.n	0xdefe
	.p2align	2
.LCPI5_0:
	.long	4026531855
.LCPI5_1:
	.long	1073914112
.LCPI5_2:
	.long	1073909768
.Lfunc_end5:
	.size	flc_write128_primitive, .Lfunc_end5-flc_write128_primitive
	.cfi_endproc
	.cantunwind
	.fnend

	.globl	flc_page_erase_primitive
	.p2align	2
	.type	flc_page_erase_primitive,%function
	.code	16
	.thumb_func
flc_page_erase_primitive:
.Lfunc_begin6:
	.fnstart
	.cfi_startproc
	.save	{r4, r5, r6, r7, lr}
	push	{r4, r5, r6, r7, lr}
	.cfi_def_cfa_offset 20
	.cfi_offset lr, -4
	.cfi_offset r7, -8
	.cfi_offset r6, -12
	.cfi_offset r5, -16
	.cfi_offset r4, -20
	.setfp	r7, sp, #12
	add	r7, sp, #12
	.cfi_def_cfa r7, 8
	.save	{r11}
	str	r11, [sp, #-4]!
	.cfi_offset r11, -24
	mov	r4, r0
	bfc	r0, #0, #19
	cmp.w	r0, #268435456
	bne	.LBB6_2
	mov	r5, r1
	bl	_ZN7flc_asm15FlashController16wait_until_ready17h3156387ccf27a694E
	ldr	r0, .LCPI6_0
	ldr	r6, .LCPI6_1
	ldr	r1, [r0]
	bic	r1, r1, #1
	str	r1, [r0]
	ldr	r0, [r6, #28]
	bic	r0, r0, #2
	str	r0, [r6, #28]
	mov	r0, r5
	bl	_ZN7flc_asm15FlashController17set_clock_divisor17hc5cf0d05434ae31dE
	ldr	r0, [r6]
	movs	r1, #2
	bfi	r0, r1, #28, #4
	str	r0, [r6]
	ldr	r0, [r6, #-8]
	movs	r1, #85
	str	r4, [r6, #-8]
	ldr	r0, [r6]
	bfi	r0, r1, #8, #8
	str	r0, [r6]
	ldr	r0, [r6]
	orr	r0, r0, #4
	str	r0, [r6]
	bl	_ZN7flc_asm15FlashController16wait_until_ready17h3156387ccf27a694E
	ldr	r0, [r6]
	movs	r1, #3
	bfi	r0, r1, #28, #4
	str	r0, [r6]
	bl	_ZN7flc_asm15FlashController9flush_icc17hdfc1ef73516cdd37E
	ldr	r11, [sp], #4
	pop.w	{r4, r5, r6, r7, lr}
	b	_ZN7flc_asm15FlashController11enable_icc017h626bac51309a41f0E
.LBB6_2:
	@APP
.Ltmp5:
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	b	.Ltmp5
	@NO_APP
	.inst.n	0xdefe
	.p2align	2
.LCPI6_0:
	.long	1073914112
.LCPI6_1:
	.long	1073909768
.Lfunc_end6:
	.size	flc_page_erase_primitive, .Lfunc_end6-flc_page_erase_primitive
	.cfi_endproc
	.cantunwind
	.fnend

	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.ident	"rustc version 1.84.1 (e71f9a9a9 2025-01-27)"
	.section	".note.GNU-stack","",%progbits
	.eabi_attribute	30, 4
