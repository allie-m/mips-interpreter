# swap_case.asm program
# For CMPSC 64
#
# Data Area
.data
    buffer: .space 100
    input_prompt:   .asciiz "Enter string:\n"
    output_prompt:   .asciiz "Output:\n"
    convention: .asciiz "Convention Check\n"
    newline:    .asciiz "\n"

.text

#
# DO NOT MODIFY THE MAIN PROGRAM 
#       OR ANY OF THE CODE BELOW, WITH 1 EXCEPTION!!!
# YOU SHOULD ONLY MODIFY THE SwapCase FUNCTION 
#       AT THE BOTTOM OF THIS CODE
#
main:
    la $a0, input_prompt    # prompt user for string input
    li $v0, 4
    syscall

    li $v0, 8       # take in input
    la $a0, buffer
    li $a1, 100
    syscall
    move $s0, $a0   # save string to s0

    ori $s1, $0, 0
    ori $s2, $0, 0
    ori $s3, $0, 0
    ori $s4, $0, 0
    ori $s5, $0, 0
    ori $s6, $0, 0
    ori $s7, $0, 0

    move $a0, $s0
    jal SwapCase

    add $s1, $s1, $s2
    add $s1, $s1, $s3
    add $s1, $s1, $s4
    add $s1, $s1, $s5
    add $s1, $s1, $s6
    add $s1, $s1, $s7
    add $s0, $s0, $s1

    la $a0, output_prompt    # give Output prompt
    li $v0, 4
    syscall

    move $a0, $s0
    jal DispString

    j Exit

DispString:
    addi $a0, $a0, 0
    li $v0, 4
    syscall
    jr $ra

Test:
	jr $ra

ConventionCheck:
    addi    $t0, $0, -1
    addi    $t1, $0, -1
    addi    $t2, $0, -1
    addi    $t3, $0, -1
    addi    $t4, $0, -1
    addi    $t5, $0, -1
    addi    $t6, $0, -1
    addi    $t7, $0, -1
    ori     $v0, $0, 4
    la      $a0, convention
    syscall
    addi    $v0, $zero, -1
    addi    $v1, $zero, -1
    addi    $a0, $zero, -1
    addi    $a1, $zero, -1
    addi    $a2, $zero, -1
    addi    $a3, $zero, -1
    addi    $k0, $zero, -1
    addi    $k1, $zero, -1
    jr      $ra

Exit:
    ori     $v0, $0, 10
    syscall

# COPYFROMHERE - DO NOT REMOVE THIS LINE

# YOU CAN ONLY MODIFY THIS FILE FROM THIS POINT ONWARDS:
SwapCase:
    #TODO: write your code here, $a0 stores the address of the string

	# save s0, s3, and s4 to the stack
	addi $sp $sp -16
	sw $s0 0($sp)
	sw $s3 4($sp)
	sw $s4 8($sp)
	sw $s5 12($sp)

	move $s4 $a0 # str_addr+i
	loop:
		# fetch current character to s3
		lb $s3 0($s4)

		# stop if we encounter \0
		beq $s3 $zero finish

		# increment str_addr+i by 1 byte
		addi $s4 $s4 1

		# is s3 a letter or not
		sltiu $s0 $s3 'A'
		nor $s0 $s0 $s0
		sltiu $t1 $s3 '{' # { comes directly after z
		and $s0 $s0 $t1
		beq $s0 $zero loop

		# print the old char
		move $a0 $s3
		li $v0 11
		syscall

       	li $a0 '\n'
       	li $v0 11
       	syscall

		# print the swapcased char
		li $t0 'a'
		blt $s3 $t0 upper
			# lower
			addi $a0 $s3 -32
			j post
		upper:
			addi $a0 $s3 32
		post:
		li $v0 11
		syscall

       	li $a0 '\n'
       	li $v0 11
       	syscall

		move $s5 $ra
		jal ConventionCheck
		move $ra $s5

		j loop
	finish:

	# pop s0/s3/s4 off the stack
	lw $s0 0($sp)
	lw $s3 4($sp)
	lw $s4 8($sp)
	lw $s5 12($sp)
	addi $sp $sp 16

    # Do not remove the "jr $ra" line below!!!
    # It should be the last line in your function code!
    jr $ra
