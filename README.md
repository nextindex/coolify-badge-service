![Deployment Status](https://badge.coolify.nxi.io/badge/a40owkkc4o08c48ccc8g8csw)

# Coolify Deployment Badge Service

A lightweight, high-performance, and secure microservice written in Rust that generates live-updating SVG status badges for applications hosted on Coolify.

## Features
- Tiny Footprint: Extremely low memory usage (~10MB RAM).
- Secure: Multi-stage Docker build using Google distroless base (no shell, no root access).
- Dynamic Width: Automatically calculates SVG width based on the length of the status text to prevent clipping.
- Real-time: Sends strict no-cache headers to ensure your README always shows the current state.

---

## Installation and Build

### 1. Local Development
Ensure you have the Rust toolchain installed.

1. Clone the project and navigate into the directory.
2. Create a .env file with the following content:
```
   COOLIFY_URL=https://app.coolify.io
   API_TOKEN=your_coolify_api_token
   PORT=3000
 ```
3. Run the service:
   `cargo run`

### 2. Build with Docker
The included Dockerfile produces a minimal, hardened production image.

```bash
docker build -t coolify-badge-service .
```


## Running with Docker Compose

This is the recommended way to deploy the service. It includes security hardening such as a read-only root filesystem.

```yaml
services:
  coolify-badge:
    build: .
    container_name: coolify-badge-service
    restart: always
    environment:
      - COOLIFY_URL=${COOLIFY_URL}
      - API_TOKEN=${API_TOKEN}
      - PORT=${PORT:-3000}
    ports:
      - "${PORT:-3000}:${PORT:-3000}"
    read_only: true
    security_opt:
      - no-new-privileges:true
    deploy:
      resources:
        limits:
          memory: 32M
          cpus: '0.1'
```

---

## How to Link the Badge

Once the service is running, use the following URL pattern:
https://your-domain/badge/application_uuid

### Markdown (GitHub / Gitlab README)

```md
![Deployment Status](https://badges.yourdomain.com/badge/your-app-uuid)
```

### HTML
```html
<img src="https://badges.yourdomain.com/badge/your-app-uuid" alt="Deploy Status">
```

### Finding your Application UUID
1. Open your Coolify Dashboard.
2. Select your Application.
3. Look at the browser URL. The random string at the end is your UUID:
   `https://app.coolify.io/.../application/too4kg8k48cswcgwcc4g80kg`

## Badge Statuses and Colors

The badge color updates automatically based on the latest deployment:

| Status | Color | Description |
| :--- | :--- | :--- |
| finished | Green | Successfully deployed |
| failed | Red | Deployment or build failed |
| in_progress | Blue | Currently building or deploying |
| queued | Gray | Waiting for a build slot |
| not_found | Orange | Invalid Application UUID |
| offline | Yellow | Service cannot reach Coolify API |

---

## Security and Performance
- Distroless: The container contains zero extra binaries (no sh, ls, or apt), significantly reducing the attack surface.
- Stripped Binary: The production build is stripped of symbols and optimized for size.
- Camo-Friendly: Works with GitHub proxy; use ?v=1 at the end of the URL if you need to force a manual refresh through GitHub cache.
