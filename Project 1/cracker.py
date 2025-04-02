import hashlib
import time
import sys

def main():
    if len(sys.argv) != 3:
        print("Invalid arguments!")
        print(f">> {sys.argv[0]} <sha256sum> <password_file>")
        exit(1)

    wanted_hash = sys.argv[1]
    password_file = sys.argv[2]

    attempts = 1
    start_time = time.time()

    print(f"Attempting to crack: {wanted_hash}!\n")

    with open(password_file, 'r') as f:
        for line in f:
            password = line.strip().encode('utf-8')
            password_hash = hashlib.sha256(password).hexdigest()

            print(
                f"[{attempts}] {line.strip()} == {password_hash}"
            )

            if password_hash == wanted_hash:
                end_time = time.time()
                duration = end_time - start_time
                print(
                    f"Password hash found after {attempts} attempts! {line.strip()} hashes to {password_hash}!"
                )
                print(f"Time taken: {duration:.2f} seconds")
                exit(0)

            attempts += 1

    end_time = time.time()
    duration = end_time - start_time
    print("Password hash not found!")
    print(f"Time taken: {duration:.2f} seconds")

if __name__ == "__main__":
    main()
