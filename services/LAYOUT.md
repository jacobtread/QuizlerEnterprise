# Architecture 

## Services

* Nginx - Sits in-front of all the services

* Frontend - User facing services
  * Hub - Management dashboard for creating, customizing, viewing analytics etc
  * Play - Used to play/join quizzes
* Backend - Backend services
  * Main Node - Main control service, contains the API and core logic
  * Play Node - Real-time quiz node that play clients use for real-time functionality
  * Analytics - Analytics ingest and management service
* Database - PostgresSQL + Timeseriesdb plugin
* Tracing - Jaeger 

Backend portions can communicate via tonic/gRPC