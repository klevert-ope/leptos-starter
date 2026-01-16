# Leptos CSR Starter

A starter template for building client-side rendered web applications with [Leptos](https://leptos.dev/) and Rust.

## Features

- 🦀 Written in Rust with Leptos framework
- ⚡ Client-Side Rendering (CSR)
- 🐳 Docker support with Nginx
- 🔄 GitHub Actions CI/CD pipeline
- 📦 Automated builds and testing
- 🚀 Ready for deployment to DockerHub

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://rustup.rs/) (latest stable version)
- [Trunk](https://trunkrs.dev/) - Build tool for Rust WASM applications
- [wasm32-unknown-unknown](https://rustwasm.github.io/wasm-pack/book/) target

## Getting Started

### 1. Clone the Repository

```bash
git clone <your-repo-url>
cd <your-repo-name>
```

### 2. Install Rust and Required Tools

If you haven't installed Rust yet:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add the WebAssembly target:

```bash
rustup target add wasm32-unknown-unknown
```

Install Trunk:

```bash
cargo install trunk
```

### 3. Run the Development Server

Start the development server with hot-reloading:

```bash
trunk serve
```

The application will be available at `http://localhost:8080`

For a different port:

```bash
trunk serve --port 3000
```

### 4. Build for Production

Create an optimized production build:

```bash
trunk build --release
```

The built files will be in the `dist/` directory.

## Project Structure

```
.
├── src/
│   ├── main.rs          # Application entry point
│   └── ...              # Your application code
├── index.html           # HTML template
├── Cargo.toml           # Rust dependencies
├── Dockerfile           # Docker configuration
├── nginx.conf           # Nginx configuration
├── docker-compose.yml   # Docker Compose setup
└── .github/
    └── workflows/
        └── build-test.yml  # CI/CD pipeline
```

## Docker Development

### Build and Run with Docker

```bash
# Build the Docker image
docker build -t leptos-app .

# Run the container
docker run -p 8080:80 leptos-app
```

### Using Docker Compose

```bash
# Start the application
docker-compose up

# Start in detached mode
docker-compose up -d

# Stop the application
docker-compose down
```

The application will be available at `http://localhost:8080`

## Development Tips

### Hot Reloading

Trunk provides automatic hot-reloading during development. Any changes to your Rust code will trigger a rebuild and refresh the browser.

### Debugging

For better error messages during development, you can set:

```bash
RUST_LOG=debug trunk serve
```

### Testing

Run your tests with:

```bash
cargo test
```

## Deployment

### Automated Deployment

This project includes a GitHub Actions workflow that:
- Runs tests on every push and pull request
- Builds the Docker image
- Pushes to DockerHub on merges to `main` or `develop`

### Manual Deployment

To deploy manually:

1. Build the production image:
   ```bash
   docker build -t your-username/leptos-app:latest .
   ```

2. Push to DockerHub:
   ```bash
   docker push your-username/leptos-app:latest
   ```

3. Deploy to your server:
   ```bash
   docker pull your-username/leptos-app:latest
   docker run -d -p 80:80 your-username/leptos-app:latest
   ```

## CI/CD Setup

To enable automated DockerHub deployment:

1. Go to your repository Settings → Secrets and variables → Actions
2. Add the following secrets:
    - `DOCKER_USERNAME`: Your DockerHub username
    - `DOCKER_PASSWORD`: Your DockerHub access token

The workflow will automatically build and push images when you push to `main` or `develop` branches.

## Common Issues

### Port Already in Use

If port 8080 is already in use:

```bash
trunk serve --port 3000
```

### WASM Target Not Found

If you get an error about the wasm32 target:

```bash
rustup target add wasm32-unknown-unknown
```

### Trunk Command Not Found

Ensure Trunk is installed and in your PATH:

```bash
cargo install trunk --locked
```

## Resources

- [Leptos Documentation](https://leptos.dev/)
- [Leptos Book](https://book.leptos.dev/)
- [Trunk Documentation](https://trunkrs.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.