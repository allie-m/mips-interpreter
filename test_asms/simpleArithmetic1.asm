.text
main:

# TODO: implement the arithmetic function: a + b - c, where a = 225, b = -22, c = -311.
#       Variables a, b, c, MUST be stored in registers $t0, $t1, $t2, respectively.
#       Think about which register the result should be stored in.
# YOUR ADDED LINES GO HERE:

li $t0 255
li $t1 -22
li $t2 -311
add $a0 $t0 $t1
sub $a0 $a0 $t2

# DO NOT EDIT THE FOLLOWING INSTRUCTIONS!
# Print to std.output
        li $v0, 1
        syscall

# End program
        li $v0, 10
        syscall
