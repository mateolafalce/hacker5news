<div align="center">

# Hacker News Top 5 Articles

<img src="public/preview.png" alt="Preview" />

</div>

A simple web application built with Rust and Axum that fetches and displays the top 5 articles from Hacker News. Perfect for those who get anxious seeing too many news  but don't have time to read everything.

## Installation

### Option 1: Using Docker

1. **Clone this repository:**

```bash
git clone https://github.com/mateolafalce/hacker5news.git
cd hacker5news
```

2. **Run with Docker Compose:**

```bash
docker compose up
```

3. **Access the application:**

```
http://localhost:3000
```

### Option 2: Build from Source

1. **Clone this repository:**

```bash
git clone https://github.com/mateolafalce/hacker5news.git
cd hacker5news
```

2. **Compile & Run**

```bash
cargo run --release
```

3. **Look at localhost**

```
http://localhost:3000
```

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- [Hacker News API](https://github.com/HackerNews/API) for providing the data
- [Axum](https://github.com/tokio-rs/axum) for the excellent web framework
