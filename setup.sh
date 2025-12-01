#!/bin/bash
mkdir -p src/{handlers,services,middleware,models,db,utils}
mkdir -p migrations
mkdir -p scripts
touch src/{main.rs,config.rs}
touch .env.example .gitignore Dockerfile docker-compose.yml
echo "target/
**/*.rs.bk
.env
.env.local
*.log" > .gitignore
