
# Kerrigan - PostgreSQL Integration

## Table of Contents

- [Overview](#overview)
- [Deployment Status](#deployment-status)
- [Storage](#storage)
- [Credentials](#credentials)
- [Migrations](#migrations)
- [Containerization](#containerization)
- [Deployment](#deployment)
- [Project Setup](#project-setup)
- [CI/CD](#ci-cd)
- [Roadmap](#roadmap)

## Overview

This project integrates the Kerrigan PostgreSQL database with the Duke_rs server, now re-implemented in Rust. The project aims to set up user and task management with a Postgres database, managing migrations, and deploying to a DigitalOcean droplet.

## Deployment Status

[![Build and Deploy Docker Image](https://github.com/Knowvus/Duke_rs/actions/workflows/deploy.yml/badge.svg)](https://github.com/Knowvus/Duke_rs/actions/workflows/deploy.yml)


## Migrations

- **Migration Tool:** `sqlx` with Rust

## Containerization & Deployment

- **Docker Container Name:** `kerrigan`
- **Container Name:** `kerrigan`
- **Hosting:** DigitalOcean Droplet

## Project Setup

### 1) Create GitHub Repository
   - Initialize a new repository on GitHub for the project.

### 2) Create DigitalOcean Droplet
   - Set up a new droplet on DigitalOcean and configure it to run the Rust application and Postgres database.

### 3) Store SSH Keys
   - Store SSH keys on both the DigitalOcean Droplet and GitHub repository secrets for secure communication.

### 4) Initialize Rust Project
   - Install Rust and Cargo on your development machine:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Create a new Rust project:
     ```bash
     cargo new kerrigan_rs
     cd kerrigan_rs
     ```

### 5) Add `sqlx` and Configure Migrations
   - Add `sqlx` and `tokio` to your `Cargo.toml`:
     ```toml
     [dependencies]
     sqlx = { version = "0.5", features = ["runtime-tokio-rustls", "postgres", "migrate"] }
     tokio = { version = "1", features = ["full"] }
     warp = "0.3"
     serde = { version = "1.0", features = ["derive"] }
     serde_json = "1.0"
     ```
   - Configure database migrations:
     ```bash
     sqlx migrate add create_user_table
     sqlx migrate run
     ```

### 6) Set Up Docker
   - Create a `Dockerfile` for the Rust server and Postgres database:
     ```Dockerfile
     FROM rust:1.70 as builder
     WORKDIR /usr/src/app
     COPY . .
     RUN cargo install --path .

     FROM debian:buster-slim
     COPY --from=builder /usr/local/cargo/bin/kerrigan_rs /usr/local/bin/kerrigan_rs
     CMD ["kerrigan_rs"]
     ```
   - Build and run the Docker container:
     ```bash
     docker build -t kerrigan .
     docker run -p 8000:8000 kerrigan
     ```

## Roadmap

### Registration Process

- **Step 1:** User enters email address and clicks "Register"
- **Step 2:** Check for duplicate email
- **Step 3:** If email is not duplicate, create a new user
- **Step 4:** Send confirmation email with a link
- **Step 5:** User clicks the confirmation link and is registered

### Milestones

- [ ] Create User Table in Postgres
- [ ] Create Task Table in Postgres
- [ ] Implement User Registration Endpoint
- [ ] Implement Task Creation Endpoint
- [ ] Add Unit, Integration, and E2E testing