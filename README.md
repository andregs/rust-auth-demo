# Auth in Rust

Based on tutorial from
https://betterprogramming.pub/structuring-rust-project-for-testability-18207b5d0243

Register: New users can be registered into the application.
Login: Registered users can log in by providing a credential and in return will receive a token that can be used for authentication.
Authenticate: Resolving a given token into a user.

                              TokenRepo -> RedisRepo
                                  |
HTTP -> RestAuthController -> AuthService
                                  |
                              CredentialRepo -> PostgresRepo

RestController: POST /register, /login, /authenticate
AuthService: register, login, authenticate
TokenRepo: generate_token, save_token, get_user_by_token
CredentialRepo: save_credential, is_credential_exists
