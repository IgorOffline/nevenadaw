services:
  postgres:
    image: pgvector/pgvector:pg17
    container_name: vector_postgres
    environment:
      POSTGRES_DB: vectordb
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    env_file:
      - .env  # For production secrets
    ports:
      - "5434:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - ai_network
    healthcheck:
      test: [ "CMD-SHELL", "pg_isready -U postgres" ]
      interval: 10s
      timeout: 5s
      retries: 5
      start_period: 30s
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 2G
        reservations:
          memory: 1G

  ollama:
    image: ollama/ollama:latest
    container_name: vector_ollama
    ports:
      - "11435:11434"
    volumes:
      - ollama_data:/root/.ollama
    networks:
      - ai_network
    healthcheck:
      test: [ "CMD", "curl", "-f", "http://localhost:11435/api/health" ]
      interval: 10s
      timeout: 5s
      retries: 10
      start_period: 60s  # Ollama needs time to download models
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 8G  # Models need more memory
        reservations:
          memory: 4G

volumes:
  postgres_data:
    name: vector_postgres_data
  ollama_data:
    name: vector_ollama_data

networks:
  ai_network:
    driver: bridge
    name: vector_ai_network

# Database credentials
POSTGRES_PASSWORD=postgres
POSTGRES_USER=postgres
POSTGRES_DB=vectordb

# Optional: Add any other environment variables
# OLLAMA_HOST=0.0.0.0
# OLLAMA_KEEP_ALIVE=24h

<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
	xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
	<modelVersion>4.0.0</modelVersion>
	<parent>
		<groupId>org.springframework.boot</groupId>
		<artifactId>spring-boot-starter-parent</artifactId>
		<version>3.5.9</version>
		<relativePath/>
	</parent>
	<groupId>igoroffline.practice</groupId>
	<artifactId>DecemberVectorsInit</artifactId>
	<version>0.1.0</version>
	<name>DecemberVectorsInit</name>
	<description>Ollama, PGvector Vector Database, PostgreSQL Diver, Spring Web, Spring Data JDBC</description>
	<url/>
	<licenses>
		<license/>
	</licenses>
	<developers>
		<developer/>
	</developers>
	<scm>
		<connection/>
		<developerConnection/>
		<tag/>
		<url/>
	</scm>
	<properties>
		<java.version>21</java.version>
		<spring-ai.version>1.1.2</spring-ai.version>
	</properties>
	<dependencies>
		<dependency>
			<groupId>org.springframework.boot</groupId>
			<artifactId>spring-boot-starter-data-jdbc</artifactId>
		</dependency>
		<dependency>
			<groupId>org.springframework.boot</groupId>
			<artifactId>spring-boot-starter-web</artifactId>
		</dependency>
		<dependency>
			<groupId>org.springframework.ai</groupId>
			<artifactId>spring-ai-advisors-vector-store</artifactId>
		</dependency>
		<dependency>
			<groupId>org.springframework.ai</groupId>
			<artifactId>spring-ai-starter-model-ollama</artifactId>
		</dependency>
		<dependency>
			<groupId>org.springframework.ai</groupId>
			<artifactId>spring-ai-starter-vector-store-pgvector</artifactId>
		</dependency>

		<dependency>
			<groupId>org.postgresql</groupId>
			<artifactId>postgresql</artifactId>
			<scope>runtime</scope>
		</dependency>
		<dependency>
			<groupId>org.springframework.boot</groupId>
			<artifactId>spring-boot-starter-test</artifactId>
			<scope>test</scope>
		</dependency>
	</dependencies>
	<dependencyManagement>
		<dependencies>
			<dependency>
				<groupId>org.springframework.ai</groupId>
				<artifactId>spring-ai-bom</artifactId>
				<version>${spring-ai.version}</version>
				<type>pom</type>
				<scope>import</scope>
			</dependency>
		</dependencies>
	</dependencyManagement>

	<build>
		<plugins>
			<plugin>
				<groupId>org.springframework.boot</groupId>
				<artifactId>spring-boot-maven-plugin</artifactId>
			</plugin>
		</plugins>
	</build>

</project>

spring:
  application:
    name: DecemberVectorsInit
  datasource:
    url: jdbc:postgresql://localhost:5434/vectordb
    username: postgres
    password: postgres
    driver-class-name: org.postgresql.Driver

