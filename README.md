# React_Rocket

## 프론트엔드

```
create-react-app frontend
```

```
axios
```

## 백엔드

```
cargo new backend --bin
```

```
rustup toolchain install nightly
```

```
rustup default nightly
```

```
rustc --version
```

```
[dependencies]
rocket = "0.4.10"
```

##

```
[dependencies]
rocket = "0.4.10"
rocket_contrib = "0.4"
serde = { version = "1.0.126", features = ["derive"] }
serde_json = "1.0.64"
serde_derive = "1.0.126"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
diesel = { version = "1.4.7", features = ["mysql"] }
dotenv = "0.15.0"
r2d2 = "*"
r2d2-diesel = "*"

```

## rocket

- restapi
- 빠른 웹 애플리케이션 작성을 위한 웹 프레임워크

## rocket_contrib

- Rocket 웹 프레임워크를 위한 커뮤니티 기여 라이브러리

## serde

- Rust 데이터 구조를 직렬화 및 역직렬화하기 위한 프레임워크

## serde_json

- Serde는 Rust 데이터 구조를 효율적이고 일반적으로 직렬화 및 역직렬화하기 위한 프레임워크

## serde_derive

- Serde는 Rust 데이터 구조를 효율적이고 일반적으로 직렬화 및 역직렬화하기 위한 프레임워크

## reqwest

- 인체공학적이며 배터리가 포함된 Rust용 HTTP 클라이언트입니다.

## tokio

- async runtime 라이브러리

## diesel

- Diesel - Rust를 위한 안전하고 확장 가능한 ORM 및 쿼리 빌더 Diesel은 쿼리에 대한 안전하고 구성 가능한 추상화 때문에 Rust에서 데이터베이스와 상호 작용하는 가장 생산적인 방법입니다.

## dotenv

- Rust를 위한 `dotenv` 구현

## r2d2

- A generic connection pool for Rust.

## r2d2-diesel

- 디젤에 대한 r2d2 지원
