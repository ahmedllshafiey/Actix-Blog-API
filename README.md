# Article Management API

This repository contains an Actix-web based API for managing articles. The API provides endpoints to create, read, update, and delete articles. 

## Endpoints

### Get Articles
Retrieve the list of all articles.

**URL**: `/articles/list`

**Method**: `GET`

**Response**:
- Status: 200 OK
- Body: JSON array of articles

### Create Article
Add a new article to the list.

**URL**: `/articles/list`

**Method**: `POST`

**Request Body**:
```json
{
    "title": "Article Title",
    "data": "Article content"
}
```

**Response**:
- Status: 200 OK
- Body: JSON array of articles including the newly created one

### Update Article
Update an existing article by its ID.

**URL**: `/articles/list/{id}`

**Method**: `PUT`

**Path Parameters**:
- `id`: ID of the article to be updated

**Request Body**:
```json
{
    "data": "Updated article content"
}
```

**Response**:
- Status: 200 OK
- Body: JSON array of articles including the updated one

### Delete Article
Delete an article by its ID.

**URL**: `/articles/list/{id}`

**Method**: `DELETE`

**Path Parameters**:
- `id`: ID of the article to be deleted

**Response**:
- Status: 200 OK
- Body: JSON array of remaining articles

## Running the API

### Prerequisites
- Rust and Cargo installed
- Dependencies specified in `Cargo.toml`

### Running the Server

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/article-management-api.git
   cd article-management-api
   ```

2. Build and run the server:
   ```sh
   cargo run
   ```

The server will start on `http://localhost:8080`.

## Project Structure

- **main.rs**: Entry point of the application
- **models.rs**: Contains the definitions for `CreateArticle`, `UpdateArticle`, and `ArticleEntry`
- **handlers.rs**: Contains the endpoint handler functions
- **state.rs**: Contains the `AppState` struct used for managing application state

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests.

## License

This project is licensed under the MIT License.