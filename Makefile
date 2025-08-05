# Compiler and flags
CC = gcc
CFLAGS = -Wall -Wextra -Iinclude -fPIC
LDFLAGS = -shared

# Directories
SRC_DIR = src
INC_DIR = include
OBJ_DIR = build

# Output
TARGET = bin/libskirk.so

# Find all .c files in src/
SRCS = $(wildcard $(SRC_DIR)/*.c)
# Convert to .o paths in build/
OBJS = $(patsubst $(SRC_DIR)/%.c, $(OBJ_DIR)/%.o, $(SRCS))

# Default target
all: $(TARGET)

# Create shared library
$(TARGET): $(OBJS)
	$(CC) $(LDFLAGS) -o $@ $^

# Compile each .c to .o in build/
$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c | $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

# Ensure build/ exists
$(OBJ_DIR):
	mkdir -p $(OBJ_DIR)

# Clean build artifacts
clean:
	rm -rf $(OBJ_DIR) $(TARGET)

.PHONY: all clean
