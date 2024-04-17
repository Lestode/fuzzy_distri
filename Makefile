DYNAMIC_SHIMS_DIR = /com.docker.devenvironments.code/src/dynamic_shims
FUZZ_CONTROLLER_DIR = /com.docker.devenvironments.code/src/fuzz-controller
PONG_SERVICE_DIR = /com.docker.devenvironments.code/src/ping_pong_sync/pong_service
PING_SERVICE_DIR = /com.docker.devenvironments.code/src/ping_pong_sync/ping_service
LIB_DIR = /com.docker.devenvironments.code/target/release/libdynamic_shims.so


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

.PHONY: build_shims build_fuzz_controller run_pong run_ping