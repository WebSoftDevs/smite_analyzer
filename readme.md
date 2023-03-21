## Smite Analyzer

![maintenance](https://img.shields.io/maintenance/yes/2025)
![version](https://img.shields.io/github/v/tag/WebSoftDevs/smite_analyzer?style=plastic)
![build](https://img.shields.io/github/actions/workflow/status/WebSoftDevs/smite_analyzer/ci-cd.yml)


![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Vue.js](https://img.shields.io/badge/vuejs-%2335495e.svg?style=for-the-badge&logo=vuedotjs&logoColor=%234FC08D)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

<strong>Smite Analyzer</strong> is a web application that provides information about daily special matches in Smite. The goal of this site is to provide interesting and up-to-date information that are not available on other sites. Such as preferred gods for each game mode.


## How to set up app localy


- Clone repo `https://github.com/WebSoftDevs/smite_analyzer.git`

- Create .env file in root directory of the project.

```
SMITE_DEV_KEY=
SMITE_DEV_ID=
DATABASE_URL=postgres://postgres:postgres@localhost/smite_analyzer
```

- Run container (--build flag only at first run) `docker compose -f docker-compose-dev.yml up --build`
