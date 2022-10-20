FROM node:latest

WORKDIR /app

COPY package.json .
RUN npm install

# Some files are ignored by .dockerignore
COPY . .
RUN npx prisma generate
