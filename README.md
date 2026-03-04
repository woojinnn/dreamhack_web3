# [alloy](https://alloy.rs/)로 문제 푸는 법
`dreamhack_utils` 안에 드림핵 문제 풀이에사용할 수 있을만한 유틸리티 함수들이 있다.
- `challenge::info`: `fetch_prob_info`, `get_info_value`, `read_info_url_from_arguments`, `rpc_url_from_info_url`
- `forge`: `build_contract_bytecode`
  + `Path::new(env!("CARGO_MANIFEST_DIR"))`을 사용하자. `Solution.sol` 파일을 문제의 cargo 메인 디렉토리에 위치시키면 관리하기 더 쉽다.


1. 문제 디렉토리에서 `cargo init <문제 이름>`
2. sc_make_boko_winner 문제처럼 Cargo.toml 설정하기. 그 안에 `Solution.sol`과 같은 파일을 넣으면 좋다.

이후 `cargo r -p <문제 이름> -- http://.../info`로 실행하면 된다.