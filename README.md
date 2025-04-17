# VLITZ - Frida 기반 CLI 동적 디버거

VLITZ는 Frida를 기반으로 하는 강력한 CLI 동적 디버깅 도구입니다. 단순한 후킹 도구가 아닌 분석, 메모리 조작, 탐색, 자동화를 하나로 통합한 확장 가능한 CLI 플랫폼입니다.

## 주요 기능

- 앱에 attach하여 동적 분석 수행
- 메서드/함수 후킹 및 호출
- 메모리 값 덤프, 변경, 추적
- 클래스/모듈/익스포트 정보 탐색
- 라벨링, 태그, 필터링 시스템
- 자동화 스크립트 실행 (.vzs)
- 루팅 탐지 우회, SSL 우회, 스피드핵 등 자동 preset 적용

## 아키텍처

VLITZ는 Rust로 작성되었으며, 모든 핵심 로직이 Rust에 있습니다. Frida 스크립트는 단순히 기능을 제공하는 RPC 호출로만 구성됩니다.

### 디렉토리 구조

- `cli/`: CLI 입력, REPL, 프롬프트 관리
- `core/`: 명령어 파싱 및 실행 로직
- `script/`: .vzs 스크립트 파서 및 실행기
- `preset/`: speedhack, unpinning 등 자동 후킹
- `runtime/`: Frida 세션 및 스크립트 실행 관리
- `shared/`: 공통 데이터 타입, 유틸, 파서 등

## 사용 방법

```bash
# 기본 사용법
vlitz <process_name>

# USB 디바이스 연결
vlitz -U <process_name>

# 프로세스 리스트 보기
vlitz ps

# 특정 PID로 연결
vlitz -p <pid>

# 스크립트 로드
vlitz -l <script.vzs> <process_name>
```

### REPL 명령어 예시

```
# 클래스 목록 보기
list class MainActivity

# 메서드 선택 후 후킹
sel 0
list method
sel 3
hook

# 메모리 주소 보기 및 수정
: 0x12345678
read float
write 500 float

# 메모리 스캔
search 47.3 float
grep float=47.3
sav 0
```

## 빌드 및 설치

```bash
# 빌드
cargo build --release

# 설치
cargo install --path .
```

## 기여하기

이슈를 보고하거나 풀 리퀘스트를 통해 기여해주세요. 모든 기여는 환영합니다!
