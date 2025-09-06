# terraform/outputs.tf

# ============================================================================
# Application URLs - Main endpoints users will access
# ============================================================================
output "application_url" {
  description = "Main application URL (frontend with API routing)"
  value       = module.load_balancer.frontend_url
}

output "direct_api_url" {
  description = "Direct CloudRUN API URL (for development/testing)"
  value       = module.cloudrun.service_url
}

output "frontend_ip" {
  description = "Global static IP address for the load balancer"
  value       = module.load_balancer.frontend_ip
}

# ============================================================================
# GitHub Actions Setup - Copy these values to GitHub secrets
# ============================================================================
output "github_secrets" {
  description = "Values needed for GitHub repository secrets (copy to GitHub settings)"
  value = {
    # Required for all workflows
    GCP_PROJECT_ID        = var.project
    GCP_REGION           = var.region
    CLOUDRUN_SERVICE_NAME = module.cloudrun.service_name

    # Workload Identity Federation (no service account keys needed!)
    WIF_PROVIDER         = module.iam.wif_provider_name
    WIF_SERVICE_ACCOUNT  = module.iam.wif_service_account_email

    # Artifact Registry and Storage
    ARTIFACT_REGISTRY_REPO = module.artifact_registry.repository_name
    GCS_BUCKET_NAME        = var.bucket_name
  }
}

# ============================================================================
# Service Information - For monitoring and management
# ============================================================================
output "services" {
  description = "Information about deployed services"
  value = {
    frontend = {
      bucket_name = module.frontend.bucket_name
      bucket_url  = "gs://${module.frontend.bucket_name}"
    }
    api = {
      service_name     = module.cloudrun.service_name
      service_url      = module.cloudrun.service_url
      location         = module.cloudrun.location
      service_account  = module.cloudrun.service_account_email
    }
    load_balancer = {
      ip_address = module.load_balancer.frontend_ip
      url_map_id = module.load_balancer.url_map_id
    }
  }
}

# ============================================================================
# Infrastructure Information - For debugging and management
# ============================================================================
output "infrastructure" {
  description = "Core infrastructure information"
  value = {
    project = var.project
    region  = var.region

    network = {
      vpc_name           = module.vpc.vpc_name
      vpc_connector_name = module.vpc_connector.connector_name
    }

    database = {
      connection_name = module.cloudsql.instance_connection_name
    }
  }
  sensitive = true  # Contains database connection info
}

# ============================================================================
# Useful Commands - Copy/paste ready commands for management
# ============================================================================
output "useful_commands" {
  description = "Useful commands for managing your deployment"
  value = {
    # Deployment Management
    deploy_rust_app = "# Use GitHub Actions or run: gcloud run deploy ${module.cloudrun.service_name} --image=IMAGE_URL --region=${var.region}"
    deploy_react_app = "# Use GitHub Actions or run: gsutil -m rsync -r -d ./build gs://${module.frontend.bucket_name}"

    # Monitoring & Debugging
    view_api_logs = "gcloud run services logs read ${module.cloudrun.service_name} --region=${var.region} --limit=100"
    describe_api = "gcloud run services describe ${module.cloudrun.service_name} --region=${var.region}"

    # Traffic Management
    rollback_script = "chmod +x scripts/rollback.sh && ./scripts/rollback.sh"
    traffic_split = "gcloud run services update-traffic ${module.cloudrun.service_name} --region=${var.region} --to-revisions=REVISION_1=50,REVISION_2=50"

    # Bucket Management
    sync_frontend = "gsutil -m rsync -r -d ./build gs://${module.frontend.bucket_name}"
    view_bucket = "gsutil ls gs://${module.frontend.bucket_name}"

    # URL Testing
    test_frontend = "curl -I ${module.load_balancer.frontend_url}"
    test_api = "curl -I ${module.load_balancer.frontend_url}/api/health"
    test_direct_api = "curl -I ${module.cloudrun.service_url}/api/health"
  }
}

# ============================================================================
# Setup Instructions
# ============================================================================
output "setup_instructions" {
  description = "Next steps to complete your setup"
  value = {
    step_1 = "Copy the 'github_secrets' values above to your GitHub repository secrets"
    step_2 = "Artifact Registry repository is automatically created: ${module.artifact_registry.repository_name}"
    step_3 = "ARTIFACT_REGISTRY_REPO is automatically set in GitHub secrets output"
    step_4 = "Push your code to trigger the deployment pipeline"
    step_5 = "Monitor deployment: ${module.load_balancer.frontend_url}"

    github_secrets_path = "GitHub → Your Repository → Settings → Secrets and variables → Actions → Repository secrets"
    documentation = "See README.md for detailed setup instructions"
  }
}

# ============================================================================
# DNS Configuration (if using custom domain)
# ============================================================================
output "dns_configuration" {
  description = "DNS configuration for custom domain (if domain_name is set)"
  value = var.domain_name != null ? {
    domain = var.domain_name
    dns_instructions = "Create an A record pointing ${var.domain_name} to ${module.load_balancer.frontend_ip}"
    verification_command = "nslookup ${var.domain_name}"
  } : null
}