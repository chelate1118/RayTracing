#include <stdio.h>
#include <cuda_runtime.h>

typedef unsigned usize;

__global__ void addArrayInt(const int* arr1, const int* arr2, int* sum, usize length) {
    usize i = blockDim.x * blockIdx.x + threadIdx.x;

    if (i < length)
        sum[i] = arr1[i] + arr2[i];
}

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

extern "C" {
    int* add_array_int(const int* arr1, const int* arr2, usize length) {
        size_t size = sizeof(float) * length;

        int* copy1 = toCudaMem(arr1, size);
        int* copy2 = toCudaMem(arr2, size);
        int* copy_sum = allocCudaMem<int>(size);

        int blocksPerGrid = 1<<10;
        int threadsPerBlock = (length - 1) / blocksPerGrid + 1;

        addArrayInt<<<threadsPerBlock, blocksPerGrid>>>(copy1, copy2, copy_sum, length);

        safeCall(
            cudaGetLastError(),
            "Failed to launch kernel: addArrayInt"
        );

        int* sum = fromCudaMem(copy_sum, size);

        safeCall(cudaFree(copy1), "Failed to free device");
        safeCall(cudaFree(copy2), "Failed to free device");
        safeCall(cudaFree(copy_sum), "Failed to free device");

        cudaDeviceReset();

        return sum;
    }
}