
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

- **Migration Tool:** `diesel` with Rust
```
Samples
@ = %40
name@gmail.com = name%40gmail.com

export PG_USER=email@domain.com
export PG_PASSWORD=password
export PG_HOST=ip_address
export PG_PORT=port
export PG_DATABASE=db_name

export DATABASE_URL=postgres://$PG_USER:$PG_PASSWORD@$PG_HOST:$PG_PORT/$PG_DATABASE
diesel migration run
```

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