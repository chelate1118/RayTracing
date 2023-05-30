__host__ void safeCall(cudaError_t err, const char* message) {
    if (err != cudaSuccess) {
        fprintf(stderr, "%s (%s)", message, cudaGetErrorString(err));
        exit(EXIT_FAILURE);
    }
}

template<typename T>
__host__ T* allocCudaMem(size_t size) {    
    T* ptr = NULL;

    safeCall(
        cudaMalloc((void**)&ptr, size),
        "Failed to allocate device memory."
    );

    return ptr;
}

template<typename T>
__host__ T* toCudaMem(const T* ptr, size_t size) {
    T* copy_ptr = allocCudaMem<T>(size);

    safeCall(
        cudaMemcpy(copy_ptr, ptr, size, cudaMemcpyHostToDevice),
        "Failed to copy memory from host to device."
    );

    return copy_ptr;
}

template<typename T>
__host__ T* fromCudaMem(const T* ptr, size_t size) {
    T* host_ptr = (T*)malloc(size);

    safeCall(
        cudaMemcpy(host_ptr, ptr, size, cudaMemcpyDeviceToHost),
        "Failed to copy memory from device to host."
    );

    return host_ptr;
}