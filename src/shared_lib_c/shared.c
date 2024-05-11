#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>    // For O_* constants
#include <sys/stat.h> // For mode constants
#include <sys/mman.h> // For shared memory
#include <unistd.h>
#include <stdint.h>
#include <string.h>

__attribute__((constructor)) void shared_memory_constructor()
{
    const char *name = "/mysharedmem";
    int shm_fd;
    void *ptr;
    const char *message = "50";
    // Create the shared memory object
    shm_fd = shm_open(name, O_CREAT | O_RDWR, 0666);
    if (shm_fd == -1)
    {
        perror("shm_open");
        exit(1);
    }

    // Configure the size of the shared memory object
    ftruncate(shm_fd, 4096); // size of the memory object

    // Memory map the shared memory object
    ptr = mmap(0, 4096, PROT_WRITE, MAP_SHARED, shm_fd, 0);
    if (ptr == MAP_FAILED)
    {
        perror("mmap");
        exit(1);
    }

    memset(ptr, 0, 4096);

    uintptr_t func_addr = (uintptr_t)getpid;
    memcpy(ptr, &func_addr, sizeof(func_addr));
    sprintf(ptr, "%s", message);
}

pid_t my_getpid()
{
    return 0;
}
