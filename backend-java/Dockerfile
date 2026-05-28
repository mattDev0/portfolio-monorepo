# Stage 1: Build the Spring Boot application
FROM eclipse-temurin:21-jdk-jammy AS builder
WORKDIR /app
COPY .mvn/ .mvn/
COPY mvnw pom.xml ./
# Make the wrapper executable (just in case)
RUN chmod +x mvnw

# Copy the source code
COPY src ./src

# Build and package the application using BuildKit cache mount
RUN --mount=type=cache,target=/root/.m2 \
    ./mvnw package -DskipTests && \
    cp target/*.jar /app/app.jar

# Stage 2: Create the lightweight runner image
FROM eclipse-temurin:21-jre-jammy
WORKDIR /app
COPY --from=builder /app/app.jar app.jar

EXPOSE 8081
CMD ["java", "-jar", "app.jar"]