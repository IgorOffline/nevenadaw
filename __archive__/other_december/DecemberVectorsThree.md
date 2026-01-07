[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)

# Database credentials
POSTGRES_PASSWORD=postgres
POSTGRES_USER=postgres
POSTGRES_DB=vectordb

# Optional: Add any other environment variables
# OLLAMA_HOST=0.0.0.0
# OLLAMA_KEEP_ALIVE=24h

[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)

services:
  postgres:
    image: pgvector/pgvector:pg17
    container_name: vector_postgres
    environment:
      POSTGRES_DB: vectordb
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    env_file:
      - .env
    ports:
      - "5434:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql
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
    entrypoint: [ "/bin/sh", "-c" ]
    command: 
      - |
        ollama serve & 
        sleep 5; 
        ollama pull all-minilm; 
        wait
    healthcheck:
      test: [ "CMD", "curl", "-f", "http://localhost:11434/api/health" ]
      interval: 10s
      timeout: 5s
      retries: 10
      start_period: 60s
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 8G
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

[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS vector_store (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    content text,
    metadata jsonb,
    embedding vector(384)
);

CREATE INDEX ON vector_store USING hnsw (embedding vector_cosine_ops);

[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)

<?xml version="1.0" encoding="UTF-8"?>
<project xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://maven.apache.org/POM/4.0.0"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
  <modelVersion>4.0.0</modelVersion>
  <parent>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-parent</artifactId>
    <version>3.5.9</version>
    <relativePath/>
  </parent>
  <groupId>igoroffline.practice</groupId>
  <artifactId>vectorthree</artifactId>
  <version>0.1.0</version>
  <name>vectorthree</name>
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


[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)

spring:
  application:
    name: vectorthree
  main:
    web-application-type: none
  datasource:
    url: jdbc:postgresql://localhost:5434/vectordb
    username: postgres
    password: postgres
    driver-class-name: org.postgresql.Driver
    hikari:
      maximum-pool-size: 10
      minimum-idle: 5
      connection-timeout: 30000
  data:
    jdbc:
      repositories:
        enabled: true
  ai:
    ollama:
      embedding:
        options:
          model: all-minilm
      base-url: http://localhost:11435

server:
  port: 8080
  servlet:
    context-path: /

[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)

package igoroffline.practice.vectorthree;

import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;

@SpringBootTest
class VectorthreeApplicationTests {

  @Test
  void contextLoads() {
  }

}


[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)

package igoroffline.practice.vectorthree;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class VectorthreeApplication {

  public static void main(String[] args) {
    SpringApplication.run(VectorthreeApplication.class, args);
  }
}


[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)

package igoroffline.practice.vectorthree.main;

public record Book(String title, String author, String description) {
}


[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)

package igoroffline.practice.vectorthree.main;

import jakarta.annotation.PostConstruct;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;

@Service
public class BookInit {

  private final static Logger log = LoggerFactory.getLogger(BookInit.class);

  private final BookRunner bookRunner;

  public BookInit(BookRunner bookRunner) {
    this.bookRunner = bookRunner;
  }

  @PostConstruct
  void bookPostConstruct() {
    //bookRunner.add();
    log.info("bookRunnerCount={}", bookRunner.count());
  }
}


[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)

package igoroffline.practice.vectorthree.main;

import org.springframework.ai.document.Document;
import org.springframework.ai.vectorstore.SearchRequest;
import org.springframework.ai.vectorstore.VectorStore;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Map;

@Service
public class BookRunner {

  private final VectorStore vectorStore;

  public BookRunner(VectorStore vectorStore) {
    this.vectorStore = vectorStore;
  }

  public int count() {
    List<Document> results = vectorStore.similaritySearch(
      SearchRequest.builder().query("novel").topK(10).build()
    );

    return results.size();
  }

  public void add() {
    var books = List.of(
      new Book("The Great Gatsby", "F. Scott Fitzgerald", "The Great Gatsby is a 1925 novel by American writer F. Scott Fitzgerald. Set in the Jazz Age on Long Island, near New York City, the novel depicts first-person narrator Nick Carraway's interactions with mysterious millionaire Jay Gatsby and Gatsby's obsession to reunite with his former lover, Daisy Buchanan."),
      new Book("To Kill a Mockingbird", "Harper Lee", "To Kill a Mockingbird is a novel by the American author Harper Lee. It was published in 1960 and was instantly successful. In the United States, it is widely read in high schools and middle schools."),
      new Book("1984", "George Orwell", "Nineteen Eighty-Four: A Novel, often referred to as 1984, is a dystopian social science fiction novel by the English novelist George Orwell. It was published on 8 June 1949 by Secker & Warburg as Orwell's ninth and final book completed in his lifetime."),
      new Book("The Catcher in the Rye", "J. D. Salinger", "The Catcher in the Rye is a novel by J. D. Salinger, partially published in serial form in 1945–1946 and as a novel in 1951. It was originally intended for adults but is often read by adolescents for its themes of angst, alienation, and as a critique on superficiality in society."),
      new Book("Lord of the Flies", "William Golding", "Lord of the Flies is a 1954 novel by Nobel Prize-winning British author William Golding. The book focuses on a group of British")
    );

    List<Document> documents = books.stream()
      .map(book -> {
        String content = String.format("Title: %s. Author: %s. Description: %s",
          book.title(), book.author(), book.description());

        return new Document(content, Map.of("author", book.author()));
      })
      .toList();

    vectorStore.add(documents);
  }
}

[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
[//]: # (DecemberVectorsThree.md)
