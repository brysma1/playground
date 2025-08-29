CC:= clang
OUT:= main
OPT:= -O3

all: $(OUT)

run:$(OUT)
	./$(OUT)

$(OUT): main.c
	$(CC) $(OPT) -std=c89 -Wall -Wextra -Werror -pedantic -x c main.c -o $(OUT)

clean:
	@rm -fr $(OUT)

PHONY: clean
