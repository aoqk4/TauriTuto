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

## Day 5. Invoke Test

1. DB를 위한 코드들을 함수화 - 모듈화 시켜서 Tauri에 집어 넣었다.
2. 프론트에서 Invoke로 적용 가능한 것 까지 확인..
   (Invoke의 작동 방식은 잘 모르겠지만 DB코드는 Rust코드를 직접 넣어도 될것 같다. 이중으로 무언가를 하는 느낌?)

## Day 6. 목표

1. 데이터 넣을 수 있게 프론트 수정해서 마음대로 DB에 넣게 하자..
2. 비밀번호에 해시함수 적용 가능한지 알아보자.

## Day 6. Invoke + input value test..

1. useState 사용해서 데이터 넣고 DB에 INSERT는 가능하나, DB ID를 AUTO_INCREMENT로 수정하자..
2. CookBook에서 Encrpytion이 가능한 코드를 찾았다. 일단 test mod에 넣고 실험하면서 돌려보자..

## Day 7. 목표

1. DB 수정하기(아니면 새로 파든가)
2. Encrpyion 코드 건들여보자.

## Day 7. Fix DB

1. DB 새로 파서 AUTO_INCREAMENT 알아봤더니 그냥 안넣으면 default로 들어가는 형식이였다..
2. param 계속 넣었는데 프론트 딴에 코드가 엉망이여서 고치는데 한참 해맸다..
3. default는 insert에서 생략이 가능했다.
4. 프론트 --> 서버로 넘기는 데이터에 대해서는 걱정 안해도 될 것 같다.
   (그냥 String 형식으로 Hook 지정하고 넘겼더니 되드라)

## Day 8. 목표

1. Encrpytion 코드를 어디다 써먹을지 생각해보자 (필요 없을수도 있을 것 같아서)
2. Tauri docs를 좀더 읽어보자. (서버에서 받아온 데이터를 어떻게 프론트에서 써야할지)
