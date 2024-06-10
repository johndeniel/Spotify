# Spotify

This is a simple web application that utilizes the Spotify API to fetch and display your personal playlist. It's designed to provide an easy way to view your favorite songs without the need to open the Spotify app. Spotify has been Dockerized for effortless deployment, and its Docker image is readily available on [Docker Hub](https://hub.docker.com/repository/docker/johndeniel/spotify/tags).

To run Spotify using Docker, execute this command in your terminal:

```bash
docker run -d -it --name spotify -p 7860:7860 johndeniel/spotify:build-v1.001
```

## Features

- **Axum Integration**: Integrates Axum, a high-performance Rust web framework, to power its backend functionality. Axum is known for its speed and efficiency, making it a great choice for building scalable web applications.

- **Spotify API Integration**: Integrates with the Spotify API to provide seamless access to music and playlist management features. Spotify API offers a comprehensive set of endpoints for interacting with user data, playlists, and music catalog, enhancing the functionality and user experience.


- **Automated Dockerization**: Includes a GitHub Action workflow for Dockerizing the FastAPI application, streamlining the deployment process with automated containerization.

- **Hugging Face Platform Integration**: Seamlessly integrates with the Hugging Face Platform, offering access to the live instance on [Hugging Face Spaces](https://huggingface.co/spaces/johndeniel/Spotify).

---

Spotify demonstrates the utilization of cutting-edge technologies like Spotify API, Docker, GitHub Actions, and the Hugging Face Platform, offering developers an efficient development experience.