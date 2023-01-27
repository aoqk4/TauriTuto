# Tauri Tutorial

## 1차 목표 : 아무거나라도 Window Application 제작

## Day 1. 맛보자.

- 진행 환경 : WSL Kali Linux

### 환경 구성

1. 설치 관련은 https://tauri.app/ko/v1/guides/ 참조.

2. Next.js + tauri로 진행.
   https://tauri.app/ko/v1/guides/getting-started/setup/next-js/

3. CSS의 경우 TailwindCSS 이용.

### Day 1 결과

1. DB를 제외하곤 환경 구성 완료
2. 간단하게 파라미터 넣어서 Rust 코드로 사칙연산 까지 콘솔 찍기.

## Day 2 목표.

1. React의 useEffect 관련해서 타입 에러 해결해보자.
2. DB 연결 해보자. (Rust로는 처음이라 몬할수도 있을 것)

## Day 2. Postgresql Setting

목표는 달성하지 못함.
Postgresql을 사용하기로 했는데 우분투 환경에서 세팅에 애를 먹었다.
해결 방법은 링크의 로그인해서 서버를 키는 것

https://dba.stackexchange.com/questions/182189/how-do-i-access-postgres-when-i-get-an-error-about-var-run-postgresql-s-pgsql

DB 테스트 코드도 CookBook에서 복붙해오긴 했지만, 수정필요

## Day 3. 목표

Day 2 와같다.

1. React의 useEffect 관련해서 타입 에러 해결해보자.
2. DB 연결 해보자. (Rust로는 처음이라 몬할수도 있을 것)

## Day 3. Postgresql Setting & test execute DB query

1. env를 통해 DB ID와 PW를 설정

2. postgresql Create 쿼리문을 보내는 테스트 메소드 완성.
   (dotenv, postgresql lib 사용)

3. React Hook 관련해서는
   https://willhart.io/post/tauri-create-react-app-tutorial-part3/ 참고해야겠다.

## Day 4. 목표

1. 프론트 디자인 구상해보자.
2. API 형식으로 테스트 해보자.

## Day 4. Postgresql INSERT test

1. Cookbook에서 제공되는 INSERT코드 테스트 메소드 --> 이제 API로 적용 시켜보자.

## Day 5. 목표

1. 프론트 디자인 구상해보자.
2. API 형식으로 테스트 해보자(데이터 받아와서 DB로 보내는 것?).
