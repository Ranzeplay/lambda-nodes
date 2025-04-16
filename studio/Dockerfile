# Stage 1: Build
FROM node:20-alpine3.20 AS builder

# Set working directory
WORKDIR /app

# Enable corepack
RUN corepack enable && corepack prepare pnpm@latest --activate

# Install dependencies
COPY package.json pnpm-lock.yaml ./
RUN pnpm install

# Copy the rest of the application code
COPY . .

# Build the project
RUN pnpm build

# Stage 2: Serve with nginx
FROM nginx:alpine

# Copy built client files to nginx's default html directory
COPY --from=builder /app/build/client /usr/share/nginx/html

# Expose port 80
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
