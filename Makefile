HOME = /home/user/Projects/fuzzy_distri
DYNAMIC_SHIMS_DIR = $(HOME)/src/dynamic_shims
FUZZ_CONTROLLER_DIR = $(HOME)/src/fuzz-controller
PONG_SERVICE_DIR = $(HOME)/src/ping_pong_sync/pong_service
PING_SERVICE_DIR = $(HOME)/src/ping_pong_sync/ping_service
PTRACE_INTERRUPT = $(HOME)/src/ptrace_interrupt

EXPERIMENTS = $(HOME)/src/experiments
LIB_DIR = $(HOME)/target/release/libdynamic_shims.so


# Default target
all: build_shims build_fuzz_controller run_pong run_ping

# Compile the dynamic shims library
build_shims:
	cd $(DYNAMIC_SHIMS_DIR) && \
	cargo build --release

# Compile and run the fuzz-controller
run_fuzz_controller:
	cd $(FUZZ_CONTROLLER_DIR) && \
	cargo run

# Run the pong service with LD_PRELOAD
run_pong:
	cd $(PONG_SERVICE_DIR) && \
	LD_PRELOAD=$(LIB_DIR) cargo run

# Run the ping service with LD_PRELOAD
run_ping:
	cd $(PING_SERVICE_DIR) && \
	LD_PRELOAD=$(LIB_DIR) cargo run

# Run the ping service with LD_PRELOAD without shims
run_ping_without_shims:
	cd $(PING_SERVICE_DIR) && \
	cargo run

run_experiments_without_shims:
	cd $(EXPERIMENTS) && \
	cargo run

run_experiments_shims:
	cd $(EXPERIMENTS) && \
	LD_PRELOAD=$(LIB_DIR) cargo run

run_experiments_ptrace:
	cd $(EXPERIMENTS) && \
	cargo build && \
	/home/user/Projects/fuzzy_distri/target/debug/experiments & \
	app_pid=$$!; \
	cd $(PTRACE_INTERRUPT) && \
	cargo run -- $$app_pid

.PHONY: build_shims build_fuzz_controller run_pong run_ping