"""Verify a file against an S3 multipart ETag ("<md5-of-part-md5s>-<n>"), or a plain md5 ETag."""
import hashlib
import sys

def multipart_etag(path: str, part_size: int) -> str:
    digests = []
    with open(path, "rb") as f:
        while chunk := f.read(part_size):
            digests.append(hashlib.md5(chunk).digest())
    if len(digests) == 1:
        return digests[0].hex()
    return f"{hashlib.md5(b''.join(digests)).hexdigest()}-{len(digests)}"

if __name__ == "__main__":
    path, expected = sys.argv[1], sys.argv[2].strip('"')
    part_size = int(sys.argv[3]) if len(sys.argv) > 3 else 1024 ** 3
    got = multipart_etag(path, part_size)
    print(f"expected {expected}\ncomputed {got}")
    sys.exit(0 if got == expected else 1)
