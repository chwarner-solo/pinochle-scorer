# 🚀 Post-Deployment Setup Checklist

After running `terraform apply`, follow these steps to complete your deployment setup.

## ✅ 1. Grab Your GitHub Secrets

Run this command to get all the secret values:
```bash
terraform output github_secrets
```

Copy each value to your **GitHub Repository → Settings → Secrets and variables → Actions → Repository secrets**:

| GitHub Secret Name | Description |
|-------------------|-------------|
| `GCP_PROJECT_ID` | Your GCP project ID |
| `GCP_REGION` | Your deployment region |
| `CLOUDRUN_SERVICE_NAME` | Your CloudRUN service name |
| `WIF_PROVIDER` | Workload Identity Federation provider |
| `WIF_SERVICE_ACCOUNT` | Service account for GitHub Actions |
| `ARTIFACT_REGISTRY_REPO` | Your Docker repository name |
| `GCS_BUCKET_NAME` | Frontend bucket suffix |

## ✅ 2. Create Artifact Registry Repository

```bash
# Replace 'your-repo-name' with your preferred name
gcloud artifacts repositories create your-repo-name \
  --repository-format=docker \
  --location=us-central1 \
  --project=your-project-id
```

Then update your GitHub secret `ARTIFACT_REGISTRY_REPO` with `your-repo-name`.

## ✅ 3. Verify Your Infrastructure

Check that everything deployed correctly:
```bash
# Get all useful commands
terraform output useful_commands

# Test your endpoints
curl -I $(terraform output -raw application_url)
curl -I $(terraform output -raw application_url)/api/health
```

## ✅ 4. Deploy Your Applications

### Option A: Push to GitHub (Recommended)
```bash
git add .
git commit -m "Initial deployment setup"
git push origin main
```

The GitHub Actions will automatically:
- Build and test your Rust API
- Deploy using blue/green strategy
- Build and deploy your React frontend

### Option B: Manual Deployment (for testing)
```bash
# Deploy Rust API manually
docker build -t your-image .
docker push your-image
gcloud run deploy your-service --image=your-image

# Deploy React frontend manually
npm run build
gsutil -m rsync -r -d ./build gs://your-bucket
```

## ✅ 5. Monitor Your Deployment

```bash
# Watch GitHub Actions
# Go to: GitHub → Your Repository → Actions

# Check CloudRUN logs
terraform output -raw useful_commands | grep view_api_logs

# Test the application
open $(terraform output -raw application_url)
```

## ✅ 6. Set Up Custom Domain (Optional)

If you set `domain_name` in your `terraform.tfvars`:

```bash
# Get DNS configuration instructions
terraform output dns_configuration
```

Add an A record in your DNS provider pointing your domain to the static IP.

## 🎯 Deployment Strategies Available

Your setup supports multiple deployment strategies:

### 🐤 Canary Deployment (Default)
```bash
# Triggered automatically on push to main
# Deploys to 10% traffic, then promotes to 100%
```

### 🔄 Blue/Green Deployment
```bash
# Manual trigger with workflow_dispatch
# Instant switch with monitoring
```

### ⚡ Direct Deployment
```bash
# For development/testing
# Immediate deployment to 100% traffic
```

### 🔄 Manual Rollback
```bash
chmod +x scripts/rollback.sh
./scripts/rollback.sh
```

## 🚨 Troubleshooting

### GitHub Actions Failing?
1. Check that all secrets are set correctly
2. Verify Artifact Registry repository exists
3. Ensure your Dockerfile is in the repository root
4. Check that your Rust binary name matches the Dockerfile

### API Health Checks Failing?
1. Ensure your Rust app has a `/api/health` endpoint
2. Check that your app listens on `$PORT` environment variable
3. Verify your app starts within 10 seconds

### Frontend Not Loading?
1. Check that React build creates a `build/` directory
2. Verify the bucket is publicly readable
3. Ensure your React router is configured for history mode

### Database Connection Issues?
1. Check VPC connector is deployed
2. Verify CloudSQL instance is in the same VPC
3. Ensure service account has CloudSQL client role

## 📊 Useful Commands

```bash
# View all terraform outputs
terraform output

# Get application URLs
terraform output application_url
terraform output direct_api_url

# View logs
gcloud run services logs read $(terraform output -raw cloudrun_service_name) --region=$(terraform output -raw infrastructure.region) --limit=50

# Describe service
gcloud run services describe $(terraform output -raw cloudrun_service_name) --region=$(terraform output -raw infrastructure.region)

# Manual rollback
./scripts/rollback.sh

# Test endpoints
curl $(terraform output -raw application_url)/api/health
curl $(terraform output -raw direct_api_url)/api/health
```

## 🎉 You're Done!

Your production-ready Rust + React application is now deployed with:

- ✅ Blue/Green deployment capabilities
- ✅ Auto-scaling CloudRUN API
- ✅ CDN-enabled React frontend
- ✅ Secure Workload Identity Federation
- ✅ Database connectivity via VPC
- ✅ SSL termination and custom domain support
- ✅ Comprehensive monitoring and logging
- ✅ One-click rollback capabilities

Navigate to your application URL and start building! 🚀