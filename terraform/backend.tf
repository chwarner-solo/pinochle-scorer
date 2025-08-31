terraform {
  backend "gcs" {
    bucket = "pinochle-scorerer-tfstate"
  }
}
