import random
import time
import numpy as np
import pkg_resources

# Print the installed packages
installed_packages = pkg_resources.working_set
installed_packages_list = sorted(
    ["%s==%s" % (i.key, i.version) for i in installed_packages]
)
print(installed_packages_list)

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
    random.seed(100)
    return [random.randint(0, n) for _ in range(n)]

#print(generate_data(n=1000))

@measure_time
def test_():
    time.sleep(2)
    print("Hello world")

#test_()

try:
    import rust_example_package as rp
    rp.say_hi()
except Exception as e:
    print(e)

