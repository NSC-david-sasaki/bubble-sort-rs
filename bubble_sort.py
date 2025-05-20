import statistics
import time
import random

def bubble_sort(arr):
    n = len(arr)
    for i in range(n):
        swapped = False
        for j in range(0, n - 1 - i):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
                swapped = True
        if not swapped:
            break
    return arr

def benchmark(sample_size: int = 10000, 
             warmup_secs: float = 2.0,
             measurement_secs: float = 6.0) -> None:
    # Generate test data
    size = 1000
    data = list(range(size))
    
    # Warm-up phase
    print("Warming up...")
    start_warmup = time.perf_counter()
    while time.perf_counter() - start_warmup < warmup_secs:
        test_data = data.copy()
        random.shuffle(test_data)
        bubble_sort(test_data)
    
    # Measurement phase
    print("Running benchmarks...")
    durations = []
    start_measurement = time.perf_counter()
    samples = 0
    
    while samples < sample_size and (time.perf_counter() - start_measurement) < measurement_secs:
        test_data = data.copy()
        random.shuffle(test_data)
        
        start = time.perf_counter()
        bubble_sort(test_data)
        duration = time.perf_counter() - start
        
        durations.append(duration * 1000)  # Convert to milliseconds
        samples += 1
    
    # Calculate statistics
    mean = statistics.mean(durations)
    stdev = statistics.stdev(durations)
    median = statistics.median(durations)
    
    print(f"\nResults (n={size}, samples={samples}):")
    print(f"Mean: {mean:.3f}ms ± {stdev:.3f}ms")
    print(f"Median: {median:.3f}ms")
    print(f"Min: {min(durations):.3f}ms")
    print(f"Max: {max(durations):.3f}ms")

if __name__ == "__main__":
    benchmark()