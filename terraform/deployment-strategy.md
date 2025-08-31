# CloudRun Deployment Strategies

## Option 1: Terraform + Image Tags (Recommended)

### Infrastructure Setup (One-time)
```bash
# Deploy base infrastructure
terraform apply

# Your infrastructure is now ready with:
# - Artifact Registry: us-central1-docker.pkg.dev/pinochle-scorerer/pinochle-scorer-registry
# - CloudRun service: Running hello container initially
# - Load balancer: Ready to route traffic
```

### Application Deployment Process
```bash
# 1. Build and push your Rust container
docker build -t us-central1-docker.pkg.dev/pinochle-scorerer/pinochle-scorer-registry/pinochle-scorer:v1.0.0 .
docker push us-central1-docker.pkg.dev/pinochle-scorerer/pinochle-scorer-registry/pinochle-scorer:v1.0.0

# 2. Update CloudRun via terraform
terraform apply -var="app_version=v1.0.0"

# 3. For blue/green deployment:
# - Deploy new version with zero traffic
# - Gradually shift traffic 0% → 50% → 100%
# - Rollback instantly if issues
```

## Option 2: GitHub Actions + CloudRun Direct Deploy

### Separate Infrastructure from App Deployment
- **Terraform**: Manages infrastructure (databases, networking, base services)
- **GitHub Actions**: Handles app deployments with blue/green

### GitHub Actions Workflow:
```yaml
name: Deploy API
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Build Container
        run: |
          docker build -t ${{ env.REGISTRY_URL }}/pinochle-scorer:${{ github.sha }} .
          docker push ${{ env.REGISTRY_URL }}/pinochle-scorer:${{ github.sha }}
      
      - name: Deploy to CloudRun (Blue/Green)
        run: |
          # Deploy new revision with 0% traffic
          gcloud run deploy pinochle-scorer-api \
            --image ${{ env.REGISTRY_URL }}/pinochle-scorer:${{ github.sha }} \
            --no-traffic \
            --tag blue
          
          # Gradually shift traffic
          gcloud run services update-traffic pinochle-scorer-api \
            --to-revisions blue=10
          
          # Health check, then full cutover
          gcloud run services update-traffic pinochle-scorer-api \
            --to-revisions blue=100
```

## Option 3: CloudRun Revisions (Built-in Blue/Green)

CloudRun has native blue/green deployment:

```bash
# Deploy new revision without traffic
gcloud run deploy pinochle-scorer-api \
  --image us-central1-docker.pkg.dev/pinochle-scorerer/pinochle-scorer-registry/pinochle-scorer:v2.0.0 \
  --no-traffic \
  --tag canary

# Split traffic between versions
gcloud run services update-traffic pinochle-scorer-api \
  --to-revisions pinochle-scorer-api-00001-abc=80,pinochle-scorer-api-00002-def=20

# Full cutover
gcloud run services update-traffic pinochle-scorer-api \
  --to-latest

# Instant rollback
gcloud run services update-traffic pinochle-scorer-api \
  --to-revisions pinochle-scorer-api-00001-abc=100
```

## Recommended Approach

1. **Use Terraform for infrastructure** (what we've built)
2. **Use GitHub Actions for app deployment** 
3. **Leverage CloudRun's built-in traffic management**

This separates concerns cleanly:
- Infrastructure changes are rare and deliberate
- App deployments are frequent and automated
- Blue/green deployments happen at the CloudRun level