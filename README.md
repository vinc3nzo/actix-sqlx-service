# `Actix-web` + `SQLx` + многие другие

Здесь предлагается вариант архитектуры, применимой к разработке веб-сервиса
с использованием данных крейтов, а также интегрирована автоматическое
создание OpenAPI-спецификации и Swagger UI с помощью крейта `utoipa`.

---

## Быстрый старт с использованием Docker
```bash
git clone git@github.com:vinc3nzo/actix-sqlx-service.git
cd actix-sqlx-service/deployment
```

### Dev-сборка
По умолчанию Swagger UI доступен на http://localhost:3000/docs/
(слеш на конце обязателен). Рекомендуется для ознакомительного
запуска.
```bash
echo "APP_SECRET=secret" > .env
docker compose -f dev.yml up -d
```

В dev-сборке запросы поступают напрямую приложению.

### Prod-сборка
В этой сборке Swagger UI отключен по умолчанию. Эту опцию, как
и многие другие, можно настроить через переменные окружения.
См. `bookstore/.example.env` для полного списка переменных
окружения.
```bash
cat > .env <<- EOF
APP_SECRET=secret
APP_ADMIN_USER=admin
APP_ADMIN_PASS=1234
APP_DATABASE_PASS=postgres
EOF

docker compose -f prod.yml up -d
```

В prod-сборке запросы проксируются nginx, который слушает на внешнем 8000 порту.
