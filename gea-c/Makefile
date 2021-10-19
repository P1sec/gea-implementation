# the compiler: gcc for C program
CC = gcc
# clang can also be used for compiling the project, as an alternative compiler
#CC = clang

# compiler flags:
#  -std  determines the language standard, c99 required
#  -s    strips all debugging symbols
#  -g    adds debugging information to the executable file
#  -O2   build optimized executable code
#  -Wall turns on most, but not all, compiler warnings
#  -Wno-unused-function add warning for unused functions
#  -fPIC position-independent generated code, required for libraries
#  -static build a static executable (warning, this requires specific OS support)
CFLAGS = -std=c99 -O2 -Wall -Wno-unused-function -fPIC

# linker flags: actually none
LFLAGS = 

all: gea12.c gea12_test.c
	$(CC) $(CFLAGS) -o test gea12.c gea12_test.c $(LFLAGS)

lib: gea12.c
	$(CC) $(CFLAGS) -shared -o gea12.so gea12.c $(LFLAGS)

clean:
	rm -f test gea12.so

