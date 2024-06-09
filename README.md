# Montinger Server

![Rust](https://img.shields.io/badge/1.78.0-%23000000.svg?style=for-the-badge&logo=rust&logoColor=%23000000&label=Rust&labelColor=gray&color=%23000000)
![MongoDB](https://img.shields.io/badge/7.0.8-%2347A248.svg?style=for-the-badge&logo=mongodb&logoColor=%2347A248&label=MONGODB&labelColor=gray&color=%2347A248)
![.ENV](https://img.shields.io/badge/.env-%23ECD53F.svg?style=for-the-badge&logo=dotenv&logoColor=%23ECD53F&color=gray)

## Configurations

Create a `.env` file in the root directory with the following details.

```conf
  # MongoDB configurations
  DB_HOST=    # DB Host IP or similar
  DB_USER=    # DB User
  DB_PASS=    # DB Password
  DB_PORT=    # DB Port
  DB=         # Database

  # gRPC configurations
  GRPC_PORT=50051
```