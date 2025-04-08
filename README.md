# StudyTime

터미널에서 실행하는 간단하고 직관적인 **타이머 CLI 도구**입니다.  
Rust로 개발되었으며, 집중과 휴식을 라운드별로 반복하며 생산성을 높일 수 있도록 도와줍니다.

![pomorust banner](https://img.shields.io/badge/Rust-CLI-blue?style=flat-square)

---

## ✨ 특징

- ⏰ 집중 / 휴식 시간 사용자 설정 가능
- 🔁 라운드 수 반복 기능
- 🖥️ 직관적인 터미널 출력
- 📦 Rust 기반, 빠르고 가볍습니다

---

## 🚀 사용법

### 실행

```bash
cargo run -- --focus 25 --breaktime 5 --rounds 4
```

### 또는 설치 후 사용:
```bash
cargo install --path .
pomorust --focus 25 --breaktime 5 --rounds 4
```

## 옵션
| 옵션            | 설명               | 기본값 |
|-----------------|--------------------|--------|
| `--focus` | 집중 시간 (분)     | `25`   |
| `--breaktime`  | 쉬는 시간 (분)     | `5`    |
| `--rounds`| 반복 라운드 수     | `1`    |

# 🔧 설치 방법
```bash
git clone https://github.com/Leehomin11/rust_cli.git
cd rust_cli
cargo build --release
```
또는 글로벌 설치:
```bash
cargo install --path .
```
