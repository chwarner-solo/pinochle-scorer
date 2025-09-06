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