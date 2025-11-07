# Cargo

Cargo 는 러스트에서 사용하는 빌드 시스템 및 패키지 매니저

## Cargo로 프로젝트 생성

```
$ cargo new hello_cargo --bin
$ cd hello_cargo
```

## Cargo 프로젝트를 빌드 하고 실행

### 빌드

```
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

### 실행

- 터미널로 실행

```
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

- cargo로 실행

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

### Cargo 컴파일 체크

이 커맨드는 코드가 컴파일되는지를 빠르게 확인해주지만 실행파일을 생성하지는 않음

```
cargo check
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

## 릴리즈 빌드

```
cargo build --release
```
