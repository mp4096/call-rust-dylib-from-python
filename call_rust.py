import numpy as np
from cffi import FFI
import ctypes


ffi = FFI()
ffi.cdef(
    "double scalar_sum(const double a, const double b);"
    "double vec_sum(const double *arr, const size_t len);"
    "double vec_mean(const double *arr, const size_t len);"
    "double *vec_cumsum(const double *arr, const size_t len);"
    "void vec_cumsum_mut(const double *arr, double *out, const size_t len);"
    )
C = ffi.dlopen("./my-dylib/target/release/libmy_dylib.so")


def scalar_sum(a, b):
    return C.scalar_sum(a, b)


def vec_sum(v):
    v = np.array(v)
    v_c = ffi.cast("double *", v.ctypes.data)
    return C.vec_sum(v_c, len(v))


def vec_mean(v):
    v = np.array(v)
    v_c = ffi.cast("double *", v.ctypes.data)
    return C.vec_mean(v_c, len(v))


def vec_cumsum(v):
    v = np.array(v)
    v_c = ffi.cast("double *", v.ctypes.data)
    ptr = C.vec_cumsum(v_c, len(v))
    buf = ffi.buffer(ptr, size=len(v)*ffi.sizeof(ptr))
    return np.frombuffer(buf, dtype=np.float64)


def vec_cumsum_mut(v):
    v = np.array(v)
    res = np.empty(v.shape, dtype=np.float64)
    v_c = ffi.cast("double *", v.ctypes.data)
    res_c = ffi.cast("double *", res.ctypes.data)
    C.vec_cumsum_mut(v_c, res_c, len(v))
    return res


if __name__ == "__main__":
    print("1.0 + 3.2 =", scalar_sum(1.0, 3.2))
    print("sum([1.1, 1.9, 1.2]) =", vec_sum([1.1, 1.9, 1.2]))
    print("mean([1.0, 3.0, 2.0]) =", vec_mean([1.0, 3.0, 2.0]))
    print("cumsum([1.1, 1.9, 1.2]) =", vec_cumsum([1.1, 1.9, 1.2]))
    print("cumsum_mut([1.1, 1.9, 1.2]) =", vec_cumsum_mut([1.1, 1.9, 1.2]))

    # Do some random regression testing
    thr = np.sqrt(np.finfo(np.float64).eps)
    comparisons = [
        (vec_sum, np.sum),
        (vec_mean, np.mean),
        (vec_cumsum, np.cumsum),
        (vec_cumsum_mut, np.cumsum),
        ]
    for _ in range(100):
        a = np.random.rand(100)
        for f_rust, f_numpy in comparisons:
            if np.linalg.norm(f_rust(a) - f_numpy(a)) < thr:
                print(".", end="")
            else:
                print("x", end="")
    print("")
