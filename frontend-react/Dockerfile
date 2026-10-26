# Stage 1: Build the React application
FROM node:24-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

# Stage 2: Serve with Nginx (Unprivileged)
FROM nginxinc/nginx-unprivileged:alpine
USER 0
RUN apk upgrade --no-cache && rm /etc/nginx/conf.d/default.conf
USER 101
# Copy custom Nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf
# Copy the built assets from the builder stage to Nginx's web folder
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]