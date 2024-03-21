# How to use:
## Compile the library:
- cd /com.docker.devenvironments.code/src/dynamic_shims
- cargo build --release
- The library has been compiled to: /com.docker.devenvironments.code/src/dynamic_shims/target/release/libdynamics_shims.so

## Run the project:
Once the library has been compiled:
- cd /com.docker.devenvironments.code/src/fuzz-controller
- cargo run
- cd /com.docker.devenvironments.code/src/ping_pong_sync/pong_service
- LD_pr
eload=/com.docker.devenvironments.code/src/dynamic_shims/target/release/libdynamic_shims.so cargo run 
- cd /com.docker.devenvironments.code/src/ping_pong_sync/ping_service
- LD_pr
eload=/com.docker.devenvironments.code/src/dynamic_shims/target/release/libdynamic_shims.so cargo run 

