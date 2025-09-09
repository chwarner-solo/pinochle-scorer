resource "google_compute_global_address" "frontend_ip" {
  project = var.project
  name = "fontend-ip"
}

resource "google_compute_backend_bucket" "frontend_backend" {
  project = var.project
  name = "frontend-backend"
  description = "Backend bucket for REACT frontend"
  bucket_name = var.frontend_bucket_name

  dynamic "cdn_policy" {
    for_each = var.enable_cdn ? [1] : []
    content {
      cache_mode = "CACHE_ALL_STATIC"
      default_ttl = 3600
      max_ttl = 86400
      negative_caching = true
      serve_while_stale = 86400
      signed_url_cache_max_age_sec = 7200
    }
  }
}

resource "google_compute_region_network_endpoint_group" "api_neg" {
  project = var.project
  name = "api-neg"
  network_endpoint_type = "SERVERLESS"
  region = var.region

  cloud_run {
    service = var.cloudrun_service_name
  }
}

resource "google_compute_backend_service" "api_backend" {
  project = var.project
  name = "api-backend-service"
  description = "Backend service for Rust API"
  protocol = "HTTP"
  timeout_sec = 30

  backend {
    group = google_compute_region_network_endpoint_group.api_neg.id
  }

  # Enable logging for debugging API requests
  log_config {
    enable = true
    sample_rate = 1.0  # Log all requests for debugging
  }
}

resource "google_compute_url_map" "main_url_map" {
  project = var.project
  name = "main-url-map"
  default_service = google_compute_backend_bucket.frontend_backend.id

  path_matcher {
    name = "main-matcher"
    default_service = google_compute_backend_bucket.frontend_backend.id

    path_rule {
      paths = ["/api", "/api/*"]
      service = google_compute_backend_service.api_backend.id
    }

    # Static assets - cached longer
    path_rule {
      paths   = ["/static/*", "/assets/*", "/favicon.ico"]
      service = google_compute_backend_bucket.frontend_backend.id
    }

    # All other paths (SPA routes) serve from frontend  
    # Note: /* will be handled by default_service, no need for explicit rule
  }

  host_rule {
    hosts = ["*"]
    path_matcher = "main-matcher"
  }
}

resource "google_compute_managed_ssl_certificate" "main_cert" {
  count = var.domain_name != null ? 1 : 0
  project = var.project
  name = "main-ssl-cert"

  managed {
    domains = [var.domain_name]
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "google_compute_url_map" "http_redirect_url_map" {
  project = var.project
  name = "https-redirect-url-map"

  default_url_redirect {
    https_redirect = var.ssl_redirect
    strip_query = false
    redirect_response_code = "MOVED_PERMANENTLY_DEFAULT"
  }
}

resource "google_compute_target_http_proxy" "main_http_proxy" {
  project = var.project
  name = "main-http-proxy"
  url_map = var.domain_name != null ? google_compute_url_map.http_redirect_url_map.id : google_compute_url_map.main_url_map.id
}
resource "google_compute_target_https_proxy" "main_https_proxy" {
  count = var.domain_name != null ? 1 : 0
  project = var.project
  name = "main-https-proxy"
  url_map = google_compute_url_map.main_url_map.id
  ssl_certificates = [google_compute_managed_ssl_certificate.main_cert[0].id]
}

resource "google_compute_global_forwarding_rule" "main_http" {
  project = var.project
  name = "main-http-forwarding-rule"
  target = google_compute_target_http_proxy.main_http_proxy.id
  port_range = "80"
  ip_address = google_compute_global_address.frontend_ip.address
}

resource "google_compute_global_forwarding_rule" "main_https" {
  count = var.domain_name != null ? 1 : 0
  name   = "main-https-fowarding-rule"
  target = google_compute_target_https_proxy.main_https_proxy[0].id
  port_range = "443"
  ip_address = google_compute_global_address.frontend_ip.address
}
