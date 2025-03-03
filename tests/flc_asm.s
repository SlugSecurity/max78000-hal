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
	.file	"flc_asm.c1ff00fd3aa9b52e-cgu.0"
	.section	.analogsucks,"ax",%progbits
	.globl	flc_read32_primitive
	.p2align	2
	.type	flc_read32_primitive,%function
	.code	16
	.thumb_func
flc_read32_primitive:
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
	lsls	r1, r0, #30
	ittt	eq
	moveq	r1, r0
	bfceq	r1, #0, #19
	cmpeq.w	r1, #268435456
	bne	.LBB0_2
	ldr	r1, .LCPI0_0
	add	r1, r0
	cmp.w	r1, #524288
	itt	lo
	ldrlo	r0, [r0]
	poplo	{r7, pc}
.LBB0_2:
	bl	_ZN7flc_asm15FlashController17set_clock_divisor19panic_cold_explicit17hf124d6e3d1e3a7c6E
	.p2align	2
.LCPI0_0:
	.long	4026531843
.Lfunc_end0:
	.size	flc_read32_primitive, .Lfunc_end0-flc_read32_primitive
	.cfi_endproc
	.cantunwind
	.fnend

	.globl	flc_write128_primitive
	.p2align	2
	.type	flc_write128_primitive,%function
	.code	16
	.thumb_func
flc_write128_primitive:
.Lfunc_begin1:
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
	.pad	#8
	sub	sp, #8
	mov	r3, r0
	bfc	r3, #0, #19
	cmp.w	r3, #268435456
	bne	.LBB1_16
	ldr	r3, .LCPI1_0
	add	r3, r0
	cmp.w	r3, #524288
	bhs	.LBB1_16
	lsls	r3, r0, #28
	bne	.LBB1_16
	ldr	r4, .LCPI1_1
