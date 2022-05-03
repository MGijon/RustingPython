import random
import time
import numpy as np

# Decorator to measure time
def measure_time(method):
    def wrapper(*args, **kwargs):
        times = []
        for i in range(10):
            # Execute before
            time_start = time.time()
            # Execute the function
            result = method(*args, **kwargs)
            # Execute after
            time_end = time.time()
            # Print measure
            t = (time_end - time_start) * 1_000
            times.append(t)
            print(f"{method.__name__}: {t}")
        print(f"Mean: {np.mean(times)} STD: {np.std(times)}")
        return result

    return wrapper


# Generate some "big" data.
def generate_data(n: int):
    # This will not be usefull in our case due to the fact that
    # could be repeated elements
    return [random.randint(0, n) for _ in range(n)]


random.seed(100)
print(generate_data(n=1000))


@measure_time
def test_():
    time.sleep(2)
    print("Hello world")


test_()


"""
def timeit(method):
    def timed(*args, **kwargs):
        ts = time.time()
        result = method(*args, **kwargs)
        te = time.time()

        if 'log_time' in kwargs:
            name = kwargs.get('log_name', method.__name__.upper())
            kwargs['log_time'][name] = int((te - ts) * 1000)
        else:
            print('%r  %2.22f ms' % (method.__name__, (te - ts) * 1000))
        return result
    return timed

@timeit
def foo():clearclear

    do_some_work()
"""
