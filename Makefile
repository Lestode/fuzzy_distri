## Define variables
HOME := /home/user/Projects/fuzzy_distri
INJECTED_SHIMS_DIR := $(HOME)/src/shared_lib_c
NEW_PTRACE := $(HOME)/src/new_ptrace
EXPERIMENT := $(HOME)/src/experiment
LIB_DIR := /home/user/Projects/fuzzy_distri/src/shared_lib_c/libsharedmem.so
PATH := /home/user/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin

export PATH 

# Default target
all: build_shims run_experiment_shims run_experiments_ptrace print_path

# Compile the dynamic shims library
build_shims:	
	cd $(INJECTED_SHIMS_DIR) && \
	gcc -fPIC -shared -o libsharedmem.so shared.c -lrt
# Run the experiment with shims
run_experiment_shims:
	cd $(EXPERIMENT) && \
	LD_PRELOAD=$(LIB_DIR) cargo run

# Run the experiments with ptrace
run_experiments_ptrace:
	cd $(experiment) && \
	cargo build && \
	( \
		sudo LD_PRELOAD=$(LIB_DIR) /home/user/Projects/fuzzy_distri/target/debug/experiment & \
		app_pid=$$! && \
		app_pid=$$(($$app_pid + 3)) && \
		cd $(new_ptrace) && \
		cargo build && \
		sudo /home/user/Projects/fuzzy_distri/target/debug/new_ptrace $$app_pid \
	)



# Phony targets
.PHONY: all build_shims run_experiment_shims run_experiments_ptrace print_path
