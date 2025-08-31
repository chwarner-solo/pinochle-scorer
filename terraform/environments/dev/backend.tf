terraform {
  backend "gcs" {
    bucket = "pinochle-scorerer-tfstate"
    prefix = "env/dev"
  }
}