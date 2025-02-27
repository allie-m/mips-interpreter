# conversion.asm program
# For CMPSC 64
#
# Don't forget to:
#   make all arguments to any function go in $a0 and/or $a1
#   make all returned values from functions go in $v0

.text
j main
conv:
	li $t0 0 # z = 0
	li $t1 8 # i = 8
	move $t2 $a0 # t2 <- y
	move $t3 $a1 # t3 <- y
	li $t4 2
	loop:
		beq $t1 $zero post_loop
		add $t0 $t0 $t3  # z += y
		sll $t2 $t2 3    # x *= 8
		sub $t0 $t0 $t2  # z -= x
		sra $t2 $t2 3    # x /= 8
		blt $t2 $t4 else # if x >= 2
			addi $t3 $t3 -1 # y -= 1
		else:
		addi $t2 $t2 1 # x += 1
		addi $t1 $t1 -1
		j loop
	post_loop:
	move $v0 $t0
    jr $ra

main:  # DO NOT MODIFY THE MAIN SECTION
    li $a0, 5
    li $a1, 7

    jal conv

    move $a0, $v0
    li $v0, 1
    syscall

exit:
	li $v0 10
	syscall
