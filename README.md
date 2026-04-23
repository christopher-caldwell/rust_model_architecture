## Overview

This project demonstrates a **modular application architecture** where core logic is completely separated from external concerns like transport layers and infrastructure.

The goal is simple: **prove that you can change how the app is exposed or what it depends on without touching the core logic**.

---

## Purpose

- Show how to structure an app using **ports and adapters**
- Keep business logic independent of frameworks, databases, and APIs
- Allow multiple interfaces (HTTP, GraphQL, gRPC) on top of the same core
- Make implementations (e.g., providers, persistence) swappable via DI

---

## Architecture

### Domain

Core business models and rules. No external dependencies.

### Application

Defines use cases through **commands and queries**, plus the interfaces (ports) it needs.

### Adapters

Concrete implementations of those interfaces (DB, APIs, schema registries, etc).

### Transport

Entry points like HTTP, GraphQL, or gRPC. These just map requests to the application layer.

---

## Key Idea

All dependencies point inward.

- Core doesn’t know about infrastructure
- Infrastructure depends on the core
- You can swap implementations without breaking the system

---

## Why It Matters

This structure keeps the codebase:

- Easy to change
- Easy to test
- Not tied to any framework or vendor

---

## Summary

This is a reference implementation of a **clean, hexagonal architecture** where:

- The core is stable
- The edges are replaceable
- Adding new protocols or providers doesn’t require rewriting the app
