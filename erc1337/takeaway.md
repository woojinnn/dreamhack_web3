# private key, public key, ECDSA, keccak256 정리.
## keccak256
- 이더리움에서 해시를 쓴다고 하면 다 keccak256을 쓴다. keccak256은 어떤 길이의 input을 받아서 256bits(=32B)의 output을 내뱉는다.
- cf. `abi_encode`도 있다. 별거 없고, 그냥 256bit에 맞춰서 패킹해주는 애다. (64bits, 64bits)의 pair를 abi encoding하게 되면 (256bits, 256bits)가 되도록 예쁘게 패딩해준다.
## Key, address의 관계
- 어떤 계정의 address는 keccak256(public_key)를 통해 만들어진다.
- private_key와 public_key는 타원곡선의 이산대수 문제를 통해 연관되어 있다. D=dG라고 할 때, d가 private key, D가 public key에 해당한다.
## ECDSA 서명
- ECDSA는 암호화가 아니라 서명이라고 부르는 것이 더 정확하다.
- ECDSA 함수는 (private_key, random_noise, data)를 받아서 r, s, v값을 만들어낸다. (서명을 만든다)
- r, s, v와 data를 통해 public_key를 알 수 있다. address는 public_key로부터 유도되므로, 이 데이터는 어떤 주소를 갖는 사람이 서명했던거구나, 알 수 있게 된다.
- 여기서 data는 보통 keccak256(data) 꼴을 쓴다. A가 data의 원본값을 모르기를 원할수도 있고, keccak256을 쓰면 256bits로 데이터가 줄어들어서 가스습비도 아낄 수 있기 때문이다. 


# ERC, EIP...
- ERC20: balanceOf, transfer, approve 등이 구현된 대체 가능한 토큰 표준
- ERC-1337: 블록체인 구독 서비스 표준 (월별 구독 이런걸 표현)
- EIP-712: 읽을 수 있는 데이터 서명.
- ERC2771Context: 가스비 대납.