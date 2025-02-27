# print_array.asm program
# For CMPSC 64
#
# Don't forget to:
#   make all arguments to any function go in $a0 and/or $a1
#   make all returned values from functions go in $v0

.data
	array: .word 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
	cout: .ascii "The contents of the array are:\n"

.text
j main
printArr:
	move $t0 $a0    # a[]
	move $t1 $a1    # i = al
	addi $t1 $t1 -1 # i -= 1
	loop:
		blt $t1 $zero post_loop
		# print statement
		move $t3 $t1     # a+i = i
		sll $t3 $t3 2    # a+i *= 4
		add $t3 $t3 $t0  # a+i += a
		lw $a0 0($t3)    # a0 = M[a+i]
		li $v0 1
		syscall
		li $a0 '\n'
		li $v0 11
		syscall
		addi $t1 $t1 -1 # i--
		j loop
	post_loop:
	jr $ra

main:  # DO NOT MODIFY THE MAIN SECTION
	li $v0, 4
	la $a0, cout
	syscall

	la $a0, array
	li $a1, 10

	jal printArr

exit:
	li $v0 10
	syscall
