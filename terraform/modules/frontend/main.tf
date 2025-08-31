# Create a Cloud Storage bucket for hosting the React frontend
resource "google_storage_bucket" "frontend" {
  project  = var.project
  name     = "${var.project}-${var.bucket_name}"
  location = var.region

  # Enable website hosting
  website {
    main_page_suffix = "index.html"
    not_found_page   = "index.html" # For React Router SPA support
  }

  # Enable uniform bucket-level access
  uniform_bucket_level_access = true

  # Configure CORS for API calls
  cors {
    origin          = ["*"]  # Will be restricted to actual domain in production
    method          = ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
    response_header = ["*"]
    max_age_seconds = 3600
  }

  # Lifecycle rules to manage old versions
  lifecycle_rule {
    condition {
      age = 30
    }
    action {
      type = "Delete"
    }
  }
}

# Make the bucket publicly readable
resource "google_storage_bucket_iam_member" "frontend_public" {
  bucket = google_storage_bucket.frontend.name
  role   = "roles/storage.objectViewer"
  member = "allUsers"
}

# Reserve a global static IP for the load balancer
resource "google_compute_global_address" "frontend_ip" {
  project = var.project
  name    = "frontend-ip"
}

# Create a backend bucket for the load balancer
resource "google_compute_backend_bucket" "frontend_backend" {
  project     = var.project
  name        = "frontend-backend"
  description = "Backend bucket for React frontend"
  bucket_name = google_storage_bucket.frontend.name

  dynamic "cdn_policy" {
    for_each = var.enable_cdn ? [1] : []
    content {
      cache_mode                   = "CACHE_ALL_STATIC"
      default_ttl                  = 3600
      max_ttl                      = 86400
      negative_caching             = true
      serve_while_stale            = 86400
      signed_url_cache_max_age_sec = 7200
    }
  }
}

# Backend service for API (CloudRun) - No health checks for serverless
resource "google_compute_backend_service" "api_backend" {
  project     = var.project
  name        = "api-backend-service"
  description = "Backend service for Rust API"
  protocol    = "HTTP"
  timeout_sec = 30

  backend {
    group = "projects/${var.project}/regions/${var.region}/networkEndpointGroups/api-neg"
  }
}

# Health check removed - not needed for serverless backends

# Network Endpoint Group for CloudRun API
resource "google_compute_region_network_endpoint_group" "api_neg" {
  project               = var.project
  name                  = "api-neg"
  network_endpoint_type = "SERVERLESS"
  region                = var.region

  cloud_run {
    service = "pinochle-scorer-api"
  }
}

# URL map for the load balancer with both frontend and API
resource "google_compute_url_map" "main_url_map" {
  project         = var.project
  name            = "main-url-map"
  default_service = google_compute_backend_bucket.frontend_backend.id

  # API routes go to CloudRun
  path_matcher {
    name            = "main-matcher"
    default_service = google_compute_backend_bucket.frontend_backend.id

    # API routes - all /api/* requests go to CloudRun
    path_rule {
      paths   = ["/api", "/api/*"]
      service = google_compute_backend_service.api_backend.id
    }

    # Static assets - can be cached longer
    path_rule {
      paths   = ["/static/*", "/assets/*", "/favicon.ico"]
      service = google_compute_backend_bucket.frontend_backend.id
    }

    # All other paths (SPA routes) serve index.html from frontend
    path_rule {
      paths   = ["/*"]
      service = google_compute_backend_bucket.frontend_backend.id
    }
  }

  host_rule {
    hosts        = ["*"]
    path_matcher = "main-matcher"
  }
}

# Google-managed SSL certificate (for custom domains)
resource "google_compute_managed_ssl_certificate" "main_cert" {
  count   = var.domain_name != null ? 1 : 0
  project = var.project
  name    = "main-ssl-cert"

  managed {
    domains = [var.domain_name]
  }

  lifecycle {
    create_before_destroy = true
  }
}

# HTTPS target proxy (only for custom domains)
resource "google_compute_target_https_proxy" "main_https_proxy" {
  count   = var.domain_name != null ? 1 : 0
  project = var.project
  name    = "main-https-proxy"
  url_map = google_compute_url_map.main_url_map.id
  ssl_certificates = [google_compute_managed_ssl_certificate.main_cert[0].id]
}

# HTTP target proxy (for redirect to HTTPS)
resource "google_compute_target_http_proxy" "main_http_proxy" {
  project = var.project
  name    = "main-http-proxy"
  url_map = google_compute_url_map.https_redirect_url_map.id
}

# URL map for HTTP to HTTPS redirect
resource "google_compute_url_map" "https_redirect_url_map" {
  project = var.project
  name    = "https-redirect-url-map"

  default_url_redirect {
    https_redirect         = true
    redirect_response_code = "MOVED_PERMANENTLY_DEFAULT"
    strip_query            = false
  }
}

# Global forwarding rules
resource "google_compute_global_forwarding_rule" "main_https" {
  count      = var.domain_name != null ? 1 : 0
  project    = var.project
  name       = "main-https-forwarding-rule"
  target     = google_compute_target_https_proxy.main_https_proxy[0].id
  port_range = "443"
  ip_address = google_compute_global_address.frontend_ip.address
}

resource "google_compute_global_forwarding_rule" "main_http" {
  project    = var.project
  name       = "main-http-forwarding-rule"
  target     = google_compute_target_http_proxy.main_http_proxy.id
  port_range = "80"
  ip_address = google_compute_global_address.frontend_ip.address
}

# Service Account for CI/CD to upload frontend files
resource "google_service_account" "frontend_deploy_sa" {
  project      = var.project
  account_id   = "frontend-deploy-sa"
  display_name = "Service Account for Frontend Deployment"
}

resource "google_project_iam_member" "frontend_deploy_storage" {
  project = var.project
  role    = "roles/storage.objectAdmin"
  member  = "serviceAccount:${google_service_account.frontend_deploy_sa.email}"

  condition {
    title       = "Frontend bucket access only"
    description = "Restrict access to frontend bucket only"
    expression  = "resource.name.startsWith(\"projects/_/buckets/${google_storage_bucket.frontend.name}\")"
  }
}