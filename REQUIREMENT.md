# Rust를 이용한 CLI 단위 변환기

# 구현 목표

- CLI(command line interface) 구현
- 길이, 넓이, 부피에 대한 변환 제공
- 단위의 이름과 단위의 기호 모두 인식 가능

# 동작

### 입력 형식

- [프로그램 이름] [입력 단위] [출력 단위] [값]
- [프로그램 이름] [값] [입력 단위] [출력 단위]

    $ conv m yd 100
    109
    
    $ conv 100 m yd
    109
    
    $ conv 100 야드 미터
    91.4

### 인식 할 수 없는 입력

    $ conv 1000 mm m
    'mm'는 구현되지 않거나 존재하지 않는 단위입니다.
    See 'conv -h'
    
    $ conv m yd not_number
    'not_number'는 수가 아닙니다.
    See 'conv -h'
    
    $ conv m m
    See 'conv -h'
    
    $ conv
    See 'conv -h'

### 도움말

    $ conv -h
    [사용법]
    conv [입력 단위] [출력 단위] [값]
    conv [값] [입력 단위] [출력 단위]
    
    =예시=
    conv m yd 100
    conv 100 m yd
    
    [단위]
    길이
    	미터 m
    	야드 yd
    	리 里
    	해리 海里
    넓이
    	제곱미터 m^2 m2
    	헥타르 ha
    	에이커 ac
    	평
    부피
    	리터 L
    	갤런 gal
    	온스 oz
    	되
    
    [버전]
    conv [-v|--version]

### 버전

    $ conv -v
    Unit Converter version 1.0.0
    
    $ conv --version
    Unit Converter version 1.0.0

# 상세 구현 목표

## CLI 구현

## 길이, 넓이, 부피에 대한 변환 제공

- 길이
    - 미터(mm)
    - 야드(yd)
    - 리(里)
    - 해리(海里)
- 넓이
    - 제곱미터(m^2, m2)
    - 헥타르(ha)
    - 에이커(ac)
    - 평
- 부피
    - 리터(L)
    - 갤런(gal)
    - 온스(oz)
    - 되

## 단위의 이름과 단위의 기호 모두 인식 가능