.LBB1_4:
	ldr	r3, [r4, #8]
	lsls	r3, r3, #7
	bmi	.LBB1_4
	ldr	r3, .LCPI1_2
	ldr	r6, .LCPI1_4
	ldr	r5, [r3]
	bic	r5, r5, #1
	str	r5, [r3]
	ldr	r5, [r4, #36]
	bic	r5, r5, #2
	str	r5, [r4, #36]
	ldr	r5, .LCPI1_3
	udiv	r5, r2, r5
	muls	r6, r5, r6
	cmn	r6, r2
	bne	.LBB1_16
	ldr	r2, [r4, #4]
	bfi	r2, r5, #0, #8
	str	r2, [r4, #4]
	ldr	r2, [r4, #8]
	movs	r5, #2
	bfi	r2, r5, #28, #4
	str	r2, [r4, #8]
	ldr	r2, [r4]
	str	r0, [r4]
	ldr	r0, [r4, #48]
	ldr	r0, [r1]
	str	r0, [r4, #48]
	ldr	r0, [r4, #52]
	ldr	r0, [r1, #4]
	str	r0, [r4, #52]
	ldr	r0, [r4, #56]
	ldr	r0, [r1, #8]
	str	r0, [r4, #56]
	ldr	r0, [r4, #60]
	ldr	r0, [r1, #12]
	str	r0, [r4, #60]
	ldr	r0, [r4, #8]
	orr	r0, r0, #1
	str	r0, [r4, #8]
.LBB1_7:
	ldr	r0, [r4, #8]
	lsls	r0, r0, #31
	bne	.LBB1_7
.LBB1_8:
	ldr	r0, [r4, #8]
	lsls	r0, r0, #7
	bmi	.LBB1_8
	ldr	r0, [r4, #8]
	movs	r1, #3
	bfi	r0, r1, #28, #4
	str	r0, [r4, #8]
	mov.w	r0, #1073741824
	ldr	r1, [r0]
	orr	r1, r1, #64
	str	r1, [r0]
.LBB1_10:
	ldr	r1, [r0]
	lsls	r1, r1, #25
	bmi	.LBB1_10
	mov.w	r0, #268435456
	ldr	r0, [r0]
	str	r0, [sp]
	mov	r0, sp
	@APP
	@NO_APP
	ldr	r0, .LCPI1_5
	ldr	r0, [r0]
	str	r0, [sp, #4]
	add	r0, sp, #4
	@APP
	@NO_APP
	ldr	r0, [r3]
	bic	r0, r0, #1
	str	r0, [r3]
	ldr.w	r0, [r3, #1536]
	movs	r0, #1
	str.w	r0, [r3, #1536]
.LBB1_12:
	ldr	r0, [r3]
	lsls	r0, r0, #15
	bpl	.LBB1_12
	ldr	r0, [r3]
	orr	r0, r0, #1
	str	r0, [r3]
.LBB1_14:
	ldr	r0, [r3]
	lsls	r0, r0, #15
	bpl	.LBB1_14
	add	sp, #8
	ldr	r11, [sp], #4
	pop	{r4, r5, r6, r7, pc}
.LBB1_16:
	bl	_ZN7flc_asm15FlashController17set_clock_divisor19panic_cold_explicit17hf124d6e3d1e3a7c6E
	.p2align	2
.LCPI1_0:
	.long	4026531855
.LCPI1_1:
	.long	1073909760
.LCPI1_2:
	.long	1073914112
.LCPI1_3:
	.long	1000000
.LCPI1_4:
	.long	4293967296
.LCPI1_5:
	.long	268443648
.Lfunc_end1:
	.size	flc_write128_primitive, .Lfunc_end1-flc_write128_primitive
	.cfi_endproc
	.cantunwind
	.fnend

	.globl	flc_page_erase_primitive
	.p2align	2
	.type	flc_page_erase_primitive,%function
	.code	16
	.thumb_func
flc_page_erase_primitive:
.Lfunc_begin2:
	.fnstart
	.cfi_startproc
	.save	{r4, r5, r7, lr}
	.pad	#8
	push	{r2, r3, r4, r5, r7, lr}
	.cfi_def_cfa_offset 24
	.cfi_offset lr, -4
	.cfi_offset r7, -8
	.cfi_offset r5, -12
	.cfi_offset r4, -16
	.setfp	r7, sp, #16
	add	r7, sp, #16
	.cfi_def_cfa r7, 8
	mov	r2, r0
	bfc	r2, #0, #19
	cmp.w	r2, #268435456
	bne	.LBB2_13
	ldr	r3, .LCPI2_0
.LBB2_2:
	ldr	r2, [r3, #8]
	lsls	r2, r2, #7
	bmi	.LBB2_2
	ldr	r2, .LCPI2_1
	ldr	r5, .LCPI2_3
	ldr	r4, [r2]
	bic	r4, r4, #1
	str	r4, [r2]
	ldr	r4, [r3, #36]
	bic	r4, r4, #2
	str	r4, [r3, #36]
	ldr	r4, .LCPI2_2
	udiv	r4, r1, r4
	muls	r5, r4, r5
	cmn	r5, r1
	bne	.LBB2_13
	ldr	r1, [r3, #4]
	bfi	r1, r4, #0, #8
	str	r1, [r3, #4]
	ldr	r1, [r3, #8]
	movs	r4, #2
	bfi	r1, r4, #28, #4
	str	r1, [r3, #8]
	ldr	r1, [r3]
	str	r0, [r3]
	movs	r1, #85
	ldr	r0, [r3, #8]
	bfi	r0, r1, #8, #8
	str	r0, [r3, #8]
	ldr	r0, [r3, #8]
	orr	r0, r0, #4
	str	r0, [r3, #8]
.LBB2_5:
	ldr	r0, [r3, #8]
	lsls	r0, r0, #7
	bmi	.LBB2_5
	ldr	r0, [r3, #8]
	movs	r1, #3
	bfi	r0, r1, #28, #4
	str	r0, [r3, #8]
	mov.w	r0, #1073741824
	ldr	r1, [r0]
	orr	r1, r1, #64
	str	r1, [r0]
.LBB2_7:
	ldr	r1, [r0]
	lsls	r1, r1, #25
	bmi	.LBB2_7
	mov.w	r0, #268435456
	ldr	r0, [r0]
	str	r0, [sp]
	mov	r0, sp
	@APP
	@NO_APP
	ldr	r0, .LCPI2_4
	ldr	r0, [r0]
	str	r0, [sp, #4]
	add	r0, sp, #4
	@APP
	@NO_APP
	ldr	r0, [r2]
	bic	r0, r0, #1
	str	r0, [r2]
	ldr.w	r0, [r2, #1536]
	movs	r0, #1
	str.w	r0, [r2, #1536]
.LBB2_9:
	ldr	r0, [r2]
	lsls	r0, r0, #15
	bpl	.LBB2_9
	ldr	r0, [r2]
	orr	r0, r0, #1
	str	r0, [r2]
.LBB2_11:
	ldr	r0, [r2]
	lsls	r0, r0, #15
	bpl	.LBB2_11
	pop	{r2, r3, r4, r5, r7, pc}
.LBB2_13:
	bl	_ZN7flc_asm15FlashController17set_clock_divisor19panic_cold_explicit17hf124d6e3d1e3a7c6E
	.p2align	2
.LCPI2_0:
	.long	1073909760
.LCPI2_1:
	.long	1073914112
.LCPI2_2:
	.long	1000000
.LCPI2_3:
	.long	4293967296
.LCPI2_4:
	.long	268443648
.Lfunc_end2:
	.size	flc_page_erase_primitive, .Lfunc_end2-flc_page_erase_primitive
	.cfi_endproc
	.cantunwind
	.fnend

	.section	.text.unlikely._ZN7flc_asm15FlashController17set_clock_divisor19panic_cold_explicit17hf124d6e3d1e3a7c6E,"ax",%progbits
	.p2align	1
	.type	_ZN7flc_asm15FlashController17set_clock_divisor19panic_cold_explicit17hf124d6e3d1e3a7c6E,%function
	.code	16
	.thumb_func
_ZN7flc_asm15FlashController17set_clock_divisor19panic_cold_explicit17hf124d6e3d1e3a7c6E:
.Lfunc_begin3:
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
	bl	_ZN4core9panicking14panic_explicit17hb6ee90eefcf59f2cE
.Lfunc_end3:
	.size	_ZN7flc_asm15FlashController17set_clock_divisor19panic_cold_explicit17hf124d6e3d1e3a7c6E, .Lfunc_end3-_ZN7flc_asm15FlashController17set_clock_divisor19panic_cold_explicit17hf124d6e3d1e3a7c6E
	.cfi_endproc
	.cantunwind
	.fnend

	.section	.text.unlikely._ZN4core9panicking9panic_fmt17haebed1323d60a452E,"ax",%progbits
	.p2align	1
	.type	_ZN4core9panicking9panic_fmt17haebed1323d60a452E,%function
	.code	16
	.thumb_func
_ZN4core9panicking9panic_fmt17haebed1323d60a452E:
.Lfunc_begin4:
	.file	1 "/rustc/bef3c3b01f690de16738b1c9f36470fbfc6ac623" "library/core/src/panicking.rs"
	.loc	1 55 0
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
.Ltmp0:
	.loc	1 75 14 prologue_end
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
.Ltmp2:
.Lfunc_end4:
	.size	_ZN4core9panicking9panic_fmt17haebed1323d60a452E, .Lfunc_end4-_ZN4core9panicking9panic_fmt17haebed1323d60a452E
	.cfi_endproc
	.cantunwind
	.fnend

	.section	.text.unlikely._ZN4core9panicking14panic_explicit17hb6ee90eefcf59f2cE,"ax",%progbits
	.p2align	1
	.type	_ZN4core9panicking14panic_explicit17hb6ee90eefcf59f2cE,%function
	.code	16
	.thumb_func
_ZN4core9panicking14panic_explicit17hb6ee90eefcf59f2cE:
.Lfunc_begin5:
	.loc	1 233 0
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
.Ltmp3:
	.loc	1 261 5 prologue_end
	bl	_ZN4core9panicking9panic_fmt17haebed1323d60a452E
.Ltmp4:
.Lfunc_end5:
	.size	_ZN4core9panicking14panic_explicit17hb6ee90eefcf59f2cE, .Lfunc_end5-_ZN4core9panicking14panic_explicit17hb6ee90eefcf59f2cE
	.cfi_endproc
	.cantunwind
	.fnend

	.section	.debug_info,"",%progbits
.Lcu_begin0:
	.long	.Ldebug_info_end0-.Ldebug_info_start0
.Ldebug_info_start0:
	.short	4
	.long	.debug_abbrev
	.byte	4
	.byte	1
	.long	.Linfo_string0
	.short	28
	.long	.Linfo_string1
	.long	.Lline_table_start0
	.long	.Linfo_string2
	.long	0
	.long	.Ldebug_ranges0
	.byte	2
	.long	.Linfo_string3
	.byte	2
	.long	.Linfo_string4
	.byte	3
	.long	.Lfunc_begin4
	.long	.Lfunc_end4-.Lfunc_begin4
	.byte	1
	.byte	87
	.long	.Linfo_string7
	.long	.Linfo_string8
	.byte	1
	.byte	55


	.byte	4
	.long	.Linfo_string5
	.long	.Linfo_string6
	.byte	1
	.short	260

	.byte	1
	.byte	5
	.long	.Lfunc_begin5
	.long	.Lfunc_end5-.Lfunc_begin5
	.byte	1
	.byte	87
	.long	.Linfo_string9
	.long	.Linfo_string10
	.byte	1
	.byte	233


	.byte	6
	.long	69
	.long	.Ltmp3
	.long	.Ltmp4-.Ltmp3
	.byte	1
	.byte	234
	.byte	5
	.byte	0
	.byte	0
	.byte	0
	.byte	0
.Ldebug_info_end0:
	.section	.text.unlikely._ZN4core9panicking9panic_fmt17haebed1323d60a452E,"ax",%progbits
.Lsec_end0:
	.section	.text.unlikely._ZN4core9panicking14panic_explicit17hb6ee90eefcf59f2cE,"ax",%progbits
.Lsec_end1:
	.section	.debug_aranges,"",%progbits
	.long	36
	.short	2
	.long	.Lcu_begin0
	.byte	4
	.byte	0
	.zero	4,255
	.long	.Lfunc_begin4
	.long	.Lsec_end0-.Lfunc_begin4
	.long	.Lfunc_begin5
	.long	.Lsec_end1-.Lfunc_begin5
	.long	0
	.long	0
	.section	.debug_ranges,"",%progbits
.Ldebug_ranges0:
	.long	.Lfunc_begin4
	.long	.Lfunc_end4
	.long	.Lfunc_begin5
	.long	.Lfunc_end5
	.long	0
	.long	0
	.section	.debug_str,"MS",%progbits,1
.Linfo_string0:
	.asciz	"clang LLVM (rustc version 1.86.0-nightly (bef3c3b01 2025-02-04))"
.Linfo_string1:
	.asciz	"library/core/src/lib.rs/@/core.42399d673965ec88-cgu.0"
.Linfo_string2:
	.asciz	"/rustc/bef3c3b01f690de16738b1c9f36470fbfc6ac623"
.Linfo_string3:
	.asciz	"core"
.Linfo_string4:
	.asciz	"panicking"
.Linfo_string5:
	.asciz	"_ZN4core9panicking13panic_display17hb27badd535428c70E"
.Linfo_string6:
	.asciz	"panic_display<&str>"
.Linfo_string7:
	.asciz	"_ZN4core9panicking9panic_fmt17haebed1323d60a452E"
.Linfo_string8:
	.asciz	"panic_fmt"
.Linfo_string9:
	.asciz	"_ZN4core9panicking14panic_explicit17hb6ee90eefcf59f2cE"
.Linfo_string10:
	.asciz	"panic_explicit"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.ident	"rustc version 1.86.0-nightly (bef3c3b01 2025-02-04)"
	.section	".note.GNU-stack","",%progbits
	.section	.debug_line,"",%progbits
.Lline_table_start0:
