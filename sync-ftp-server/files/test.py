import os
import random

def generate_large_file(filename, size_in_gb):
    """Generate a binary file with random content of the specified size."""
    size_in_bytes = size_in_gb * (1024 ** 3)  # Convert GB to bytes
    chunk_size = 1024 * 1024  # 1MB chunk size
    written = 0
    
    with open(filename, "wb") as f:
        while written < size_in_bytes:
            chunk = os.urandom(min(chunk_size, size_in_bytes - written))
            f.write(chunk)
            written += len(chunk)
            print(f"Written {written / (1024 ** 3):.2f} GB...", end="\r")
    
    print(f"\nFile '{filename}' of size {size_in_gb} GB generated successfully.")

if __name__ == "__main__":
    generate_large_file("test_file.bin", 15)

