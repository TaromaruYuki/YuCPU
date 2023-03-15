# Functions

Functions in YuCPU's assembly is done in a specific way. This is comparable to other assembly's functions.

1. Push arguments onto the stack, starting with the n'th arg then down to the 1st.
2. Call the function, which will push the return addr onto the stack
3. In the function, push the RBP onto the stack to return to it later.
4. Move the RSP into RBP
5. Get arguments as needed with the RBP.

Example:

```
.main start

.text

@main:
	PSH 3                           ; Argument 3
	PSH 2                           ; Argument 2
	PSH 1                           ; Argument 1

	CALL add3

	POP                             ; Pop arguments
	POP
	POP

	HLT ; Halt CPU

@add3:
	PSH RBP
	MOV RBP, RSP

	SUB RBP, 6
	LD R1, RBP                      ; Argument 1

	SUB RBP, 2
	LD R2, RBP                      ; Argument 2

	SUB RBP, 2
	LD R3, RBP                      ; Argument 3

	ADD RBP, 10                     ; Reset Base Pointer

	...                             ; Code here

	POP RBP                         ; Restore Base Pointer

	RET                             ; Return to start
```

`add3` is the function. We give it 1, 2, and 3 as a parameter.

A equivalent to this in C would be:

```c
void add3(short a, short b, short c) {
    // Code here
}
```
