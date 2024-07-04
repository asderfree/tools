# Rust Makefile

# 定义编译器和编译选项
RUSTC = rustc
RUSTFLAGS = -O

# 定义源代码和输出目录
SRC_DIR = src
OUT_DIR = target/release

# 源文件列表和可执行文件名称
SRCS = $(wildcard $(SRC_DIR)/*.rs)
BIN_NAME = dlimiter
ARGS = ./test/f1 ./test/f2

.PHONY: all clean run

# 默认构建规则
all: $(OUT_DIR)/$(BIN_NAME)

# 编译可执行文件
$(OUT_DIR)/$(BIN_NAME): $(SRCS)
	@mkdir -p $(OUT_DIR)
	$(RUSTC) $(RUSTFLAGS) -o $@ $^

# 运行可执行文件
run: $(OUT_DIR)/$(BIN_NAME)
	RUST_BACKTRACE=full ./$(OUT_DIR)/$(BIN_NAME) $(ARGS)

# 清理构建结果
clean:
	@rm -rf $(OUT_DIR)
