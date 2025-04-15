# URL Shortener API

A lightweight, high-performance URL shortening service built with Rust, Actix Web, and MySQL.

## Features

- Create short URLs for any valid web address
- HTTP redirection to original URLs
- MySQL database for persistent storage
- RESTful API design
- Environment-based configuration

## Tech Stack

- **Backend**: Rust with Actix Web framework
- **Database**: MySQL
- **Crates**:
  - `actix-web`: Web framework
  - `mysql`: Database connectivity
  - `rand`: Random short URL generation
  - `serde`: Serialization/deserialization

## API Endpoints

### Create Short URL

```
POST /shorten
```

**Request Body**:
```json
{
  "url": "https://example.com/very/long/url/that/needs/shortening"
}
```

**Response**:
```json
{
  "short_url": "http://localhost:8080/Ab3x7Yz9",
  "original_url": "https://example.com/very/long/url/that/needs/shortening"
}
```

### Redirect to Original URL

```
GET /{short_code}
```

When accessing a short URL, the service automatically redirects to the original URL.

## Installation

### Prerequisites

- Rust and Cargo (1.70.0+)
- MySQL Server (5.7+ or 8.0+)

### Setup Database

```sql
CREATE DATABASE url_shortener;
CREATE USER 'shortener_user'@'%' IDENTIFIED BY 'your_password';
GRANT ALL PRIVILEGES ON url_shortener.* TO 'shortener_user'@'localhost';
FLUSH PRIVILEGES;
```
### Change the bind address for the MySQL server
Change bind address in /etc/mysql/mysql.conf.d/mysqld.cnf from local host to 0.0.0.0
bind-address = 0.0.0.0

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/yourusername/url-shortener.git
cd url-shortener

# Build the project
cargo build --release
```

### Configuration

The application uses environment variables for configuration:

```bash
# Database connection
export DATABASE_URL="mysql://shortener_user:your_password@localhost:3306/url_shortener"

# Server settings
export HOST="127.0.0.1"
export PORT="8080"

# Base URL for generated short URLs
export BASE_URL="http://localhost:8080"
```

## Running the Application

```bash
# Run the container from the image built
docker run -e HOST=0.0.0.0 -e PORT=80 -e DATABASE_URL=mysql://shortener_user:s3cur3_p4ssw0rd@127.0.0.1:3306/url_shortener -p 65530:80 url-shortner

# Run directly
cargo run --release

# Or use the binary
./target/release/url-shortener
```

The server will start at http://127.0.0.1:8080 (or whatever HOST:PORT you configured).

## Usage Examples

### Creating a Short URL

Using curl:

```bash
curl -X POST http://localhost:8080/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com/very/long/url/that/needs/shortening"}'
```

### Using a Short URL

Simply open in a browser:
```
http://localhost:8080/Ab3x7Yz9
```

Or with curl:
```bash
curl -L http://localhost:8080/Ab3x7Yz9
```


## Future Improvements

- Add custom URL support
- Implement analytics for link clicks
- Add user authentication
- Set expiration dates for links
- Add caching layer for frequently accessed URLs
- Implement rate limiting


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
