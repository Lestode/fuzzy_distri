HOME = /home/user/Projects/fuzzy_distri
INJECTED_SHIMS_DIR = $(HOME)/src/injected_shims
NEW_PTRACE = $(HOME)/src/new_ptrace

EXPERIMENT = $(HOME)/src/experiment
LIB_DIR = $(HOME)target/release/libinjected_shims.so


# Default target
all: build_shims build_fuzz_controller run_pong run_ping

# Compile the dynamic shims library
build_shims:
	cd $(INJECTED_SHIMS_DIR) && \
	cargo build --release

run_experiment_shims:
	cd $(EXPERIMENT) && \
	LD_PRELOAD=$(LIB_DIR) cargo run

run_experiments_ptrace:
	cd $(EXPERIMENT) && \
	cargo build && \
	/home/user/Projects/fuzzy_distri/target/debug/experiment & \
	app_pid=$$!; \
	cd $(NEW_PTRACE) && \
	cargo run -- $$app_pid

.PHONY: build_shims build_fuzz_controller run_pong run_ping