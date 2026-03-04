from Crypto.Hash import keccak
import subprocess
import sys
import requests

base_url = "http://host3.dreamhack.games:15374/9d80fd777c7e/"
level_contract_address = "0xBe038E971A3c7b689548518b8940265B998bC57c"


def keccak256(inp):
    hasher = keccak.new(digest_bits=256)
    hasher.update(inp)
    return hasher.digest()


def get_storage_at(address, index):
    result = subprocess.run(
        ["cast", "storage", "--rpc-url", base_url + "rpc/", address, hex(index)],
        capture_output=True,
    )
    return int(result.stdout, 16)


def pack_uint(value):
    return value.to_bytes(32, "big")


for key in range(1000):
    h = keccak256(pack_uint(key % 10) + pack_uint(0))
    h = keccak256(pack_uint((key // 10) % 10) + h)
    h = keccak256(pack_uint(key // 100) + h)

    slot = int.from_bytes(h, "big")
    res = get_storage_at(level_contract_address, slot)

    if res == 1:
        print(f"Ans: {key}")
