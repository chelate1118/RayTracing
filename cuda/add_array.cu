#include <stdio.h>
#include <cuda_runtime.h>

typedef unsigned usize;

__global__ void addArrayInt(const int* arr1, const int* arr2, int* sum, usize length) {
    usize i = blockDim.x * blockIdx.x + threadIdx.x;

    if (i < length)
        sum[i] = arr1[i] + arr2[i];
}

extern "C" {
    int* add_array_int(const int* arr1, const int* arr2, usize length) {
        size_t size = sizeof(float) * length;

        int* sum = (int*)malloc(size);

        int* copy1 = NULL;
        int* copy2 = NULL;
        int* copy_sum = NULL;

        auto err1 = cudaMalloc((void**)&copy1, size);
        auto err2 = cudaMalloc((void**)&copy2, size);
        auto err3 = cudaMalloc((void**)&copy_sum, size);

        if (err1 != cudaSuccess || err2 != cudaSuccess || err3 != cudaSuccess) {
            fprintf(stderr, "Failed to allocate device array");
            exit(EXIT_FAILURE);
        }

        err1 = cudaMemcpy(copy1, arr1, size, cudaMemcpyHostToDevice);
        err2 = cudaMemcpy(copy2, arr2, size, cudaMemcpyHostToDevice);

        if (err1 != cudaSuccess || err2 != cudaSuccess) {
            fprintf(stderr, "Failed to copy host to device");
            exit(EXIT_FAILURE);
        }

        int threadsPerBlock = 1024;
        int blocksPerGrid = (length - 1) / threadsPerBlock + 1;

        addArrayInt<<<threadsPerBlock, blocksPerGrid>>>(copy1, copy2, copy_sum, length);

        err1 = cudaGetLastError();

        if (err1 != cudaSuccess) {
            fprintf(stderr, "Failed to launch kernel");
            exit(EXIT_FAILURE);
        }

        err1 = cudaMemcpy(sum, copy_sum, size, cudaMemcpyDeviceToHost);

        if (err1 != cudaSuccess) {
            fprintf(stderr, "Failed to copy device to host");
            exit(EXIT_FAILURE);
        }

        err1 = cudaFree(copy1);
        err2 = cudaFree(copy2);
        err3 = cudaFree(copy_sum);

        if (err1 != cudaSuccess || err2 != cudaSuccess || err3 != cudaSuccess) {
            fprintf(stderr, "Failed to free device");
            exit(EXIT_FAILURE);
        }

        cudaDeviceReset();

        return sum;
    }
}