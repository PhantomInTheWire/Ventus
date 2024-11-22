import os

# File path for the output file
file_path = 'random_file.bin'

# Define the size of the file (1 GB)
file_size = 100 * 1024 * 1024 * 1024  # 1 GB in bytes

# Open the file in write-binary mode
with open(file_path, 'wb') as f:
    # Write random data to the file in chunks to avoid memory overload
    chunk_size = 1024 * 1024  # 1 MB chunks
    for _ in range(file_size // chunk_size):
        f.write(os.urandom(chunk_size))

print(f"1 GB random binary file created at {file_path}")

