## Define variables
HOME := /home/user/Projects/fuzzy_distri
INJECTED_SHIMS_DIR := $(HOME)/src/injected_shims
NEW_PTRACE := $(HOME)/src/new_ptrace
EXPERIMENT := $(HOME)/src/experiment
LIB_DIR := /home/user/Projects/fuzzy_distri/target/release/libinjected_shims.so
PATH := /home/user/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin

export PATH 

# Default target
all: build_shims run_experiment_shims run_experiments_ptrace print_path

# Compile the dynamic shims library
build_shims:	
	cd $(INJECTED_SHIMS_DIR) && \
	cargo build --release

# Run the experiment with shims
run_experiment_shims:
	cd $(EXPERIMENT) && \
	LD_PRELOAD=$(LIB_DIR) cargo run

# Run the experiments with ptrace
run_experiments_ptrace:
	cd $(EXPERIMENT) && \
	cargo build && \
	LD_PRELOAD=$(LIB_DIR) /home/user/Projects/fuzzy_distri/target/debug/experiment & \
	app_pid=$$!; \
	cd $(NEW_PTRACE) && \
	cargo run -- $$app_pid


run_experiments_ptrace_debug:
	cd $(EXPERIMENT) && \
	cargo build && \
	LD_PRELOAD=$(LIB_DIR) /home/user/Projects/fuzzy_distri/target/debug/experiment & \
	app_pid=$$!; \
	cd $(NEW_PTRACE) && \
	RUST_BACKTRACE=1 cargo run -- $$app_pid


# Phony targets
.PHONY: all build_shims run_experiment_shims run_experiments_ptrace print_path
