# Rust GRPC + tRPC + Hono Workspace Project 🦀⚡️

Полноценный workspace-проект, объединяющий микросервисы на Rust с gRPC, генератор нагрузки, ingestion в ClickHouse, tRPC API на TypeScript + Hono, и PostgreSQL как основную БД.

## 📦 Стек технологий

- **Rust** — основной язык, микросервисы (`ingester`, `generator`)
- **ClickHouse** — высокоскоростная аналитическая БД
- **PostgreSQL** — основная реляционная база данных
- **gRPC** — для связи между `generator` и `ingester`
- **Hono + tRPC** — frontend/backend API на TypeScript
- **Docker Compose** — для оркестрации
- **TOML-конфиги** — конфигурация приложений
- **Drizzle ORM** — ORM для работы с PostgreSQL

## 📁 Структура проекта

```bash
.
├── apps/
│   ├── ts-api/         # TypeScript API на Hono + tRPC
│   ├── ingester/       # Rust-сервис для записи данных в ClickHouse
│   └── generator/      # Генератор нагрузки на Rust
├── packages/
│   └── rust-deps/      # Общие зависимости для Rust-сервисов
│       └── proto/      # Протобуфы для gRPC
├── migrations/
│   ├── clickhouse/     # Миграции для ClickHouse
│   └── postgres/       # Миграции для PostgreSQL
├── docker-compose.yaml # Конфигурация Docker
├── Dockerfile          # Мульти-стейдж сборка Rust-сервисов
└── Makefile            # Команды для управления проектом
```

## 🚀 Установка и запуск

### Предварительные требования

- Docker и Docker Compose
- Rust (для локальной разработки)
- Node.js/Bun (для локальной разработки TS-API)

### Запуск в Docker

```bash
# Создайте файл .env с необходимыми переменными
cp .env.example .env

# Запустите все сервисы
make compose
```

### Локальная разработка

```bash
# Запуск Ingester
make ingester

# Запуск Generator
make generator

# Запуск обоих сервисов
make workspace

# Запуск TS-API
cd apps/ts-api
bun run dev
```

## 🔧 Конфигурация

### Переменные окружения

Создайте файл `.env` со следующими переменными:

```
CLICKHOUSE_PASSWORD=ваш_пароль
POSTGRES_PASSWORD=ваш_пароль
```

### Строки подключения к базам данных

- **PostgreSQL**: `postgresql://postgres:password@postgres:5432/postgres`
- **ClickHouse**: `http://clickhouse:8123` или `tcp://clickhouse:9000`

## 🗄️ Миграции

Проект использует Goose для управления миграциями базы данных:

```bash
# Миграции PostgreSQL
make migrations-up-pg

# Миграции ClickHouse
make migrations-new-ch MIGRATION_NAME=имя_миграции
```

## 📝 API

TS-API предоставляет REST интерфейс для работы с данными:

- `GET /posts` - получить список постов
- `POST /posts` - создать новый пост
- `DELETE /posts/:id` - удалить пост

## 🔍 Особенности проекта

- Высокопроизводительная обработка данных с использованием Rust
- Аналитика в реальном времени с ClickHouse
- Безопасное хранение данных в PostgreSQL
- Современный API с tRPC и Hono
- Полная контейнеризация для простоты развертывания
- Асинхронные операции с использованием tokio

## 📡 Тестирование gRPC API с помощью grpcui

Для удобного тестирования gRPC API вы можете использовать grpcui - графический интерфейс для взаимодействия с gRPC-сервисами:

### Установка grpcui

```bash
go install github.com/fullstorydev/grpcui/cmd/grpcui@latest
```

### Запуск интерфейса

```bash
# Локальное тестирование
grpcui -plaintext localhost:50051

# Для тестирования в Docker
grpcui -plaintext -bind=0.0.0.0 -port=8080 ingester:50051
```

После запуска откройте браузер по адресу http://localhost:50051 (или другому указанному порту), чтобы увидеть интерактивный интерфейс для тестирования gRPC методов.

```

## 🤝 Вклад в проект

Вклады приветствуются! Для внесения изменений:

1. Форкните репозиторий
2. Создайте ветку для фичи (`git checkout -b feature/amazing-feature`)
3. Закоммитьте изменения (`git commit -m 'Add some amazing feature'`)
4. Отправьте ветку (`git push origin feature/amazing-feature`)
5. Откройте Pull Request
```
