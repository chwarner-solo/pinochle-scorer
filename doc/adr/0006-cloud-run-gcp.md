# ADR 6: Deploying to Cloud Run on GCP

## Status
Accepted

## Context
We want a scalable, managed, and cost-effective deployment solution for the application.

## Decision
The application will be containerized and deployed to Google Cloud Run. This enables easy scaling, managed infrastructure, and integration with other GCP services.

## Consequences
- Simplified deployment and scaling
- Lower operational overhead
- Ties deployment to GCP; migration to another provider would require changes
