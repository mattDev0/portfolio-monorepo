# Stage 1: Build the Spring Boot application
FROM eclipse-temurin:21-jdk-jammy AS builder
WORKDIR /app
COPY .mvn/ .mvn/
COPY mvnw pom.xml ./
# Make the wrapper executable (just in case)
RUN chmod +x mvnw
# Download dependencies
RUN ./mvnw dependency:go-offline
# Copy the source code and package it
COPY src ./src
RUN ./mvnw clean package -DskipTests

# Stage 2: Create the lightweight runner image
FROM eclipse-temurin:21-jre-jammy
WORKDIR /app
COPY --from=builder /app/target/*.jar app.jar

EXPOSE 8081
CMD ["java", "-jar", "app.jar"]