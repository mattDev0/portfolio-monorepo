# Stage 1: Build the Spring Boot application
FROM eclipse-temurin:25-jdk-jammy AS builder
WORKDIR /app
COPY .mvn/ .mvn/
COPY mvnw pom.xml ./
# Make the wrapper executable (just in case)
RUN chmod +x mvnw

# Download dependencies alone first, so this layer stays cached until pom.xml
# changes.
# hadolint ignore=DL3059
RUN ./mvnw -B dependency:go-offline

# Copy the source code and build
COPY src ./src
RUN ./mvnw -B package -DskipTests && \
    cp target/*.jar /app/app.jar

# Stage 2: Create the lightweight runner image
FROM eclipse-temurin:25-jre-jammy
WORKDIR /app

# Pick up Ubuntu security fixes that land before a new base image is published
RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/*

# Create a non-root user and group with numeric UID/GID
RUN groupadd -g 10001 -r appuser && useradd -u 10001 -m -r -g appuser appuser

# Copy the built jar and assign ownership to appuser
COPY --from=builder --chown=appuser:appuser /app/app.jar app.jar

# Switch to the non-root user (using numeric ID for Kubernetes runAsNonRoot verification)
USER 10001

EXPOSE 8081
CMD ["java", "-jar", "app.jar"]