# terraform/versions.tf - GOOGLE PROVIDER ONLY (no beta)

terraform {
  required_version = ">= 1.0"

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 6.49"  # Latest stable series
    }

    random = {
      source  = "hashicorp/random"
      version = "~> 3.0"
    }

    time = {
      source  = "hashicorp/time"
      version = "~> 0.9"
    }

    # NO GOOGLE-BETA! We don't need it!
  }

  backend "gcs" {
    bucket = "pinochle-scorer-471315-tfstate"
  }
}


# Configure the Google Provider
provider "google" {
  project = var.project
  region  = var.region
}