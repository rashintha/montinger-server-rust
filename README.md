# Montinger Server

![Static Badge](https://img.shields.io/badge/1.78.0-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white&label=Rust&labelColor=%23000000&color=gray)
![MongoDB](https://img.shields.io/badge/MongoDB-%234ea94b.svg?style=for-the-badge&logo=mongodb&logoColor=white)

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